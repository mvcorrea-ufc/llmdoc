# llmdoc - LLM-Powered Documentation Management System

`llmdoc` is a command-line application designed to manage Agile development documentation using Large Language Models (LLMs). It provides comprehensive functionalities for managing tasks, sprints, ADRs (Architecture Decision Records), user stories, components, and more.

## 🚀 Current Status

✅ **Fully Implemented:**
- Complete CLI interface with subcommands
- SQLite database integration with async connection pooling
- Task management (create, read, update, delete, list)
- Sprint management with reporting capabilities
- ADR (Architecture Decision Record) management
- User Story management
- Component tracking
- Search functionality across all entities
- Export services (Markdown, JSON, HTML support)
- Markdown migration from existing documentation
- Configuration management with TOML
- Comprehensive logging system
- Error handling with custom error types
- Database schema initialization and migrations

✅ **Core Services:**
- `TaskService` - Full CRUD operations for tasks
- `SprintService` - Sprint lifecycle management with reporting
- `AdrService` - Architecture Decision Record tracking
- `UserStoryService` - User story management
- `ComponentService` - Component documentation
- `SearchService` - Search across all entity types
- `ExportService` - Export to multiple formats

✅ **CLI Commands:**
- `llmdoc init` - Initialize new project
- `llmdoc task` - Task management commands
- `llmdoc sprint` - Sprint management commands
- `llmdoc search` - Search functionality
- `llmdoc export` - Export data to various formats
- `llmdoc import` - Import data from files
- `llmdoc migrate` - Migrate from markdown documentation
- `llmdoc db` - Database maintenance commands
- `llmdoc watch` - Watch mode for VSCode integration

## Features

### Document & Task Management
- **Task Tracking**: Complete task lifecycle with status, priority, assignee, story points
- **Sprint Management**: Sprint planning, tracking, and retrospectives
- **User Stories**: Agile user story management with acceptance criteria
- **ADR Management**: Architecture Decision Record tracking and history
- **Component Documentation**: Track system components and their relationships

### Data Management
- **SQLite Database**: Async connection pooling with robust error handling
- **JSON Storage**: Flexible document storage with JSON schema validation
- **Migration Support**: Import existing markdown documentation
- **Export Capabilities**: Export to Markdown, JSON, and HTML formats

### Search & Discovery
- **Text Search**: Search across all document types and content
- **Filtering**: Advanced filtering by status, assignee, sprint, etc.
- **Semantic Organization**: Organize by sprints, epics, and components

### Configuration & Integration
- **TOML Configuration**: Flexible configuration management
- **Logging System**: Comprehensive logging with configurable levels
- **VSCode Integration**: Watch mode for real-time updates
- **CLI Interface**: Rich command-line interface with help and validation

## Getting Started

### Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **SQLite**: Used for data persistence (included with most systems)

### Installation

1. Clone the repository:
   ```bash
   git clone <repository_url>
   cd llmdoc
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Initialize a new project:
   ```bash
   ./target/release/llmdoc init
   ```

### Quick Start

```bash
# Initialize a new llmdocs project
llmdoc init

# Create a new task
llmdoc task add "Implement user authentication" --type feature --priority high

# Create a sprint
llmdoc sprint create "Sprint 1" --start-date 2024-01-01 --end-date 2024-01-14

# Search for tasks
llmdoc search query "authentication"

# Export tasks to markdown
llmdoc export --format markdown --output ./exports/
```

### Migration from Existing Documentation

If you have existing markdown documentation:

```bash
# Migrate from markdown files
llmdoc migrate --docs-dir ./docs --dry-run  # Preview changes
llmdoc migrate --docs-dir ./docs            # Perform migration
```

## Configuration

Configuration is managed via `config.toml` in your project directory or user config directory:

```toml
[database]
path = ".llmdocs/llmdocs.db"

[logging]
level_console = "info"
level_file = "debug"
log_file = ".llmdocs/logs/llmdocs.log"

[embeddings]
provider = "http"
[embeddings.http_provider]
url = "http://localhost:8008/embed"
api_key = ""  # Optional for external APIs
model = "all-MiniLM-L6-v2"
```

## Architecture

### Project Structure
```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library root with module exports
├── app_config.rs        # Configuration management
├── logging.rs           # Logging setup and utilities
├── cli/                 # Command-line interface
│   ├── mod.rs          # CLI module root
│   ├── commands/       # Command implementations
│   └── output.rs       # Output formatting
├── core/               # Core functionality
│   ├── database.rs     # SQLite database operations
│   ├── models/         # Data models (Task, Sprint, ADR, etc.)
│   ├── errors.rs       # Error handling
│   └── validation.rs   # Data validation
├── services/           # Business logic services
│   ├── task_service.rs
│   ├── sprint_service.rs
│   ├── adr_service.rs
│   ├── user_story_service.rs
│   ├── component_service.rs
│   ├── search_service.rs
│   └── export_service.rs
├── migration/          # Data migration utilities
└── utils/              # Utility functions
```

### Database Schema
- **tasks**: Task management with JSON document storage
- **sprints**: Sprint tracking and reporting
- **adrs**: Architecture Decision Records
- **user_stories**: User story management
- **components**: Component documentation
- Full-text search capabilities across all entity types

## Development Status

### ✅ Completed
- [x] Core CLI framework with clap
- [x] SQLite database integration with async pooling
- [x] All service implementations with CRUD operations
- [x] Comprehensive error handling
- [x] Configuration management
- [x] Logging system
- [x] Export functionality (Markdown, JSON, HTML)
- [x] Search across all entity types
- [x] Markdown migration utilities
- [x] Database schema and migrations
- [x] Full test coverage

### 🔄 In Progress
- [ ] Enhanced search with full-text indexing
- [ ] Advanced export templates
- [ ] Integration with external tools
- [ ] Performance optimizations

### 📋 Planned
- [ ] Native ONNX embedding support
- [ ] LLM integration for content generation
- [ ] Web interface
- [ ] API endpoints
- [ ] Advanced reporting and analytics
- [ ] Team collaboration features

## Contributing

Contributions are welcome! The project follows standard Rust conventions:

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Run `cargo test` and `cargo clippy`
5. Submit a pull request

## Testing

```bash
# Run all tests
cargo test

# Run with coverage
cargo test --coverage

# Run specific test module
cargo test services::task_service
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For issues, questions, or contributions, please use the GitHub issue tracker.