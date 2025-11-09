# Workshop CLI Integration

This project uses Workshop, a persistent context tool for maintaining institutional knowledge across sessions.

**Note for Windows:** Workshop hooks are currently disabled on Windows due to Claude Code freezing issues. You'll need to manually run Workshop commands to load context and capture sessions. Start each session with `workshop context` to load existing knowledge.

## Workshop Commands

**At the start of each session (especially on Windows):**
- Run `workshop context` to load project knowledge
- Run `workshop recent` to see what was worked on recently
- Run `workshop why "<topic>"` if you need to understand past decisions

**Use Workshop liberally throughout the session to:**
- Record decisions: `workshop decision "<text>" -r "<reasoning>"`
- Document gotchas: `workshop gotcha "<text>" -t tag1 -t tag2`
- Add notes: `workshop note "<text>"`
- Track preferences: `workshop preference "<text>" --category code_style`
- Manage state: `workshop goal add "<text>"` and `workshop next "<text>"`

**Query context (use these frequently!):**
- `workshop why "<topic>"` - THE KILLER FEATURE! Answers "why did we do X?" - prioritizes decisions with reasoning
- `workshop context` - View session summary
- `workshop search "<query>"` - Find relevant entries
- `workshop recent` - Recent activity
- `workshop summary` - Activity overview
- `workshop sessions` - View past session history
- `workshop session last` - View details of the most recent session

**Important:** Workshop helps maintain continuity across sessions. Document architectural decisions, failed approaches, user preferences, and gotchas as you discover them.

**Best Practice:** When you wonder "why did we choose X?" or "why is this implemented this way?", run `workshop why "X"` first before asking the user!

## Importing Past Sessions

Workshop can import context from past Claude Code sessions stored in JSONL transcript files:

