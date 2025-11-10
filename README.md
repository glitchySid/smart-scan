# Smart Scan

Smart Scan is a Rust application that processes screenshots, extracts text using OCR, and stores the data in a SQLite database. It also provides a command-line interface to search through the extracted text.

## Features

- **Screenshot Processing**: Automatically scans a specified directory for screenshot files.
- **OCR Text Extraction**: Uses the `ocrs-cli` tool to extract text from images.
- **SQLite Database**: Stores the extracted text along with file paths and creation timestamps.
- **Search Functionality**: Allows searching the database for text within screenshots.

## Prerequisites

- Rust (latest stable version recommended)
- `ocrs-cli`: A command-line OCR tool.

## Installation

1.  **Install `ocrs-cli`**:
    ```bash
    cargo install ocrs-cli --locked
    ```

2.  **Clone the repository**:
    ```bash
    git clone https://github.com/your-username/smart-scan.git
    cd smart-scan
    ```

3.  **Build the project**:
    ```bash
    cargo build --release
    ```

## Usage

### Processing Screenshots

To process screenshots, run the application without any arguments. It will scan the `/Users/siddheshmhatre/Documents/ScreenShot` directory (this path is currently hardcoded in `src/main.rs` and can be changed there).

```bash
cargo run
```

### Searching Screenshots

To search for text within the processed screenshots, use the `search` command followed by your query:

```bash
cargo run search "your search query"
```

Example:

```bash
cargo run search "rust programming"
```

## Important Note on Screenshots

This application is currently configured to open screenshots, which **only works on macOS**. The screenshot processing logic might need adjustments for other operating systems.

## Project Structure

- `src/main.rs`: Main application logic, handles file scanning, OCR processing, and database interaction.
- `src/lib.rs`: Defines the `SSData` struct and the `process_screenshot` function.
- `src/db.rs`: Contains functions for database initialization, data insertion, and querying using `rusqlite`.
- `Cargo.toml`: Project dependencies and metadata.
- `screenshots.db`: The SQLite database file (generated after first run).

## Contributing

Feel free to contribute to this project by opening issues or submitting pull requests.

## License
MIT License

