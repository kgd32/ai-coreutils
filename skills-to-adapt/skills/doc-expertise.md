# Documentation Agent - Expertise Knowledge Base

> **Purpose**: Shared senior-level context for all documentation skills
> **Last Updated**: 2026-01-18

---

## Core Principle: Document ALL Work

**CRITICAL**: The documentation agent should be spawned after **ANY** work completion:

- **Every phase** (Phases 1-14)
- **Every task** (regardless of size)
- **Every feature** (no matter how small)
- **Every bug fix** (even trivial ones)
- **Every test** (unit tests, E2E, verification)
- **Every refactor** (code cleanup, reorganization)
- **Every config change** (any configuration modification)
- **Every architectural choice** (ADRs for all structural decisions)

> **Rule**: If files were changed, documentation must be updated.

---

## Architecture Decision Records (ADRs)

**IMPORTANT**: Every structural choice MUST be documented as an ADR in `docs/adr/`.

### What Requires an ADR

Create an ADR for:

- **ZMQ patterns** (PUSH/PULL vs PUB/SUB vs DEALER/ROUTER choices)
- **Database schema** (indexing strategies, table relationships)
- **Library integrations** (why XState, why ZeroMQ, why SQLite)
- **State management** (how state flows between components)
- **Communication protocols** (agent-to-agent messaging patterns)
- **Error handling strategies** (crash recovery, retry logic)
- **Performance decisions** (caching, connection pooling)
- **Security choices** (authentication, authorization, encryption)
- **Deployment architecture** (how components are deployed)

### ADR Template

```markdown
# ADR NNN: [Short Title]

**Status**: Accepted | Proposed | Deprecated | Superseded
**Date**: YYYY-MM-DD
**Decision Makers**: [Who made the decision]
**Related**: [Link to related ADRs]

## Context

[What is the situation that led to this decision? What problem are we trying to solve?]

## Decision

[What is the change that we're proposing and/or doing?]

## Consequences

- **Positive**: [Benefits of this decision]
- **Negative**: [Drawbacks or risks]
- **Neutral**: [Things that are neither good nor bad]

## Alternatives Considered

1. **[Alternative 1]**: [Description] - Rejected because [reason]
2. **[Alternative 2]**: [Description] - Rejected because [reason]

## Implementation

[How was this decision implemented? References to code/files]

## References

- [Link to relevant documentation]
- [Link to related issues/discussions]
```

### ADR Numbering

- ADRs are numbered sequentially: `001`, `002`, `003`, etc.
- Use 3-digit padding: `001-choose-zmq.md`, `002-sqlite-indexing.md`
- Check existing ADRs to determine next number

### ADR Storage

- Location: `docs/adr/NNN-short-title.md`
- Index: `docs/adr/index.md` (auto-generated list of all ADRs)

---

## System Design: Source of Truth

**IMPORTANT**: Maintain `docs/system-design.md` as the authoritative map of Agent Village interactions.

### What System Design Contains

The system design document tracks:

1. **Component Interaction Map**
   - How each component connects to others
   - Data flow between components
   - Communication patterns (ZMQ, HTTP, etc.)

2. **Data Structures**
   - Database schemas and relationships
   - Message formats between agents
   - State machine structures

3. **Technology Stack**
   - All libraries and frameworks used
   - Why each was chosen (reference to ADR)
   - Version requirements

4. **Deployment Topology**
   - How components are deployed
   - Port allocations
   - Process dependencies

### System Design Template

```markdown
# Agent Village System Design

> **Purpose**: Source of truth for Agent Village architecture
> **Last Updated**: YYYY-MM-DD

## Component Overview

[High-level diagram and description]

## Component Interactions

### [Component A] ↔ [Component B]
- **Protocol**: [ZMQ/HTTP/etc.]
- **Pattern**: [PUSH/PULL, PUB/SUB, etc.]
- **Data Format**: [JSON, MessagePack, etc.]
- **Ports**: [5555, 3000, etc.]
- **ADR**: [Link to relevant ADR]

## Data Structures

### [Database Schema]
- Tables, indexes, relationships
- ADR reference for structural choices

### [Message Formats]
- Inter-agent communication formats
- State machine context structures

## Technology Stack

| Component | Technology | Version | ADR |
|-----------|-----------|---------|-----|
| State Machines | XState | 5.x | [ADR] |
| Messaging | ZeroMQ | latest | [ADR] |
| Database | SQLite | 3.x | [ADR] |

## Deployment

[Process topology, port allocations, etc.]
```

