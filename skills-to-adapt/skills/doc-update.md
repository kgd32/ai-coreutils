# Documentation Update Skill

> **Purpose**: Document specific feature/change with targeted updates
> **Mode**: Read-only code access, documentation write access

---

## Core Principle: Document ALL Changes

**CRITICAL**: This skill handles documentation for **ANY** work completion:

- **Every phase** (Phases 1-14)
- **Every task** (all sizes, all complexities)
- **Every feature** (major or minor)
- **Every bug fix** (trivial or complex)
- **Every test** (all test types)
- **Every refactor** (any cleanup)
- **Every config change** (any modification)

> **Rule**: All code changes require documentation updates.

---

## Invocation

```
skill: "doc-update" --target "<topic>"
```

## Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `target` | string | Yes | Topic or feature to document |

### Examples

```
skill: "doc-update" --target "phase 7 completion"
skill: "doc-update" --target "crash recovery testing"
skill: "doc-update" --target "new planner machine pattern"
skill: "doc-update" --target "XState actor lifecycle changes"
```

---

## Behavior

### Step 1: Analyze Target

Parse the target topic to determine:

1. **Type**: Phase completion, feature, bug fix, testing, etc.
2. **Scope**: Which documentation files are affected
3. **Keywords**: Terms to search in codebase

### Step 2: Read Relevant Code (Read-Only)

Based on target type, read appropriate code files:

| Target Type | Files to Read |
|-------------|---------------|
| Phase completion | `src/orchestration/machines/*.ts`, `src/orchestration/actors/*.ts` |
| Feature addition | Feature-specific source files |
| Bug fix | Fixed files + related tests |
| Testing | Test files + implementation |
| Architecture | `src/orchestration/*.ts` |

### Step 3: Identify Affected Documentation

Determine which docs need updates:

| Target | Affected Docs |
|--------|---------------|
| Phase N completion | `XSTATE-MIGRATION-PLAN.md`, `CLAUDE.md`, roadmaps |
| New feature | `ARCHITECTURE.md`, feature-specific docs |
| Bug fix | `XSTATE-BEST-PRACTICES.md`, session logs |
| Testing | Session logs, `PHASE-*-COMPLETE.md` |
| Architecture | `ARCHITECTURE.md`, `CLAUDE.md` |

### Step 4: Create or Update Documentation

#### For Phase Completions

1. **Update Migration Plan** (`XSTATE-MIGRATION-PLAN.md`):
   ```markdown
   ### Phase N: [Name] ✅
   - **Status**: Complete
   - **Completed**: YYYY-MM-DD
   - **Session**: [session-link]
   ```

2. **Update CLAUDE.md**:
   ```markdown
   **Phase**: N Complete ✅
   ```

3. **Create Session Log**:
   ```markdown
   # Phase N Completion

   **Date**: YYYY-MM-DD
   **Branch**: xstate-migration-gemini

   ## Executive Summary
   - **Status**: ✅ Complete
   - **Duration**: [time]
   - **Outcome**: [result]
   ```

4. **Create Completion Document**:
   - `docs/PHASE-N-COMPLETE.md` with verification results

#### For Features

1. **Update Architecture** (`ARCHITECTURE.md`):
   - Add feature description to relevant section
   - Update diagrams if needed

2. **Update CLAUDE.md**:
   - Add to "Quick Reference" section

3. **Create Feature Doc** (if needed):
   - `docs/FEATURE-[name].md`

#### For Bug Fixes

1. **Update Session Log**:
   ```markdown
   # Bug Fix: [Description]

   ## Issue
   [Problem description]

   ## Root Cause
   [Technical details]

   ## Fix
   [Solution implemented]

   ## Files Changed
   - `path/to/file.ts:line` - [change]
   ```

2. **Update Best Practices** (`XSTATE-BEST-PRACTICES.md`):
   - Add anti-pattern if applicable

### Step 5: Update Cross-References

Ensure all related documents reference each other:

```markdown
See [Phase N Complete](docs/PHASE-N-COMPLETE.md) for details.
Reference: [Architecture](docs/ARCHITECTURE.md)
```

### Step 6: Commit and Push

```bash
git add docs/ sessions/ CLAUDE.md
git commit -m "docs: [target] - [brief summary]"
git push
```

---

## Target Type Handling

### Phase Completion

```
target: "phase N completion"
```

**Actions**:
1. Update `XSTATE-MIGRATION-PLAN.md` phase status
2. Create `PHASE-N-COMPLETE.md` verification doc
3. Update `CLAUDE.md` current phase
4. Create session log with completion summary
5. Update any related roadmap documents

