# LLMDocs - Comprehensive User Guide

## 🚨 CRITICAL: Database-Driven Documentation System

**LLMDocs is a revolutionary approach to documentation management specifically designed for LLM-assisted development workflows.**

All documentation is stored in a **SQLite database**, not in markdown files. The system is designed to be managed entirely by AI assistants and provides rich querying, search, and export capabilities.

## Table of Contents

1. [Quick Start Guide](#quick-start-guide)
2. [Installation & Setup](#installation--setup)
3. [Project Initialization](#project-initialization)
4. [Core Concepts](#core-concepts)
5. [Complete Command Reference](#complete-command-reference)
6. [Advanced Usage Patterns](#advanced-usage-patterns)
7. [Migration from Existing Projects](#migration-from-existing-projects)
8. [LLM Integration Patterns](#llm-integration-patterns)
9. [Export & Reporting](#export--reporting)
10. [Troubleshooting](#troubleshooting)

---

## Quick Start Guide

### 1. Installation
```bash
# Clone and build the project
git clone <repository_url>
cd llmdoc
cargo build --release

# Add to PATH (optional)
export PATH="$PATH:$(pwd)/target/release"
```

### 2. Initialize Your First Project
```bash
# Initialize in current directory
llmdocs init

# Initialize with force (overwrites existing)
llmdocs init --force
```

### 3. Create Your First Task
```bash
# Interactive mode
llmdocs task add

# Or direct JSON input
llmdocs task add '{"id": "TASK-001", "title": "Setup project", "status": "todo", "type": "task"}'
```

### 4. Explore Your Data
```bash
# List all tasks
llmdocs task list

# Search across all content
llmdocs search query "setup"

# Export to markdown for human review
llmdocs export --format markdown --output ./docs_export
```

---

## Installation & Setup

### Prerequisites
- **Rust 1.70+** - Install from [rustup.rs](https://rustup.rs/)
- **SQLite** - Usually pre-installed on most systems

### Building from Source
```bash
# Clone repository
git clone <repository_url>
cd llmdoc

# Build optimized binary
cargo build --release

# Run tests
cargo test

# Install globally (optional)
cargo install --path .
```

### Configuration
LLMDocs uses TOML configuration files. Create `llmdocs.toml` in your project root:

```toml
# Database configuration
database_url = ".llmdocs/llmdocs.db"

# Logging configuration
log_level_console = "info"
log_level_file = "debug"
log_file = ".llmdocs/logs/llmdocs.log"
schema_dir = ".llmdocs/schemas"

# Embeddings configuration
[embeddings]
provider = "http"  # Options: "http", "native", "none"

[embeddings.http_provider]
url = "http://localhost:8008/embed"  # External embedding service
api_key = ""  # Optional API key
model = "all-MiniLM-L6-v2"  # Optional model specification

[embeddings.native_provider]
model_path = "./models/embedding-model.onnx"  # Path to local ONNX model

# Export configuration
[export]
default_export_dir = "./docs_export"
default_format = "markdown"
```

### Embedding Providers
LLMDocs supports multiple embedding providers:

1. **HTTP Provider** - Connect to external embedding services
   - OpenAI API endpoints
   - Hugging Face Inference API
   - Custom embedding services with compatible API
   
2. **Native Provider** - Use local ONNX models (planned feature)
   - Local inference with ONNX models
   - No network dependency
   - Private data handling
   
3. **None** - Disable semantic search features
   - Text-only search available
   - Reduced functionality but faster setup

---

## Project Initialization

### New Project Setup
```bash
# Basic initialization
llmdocs init

# Force initialization (overwrites existing database)
llmdocs init --force

# Custom database location
llmdocs --database /path/to/custom.db init
```

### Directory Structure Created
```
your-project/
├── .llmdocs/
│   ├── llmdocs.db          # Main database
│   ├── config.toml         # Local configuration
│   └── logs/
│       └── llmdocs.log     # Application logs
├── llmdocs.toml            # Project configuration
└── docs_export/            # Default export directory
```

### Initial Configuration
After initialization, customize your `llmdocs.toml`:

```toml
# Database configuration
database_url = ".llmdocs/llmdocs.db"

# Logging configuration
log_level_console = "info"
log_level_file = "debug"
log_file = ".llmdocs/logs/llmdocs.log"
schema_dir = ".llmdocs/schemas"

# Embeddings configuration
[embeddings]
provider = "http"  # Options: "http", "native", "none"

[embeddings.http_provider]
url = "http://localhost:8008/embed"  # External embedding service
api_key = ""  # Optional for OpenAI/HuggingFace APIs
model = "all-MiniLM-L6-v2"  # Model specification

[embeddings.native_provider]
model_path = "./models/embedding-model.onnx"  # For future native support

# Export configuration
[export]
default_export_dir = "./docs_export"
default_format = "markdown"
```

---

## Core Concepts

### 1. Tasks
Tasks are the fundamental unit of work tracking in LLMDocs.

**Task Schema:**
```json
{
  "id": "TASK-XXX",           // Required: Format TASK-[0-9]+
  "title": "string",          // Required: 1-200 characters
  "description": "string",    // Optional: Detailed description
  "status": "todo|in_progress|done|blocked|cancelled",
  "type": "feature|bug|task|epic|story|spike",
  "priority": "low|medium|high|critical",
  "assignee": "string",       // Optional: Who's working on it
  "sprint_id": "string",      // Optional: Associated sprint
  "parent_id": "string",      // Optional: For task hierarchies
  "story_points": 1-21,       // Optional: Fibonacci estimation
  "labels": ["tag1", "tag2"], // Optional: Categorization
  "created_at": "ISO8601",    // Auto-generated
  "updated_at": "ISO8601",    // Auto-updated
  "due_date": "YYYY-MM-DD",   // Optional: Deadline
  "component_ids": ["COMP-1"] // Optional: Related components
}
```

### 2. Sprints
Sprints organize tasks into time-boxed iterations.

**Sprint Schema:**
```json
{
  "id": "SPRINT-XXX",
  "name": "string",
  "description": "string",
  "start_date": "YYYY-MM-DD",
  "end_date": "YYYY-MM-DD",
  "status": "planning|active|completed|cancelled",
  "goal": "string",
  "velocity": 0,
  "created_at": "ISO8601",
  "updated_at": "ISO8601"
}
```

### 3. Architecture Decision Records (ADRs)
ADRs document important architectural decisions.

**ADR Schema:**
```json
{
  "id": "ADR-XXX",
  "title": "string",
  "status": "proposed|accepted|rejected|superseded",
  "context": "string",        // Why is this decision needed?
  "decision": "string",       // What decision was made?
  "consequences": "string",   // What are the implications?
  "alternatives": "string",   // What other options were considered?
  "created_at": "ISO8601",
  "updated_at": "ISO8601",
  "author": "string",
  "superseded_by": "string"   // For linked decisions
}
```

### 4. User Stories
User stories capture requirements from user perspective.

**User Story Schema:**
```json
{
  "id": "STORY-XXX",
  "title": "string",
  "description": "string",
  "acceptance_criteria": ["criterion1", "criterion2"],
  "priority": "low|medium|high|critical",
  "status": "draft|ready|in_progress|done|cancelled",
  "story_points": 1-21,
  "epic_id": "string",
  "created_at": "ISO8601",
  "updated_at": "ISO8601"
}
```

### 5. Components
Components represent system modules or architectural elements.

**Component Schema:**
```json
{
  "id": "COMP-XXX",
  "name": "string",
  "description": "string",
  "type": "service|library|database|interface|external",
  "status": "active|deprecated|planned",
  "owner": "string",
  "repository": "string",
  "documentation_url": "string",
  "dependencies": ["COMP-1", "COMP-2"],
  "created_at": "ISO8601",
  "updated_at": "ISO8601"
}
```

---

## Complete Command Reference

### Task Management

#### Create Tasks
```bash
# Interactive mode (recommended for beginners)
llmdocs task add

# Direct JSON input
llmdocs task add '{"id": "TASK-001", "title": "Implement login", "status": "todo", "type": "feature"}'

# Complex task with all fields
llmdocs task add '{
  "id": "TASK-002",
  "title": "Fix authentication bug",
  "description": "Users cannot login after password reset",
  "status": "todo",
  "type": "bug",
  "priority": "high",
  "assignee": "john.doe",
  "story_points": 3,
  "labels": ["security", "urgent"],
  "due_date": "2024-02-15"
}'
```

#### Query Tasks
```bash
# Get specific task
llmdocs task get TASK-001

# Get task with history
llmdocs task get TASK-001 --history

# List all tasks
llmdocs task list

# Filter by status
llmdocs task list --status in_progress

# Filter by assignee
llmdocs task list --assignee john.doe

# Filter by sprint
llmdocs task list --sprint SPRINT-001

# Output formats
llmdocs task list --format table    # Default
llmdocs task list --format json
llmdocs task list --format csv
```

#### Update Tasks
```bash
# Update status
llmdocs task update TASK-001 --status in_progress

# Update assignee
llmdocs task update TASK-001 --assignee jane.smith

# Update multiple fields
llmdocs task update TASK-001 --status done --points 5 --sprint SPRINT-002
```

#### Delete Tasks
```bash
# Interactive confirmation
llmdocs task delete TASK-001

# Force delete without confirmation
llmdocs task delete TASK-001 --force
```

#### Bulk Operations
```bash
# Bulk status update
llmdocs task bulk status TASK-001,TASK-002,TASK-003 done

# Bulk sprint assignment
llmdocs task bulk sprint TASK-001,TASK-002 SPRINT-001
```

### Sprint Management

#### Create Sprints
```bash
# Interactive mode
llmdocs sprint create

# Direct command
llmdocs sprint create --name "Sprint 1" --start 2024-01-15 --end 2024-01-29

# With goal and description
llmdocs sprint create --name "Sprint 2" --start 2024-02-01 --end 2024-02-14 --goal "Implement user authentication"
```

#### Manage Sprints
```bash
# List all sprints
llmdocs sprint list

# Get current active sprint
llmdocs sprint current

# Get sprint details
llmdocs sprint get SPRINT-001

# Start a sprint
llmdocs sprint start SPRINT-001

# Complete a sprint
llmdocs sprint complete SPRINT-001

# Sprint report
llmdocs sprint report SPRINT-001
```

### Search & Discovery

#### Text Search
```bash
# Simple text search
llmdocs search query "authentication"

# Search in specific entity types
llmdocs search query "bug fix" --type task

# Search with filters
llmdocs search query "login" --status in_progress --assignee john
```

#### Semantic Search (with embedding provider)
```bash
# Semantic search across all entities (requires embedding provider)
llmdocs search semantic "How to handle user permissions"

# Search for similar tasks
llmdocs search similar TASK-001
```

### ADR Management

#### Create ADRs
```bash
# Interactive mode
llmdocs adr add

# Direct JSON
llmdocs adr add '{
  "id": "ADR-001",
  "title": "Use PostgreSQL for primary database",
  "status": "proposed",
  "context": "We need a reliable, scalable database solution",
  "decision": "Adopt PostgreSQL as our primary database",
  "consequences": "Better performance and reliability, but requires PostgreSQL expertise"
}'
```

#### Manage ADRs
```bash
# List ADRs
llmdocs adr list

# Get ADR details
llmdocs adr get ADR-001

# Update ADR status
llmdocs adr update ADR-001 --status accepted

# Supersede an ADR
llmdocs adr supersede ADR-001 ADR-002
```

### Component Management

#### Create Components
```bash
# Interactive mode
llmdocs component add

# Direct JSON
llmdocs component add '{
  "id": "COMP-001",
  "name": "Authentication Service",
  "description": "Handles user authentication and authorization",
  "type": "service",
  "status": "active",
  "owner": "security-team"
}'
```

#### Manage Components
```bash
# List components
llmdocs component list

# Get component details
llmdocs component get COMP-001

# Update component
llmdocs component update COMP-001 --status deprecated

# Show component dependencies
llmdocs component dependencies COMP-001
```

### User Story Management

#### Create User Stories
```bash
# Interactive mode
llmdocs story add

# Direct JSON
llmdocs story add '{
  "id": "STORY-001",
  "title": "As a user, I want to reset my password",
  "description": "Users need to be able to reset forgotten passwords",
  "acceptance_criteria": [
    "User can request password reset via email",
    "Reset link expires after 24 hours",
    "User must verify new password"
  ],
  "priority": "high",
  "story_points": 5
}'
```

#### Manage User Stories
```bash
# List stories
llmdocs story list

# Get story details
llmdocs story get STORY-001

# Update story
llmdocs story update STORY-001 --status ready
```

### Database Operations

#### Maintenance
```bash
# Database status
llmdocs db status

# Create backup
llmdocs db backup

# Restore from backup
llmdocs db restore backup-20240115.db

# Vacuum database
llmdocs db vacuum

# Run migrations
llmdocs db migrate
```

#### Data Management
```bash
# Clear all data (dangerous!)
llmdocs db clear --confirm

# Export database
llmdocs db export --format sql
```

### Import/Export Operations

#### Export Data
```bash
# Export all data to markdown
llmdocs export --format markdown --output ./docs

# Export specific entities
llmdocs export --format json --filter tasks --output ./backup

# Export with history
llmdocs export --format html --include-history --output ./report
```

#### Import Data
```bash
# Import from JSON file
llmdocs import data.json

# Import with merge (don't overwrite existing)
llmdocs import data.json --merge

# Import from CSV
llmdocs import tasks.csv --type tasks
```

### Migration Operations

#### From Existing Documentation
```bash
# Dry run migration (preview changes)
llmdocs migrate --docs-dir ./old-docs --dry-run

# Perform migration
llmdocs migrate --docs-dir ./old-docs

# Migrate specific file types
llmdocs migrate --docs-dir ./docs --pattern "*.md"
```

### Watch Mode

#### Development Integration
```bash
# Watch for file changes (VSCode integration)
llmdocs watch

# Custom interval
llmdocs watch --interval 5
```

---

## Advanced Usage Patterns

### 1. Task Hierarchies and Dependencies

#### Creating Epic-Story-Task Hierarchies
```bash
# Create an epic
llmdocs task add '{
  "id": "TASK-100",
  "title": "User Management System",
  "type": "epic",
  "description": "Complete user management functionality"
}'

# Create stories under the epic
llmdocs task add '{
  "id": "TASK-101",
  "title": "User Registration",
  "type": "story",
  "parent_id": "TASK-100",
  "story_points": 8
}'

llmdocs task add '{
  "id": "TASK-102",
  "title": "User Authentication",
  "type": "story", 
  "parent_id": "TASK-100",
  "story_points": 5
}'

# Create tasks under stories
llmdocs task add '{
  "id": "TASK-103",
  "title": "Create registration form",
  "type": "task",
  "parent_id": "TASK-101",
  "story_points": 3
}'
```

#### Querying Hierarchies
```bash
# Find all children of an epic
llmdocs search query "parent_id:TASK-100"

# Get epic with all descendants
llmdocs task get TASK-100 --include-children
```

### 2. Sprint Planning Workflow

#### Complete Sprint Planning Session
```bash
# Create new sprint
llmdocs sprint create --name "Sprint 5" --start 2024-02-01 --end 2024-02-14 --goal "Complete user authentication"

# Add tasks to sprint
llmdocs task update TASK-101 --sprint SPRINT-005
llmdocs task update TASK-102 --sprint SPRINT-005
llmdocs task update TASK-103 --sprint SPRINT-005

# Start the sprint
llmdocs sprint start SPRINT-005

# Track progress
llmdocs sprint report SPRINT-005

# Complete sprint
llmdocs sprint complete SPRINT-005
```

#### Sprint Reporting
```bash
# Velocity tracking
llmdocs sprint velocity --last 5

# Burndown data
llmdocs sprint burndown SPRINT-005

# Sprint retrospective data
llmdocs sprint retrospective SPRINT-005
```

### 3. Component Architecture Tracking

#### Define System Architecture
```bash
# Create core components
llmdocs component add '{
  "id": "COMP-AUTH",
  "name": "Authentication Service",
  "type": "service",
  "owner": "backend-team",
  "dependencies": ["COMP-DB", "COMP-CACHE"]
}'

llmdocs component add '{
  "id": "COMP-DB",
  "name": "Primary Database",
  "type": "database",
  "status": "active"
}'

llmdocs component add '{
  "id": "COMP-CACHE",
  "name": "Redis Cache",
  "type": "external",
  "status": "active"
}'
```

#### Link Tasks to Components
```bash
llmdocs task add '{
  "id": "TASK-104",
  "title": "Optimize authentication performance",
  "type": "task",
  "component_ids": ["COMP-AUTH", "COMP-CACHE"]
}'
```

### 4. Decision Tracking with ADRs

#### Document Major Decisions
```bash
llmdocs adr add '{
  "id": "ADR-001",
  "title": "Use JWT for authentication tokens",
  "status": "proposed",
  "context": "Need secure, scalable authentication mechanism",
  "decision": "Implement JWT-based authentication with short-lived access tokens and refresh tokens",
  "consequences": "Pros: Stateless, scalable. Cons: Token management complexity",
  "alternatives": "Session-based auth, OAuth2 only"
}'

# Accept the decision
llmdocs adr update ADR-001 --status accepted

# Later, supersede with new decision
llmdocs adr add '{
  "id": "ADR-002", 
  "title": "Migrate to OAuth2 with PKCE",
  "status": "proposed",
  "context": "JWT implementation proving complex for mobile clients",
  "decision": "Migrate to OAuth2 with PKCE for better mobile security"
}'

llmdocs adr supersede ADR-001 ADR-002
```

### 5. Advanced Search Patterns

#### Complex Filtering
```bash
# Find high-priority bugs assigned to specific person
llmdocs search query "type:bug priority:high assignee:john.doe"

# Find tasks in specific sprint with specific status
llmdocs search query "sprint:SPRINT-005 status:in_progress"

# Find overdue tasks
llmdocs search query "due_date:<2024-01-15 status:!done"
```

#### Semantic Search with Context (requires embedding provider)
```bash
# Find similar implementation patterns
llmdocs search semantic "How to implement rate limiting in authentication service"

# Find related decisions
llmdocs search semantic "database performance optimization decisions"
```

---

## Migration from Existing Projects

### 1. From Markdown Documentation

#### Automatic Migration
```bash
# Preview migration (recommended first step)
llmdocs migrate --docs-dir ./docs --dry-run

# Perform migration
llmdocs migrate --docs-dir ./docs

# Migrate with specific patterns
llmdocs migrate --docs-dir ./docs --pattern "**/*.md" --exclude "README.md"
```

#### Custom Migration Rules
Create `migration.toml`:
```toml
[rules]
# Map file patterns to entity types
tasks = "tasks/**/*.md"
adrs = "decisions/**/*.md"
stories = "requirements/**/*.md"

[parsing]
# Custom parsing rules
task_id_pattern = "TASK-\\d+"
title_from_header = true
metadata_from_frontmatter = true
```

### 2. From Jira/GitHub Issues

#### Export and Convert
```bash
# Export from Jira to JSON
# (Use Jira export functionality)

# Convert to LLMDocs format
llmdocs import jira-export.json --type jira

# Map fields
llmdocs import jira-export.json --field-mapping '{
  "summary": "title",
  "description": "description", 
  "status": "status",
  "assignee.displayName": "assignee"
}'
```

### 3. From Confluence/Wiki

#### Content Migration
```bash
# Export wiki to markdown
# (Use wiki export tools)

# Import structured content
llmdocs migrate --docs-dir ./wiki-export --type confluence

# Handle custom formats
llmdocs migrate --docs-dir ./wiki --parser custom --rules ./confluence-rules.toml
```

---

## LLM Integration Patterns

### 1. Prompts for AI Assistants

#### Essential Prompts for LLM Usage

**Project Context Prompt:**
```
You are working with LLMDocs, a database-driven documentation system. ALL documentation is stored in SQLite database, never edit .md files directly. Always use `llmdocs` commands.

Current project context:
- Database location: .llmdocs/llmdocs.db
- Current sprint: $(llmdocs sprint current | jq -r '.id // "none"')
- Active tasks: $(llmdocs task list --status in_progress --format json | jq length) tasks

Available commands: task, sprint, search, adr, component, story, export, import, migrate
```

**Task Management Prompt:**
```
When managing tasks in LLMDocs:

1. Always use valid task IDs (TASK-XXX format)
2. Required fields: id, title, status, type
3. Valid statuses: todo, in_progress, done, blocked, cancelled
4. Valid types: feature, bug, task, epic, story, spike

Before creating tasks, search for existing similar tasks:
llmdocs search query "your search terms"

Example task creation:
llmdocs task add '{"id": "TASK-001", "title": "Task title", "status": "todo", "type": "feature"}'
```

**Development Workflow Prompt:**
```
LLMDocs Development Workflow:

1. Before starting work:
   ```bash
   # Check current sprint
   llmdocs sprint current
   
   # Find or create task
   llmdocs search query "feature description"
   llmdocs task add '{"id": "TASK-XXX", "title": "...", "status": "todo", "type": "feature"}'
   
   # Start work
   llmdocs task update TASK-XXX --status in_progress --assignee $(whoami)
   ```

2. During development:
   ```bash
   # Document decisions
   llmdocs adr add '{"id": "ADR-XXX", "title": "...", "status": "proposed", ...}'
   
   # Update progress
   llmdocs task update TASK-XXX --status in_progress
   ```

3. After completion:
   ```bash
   # Complete task
   llmdocs task update TASK-XXX --status done
   
   # Export for review
   llmdocs export --format markdown
   ```
```

#### LLM Assistant Integration Scripts

**Smart Task Creation:**
```bash
#!/bin/bash
# create-smart-task.sh
# Usage: ./create-smart-task.sh "Implement user login feature"

DESCRIPTION="$1"
TASK_ID=$(llmdocs task list --format json | jq -r '.[].id' | grep "TASK-" | sort -V | tail -1 | sed 's/TASK-//' | awk '{print $1+1}')
TASK_ID="TASK-$(printf "%03d" $TASK_ID)"

# Check for similar tasks
SIMILAR=$(llmdocs search query "$DESCRIPTION" --format json | jq length)
if [ "$SIMILAR" -gt 0 ]; then
    echo "Found $SIMILAR similar tasks:"
    llmdocs search query "$DESCRIPTION" --format table
    read -p "Continue creating new task? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Create task
llmdocs task add "{
  \"id\": \"$TASK_ID\",
  \"title\": \"$DESCRIPTION\",
  \"status\": \"todo\",
  \"type\": \"feature\"
}"

echo "Created task: $TASK_ID"
```

**Development Status Report:**
```bash
#!/bin/bash
# dev-status.sh
# Generates development status for LLM context

echo "=== LLMDocs Development Status ==="
echo
echo "Current Sprint:"
llmdocs sprint current | jq -r '.name // "No active sprint"'
echo
echo "My Active Tasks:"
llmdocs task list --assignee $(whoami) --status in_progress --format table
echo
echo "Recent Decisions:"
llmdocs adr list --recent 5 --format table
echo
echo "Next Priority Tasks:"
llmdocs task list --status todo --priority high --format table | head -10
```

### 2. AI-Driven Workflows

#### Automated Task Breakdown
```bash
# AI-assisted epic breakdown
llmdocs task get TASK-100 | jq -r '.description' | ai-break-down-epic.py | while read subtask; do
    TASK_ID=$(generate-next-task-id.sh)
    llmdocs task add "{
        \"id\": \"$TASK_ID\",
        \"title\": \"$subtask\",
        \"parent_id\": \"TASK-100\",
        \"status\": \"todo\",
        \"type\": \"task\"
    }"
done
```

#### Smart Sprint Planning
```bash
# AI-assisted sprint planning
CURRENT_VELOCITY=$(llmdocs sprint velocity --average 3)
AVAILABLE_TASKS=$(llmdocs task list --status todo --format json)

echo "$AVAILABLE_TASKS" | ai-suggest-sprint-content.py --velocity $CURRENT_VELOCITY | while read task_id; do
    llmdocs task update $task_id --sprint $(llmdocs sprint current | jq -r '.id')
done
```

### 3. Integration with Development Tools

#### Git Hook Integration
```bash
# .git/hooks/pre-commit
#!/bin/bash
# Update task status on commit

BRANCH=$(git branch --show-current)
if [[ $BRANCH =~ TASK-([0-9]+) ]]; then
    TASK_ID="TASK-${BASH_REMATCH[1]}"
    llmdocs task update $TASK_ID --status in_progress
    echo "Updated $TASK_ID to in_progress"
fi
```

#### IDE Integration (VSCode)
```json
// .vscode/tasks.json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "LLMDocs: Create Task",
            "type": "shell",
            "command": "llmdocs",
            "args": ["task", "add"],
            "group": "build"
        },
        {
            "label": "LLMDocs: My Tasks",
            "type": "shell", 
            "command": "llmdocs",
            "args": ["task", "list", "--assignee", "${env:USER}", "--format", "table"],
            "group": "build"
        }
    ]
}
```

---

## Export & Reporting

### 1. Standard Export Formats

#### Markdown Documentation
```bash
# Full documentation export
llmdocs export --format markdown --output ./docs

# Custom template
llmdocs export --format markdown --template ./custom-template.hbs --output ./docs

# Filtered export
llmdocs export --format markdown --filter "sprint:SPRINT-005" --output ./sprint-5-docs
```

#### JSON Data Export
```bash
# Complete data dump
llmdocs export --format json --output ./backup.json

# Specific entity types
llmdocs export --format json --types "tasks,sprints" --output ./project-data.json

# With metadata
llmdocs export --format json --include-metadata --output ./full-export.json
```

#### HTML Reports
```bash
# Interactive HTML report
llmdocs export --format html --output ./reports --template dashboard

# Sprint report
llmdocs export --format html --filter "sprint:SPRINT-005" --template sprint-report --output ./sprint-5-report.html
```

### 2. Custom Reporting

#### Sprint Reports
```bash
# Velocity report
llmdocs sprint report SPRINT-005 --format json | jq '{
  sprint: .name,
  planned_points: .planned_story_points,
  completed_points: .completed_story_points,
  velocity: .velocity
}'

# Burndown data
llmdocs sprint burndown SPRINT-005 --format csv > burndown.csv
```

#### Team Performance Reports
```bash
# Individual performance
llmdocs task list --assignee john.doe --format json | jq '[.[] | select(.status == "done")] | length'

# Team velocity by sprint
for sprint in $(llmdocs sprint list --format json | jq -r '.[].id'); do
    echo "Sprint: $sprint"
    llmdocs sprint report $sprint | jq '.velocity'
done
```

#### Technical Debt Tracking
```bash
# Find technical debt tasks
llmdocs search query "tech debt OR refactor OR TODO" --format table

# Component health report
llmdocs component list --format json | jq '[.[] | {name: .name, status: .status, task_count: (.task_count // 0)}]'
```

### 3. Automated Reporting

#### Daily Standup Report
```bash
#!/bin/bash
# daily-standup.sh

echo "=== Daily Standup Report ==="
echo "Date: $(date)"
echo

echo "Yesterday's Completed Tasks:"
llmdocs task list --completed-since yesterday --format table

echo "Today's Planned Tasks:"
llmdocs task list --status in_progress --format table

echo "Blockers:"
llmdocs task list --status blocked --format table

echo "Sprint Progress:"
CURRENT_SPRINT=$(llmdocs sprint current | jq -r '.id')
if [ "$CURRENT_SPRINT" != "null" ]; then
    llmdocs sprint report $CURRENT_SPRINT
fi
```

#### Weekly Status Report
```bash
#!/bin/bash
# weekly-report.sh

WEEK_START=$(date -d "last Monday" +%Y-%m-%d)
WEEK_END=$(date -d "next Sunday" +%Y-%m-%d)

echo "=== Weekly Status Report ($WEEK_START to $WEEK_END) ==="

# Completed tasks this week
echo "## Completed This Week"
llmdocs task list --completed-since $WEEK_START --format markdown

# ADRs created this week
echo "## New Decisions"
llmdocs adr list --created-since $WEEK_START --format markdown

# Sprint progress
echo "## Sprint Progress"
CURRENT_SPRINT=$(llmdocs sprint current | jq -r '.id')
if [ "$CURRENT_SPRINT" != "null" ]; then
    llmdocs sprint report $CURRENT_SPRINT --format markdown
fi

# Export to file
llmdocs export --format markdown --since $WEEK_START --output ./weekly-reports/report-$WEEK_START.md
```

---

## Troubleshooting

### Common Issues

#### 1. Database Connection Issues
```bash
# Check database status
llmdocs db status

# Fix corrupted database
llmdocs db repair

# Reset database (WARNING: deletes all data)
llmdocs db reset --confirm
```

#### 2. Migration Problems
```bash
# Validate before migration
llmdocs migrate --docs-dir ./docs --validate-only

# Migration with verbose logging
llmdocs --verbose migrate --docs-dir ./docs

# Rollback migration
llmdocs db rollback --to-version 1
```

#### 3. Search Not Working
```bash
# Check embedding service (if using HTTP provider)
curl http://localhost:8008/health  # Or your configured URL

# Test basic text search (works without embeddings)
llmdocs search query "test"

# Check current configuration
llmdocs db status

# Use text-only search mode
# (semantic search requires embedding provider)
llmdocs search query "authentication" --text-only
```

#### 4. Performance Issues
```bash
# Database statistics
llmdocs db stats

# Vacuum database
llmdocs db vacuum

# Analyze slow queries
llmdocs db analyze --slow-queries
```

### Error Recovery

#### Backup and Restore
```bash
# Create backup before major operations
llmdocs db backup --output ./backup-$(date +%Y%m%d).db

# Restore from backup
llmdocs db restore ./backup-20240115.db

# Automated backups
crontab -e
# Add: 0 2 * * * /path/to/llmdocs db backup --output /backups/llmdocs-$(date +%Y%m%d).db
```

#### Data Validation
```bash
# Validate data integrity
llmdocs db validate

# Fix data inconsistencies
llmdocs db repair --dry-run  # Preview fixes
llmdocs db repair           # Apply fixes

# Rebuild indexes
llmdocs db reindex
```

### Performance Optimization

#### Database Tuning
```toml
# llmdocs.toml
[database]
connection_pool_size = 10
query_timeout = 30
cache_size = 10000
journal_mode = "WAL"
synchronous = "NORMAL"
```

#### Search Optimization
```toml
# llmdocs.toml
[embeddings]
batch_size = 100
cache_embeddings = true
update_on_save = false  # Manual updates for better performance

[search]
index_refresh_interval = 3600  # 1 hour
full_text_search = true
```

---

## Advanced Configuration

### Complete Configuration Reference

```toml
# llmdocs.toml - Complete configuration example

# Database configuration
database_url = ".llmdocs/llmdocs.db"

# Logging configuration
log_level_console = "info"     # trace, debug, info, warn, error
log_level_file = "debug"
log_file = ".llmdocs/logs/llmdocs.log"
schema_dir = ".llmdocs/schemas"

# Embeddings configuration
[embeddings]
provider = "http"              # Options: "http", "native", "none"

# HTTP provider configuration (for external APIs)
[embeddings.http_provider]
url = "http://localhost:8008/embed"  # External embedding service
api_key = ""                         # Optional API key for services like OpenAI
model = "all-MiniLM-L6-v2"          # Optional model specification

# Native provider configuration (future feature)
[embeddings.native_provider]
model_path = "./models/embedding-model.onnx"  # Path to local ONNX model

# Export configuration
[export]
default_export_dir = "./docs_export"
default_format = "markdown"    # Options: "markdown", "json", "html"
```

### Environment Variables

```bash
# Database configuration
export LLMDOCS__DATABASE_URL=".llmdocs/llmdocs.db"
export LLMDOCS__LOG_LEVEL_CONSOLE="debug"

# Embedding configuration
export LLMDOCS__EMBEDDINGS__PROVIDER="http"
export LLMDOCS__EMBEDDINGS__HTTP_PROVIDER__URL="https://api.openai.com/v1/embeddings"
export LLMDOCS__EMBEDDINGS__HTTP_PROVIDER__API_KEY="sk-your-api-key"

# Export configuration  
export LLMDOCS__EXPORT__DEFAULT_EXPORT_DIR="./docs"
export LLMDOCS__EXPORT__DEFAULT_FORMAT="markdown"
```

## Common Embedding Service Examples

### OpenAI API Configuration
```toml
[embeddings]
provider = "http"

[embeddings.http_provider]
url = "https://api.openai.com/v1/embeddings"
api_key = "sk-your-openai-api-key"
model = "text-embedding-ada-002"
```

### Hugging Face Inference API
```toml
[embeddings]
provider = "http"

[embeddings.http_provider]
url = "https://api-inference.huggingface.co/pipeline/feature-extraction/sentence-transformers/all-MiniLM-L6-v2"
api_key = "hf_your-huggingface-token"
model = "all-MiniLM-L6-v2"
```

### Custom Local Service
```toml
[embeddings]
provider = "http"

[embeddings.http_provider]
url = "http://localhost:8008/embed"
api_key = ""  # Not required for local services
model = "custom-model"
```

### Setting Up Embedding Services

#### Using OpenAI API
1. Sign up for OpenAI API access
2. Get your API key from the OpenAI dashboard
3. Configure LLMDocs with the OpenAI embedding endpoint
4. Supports models like `text-embedding-ada-002`

#### Using Hugging Face Inference API
1. Create a Hugging Face account
2. Generate an API token from your settings
3. Use the Inference API endpoint for embedding models
4. Free tier available with rate limits

#### Local Embedding Services
For private data or offline usage, you can run local embedding services:

1. **Ollama** - Run local language models including embeddings
2. **Text Embeddings Inference** - Hugging Face's embedding server
3. **Custom FastAPI services** - Build your own embedding API
4. **LiteLLM Proxy** - Universal proxy for various embedding APIs

#### Native ONNX Support (Planned)
Future versions will support local ONNX models:
- No network dependency
- Complete privacy
- Faster inference for small models
- Configurable via `native_provider` settings

---

## Conclusion

LLMDocs revolutionizes documentation management by providing:

1. **Database-driven storage** - No more scattered markdown files
2. **Rich querying capabilities** - Find information instantly
3. **LLM-first design** - Perfect for AI-assisted development
4. **Flexible export options** - Generate docs when needed
5. **Comprehensive tracking** - Tasks, decisions, components, and more

### Key Benefits for LLM Workflows:

- **Single source of truth** - All project information in one place
- **Structured data** - Consistent schemas for reliable AI interaction
- **Powerful search** - Both text and semantic search capabilities
- **Audit trail** - Complete history of all changes
- **Export flexibility** - Generate documentation on demand

### Getting Help:

- **Documentation**: This guide covers all functionality
- **Examples**: See `examples/` directory for real-world usage
- **Issues**: Report bugs and feature requests on GitHub
- **Community**: Join discussions in project forums

### Quick Reference Card:

```bash
# Essential commands every LLM should know
llmdocs init                                    # Initialize project
llmdocs task add '{"id":"TASK-001","title":"...","status":"todo","type":"task"}'
llmdocs task list --status in_progress         # Show active work
llmdocs search query "your search terms"       # Find information
llmdocs sprint current                          # Current sprint info
llmdocs export --format markdown               # Generate docs
```

**Remember: The database is your single source of truth. Always use `llmdocs` commands, never edit exported files directly.**