### When to Update System Design

Update `docs/system-design.md` when:

- New component added
- Component interaction changes
- New technology added
- Data structure changes
- Deployment topology changes

---

## TSDoc Documentation

**IMPORTANT**: All exported interfaces and classes MUST have TSDoc comments.

### When to Add TSDoc

Add TSDoc during:

- **Sweep mode**: Scan all exported types and add missing TSDoc
- **Update mode**: Add TSDoc to newly created/modified exports
- **New code**: Ensure all new exports have TSDoc

### TSDoc Template

```typescript
/**
 * Brief description of the type/function.
 *
 * @remarks
 * Detailed explanation if needed. Can include:
 * - Usage examples
 * - Implementation notes
 * - Performance considerations
 *
 * @example
 * ```typescript
 * const result = createPlannerMachine(bridge);
 * ```
 *
 * @param param1 - Description of parameter
 * @param param2 - Description of parameter
 * @returns Description of return value
 *
 * @throws {@link ErrorName} - When error occurs
 *
 * @beta - For unstable APIs (optional)
 * @internal - For internal APIs (optional)
 * @public - For public APIs (optional)
 */
export interface MyInterface {
  /**
   * Description of property.
   *
   * @remarks
   * Additional context if needed.
   */
  property: string;
}
```

### TSDoc for Different Types

**Interfaces**:
```typescript
/**
 * Configuration for the planner machine.
 *
 * @remarks
 * Contains all settings needed to create and run the planner state machine.
 * The bridge provides infrastructure adapters for external services.
 */
export interface PlannerMachineConfig {
  /** The A2A bridge for agent communication. */
  bridge: A2ABridge;

  /** Maximum number of retries for failed operations. @defaultValue 3 */
  maxRetries?: number;
}
```

**Classes**:
```typescript
/**
 * Manages agent lifecycle and communication.
 *
 * @remarks
 * The Village class is the main orchestrator for Agent Village.
 * It initializes all services and manages agent processes.
 *
 * @example
 * ```typescript
 * const village = Village.load({ villagePath: '.agent-village' });
 * await village.start();
 * ```
 */
export class Village {
  // ...
}
```

**Functions**:
```typescript
/**
 * Creates a planner machine with the given configuration.
 *
 * @param config - The planner machine configuration
 * @returns A configured planner machine
 *
 * @example
 * ```typescript
 * const machine = createPlannerMachine({
 *   bridge: myBridge,
 *   maxRetries: 5
 * });
 * ```
 */
export function createPlannerMachine(config: PlannerMachineConfig): {
  // ...
}
```

**Type Aliases**:
```typescript
/**
 * Represents the possible states of a task.
 *
 * @remarks
 * Tasks flow through these states as they are processed.
 * The state transitions are validated by the bead service.
 */
export type TaskState = 'ready' | 'in_progress' | 'review_requested' | 'approved' | 'complete' | 'blocked';
```

### TSDoc Tags to Use

| Tag | Purpose | Example |
|-----|---------|---------|
| `@param` | Document parameters | `@param bridge - The A2A bridge` |
| `@returns` | Document return value | `@returns A configured machine` |
| `@throws` | Document errors | `@throws {@link BridgeError} - When bridge fails` |
| `@example` | Show usage | `@example \`const x = foo()\`` |
| `@remarks` | Additional details | Extended explanation |
| `@defaultValue` | Default value | `@defaultValue 3` |
| `@beta` | Unstable API | For experimental features |
| `@internal` | Internal only | For private APIs |
| `@public` | Public API | Explicit public API |
| `@see` | Cross-reference | `@see {@link createPlannerMachine}` |
| `@link` | Link to symbol | `{@link A2ABridge}` |

---

## Documentation Consolidation

**IMPORTANT**: Scattered documentation should be consolidated into a single source of truth.

### When to Consolidate

Consolidate documentation when:

- Same information exists in multiple files
- Conflicting information across documents
- Information is split between outdated and current docs
- Multiple docs cover overlapping topics
- Session logs contain reference material that should be in main docs

### Consolidation Rules

1. **One Source of Truth**: Each piece of information should exist in ONE primary location
2. **Cross-Reference**: Other locations should link to the source of truth
3. **Archive, Don't Delete**: Move outdated docs to archive, don't delete them
4. **Update References**: When consolidating, update all references to point to new location