### Feature Addition

```
target: "new feature name"
```

**Actions**:
1. Document feature in `ARCHITECTURE.md`
2. Add usage examples to `CLAUDE.md`
3. Create feature-specific doc if complex
4. Add to session log
5. Update cross-references

### Bug Fix

```
target: "bug fix: description"
```

**Actions**:
1. Create session log with bug details
2. Update `XSTATE-BEST-PRACTICES.md` if anti-pattern
3. Update `CLAUDE.md` if user-facing
4. Document workaround if applicable

### Testing/Verification

```
target: "testing: phase N"
target: "e2e verification"
```

**Actions**:
1. Create session log with test results
2. Update `PHASE-N-COMPLETE.md` with verification status
3. Document any issues found
4. Update `IMPROVEMENTS.md` if issues remain

### Architecture Change

```
target: "architecture: new component"
```

**Actions**:
1. Update `ARCHITECTURE.md` diagrams
2. Document new component responsibilities
3. Update integration points
4. Add to session log

### New Area (Web UI, Mobile, API, etc.)

```
target: "new area: web ui"
target: "new area: mobile app"
target: "new area: REST API"
target: "new area: dashboard"
```

**IMPORTANT**: For entirely new areas, the doc-agent must **create** the documentation structure, not just update existing docs.

**Actions**:

1. **Create Area Documentation** (`docs/AREA-NAME.md`):
   ```markdown
   # [Area Name] Documentation

   > **Purpose**: [What this area does]
   > **Status**: [In Development/Alpha/Beta/Production]
   > **Last Updated**: YYYY-MM-DD

   ## Overview
   [High-level description of the area]

   ## Architecture
   [Technical architecture, components, data flow]

   ## Setup
   [Installation and configuration instructions]

   ## Usage
   [How to use this area]

   ## API Reference (if applicable)
   [API endpoints, methods, etc.]

   ## Development
   [How to develop/extend this area]

   ## Testing
   [How to test this area]

   ## Related Documentation
   - [Link to related docs]
   ```

2. **Update Main Architecture** (`ARCHITECTURE.md`):
   - Add new area to system overview diagram
   - Document integration points with existing system
   - Add data flow between new area and existing components

3. **Update CLAUDE.md**:
   - Add area to "File Locations" section
   - Add area-specific commands to "Quick Start"
   - Reference new area documentation

4. **Create Session Log**:
   - Document area creation
   - List all files created
   - Note any decisions made

5. **Update Expertise Files** (if applicable):
   - Add area-specific patterns to `doc-expertise.md`
   - Document area-specific best practices

**Example: Web UI Area**

```
target: "new area: web ui"
```

**Creates**:
- `docs/WEB-UI.md` - Main Web UI documentation
- `docs/WEB-UI-COMPONENTS.md` - Component library (if needed)
- `docs/WEB-UI-STATE-MANAGEMENT.md` - State management docs (if needed)
- `sessions/2026-01-18_web-ui-area-creation.md` - Session log

**Updates**:
- `ARCHITECTURE.md` - Add Web UI to system diagram
- `CLAUDE.md` - Add Web UI section and commands
- `README.md` - Add Web UI to overview (if public-facing)

### Architectural Decision (ADR)

```
target: "adr: zmq pattern choice"
target: "adr: sqlite indexing strategy"
target: "adr: library integration: xstate"
target: "adr: error handling strategy"
```

**IMPORTANT**: Every structural/architectural choice MUST be documented as an ADR.

**Actions**:

1. **Determine next ADR number**:
   ```bash
   ls docs/adr/ | wc -l  # Count existing ADRs
   # Next number = count + 1, padded to 3 digits
   ```

2. **Create ADR file** (`docs/adr/NNN-short-title.md`):
   ```markdown
   # ADR NNN: [Short Title]

   **Status**: Accepted
   **Date**: YYYY-MM-DD
   **Context**: [What led to this decision?]
   **Related**: [Link to related ADRs]

   ## Context

   [What is the situation? What problem are we solving?]

   ## Decision

   [What change are we making/proposing?]

   ## Consequences

   - **Positive**: [Benefits]
   - **Negative**: [Drawbacks/risks]
   - **Neutral**: [Neither good nor bad]

   ## Alternatives Considered

   1. **[Alternative 1]**: [Description] - Rejected because [reason]
   2. **[Alternative 2]**: [Description] - Rejected because [reason]

   ## Implementation

   - **Files**: [List of files implementing this decision]
   - **Code References**: [Links to relevant code]
   - **Tests**: [How this is tested]

   ## References

   - [Link to related documentation]
   - [Link to issues/discussions]
   ```

