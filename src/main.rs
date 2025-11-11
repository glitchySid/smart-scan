use futures::stream::{self, StreamExt}; // <-- For concurrent processing
use smart_scan::db;
use smart_scan::open_image::open;
use smart_scan::screenshot::SSData;
use smart_scan::screenshot::process_screenshot;
use std::env;
use std::io;
use walkdir::WalkDir; // <-- For error handling

fn search_ss_data(query: &str) {
    let conn = db::init_db().expect("Failed to initialize database");
    match db::query_ss_data(&conn, query) {
        Ok(results) => {
            println!("Found {} results for query '{}':", results.len(), query);
            for res in results {
                println!("- File: {}", res.file);
                open(&res.file);
            }
        }
        Err(e) => {
            eprintln!("Failed to query database: {}", e);
        }
    }
}

#[tokio::main] // <-- Add the tokio main macro
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "search" {
        if args.len() > 2 {
            search_ss_data(&args[2]);
        } else {
            println!("Please provide a search query.");
        }
        return;
    }

    // <-- Make main async
    let mut screenshots = Vec::new();

    // --- 1. Collect all files (this part is synchronous and fast) ---
    let walk_path = "/Users/siddheshmhatre/Documents/ScreenShot";
    println!("Scanning for files in {}...", walk_path);

    for file in WalkDir::new(walk_path)
        .into_iter()
        .filter_map(|file| file.ok())
    {
        if file.metadata().unwrap().is_file() {
            screenshots.push(file);
        }
    }
    println!("Found {} files to process.", screenshots.len());

    // --- 2. Initialize DB ---
    let conn = db::init_db().expect("Failed to initialize database");
    println!("Database initialized.");

    // --- 3. Process all files concurrently ---

    // Set a limit for how many processes to run at once
    // Running thousands at once will crash your system!
    const CONCURRENT_LIMIT: usize = 10;

    let results = stream::iter(screenshots) // Create a stream from the file list
        .map(process_screenshot) // Map each file to our async function
        .buffer_unordered(CONCURRENT_LIMIT) // Run up to 10 futures at a time
        .collect::<Vec<Result<SSData, io::Error>>>() // Collect all results
        .await;

    // --- 4. Collect the successful results and insert into DB ---
    let mut successful_count = 0;
    let mut failed_count = 0;

    for res in results {
        match res {
            Ok(data) => match db::insert_ss_data(&conn, &data) {
                Ok(_) => successful_count += 1,
                Err(e) => {
                    eprintln!("Failed to insert data into DB: {}", e);
                    failed_count += 1;
                }
            },
            Err(e) => {
                eprintln!("A task failed: {}", e);
                failed_count += 1;
            }
        }
    }

    println!("---");
    println!("Successfully processed and inserted: {}", successful_count);
    println!("Failed to process: {}", failed_count);
}