### Consolidation Process

1. **Identify Duplicates**:
   - Search for similar topics across docs
   - Find conflicting information
   - Locate reference material in session logs

2. **Choose Primary Location**:
   - Prefer `docs/` over `sessions/` for reference material
   - Prefer specific docs over general docs
   - Prefer newer docs over older docs

3. **Consolidate Content**:
   - Merge duplicate sections
   - Resolve conflicts (prefer current implementation)
   - Organize logically

4. **Update Cross-References**:
   - Update all links to point to consolidated location
   - Add "See also" references where appropriate

5. **Archive Outdated Docs**:
   - Move to `docs/archive/` directory
   - Add redirect notice at top of archived file
   - Update index files

### Archive Template

For archived files, add at the top:

```markdown
> **⚠️ ARCHIVED**: This document has been archived and is kept for historical reference only.
> **Archived Date**: YYYY-MM-DD
> **Superseded By**: [Link to current document]
> **Reason**: [Why this was archived - replaced, obsolete, etc.]

---

[Original document content follows...]
```

### Archive Storage

- Location: `docs/archive/YYYY-MM-DD-original-name.md`
- Index: `docs/archive/index.md` (auto-generated list of archived docs)
- Naming: Prefix with date for easy sorting

---

## Documentation: Source of Truth Locations

**IMPORTANT**: Keep documentation in its proper location to maintain a single source of truth.

### Primary Documentation Locations

| Information Type | Primary Location | Examples |
|-----------------|------------------|----------|
| Architecture overview | `docs/ARCHITECTURE.md` | System design, components |
| XState patterns | `docs/XSTATE-BEST-PRACTICES.md` | State machine patterns |
| API reference | `docs/API.md` or area-specific | Endpoints, methods |
| Configuration | `docs/CONFIGURATION.md` | Settings, options |
| Development | `docs/DEVELOPMENT.md` | Setup, building, testing |
| Deployment | `docs/DEPLOYMENT.md` | Installation, operations |
| Migration plans | `docs/XSTATE-MIGRATION-PLAN.md` | Phase tracking |
| ADRs | `docs/adr/NNN-title.md` | Architectural decisions |
| System design | `docs/system-design.md` | Component interactions |
| Session logs | `sessions/YYYY-MM-DD_*.md` | Temporary work records |
| Quick reference | `CLAUDE.md` | Cheat sheet for agents |

### Session Logs vs Documentation

**Session Logs** (`sessions/*.md`):
- Temporary record of work done
- Not reference material
- Can be extracted into proper docs
- Eventually archived

**Documentation** (`docs/*.md`):
- Permanent reference material
- Source of truth
- Maintained and updated
- Cross-referenced

### When to Move from Session Logs to Docs

Extract content from session logs to documentation when:

- **Reference material**: Code examples, patterns, best practices
- **Decisions made**: Architectural choices (create ADR instead)
- **Procedures**: How-to guides, setup instructions
- **Troubleshooting**: Common issues and solutions

---

## Technical Debt Prevention

**CRITICAL**: Every hack, temporary workaround, or compromise MUST be recorded as technical debt.

### What Requires Technical Debt Tracking

Create a technical debt record for:

- **Hacks** - Quick fixes that bypass proper implementation
- **Temporary workarounds** - Short-term solutions to meet deadlines
- **Known issues** - Bugs deferred for later
- **Performance compromises** - Slower implementations for speed
- **Missing features** - Features skipped to ship faster
- **Code duplication** - Copied code instead of proper refactoring
- **Missing tests** - Untested code due to time constraints
- **TODO/FIXME comments** - Any code with TODO/FIXME that represents debt

### Technical Debt Template

```markdown
# Technical Debt: {Short Title}

> **ID**: tech-debt-{timestamp}-{id}
> **Created**: YYYY-MM-DD
> **Severity**: 🔴 Critical | 🟡 Medium | 🟢 Low
> **Status**: Open | In Progress | Resolved
> **Assigned To**: [Who should fix this]

## Description

[What is the hack/workaround? Why was it necessary?]

## Context

- **Deadline**: [What deadline forced this compromise?]
- **Impact**: [What does this affect?]
- **Temporary Solution**: [What was done instead?]

## The Problem

[Describe the issue that the hack/workaround creates]

## Proper Solution

[How this should be properly implemented]

## Estimated Effort

- **Time**: [Estimated hours/days]
- **Complexity**: [Low/Medium/High]
- **Risk**: [Risk level if not fixed]

## Dependencies

- [ ] Other tech debt items
- [ ] Feature work
- [ ] Refactoring work

## References

- **Files**: `src/path/to/file.ts:line`
- **Related ADR**: [Link if architectural debt]
- **Related Issue**: [Issue tracker link]

## Acceptance Criteria

- [ ] Proper solution implemented
- [ ] Hack/workaround removed
- [ ] Tests added
- [ ] Documentation updated
```