- **When to suggest:** If the user mentions wanting context from previous sessions, or asks "why" questions that might be answered by historical context, suggest running `workshop import --execute`
- **First-time import:** Always ask the user before running import for the first time - it can extract hundreds of entries from historical sessions
- **What it does:** Analyzes JSONL transcripts and automatically extracts decisions, gotchas, and preferences from past conversations
- **Command:** `workshop import --execute` (without --execute it's just a preview)
- **Location:** By default, imports from the current project's JSONL files in `~/.claude/projects/`

**Important:** You have permission to run `workshop import --execute`, but always ask the user first, especially if import has never been run in this project. Let them decide if they want to import historical context.



# Jility MCP Tools

Jility provides an MCP (Model Context Protocol) server that allows Claude Code to interact with tickets programmatically. This enables AI-assisted project management workflows.

## Available Tools

The Jility MCP server is configured in `.mcp.json` and provides the following tools:

### Ticket Management
- **`mcp__jility__create_ticket`** - Create a new ticket
  - Parameters: `title`, `description`, `status`, `story_points`, `labels`, `assignees`, `epic_id`, `parent_id`
  - Returns: Created ticket with ID and number (e.g., JIL-42)

- **`mcp__jility__list_tickets`** - List tickets with optional filters
  - Parameters: `status` (array), `assignee`, `labels`, `limit`, `offset`
  - Returns: Array of tickets matching filters

- **`mcp__jility__get_ticket`** - Get full ticket details
  - Parameters: `ticket_id`
  - Returns: Ticket with comments, dependencies, commits, history

- **`mcp__jility__update_status`** - Update ticket status
  - Parameters: `ticket_id`, `status` (backlog, todo, in_progress, review, done, blocked)

- **`mcp__jility__update_description`** - Update ticket description
  - Parameters: `ticket_id`, `content`, `operation` (replace_all, append, prepend, replace_lines, replace_section)

### Collaboration
- **`mcp__jility__add_comment`** - Add a comment to a ticket
  - Parameters: `ticket_id`, `content`
  - Supports `@mentions` for notifications

- **`mcp__jility__assign_ticket`** - Assign ticket to team members
  - Parameters: `ticket_id`, `assignees` (array, empty array to unassign)
  - Supports pair programming (multiple assignees)

### Workflow
- **`mcp__jility__claim_ticket`** - Claim an unassigned ticket
  - Parameters: `ticket_id`
  - Auto-assigns to agent and moves to `in_progress`

### Dependencies
- **`mcp__jility__add_dependency`** - Mark ticket dependency
  - Parameters: `ticket_id`, `depends_on` (blocker ticket ID)

- **`mcp__jility__remove_dependency`** - Remove dependency

- **`mcp__jility__get_dependency_graph`** - Get full dependency tree

### Batch Operations
- **`mcp__jility__create_tickets_batch`** - Create multiple tickets at once
  - Useful for breaking down epics into sub-tasks
  - Parameters: `tickets` (array), `parent_id` (optional)

### Search
- **`mcp__jility__search_tickets`** - Full-text search across tickets
  - Parameters: `query`, `limit`
  - Searches titles, descriptions, and comments

### Templates
- **`mcp__jility__list_templates`** - List available ticket templates

- **`mcp__jility__create_from_template`** - Create ticket from template
  - Parameters: `template`, `variables` (for substitution)

### Git Integration
- **`mcp__jility__link_commit`** - Link a git commit to a ticket
  - Parameters: `ticket_id`, `commit_hash`, `commit_message`

## Usage Examples

### Creating a Feature Ticket
```typescript
mcp__jility__create_ticket({
  title: "Add dark mode toggle",
  description: "Implement theme switcher in navbar using Tailwind dark mode",
  status: "backlog",
  story_points: 3,
  labels: ["feature", "frontend", "ui"]
})
```

### Planning a Sprint
```typescript
// List backlog tickets
const backlog = await mcp__jility__list_tickets({
  status: ["backlog"],
  limit: 20
})

// Create tickets for a new epic
await mcp__jility__create_tickets_batch({
  parent_id: "epic-id",
  tickets: [
    { title: "Task 1", story_points: 2, labels: ["frontend"] },
    { title: "Task 2", story_points: 3, labels: ["backend"] },
    { title: "Task 3", story_points: 5, labels: ["testing"] }
  ]
})
```

### Working on a Ticket
```typescript
// Claim ticket
await mcp__jility__claim_ticket({ ticket_id: "ticket-id" })

// Add progress update
await mcp__jility__add_comment({
  ticket_id: "ticket-id",
  content: "Implemented the UI components. Moving to backend integration."
})

// Mark complete
await mcp__jility__update_status({
  ticket_id: "ticket-id",
  status: "done"
})
```

## Best Practices

1. **Always set story points** when creating tickets - helps with sprint planning
2. **Use descriptive titles** - should be clear without reading the description
3. **Add labels** for categorization - use consistent label names (frontend, backend, bug, feature, etc.)
4. **Link related tickets** - use dependencies to track blockers
5. **Update status regularly** - keep the board accurate
6. **Add comments for context** - explain decisions and progress
7. **Use batch operations** when creating multiple related tickets

---

# Story Point Estimation

Jility uses the **Practical Fibonacci** sequence for story point estimation. Story points measure effort and complexity, not time.

## Practical Fibonacci Scale

**0 - No Points**
- No effort is required, or there is effort but no business value delivered
- Example: Behavioral changes from Scrum Retrospective

**1 - Extra Small**
- Developers feel they understand most requirements and consider it relatively easy
- Probably the smallest item in the Sprint
- Most likely completed in one day

**2 - Small**
- A little bit of thought, effort, or problem-solving is required
- Developers have done this a lot and have confidence in the requirements
- Or, it sounds extra small, but hedge the bet just a bit

**3 - Average**
- Developers have done this a lot; they know what needs to be done
- May have a few extra steps, but that's it
- Unlikely to need research

**5 - Large**
- Complex work, or developers don't do this very often
- Most developers will need assistance from someone else on the team
- Probably one of the largest items that can be completed within a Sprint

**8 - Extra Large**
- Going to take significant time and research
- Probably needs more than one developer to complete within two weeks
- Developers need to make several assumptions that increase the risk
- Could affect getting it Done

**13 - Warning!**
- Complex piece of work with a lot of unknowns
- Requires multiple assumptions to size
- **Too much to complete in one Sprint**
- **Should be split into multiple items** that can be completed independently

**21 - Hazard!**
- Reflects too much complexity to be done within one Sprint
- **Needs to be refined more**
- Large size indicates more risk, assumptions, and dependencies

**? - Danger!**
- As a developer, we don't want to do this work the way it's currently written
- Very complex and cannot be completed in the timeframe of an iteration/Sprint
- Requirements are so fuzzy that it's rife with danger
- **Needs clarification and breakdown before estimation**

## Estimation Guidelines

1. **Compare, don't estimate in hours** - Story points are relative, not absolute
2. **Team consensus** - Use planning poker for collaborative estimation
3. **Consider complexity, not just time** - Account for unknowns and risks
4. **Split large items** - Anything 13+ should be broken down
5. **Velocity emerges over time** - Track completed points per sprint to predict capacity
6. **Re-estimate if needed** - If work uncovers more complexity, it's okay to adjust

## Red Flags

- **13 or higher**: Too big for one sprint - break it down
- **Multiple 8s in a sprint**: Risk of overcommitment
- **Lots of ?s**: Requirements need clarification
- **Wide estimation variance**: Team doesn't understand the work the same way

## Typical Sprint Capacity

Based on Jility's sprint planning:
- Default capacity: **40-70 story points** per 2-week sprint
- Configurable per workspace (see JIL-23, JIL-26)
- Should be based on team's **historical velocity** from completed sprints