3. **Update ADR index** (`docs/adr/index.md`):
   ```markdown
   # Architecture Decision Records

   | ADR | Title | Status | Date |
   |-----|-------|--------|------|
   | [001](001-choose-zmq.md) | Choose ZeroMQ for Messaging | Accepted | 2026-01-18 |
   | [002](002-sqlite-indexing.md) | SQLite Indexing Strategy | Accepted | 2026-01-18 |
   ```

4. **Update System Design** (`docs/system-design.md`):
   - Reference the new ADR in relevant sections
   - Update component interaction map if needed
   - Update technology stack table

5. **Create Session Log**:
   - Document the decision and rationale
   - Link to the ADR
   - Note any alternatives considered

**Example: ZMQ Pattern Choice**

```
target: "adr: zmq push-pull for task distribution"
```

**Creates**:
- `docs/adr/003-zmq-push-pull-task-distribution.md`
- Updates to `docs/adr/index.md`
- Updates to `docs/system-design.md` (component interactions)
- Session log with decision rationale

### System Design Update

```
target: "system design: add component"
target: "system design: update interaction"
target: "system design: new data flow"
```

**IMPORTANT**: `docs/system-design.md` is the "Source of Truth" for how Agent Village components interact.

**Actions**:

1. **Update `docs/system-design.md`**:
   - Add/modify component in Component Overview
   - Update Component Interactions section
   - Update Data Structures section
   - Update Technology Stack table
   - Update Deployment topology

2. **Reference relevant ADRs**:
   - Link to ADRs that justify design choices
   - Ensure consistency between ADRs and system design

3. **Create Session Log**:
   - Document what changed in system design
   - Reference updated sections
   - Link to related ADRs

**System Design Sections**:

```markdown
# Agent Village System Design

## Component Overview
[Diagram and descriptions]

## Component Interactions
### Planner ↔ Broker
- **Protocol**: ZeroMQ DEALER/ROUTER
- **Pattern**: RPC
- **Ports**: 5557
- **ADR**: [001](adr/001-choose-zmq.md)

## Data Structures
### Database Schema
- Tables, indexes, relationships
- **ADR**: [002](adr/002-sqlite-indexing.md)

### Message Formats
- Inter-agent communication

## Technology Stack
| Component | Technology | Version | ADR |
|-----------|-----------|---------|-----|
| State Machines | XState | 5.x | [005](adr/005-xstate-choice.md) |
| Messaging | ZeroMQ | latest | [001](adr/001-choose-zmq.md) |

## Deployment
[Process topology, ports, dependencies]
```

### Technical Debt

```
target: "tech debt: hack description"
target: "tech debt: workaround for issue"
target: "tech debt: temporary solution"
```

**CRITICAL**: Every hack, temporary workaround, or compromise MUST be recorded as technical debt.

**Actions**:

1. **Create debt file** (`docs/debt/tech-debt-{timestamp}-{id}.md`):
   ```markdown
   # Technical Debt: {Short Title}

   > **ID**: tech-debt-{timestamp}-{id}
   > **Created**: YYYY-MM-DD
   > **Severity**: 🔴 Critical | 🟡 Medium | 🟢 Low
   > **Status**: Open

   ## Description
   [What is the hack/workaround? Why was it necessary?]

   ## Context
   - **Deadline**: [What deadline forced this?]
   - **Impact**: [What does this affect?]
   - **Temporary Solution**: [What was done?]

   ## The Problem
   [What issue does this create?]

   ## Proper Solution
   [How should this be properly implemented?]

   ## Estimated Effort
   - **Time**: [hours/days]
   - **Complexity**: [Low/Medium/High]
   - **Risk**: [Risk if not fixed]

   ## References
   - **Files**: `src/path/to/file.ts:line`
   - **Related ADR**: [Link if applicable]

   ## Acceptance Criteria
   - [ ] Proper solution implemented
   - [ ] Hack/workaround removed
   - [ ] Tests added
   - [ ] Documentation updated
   ```

2. **Update debt index** (`docs/debt/index.md`):
   - Add entry to appropriate severity section
   - Update counts

3. **Update CLAUDE.md**:
   - Add to "Technical Debt" section
   - Include link to debt file
   - Update severity counts

4. **Update Roadmap**:
   - Flag affected phase with debt reference
   - Add debt item to phase blocking issues