### Technical Debt File Naming

- **Pattern**: `tech-debt-{timestamp}-{id}.md`
- **Example**: `tech-debt-20260118-001-actor-stop-leak.md`
- **Location**: `docs/debt/`

### Technical Debt Severity

| Severity | Badge | When to Use | Fix Timeline |
|----------|-------|-------------|--------------|
| **Critical** | 🔴 | Security risks, data loss, crashes | Immediate |
| **Medium** | 🟡 | Performance issues, maintainability | Next sprint |
| **Low** | 🟢 | Nice-to-have, minor issues | Backlog |

### Technical Debt Tracking Locations

1. **Individual Debt Files**: `docs/debt/tech-debt-{timestamp}-{id}.md`
2. **Debt Index**: `docs/debt/index.md` (auto-generated)
3. **CLAUDE.md**: Add to "Technical Debt" section for visibility
4. **Roadmap**: Flag relevant phases with debt items
5. **IMPROVEMENTS.md**: Cross-reference for tracking

### Debt Index Template

```markdown
# Technical Debt Index

> **Total Items**: N
> **Critical**: N | **Medium**: N | **Low**: N

## Critical Debt (🔴)

| ID | Title | Created | Severity | Status | Assigned To |
|----|-------|---------|----------|--------|-------------|
| [001](tech-debt-20260118-001.md) | Actor Stop Leak | 2026-01-18 | 🔴 | Open | - |

## Medium Debt (🟡)

| ID | Title | Created | Severity | Status | Assigned To |
|----|-------|---------|----------|--------|-------------|
| [002](tech-debt-20260118-002.md) | Missing TSDoc | 2026-01-18 | 🟡 | In Progress | - |

## Low Debt (🟢)

| ID | Title | Created | Severity | Status | Assigned To |
|----|-------|---------|----------|--------|-------------|
| [003](tech-debt-20260118-003.md) | Code Duplication | 2026-01-18 | 🟢 | Open | - |
```

### When to Create Technical Debt Records

**During implementation**:
```typescript
// HACK: Temporary workaround to meet deadline
// Tech Debt: docs/debt/tech-debt-20260118-001.md
// TODO: Properly implement actor cleanup in Phase 8
function stopActor(actor: Actor) {
  // Temporary: just kill the actor
  actor.stop(); // Doesn't wait for cleanup
  // Should: implement proper shutdown with timeout
}
```

**After implementing hack**:
```
skill: "doc-update" --target "tech debt: actor stop workaround"
```

### Tech Debt in CLAUDE.md

Add a visible section to CLAUDE.md:

```markdown
## Technical Debt ⚠️

**Open Items**: N (🔴 N | 🟡 N | 🟢 N)

### Critical (Must Fix)

- [tech-debt-001](docs/debt/tech-debt-20260118-001.md) - Actor stop leak causing memory issues

### Medium (Next Sprint)

- [tech-debt-002](docs/debt/tech-debt-20260118-002.md) - Missing TSDoc on 15 exports

### Low (Backlog)

- [tech-debt-003](docs/debt/tech-debt-20260118-003.md) - Code duplication in broker setup

> **All agents MUST review technical debt before starting work**
> **Check if your work relates to any open debt items**
```

### Tech Debt Resolution Workflow

1. **Identify** - Create debt record when hack/workaround implemented
2. **Assign** - Assign to appropriate phase/sprint
3. **Prioritize** - Critical debt blocks releases
4. **Implement** - Proper solution
5. **Verify** - Tests pass, hack removed
6. **Close** - Mark debt as resolved, archive record

### Resolved Debt

When debt is resolved:

1. **Mark as resolved** in debt file:
   ```markdown
   > **Status**: Resolved ✅
   > **Resolved Date**: YYYY-MM-DD
   > **Resolved By**: [Who fixed it]
   > **Resolution**: [Brief description of fix]
   ```

