# llmdoc - LLM-Powered Document Management System

`llmdoc` is a command-line application designed to manage and interact with your documents using the power of Large Language Models (LLMs). It aims to provide functionalities for document ingestion, storage, semantic search, summarization, and more, all accessible through a user-friendly CLI.

## Features (Current & Planned)

*   **Document Ingestion**: Support for various document formats (initially Markdown, plain text).
*   **Metadata Extraction**: Automatic extraction of key metadata from documents.
*   **Persistent Storage**: Uses SQLite for storing document metadata and potentially vectorized content.
*   **Schema Validation**: Ensures document metadata conforms to a defined JSON schema.
*   **Embeddings Generation**: (Planned) Generate vector embeddings for semantic search.
*   **Semantic Search**: (Planned) Find documents based on meaning, not just keywords.
*   **Summarization & Q&A**: (Planned) Leverage LLMs for document summarization and answering questions about document content.
*   **CLI Interface**: All functionalities accessible via a command-line interface.
*   **Configuration**: Customizable behavior through a `config.toml` file.
*   **Robust Logging**: Detailed logging for diagnostics and monitoring.

## Getting Started

### Prerequisites

*   **Rust**: Ensure you have Rust installed. You can get it from [rustup.rs](https://rustup.rs/).
*   **SQLite**: `llmdoc` uses SQLite for its database. Ensure SQLite is installed on your system.

### Building

1.  Clone the repository:
    ```bash
    git clone <repository_url> 
    cd llmdoc
    ```
    *(Replace `<repository_url>` with the actual URL once available)*
2.  Build the project:
    ```bash
    cargo build
    ```
    For a release build:
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/debug/llmdoc` or `target/release/llmdoc`.

### Running Tests
    ```bash
    cargo test
    ```

## Usage

`llmdoc` is a CLI application. The primary entry point is the `llmdoc` executable.

*(More detailed usage instructions and commands will be added as features are implemented.)*

Example (conceptual):
```bash
./target/debug/llmdoc add my_document.md
./target/debug/llmdoc search "concepts related to AI ethics"
```

## Configuration

`llmdoc` can be configured via a `config.toml` file located in a platform-specific configuration directory (e.g., `~/.config/llmdoc/config.toml` on Linux). If the file doesn't exist, it will be created with default values upon first run or when certain commands are executed.

Key configurable aspects include:
*   Database path
*   Log file paths and levels
*   (Future) LLM provider details and API keys
*   (Future) Embedding model preferences

## Project Structure

A brief overview of the main modules:

*   `src/main.rs`: Main application entry point, CLI argument parsing.
*   `src/lib.rs`: Library crate root, defines core modules.
*   `src/cli/`: Handles command-line interface logic.
*   `src/core/`: Core functionalities:
    *   `database.rs`: SQLite database interactions.
    *   `models.rs`: Data structures for documents, etc.
    *   `validation.rs`: JSON schema validation for document metadata.
    *   `errors.rs`: Custom error types.
*   `src/services/`: Higher-level services combining core functionalities (e.g., document service).
*   `src/app_config.rs`: Application configuration management.
*   `src/logging.rs`: Logging setup.
*   `src/embeddings/`: (Planned) Embeddings generation logic.
*   `src/export/`: (Planned) Document export functionalities.

## Contributing

Contributions are welcome! Please refer to `CONTRIBUTING.md` (to be created) for guidelines.

## License

This project is licensed under the MIT License - see the `LICENSE` file (to be created) for details.