5. **Create Session Log**:
   - Document the hack/workaround
   - Link to debt file
   - Note why proper solution was deferred

**Example: Technical Debt**

```
target: "tech debt: actor stop workaround"
```

**Creates**:
- `docs/debt/tech-debt-20260118-001-actor-stop-workaround.md`
- Updates to `docs/debt/index.md`

**Updates**:
- `CLAUDE.md` - Adds to Technical Debt section
- Roadmap - Flags Phase 7 with debt reference
- `docs/IMPROVEMENTS.md` - Links to debt item

**Context**:
- **Severity**: 🟡 Medium
- **Description**: Actor cleanup doesn't wait for child actors to stop
- **Impact**: Memory leak in long-running processes
- **Deadline**: Phase 7 completion deadline
- **Proper solution**: Implement proper shutdown with timeout
- **Files**: `src/orchestration/actors/planner.actor.ts:89`

---

## Output

### Console Output
```
Documentation Update: [target]
Analyzing target...
Reading relevant code...
Identifying affected docs...
Updating documentation...
- Updated: XSTATE-MIGRATION-PLAN.md
- Created: PHASE-7-COMPLETE.md
- Updated: CLAUDE.md
- Created: sessions/2026-01-18_phase-7-completion.md
Committed and pushed.
```

### Session Log Format

```markdown
# Session: [Target Title]

**Date**: YYYY-MM-DD
**Branch**: xstate-migration-gemini

## Executive Summary
- **Type**: [Phase Completion/Feature/Bug Fix/Testing]
- **Status**: [Completed/Verified]
- **Outcome**: [Brief result]

## Details

### Changes Made
[List of documentation updates]

### Files Referenced
- [File with line numbers]

### Related Documentation
- [Links to related docs]
```

---

## Common Target Patterns

### Phase Completion
```
"phase 7"
"phase 7 complete"
"phase 7 completion"
"phase 7 verification"
```

### Features
```
"new feature: zero mq bridge"
"feature: crash recovery"
"feature: task retry logic"
```

### Bug Fixes
```
"bug fix: actor lifecycle"
"fix: memory leak"
"hotfix: state corruption"
```

### Testing
```
"testing: crash recovery"
"e2e: phase 6"
"verification: phase 7"
```

### New Areas
```
"new area: web ui"
"new area: mobile app"
"new area: REST API"
"new area: dashboard"
"new area: admin panel"
"new area: cli"
"new area: monitoring"
```

### Architectural Decisions (ADRs)
```
"adr: zmq pattern choice"
"adr: sqlite indexing strategy"
"adr: library integration: xstate"
"adr: error handling strategy"
"adr: crash recovery approach"
"adr: state management pattern"
"adr: agent communication protocol"
```

### System Design Updates
```
"system design: add component"
"system design: update interaction"
"system design: new data flow"
"system design: deployment topology"
```

### Technical Debt
```
"tech debt: actor stop workaround"
"tech debt: missing error handling"
"tech debt: hardcoded config values"
"tech debt: temporary api bypass"
"tech debt: performance compromise"
"tech debt: missing tests"
```

---

## Important Notes

- **Read-Only Code**: Only reads code for understanding, **except TSDoc comments**
- **TSDoc Exception**: TSDoc comments are added to code files (`.ts`) to document exports
- **ADR Requirement**: Every architectural choice must have an ADR in `docs/adr/`
- **System Design**: `docs/system-design.md` is the source of truth for component interactions
- **Tech Debt**: Every hack/workaround MUST have a debt record in `docs/debt/`
- **Documentation Files**: Writes to `docs/`, `sessions/`, `.claude/` + TSDoc in `.ts` files
- **Auto-Commit**: Always commits and pushes changes
- **Session Log**: Creates dated session log for every update
- **Cross-References**: Updates all related documents
- **Current Branch**: Always commits to `xstate-migration-gemini`

### Files Modified During Update

**Documentation Files** (always):
- `docs/*.md`
- `docs/adr/*.md` (for ADRs)
- `docs/system-design.md` (for system design changes)
- `docs/debt/*.md` (for technical debt)
- `docs/debt/index.md` (debt tracking)
- `sessions/*.md`
- `CLAUDE.md`
- `.claude/*`

**Code Files** (TSDoc only):
- `src/**/*.ts` - Only to add TSDoc comments to exports
- No logic changes, only comments

---

---

## Examples

### Example 1: Phase 7 Completion
```
skill: "doc-update" --target "phase 7"
```

**Creates**:
- `docs/PHASE-7-COMPLETE.md`
- `sessions/2026-01-18_phase-7-completion.md`