2. **Archive** to `docs/debt/resolved/`

3. **Update index** - Move from active to resolved

4. **Update CLAUDE.md** - Remove from active debt section

---

## XState v5 Expertise

### Golden Rules

#### Rule #1: ALWAYS use `matches()` for state comparison
```typescript
// ❌ WRONG - Direct value comparison
if (snapshot.value === 'completed') { }

// ✅ CORRECT - Use matches() method
if (snapshot.matches('completed')) { }
```

#### Rule #2: `waitFor` receives snapshots
```typescript
// ❌ WRONG - Using status property
await waitFor(actor, s => s.status === 'done');

// ✅ CORRECT - Using matches() on snapshot
await waitFor(actor, s => s.matches('completed'));
```

#### Rule #3: Actor lifecycle management
```typescript
// Correct pattern
const actor = createActor(machine);
actor.start();
// ... use actor
actor.stop();
```

### State Machine Patterns

#### Planner Machine (`src/orchestration/machines/plannerMachine.ts`)
- **States**: `idle`, `planning`, `implementing`, `reviewing`, `completed`, `failed`
- **Events**: `PLAN`, `IMPLEMENT`, `REVIEW`, `COMPLETE`, `FAIL`
- **Context**: Holds plan, tasks, results

#### Task Lifecycle
- **States**: `ready`, `implementing`, `reviewing`, `complete`, `failed`
- **Transitions**: Ready → Implementing → Reviewing → Complete

### Common XState Pitfalls

1. **Using `.value` instead of `.matches()`**
   - `.value` returns nested state objects
   - `.matches()` handles state hierarchy correctly

2. **Forgetting to start actors**
   - `createActor()` creates but doesn't start
   - Must call `.start()` explicitly

3. **Not handling snapshot context**
   - Snapshots contain `.context` property
   - Context changes between state transitions

---

## Agent Village Architecture

### Core Components

```
Agent Village
├── Orchestration Layer (XState)
│   ├── Machines: State machine definitions
│   ├── Actors: Running state machine instances
│   └── Bridges: Connect XState to infrastructure
├── Infrastructure Layer
│   ├── Database: SQLite for persistence
│   ├── A2A Client: Agent-to-agent communication
│   ├── Wiki: Knowledge storage
│   └── EventBus: Event pub/sub
└── Agent Layer
    ├── Planner Agent: Creates execution plans
    ├── Implementer Agent: Executes tasks
    └── Reviewer Agent: Validates results
```

### Key Files

| File | Purpose |
|------|---------|
| `src/orchestration/index.ts` | Public API exports |
| `src/orchestration/types.ts` | All orchestration types |
| `src/orchestration/machines/plannerMachine.ts` | Planner state machine |
| `src/orchestration/actors/planner.actor.ts` | Planner actor logic |
| `src/orchestration/bridges/a2a.bridge.ts` | A2A infrastructure bridge |
| `src/agents/planner-agent-xstate.ts` | Planner using XState |
| `src/llm/executors/*.ts` | LLM executors |

### A2A Bridge Pattern

```typescript
const bridge = createA2ABridge({ a2aClient, database, wiki, eventBus });
const machine = createPlannerMachine(bridge.createPlannerActors());
const actor = createActor(machine);
actor.start();
```

### Task Execution Flow

1. **Planning**: Planner creates tasks and sub-tasks
2. **Implementation**: Tasks assigned to implementer agents
3. **Review**: Results validated by reviewer agents
4. **Completion**: Successful tasks marked complete

---

## TypeScript Conventions

### Type Imports
```typescript
// Use type-only imports when possible
import type { PlannerMachineContext } from './types.ts';

// For values, use regular imports
import { createPlannerMachine } from './plannerMachine.ts';
```

### File Extensions
- Use `.ts` for source files
- Use `.js` for import statements (ESM)

### Naming Conventions
- **State machines**: `camelMachine.ts` (e.g., `plannerMachine.ts`)
- **Actors**: `camel.actor.ts` (e.g., `planner.actor.ts`)
- **Bridges**: `camel.bridge.ts` (e.g., `a2a.bridge.ts`)
- **Types**: `types.ts` in each module

---

## Documentation Standards

### Status Badges
| Badge | Meaning |
|-------|---------|
| ✅ | Complete/Verified |
| 🚧 | In Progress |
| 🔴 | Critical Issue |
| ⚠️ | Warning/Caution |
| 📖 | Reference Material |
| 🔄 | Under Review |

