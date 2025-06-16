# LLMDocs Guide for LLMs

## 🚨 CRITICAL: Database-Based Documentation System

This project uses **LLMDocs** - ALL documentation is stored in a SQLite database, NOT in markdown files.

**NEVER edit .md files directly** - use `llmdocs` commands exclusively.

## Quick Reference

### Essential Commands

```bash
# Task Management
llmdocs task add '{"id": "TASK-001", "title": "Implement feature", "status": "todo", "type": "feature"}'
llmdocs task get TASK-001
llmdocs task list --status in_progress
llmdocs task update TASK-001 --status done

# Sprint Management
llmdocs sprint current
llmdocs sprint create --name "Sprint 5" --start 2025-01-01 --end 2025-01-14
llmdocs sprint start sprint-5
llmdocs sprint complete sprint-5

# Search
llmdocs search query "authentication"
llmdocs search text "bug fix"

# Export (for human review)
llmdocs export --format markdown
```

## Task Schema (REQUIRED fields)

```json
{
  "id": "TASK-XXX",        // Format: TASK-[0-9]+
  "title": "string",       // Required, 1-200 chars
  "status": "todo|in_progress|done|blocked|cancelled",
  "type": "feature|bug|task|epic|story|spike"
}
```

## Common Workflows

### 1. Starting Work on a Task

```bash
# Get current sprint
SPRINT=$(llmdocs sprint current | jq -r '.id')

# Create task in sprint
llmdocs task add '{
  "id": "TASK-123",
  "title": "Add user authentication",
  "status": "todo",
  "type": "feature",
  "sprint_id": "'$SPRINT'"
}'

# Start work
llmdocs task update TASK-123 --status in_progress --assignee "john"
```

### 2. Task Decomposition

```bash
# Create parent epic
llmdocs task add '{"id": "TASK-100", "title": "User Management", "type": "epic"}'

# Create subtasks
llmdocs task add '{"id": "TASK-101", "title": "User registration", "type": "task", "parent_id": "TASK-100"}'
llmdocs task add '{"id": "TASK-102", "title": "User login", "type": "task", "parent_id": "TASK-100"}'
```

### 3. Finding Information

```bash
# Semantic search across all content
llmdocs search query "how to handle authentication"

# List specific tasks
llmdocs task list --sprint sprint-5 --assignee john

# Get task with history
llmdocs task get TASK-123 --history
```

## Best Practices

1. **Update Immediately**: Change task status as work progresses
2. **Use Semantic Search**: Find similar implementations before starting
3. **Link Dependencies**: Maintain task relationships
4. **Document Decisions**: Create ADRs for architectural choices

## Error Prevention

- ✅ Always use correct ID formats (TASK-123, sprint-5, comp-auth)
- ✅ Validate JSON before submitting
- ✅ Check task exists before referencing
- ❌ Never edit .md files directly
- ❌ Don't create duplicate IDs

## Integration with Development

```bash
# Before committing
llmdocs task update TASK-123 --status done

# Link to PR
llmdocs task update TASK-123 --pr "https://github.com/org/repo/pull/45"

# After deployment
llmdocs task update TASK-123 --deployed "production"
```

Remember: The database is the single source of truth!