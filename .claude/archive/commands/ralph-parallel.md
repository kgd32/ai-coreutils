---
description: Spawn multiple Ralph loops in parallel to work on non-conflicting tasks simultaneously.
arguments:
  - name: count
    description: Number of parallel loops to spawn (default: 2, max: 5)
    required: false
  - name: phase
    description: Focus on specific phase (e.g., "mvp", "phase_2", "phase_3")
    required: false
---

# Ralph Loop - Parallel Execution

<function_calls>
<invoke name="Task">
<parameter name="subagent_type">general-purpose</parameter>
<parameter name="prompt">You are the Ralph Parallel Orchestrator. Your job is to find non-conflicting tasks and spawn multiple Ralph loops in parallel.

## CRITICAL: Your Role

You are NOT doing the work yourself. You are an ORCHESTRATOR that:
1. Analyzes tasks in `.ralph/ralph.yml`
2. Finds tasks that can run in parallel
3. Spawns multiple subagents in parallel
4. Monitors their progress
5. Aggregates results

## Your Process

### PHASE 1: Load and Analyze Tasks

1. **Read `.ralph/ralph.yml`:**
   - Parse all tasks
   - Extract dependencies for each task
   - Check current status (only consider "todo" tasks)

2. **Build dependency graph:**
   ```
   task A depends on [B, C]
   task B depends on [D]
   task C depends on []
   task D depends on []
   ```

3. **Find parallelizable tasks:**

   Tasks CAN run in parallel if:
   - Status is "todo"
   - No unresolved dependencies (all deps are "done")
   - Not working on same files/components
   - In the same phase (or user-specified phase)

   Tasks CANNOT run in parallel if:
   - One depends on the other
   - Both depend on a common incomplete task
   - They work on the same files (heuristic: same directory or similar task names)

4. **Group tasks by priority:**
   - Group 1: All "critical" priority that can run in parallel
   - Group 2: All "high" priority that can run in parallel
   - Group 3: All "medium" priority that can run in parallel

### PHASE 2: Select Tasks for Parallel Execution

1. **Determine how many tasks to spawn:**
   - User specified: `$ARGUMENTS.count` (default: 2, max: 5)
   - Find at most N tasks that can run in parallel
   - Prefer higher priority tasks

2. **Example selection:**

   ```
   Available todo tasks (after dependency check):
   - task-a (critical) ✓ can run
   - task-b (critical) ✓ can run
   - task-c (critical) ✗ depends on task-a
   - task-d (high) ✓ can run
   - task-e (medium) ✓ can run

   With count=2, select: task-a, task-b (both critical)
   ```

3. **Update task statuses:**
   - For each selected task, change status from "todo" to "in-progress"
   - Save to `.ralph/ralph.yml`

### PHASE 3: Spawn Parallel Agents

Use the Task tool with `run_in_background: true` to spawn each agent:

```
For each selected task:
  Task(
    subagent_type: "general-purpose",
    prompt: [Individual agent prompt for this task],
    run_in_background: true
  )
```

**Each individual agent prompt should include:**

```
You are a Ralph Loop agent working on a SINGLE TASK.

Your assigned task: [task-id]
Task title: [task title]
Task description: [task description]
Acceptance criteria: [criteria]

## Context
- Read CLAUDE.md for project knowledge
- Read .ralph/prompt.md for workflow
- Read .ralph/scratchpad.md for context

## Your Job
1. Work ONLY on this task
2. Follow the workflow from .ralph/prompt.md
3. Update CLAUDE.md with learnings
4. When complete, mark task as "done" in .ralph/ralph.yml
5. Update .ralph/scratchpad.md with completion note
6. Output: TASK_COMPLETE: [task-id]

## Exit Marker
When your task is complete, output exactly:
TASK_COMPLETE: [task-id]

Then exit.
```

### PHASE 4: Monitor Progress

1. **Track spawned agents:**
   - Save agent IDs from Task tool results
   - Store in `.ralph/parallel-state.json`:
     ```json
     {
       "parallel_run_id": "unique-id",
       "started_at": "timestamp",
       "agents": [
         {"task_id": "task-a", "agent_id": "...", "status": "running"},
         {"task_id": "task-b", "agent_id": "...", "status": "running"}
       ],
       "tasks_assigned": ["task-a", "task-b"]
     }
     ```

2. **Wait for completion:**
   - Periodically check agent status using TaskOutput tool
   - Look for `TASK_COMPLETE: [task-id]` in output
   - Update agent status in state file

3. **Handle completion:**
   - When agent completes, verify task is marked "done" in ralph.yml
   - If task failed or agent crashed, mark task as "todo" again with notes

### PHASE 5: Aggregate and Report

When all agents complete:

1. **Generate summary:**
   ```
   ┌─────────────────────────────────────────────────────────────┐
   │              🔄 Ralph Parallel Run Complete                  │
   ├─────────────────────────────────────────────────────────────┤
   │                                                              │
   │  Parallel Run ID: abc-123                                    │
   │  Started: 2026-01-19 10:00:00                                │
   │  Completed: 2026-01-19 10:15:00                              │
   │  Duration: 15 minutes                                        │
   │                                                              │
   │  Tasks Completed:                                            │
   │  ✓ task-a (implement-auth) - 8 minutes                       │
   │  ✓ task-b (setup-database) - 12 minutes                      │
   │                                                              │
   │  Tasks Failed:                                               │
   │  ✗ task-c (config-errors) - see .ralph/scratchpad.md         │
   │                                                              │
   │  Time Saved: ~7 minutes (vs sequential)                      │
   │                                                              │
   └─────────────────────────────────────────────────────────────┘
   ```

2. **Update files:**
   - `.ralph/scratchpad.md` - Add parallel run summary
   - `.ralph/ralph.yml` - Verify all task statuses
   - `.ralph/state.json` - Update iteration counters

3. **Output final status:**
   - If all tasks complete → `LOOP_CONTINUE` (more work to do)
   - If phase complete → `LOOP_COMPLETE_PHASE_N`
   - If all complete → `LOOP_COMPLETE`

## Conflict Detection Heuristics

Tasks might conflict if:

1. **Same file paths:**
   ```
   task-a: "Implement user model in src/models/user.ts"
   task-b: "Add user validation in src/models/user.ts"
   → CONFLICT: Both work on src/models/user.ts
   ```

2. **Same component:**
   ```
   task-a: "Implement auth service"
   task-b: "Add auth middleware"
   → CONFLICT: Both affect auth system
   ```

3. **Shared dependencies:**
   ```
   task-a: depends on [setup-config]
   task-b: depends on [setup-config]
   setup-config: status = "todo"
   → CONFLICT: Both wait on same incomplete task
   ```

## Safe to Parallelize

```
✓ Different components (auth + database)
✓ Independent features (user-mgmt + notifications)
✓ Tests for different modules
✓ Documentation for different sections
```

## BEGIN

1. Read .ralph/ralph.yml
2. Build dependency graph
3. Find $ARGUMENTS.count (default: 2) parallelizable tasks
4. Update task statuses to "in-progress"
5. Spawn agents in parallel with run_in_background=true
6. Monitor and aggregate results
7. Report summary</parameter>
<parameter name="description">Orchestrate parallel Ralph loops</parameter>
<parameter name="model">sonnet</parameter>
</invoke>
</function_calls>

---

**Parallel Ralph loops spawned.** Check `.ralph/parallel-state.json` for agent status.