### Cross-References
```markdown
See [Architecture](docs/ARCHITECTURE.md) for details.
Reference: [XState Best Practices](docs/XSTATE-BEST-PRACTICES.md)
```

### Session Log Format
- **Filename**: `YYYY-MM-DD_HH-MM_topic.md`
- **Structure**:
  ```markdown
  # Session: [Descriptive Title]

  **Date**: YYYY-MM-DD HH:MM
  **Branch**: branch-name

  ## Executive Summary
  Brief overview...

  ## Details
  Full content...
  ```

### Executive Summary Template
```markdown
## Executive Summary
- **Status**: [Completed/Failed/In Progress]
- **Duration**: [Time taken]
- **Outcome**: [Result]
- **Next Steps**: [What's next]
```

---

## Common Documentation Issues

### 1. Outdated Status References
- **Symptom**: Documentation says "Phase 5" but code is Phase 6
- **Fix**: Update status badges and phase references
- **Check**: `docs/XSTATE-MIGRATION-PLAN.md` vs actual implementation

### 2. Stale Dates
- **Symptom**: "Last Updated: 2024-..." when it's 2026
- **Fix**: Update all "Last Updated" fields to current date

### 3. Broken Cross-References
- **Symptom**: Link points to non-existent file
- **Fix**: Verify file exists or update link

### 4. Unsorted Session Logs
- **Symptom**: Session logs not in chronological order
- **Fix**: Sort by filename (which includes date)

### 5. Inconsistent Terminology
- **Symptom**: "actor" vs "Agent" vs "agent"
- **Fix**: Use consistent capitalization
  - "actor" = XState actor (lowercase)
  - "Agent" = Agent Village agent (capitalized)

---

## Code Reading Guidelines

### What to Read (Read-Only Access)
- ✅ State machine definitions
- ✅ Type definitions
- ✅ Implementation files (for understanding)
- ✅ Test files (for verification)

### What NOT to Modify
- ❌ Source code files
- ❌ Test files
- ❌ Configuration files (except `.claude/`)
- ❌ Build artifacts

### Documentation Files (CAN Modify)
- ✅ `docs/*.md`
- ✅ `sessions/*.md`
- ✅ `README.md`
- ✅ `CLAUDE.md`
- ✅ `.claude/*`

---

## Git Workflow

### Commit Message Format
```
docs: [brief summary]

[optional detailed description]
```

### Auto-Commit Pattern
```bash
# Stage documentation changes
git add docs/ sessions/ .claude/

# Commit with standard prefix
git commit -m "docs: [summary]"

# Push to current branch
git push
```

### Branch Context
- **Current Branch**: `xstate-migration-gemini`
- **Main Branch**: `main`
- Always push to feature branch, not main

---

## LLM Integration Knowledge

### Default Configuration
- **Model**: `gemini-2.5-flash`
- **Provider**: Google Gemini
- **Config File**: `config.example.json`

### API Key Setup
```bash
# Environment variable
export GEMINI_API_KEY="your-key"

# Or .env file
echo "GEMINI_API_KEY=your-key" > .env
```

### Executor Types
- **Gemini**: `src/llm/executors/gemini.executor.ts`
- **OpenAI**: `src/llm/executors/openai.executor.ts`
- **Anthropic**: `src/llm/executors/anthropic.executor.ts`

---

## Search Patterns for Documentation Updates

### Find Phase References
```bash
grep -r "Phase [0-9]" docs/
```

### Find Status Badges
```bash
grep -r "\\[ \\?\\]" docs/
```

### Find Date References
```bash
grep -r "Last Updated" docs/
grep -r "202[0-9]" docs/
```

### Find Cross-References
```bash
grep -r "\\[.*\\](.*\\.md)" docs/
```

---

## Improvements Categorization

### 🔴 Critical (Breaking Issues)
- Inaccurate status claims (e.g., "Complete" when not)
- Broken functionality described as working
- Security vulnerabilities
- Data loss bugs

### 🟡 Medium (Inconsistencies)
- Outdated dates
- Missing documentation for new features
- Inconsistent terminology
- Minor inaccuracies

### 🟢 Low (Cosmetic)
- Typos
- Formatting issues
- Style inconsistencies
- Missing whitespace

---

**Last Updated**: 2026-01-18
**Maintained By**: Documentation Agent System
