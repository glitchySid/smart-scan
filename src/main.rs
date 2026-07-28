use futures::stream::{self, StreamExt};
use smart_scan::embedding::Embedder;
use smart_scan::python;
use smart_scan::screenshot::SSData;
use smart_scan::screenshot::process_screenshot;
use smart_scan::vector::VectorStore;
use std::env;
use std::io;
use walkdir::WalkDir;

const DEFAULT_SCAN_DIR: &str = "/Users/siddheshmhatre/Documents/ScreenShot";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "scan" => {
            let path = args.get(2).map(|s| s.as_str()).unwrap_or(DEFAULT_SCAN_DIR);
            cmd_scan(path).await;
        }
        "search" => {
            if args.len() < 3 {
                eprintln!("Usage: smart-scan search <query>");
                return;
            }
            cmd_search(&args[2]).await;
        }
        "ask" => {
            if args.len() < 3 {
                eprintln!("Usage: smart-scan ask <question>");
                return;
            }
            cmd_ask(&args[2]).await;
        }
        "categorize" => {
            if args.len() < 4 {
                eprintln!("Usage: smart-scan categorize <file> <text>");
                return;
            }
            cmd_categorize(&args[2], &args[3]).await;
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("smart-scan - Screenshot OCR + AI agents");
    println!();
    println!("Commands:");
    println!("  scan [path]                   Scan directory, OCR, ingest into vector DB");
    println!("  search <query>                Semantic search over screenshots");
    println!("  ask <question>                RAG agent: answer questions about screenshots");
    println!("  categorize <file> <text>      Categorize a screenshot");
}

async fn cmd_scan(walk_path: &str) {
    println!("Scanning for files in {}...", walk_path);

    let screenshots = WalkDir::new(walk_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.metadata().map(|m| m.is_file()).unwrap_or(false))
        .collect::<Vec<_>>();

    println!("Found {} files to process.", screenshots.len());

    let mut embedder = match Embedder::new() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to load embedding model: {}", e);
            return;
        }
    };

    let store = match VectorStore::connect("lance_db").await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to connect to LanceDB: {}", e);
            return;
        }
    };

    const CONCURRENT_LIMIT: usize = 10;

    let results = stream::iter(screenshots)
        .map(process_screenshot)
        .buffer_unordered(CONCURRENT_LIMIT)
        .collect::<Vec<Result<SSData, io::Error>>>()
        .await;

    let mut successful_count = 0;
    let mut failed_count = 0;

    for res in results {
        match res {
            Ok(data) => {
                let embedding = match embedder.embed(&[&data.content]) {
                    Ok(e) => e,
                    Err(e) => {
                        eprintln!("Failed to embed text: {}", e);
                        failed_count += 1;
                        continue;
                    }
                };

                match store.insert(&data.file, &data.content, embedding[0].clone()).await {
                    Ok(_) => successful_count += 1,
                    Err(e) => {
                        eprintln!("Failed to insert into LanceDB: {}", e);
                        failed_count += 1;
                    }
                }
            }
            Err(e) => {
                eprintln!("A task failed: {}", e);
                failed_count += 1;
            }
        }
    }

    println!("---");
    println!("Successfully processed and ingested: {}", successful_count);
    println!("Failed to process: {}", failed_count);
}

async fn cmd_search(query: &str) {
    println!("Searching for '{}'...", query);

    let mut embedder = match Embedder::new() {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to load embedding model: {}", e);
            return;
        }
    };

    let store = match VectorStore::connect("lance_db").await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to connect to LanceDB: {}", e);
            return;
        }
    };

    let query_embedding = match embedder.embed(&[query]) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to embed query: {}", e);
            return;
        }
    };

    match store.search(query_embedding[0].clone(), 5).await {
        Ok(results) => {
            for (i, result) in results.iter().enumerate() {
                println!("[{}] (score: {:.4}) {}", i + 1, result.score, result.file);
                println!("    {}", &result.content[..result.content.len().min(200)]);
                println!();
            }
        }
        Err(e) => eprintln!("Search failed: {}", e),
    }
}

async fn cmd_ask(question: &str) {
    println!("Asking: {}", question);

    match python::agent(question).await {
        Ok(json_str) => println!("{}", json_str),
        Err(e) => eprintln!("Agent failed: {}", e),
    }
}

async fn cmd_categorize(file: &str, text: &str) {
    println!("Categorizing {}...", file);

    match python::categorize(file, text).await {
        Ok(json_str) => println!("{}", json_str),
        Err(e) => eprintln!("Categorize failed: {}", e),
    }
}