**Updates**:
- `XSTATE-MIGRATION-PLAN.md` - Phase 7 status
- `CLAUDE.md` - Current phase

### Example 2: Bug Fix
```
skill: "doc-update" --target "bug fix: actor memory leak"
```

**Creates**:
- `sessions/2026-01-18_actor-memory-leak-fix.md`

**Updates**:
- `XSTATE-BEST-PRACTICES.md` - Anti-pattern section
- `IMPROVEMENTS.md` - Issue marked resolved

### Example 3: New Area - Web UI
```
skill: "doc-update" --target "new area: web ui"
```

**Creates**:
- `docs/WEB-UI.md` - Main Web UI documentation
- `docs/WEB-UI-COMPONENTS.md` - Component library
- `docs/WEB-UI-STATE-MANAGEMENT.md` - State management
- `sessions/2026-01-18_web-ui-area-creation.md` - Session log

**Updates**:
- `ARCHITECTURE.md` - Adds Web UI to system diagram
- `CLAUDE.md` - Adds Web UI section with commands
- `README.md` - Adds Web UI to overview

**Context**:
- **Area**: Web UI
- **Technology**: React/Next.js (for example)
- **Purpose**: User interface for Agent Village
- **Integration**: Connects to existing ZeroMQ broker
- **Files created**: `src/web-ui/` directory structure

### Example 4: Architectural Decision - ZMQ Pattern
```
skill: "doc-update" --target "adr: zmq push-pull for task distribution"
```

**Creates**:
- `docs/adr/003-zmq-push-pull-task-distribution.md` - Full ADR document
- `sessions/2026-01-18_adr-zmq-push-pull.md` - Session log

**Updates**:
- `docs/adr/index.md` - Adds ADR to index
- `docs/system-design.md` - Updates component interactions with ADR reference
- `CLAUDE.md` - References ADR if user-facing

**Context**:
- **Decision**: Use PUSH/PULL pattern for task distribution
- **Alternatives considered**: PUB/SUB (rejected - no load balancing), DEALER/ROUTER (rejected - too complex for one-way)
- **Positive consequences**: Simple load balancing, backpressure handling
- **Negative consequences**: Requires workers to actively pull tasks
- **Implementation**: `src/services/zeromq-broker.ts:55`
- **Files**: Broker at port 5555, implementers connect as PULL

### Example 5: System Design Update - New Component
```
skill: "doc-update" --target "system design: add crash recovery service"
```

**Creates**:
- `sessions/2026-01-18_system-design-crash-recovery.md` - Session log
- `docs/adr/004-crash-recovery-strategy.md` - ADR if new architectural choice

**Updates**:
- `docs/system-design.md`:
  - Adds Crash Recovery Service to Component Overview
  - Adds Planner ↔ Crash Recovery interaction
  - Updates Technology Stack table
  - Updates Deployment topology

**Context**:
- **Component**: Crash Recovery Service
- **Purpose**: Monitor and restart crashed agents
- **Protocol**: ZeroMQ PUB/SUB for monitoring
- **ADR**: Links to crash recovery strategy ADR
- **Files**: `src/services/crash-recovery.ts`

### Example 6: Technical Debt - Actor Stop Workaround
```
skill: "doc-update" --target "tech debt: actor stop workaround"
```

**Creates**:
- `docs/debt/tech-debt-20260118-001-actor-stop-workaround.md` - Full debt record
- `sessions/2026-01-18_tech-debt-actor-stop.md` - Session log

**Updates**:
- `docs/debt/index.md` - Adds debt to index
- `CLAUDE.md` - Adds to Technical Debt section
- Roadmap - Flags Phase 7 with debt reference
- `docs/IMPROVEMENTS.md` - Links to debt item

**Context**:
- **Severity**: 🟡 Medium
- **Description**: Actor cleanup doesn't wait for child actors to stop
- **Impact**: Memory leak in long-running processes
- **Deadline**: Phase 7 completion deadline forced workaround
- **Proper solution**: Implement proper shutdown with timeout
- **Estimated effort**: 4 hours, Medium complexity
- **Files**: `src/orchestration/actors/planner.actor.ts:89`
- **Assigned to**: Phase 8 (next iteration)

---

**Related Skills**:
- `doc-agent.md` - Main entry point
- `doc-sweep.md` - Comprehensive sweep
- `doc-fixes.md` - Improvements tracker
- `doc-expertise.md` - Shared knowledge

**Last Updated**: 2026-01-18
