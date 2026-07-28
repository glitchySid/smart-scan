# Smart Scan

Screenshot OCR + AI agents for semantic search and RAG.

## Features

- **OCR**: Extracts text from screenshots via `ocrs-cli`
- **Vector DB**: Stores embeddings in LanceDB (native Rust)
- **Embedding**: Uses `all-MiniLM-L6-v2` via fastembed (384-dim)
- **Search**: Semantic vector search over screenshot text
- **RAG Agent**: Answers questions using retrieved context + Ollama LLM
- **Categorize**: Classifies screenshots by content similarity

## Prerequisites

- Rust (edition 2024, rust-version 1.85)
- `ocrs-cli`: `cargo install ocrs-cli --locked`
- Python 3.13+ with `uv`
- Ollama with `llama3.2:1b`

## Installation

```bash
git clone https://github.com/your-username/smart-scan.git
cd smart-scan

# Install Rust deps
cargo build --release

# Install Python deps (agents)
uv sync --project agents
```

## Usage

```bash
# Scan directory (OCR → embed → LanceDB)
smart-scan scan /path/to/screenshots
smart-scan scan  # defaults to ~/Documents/ScreenShot

# Semantic search (native Rust)
smart-scan search "error messages in terminal"

# RAG agent (Python → Ollama)
smart-scan ask "what coding errors did I encounter today"

# Categorize screenshot
smart-scan categorize /path/to/file.png "screenshot content"
```

## Configuration

Edit `agents/.env` to override defaults:

```env
LLM_MODEL=llama3.2:1b
EMBEDDING_MODEL=all-MiniLM-L6-v2
CHUNK_SIZE=500
CHUNK_OVERLAP=50
SIMILARITY_THRESHOLD=0.3
LANCE_DIR=/path/to/lance_db
TABLE_NAME=screenshots
```

## Architecture

```
smart-scan/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── embedding.rs     # fastembed wrapper (Rust)
│   ├── vector.rs        # LanceDB insert/search (Rust)
│   ├── screenshot.rs    # OCR via ocrs-cli
│   └── python.rs        # Python subprocess helper
├── agents/
│   ├── config.py        # Shared config + LanceDB connection
│   ├── query.py         # Semantic search
│   ├── agent.py         # RAG agent (Ollama)
│   ├── categorize.py    # Embedding-based classification
│   └── .env             # Environment variables
├── lance_db/            # LanceDB data (auto-created)
└── Cargo.toml
```

### Ingest flow

```
screenshot → ocrs (Rust) → fastembed (Rust) → LanceDB (Rust)
```

No Python subprocess per file. ~10-100x faster than previous ChromaDB approach.

### Search flow

```
query → fastembed (Rust) → LanceDB vector search (Rust) → results
```

### RAG flow

```
query → sentence-transformers (Python) → LanceDB → Ollama LLM → answer
```

## Project Structure

- `src/main.rs` — CLI commands: scan, search, ask, categorize
- `src/embedding.rs` — Text embedding via fastembed
- `src/vector.rs` — LanceDB wrapper (insert, search)
- `src/screenshot.rs` — OCR processing
- `src/python.rs` — Python subprocess calls
- `agents/` — Python AI agents (search, RAG, categorize)
- `Cargo.toml` — Rust dependencies

## License

MIT
