# Documentation Sweep Skill

> **Purpose**: Comprehensive documentation audit and update
> **Mode**: Read-only code access, documentation write access

---

## Invocation

```
skill: "doc-sweep"
```

## Parameters

None required. Performs a full sweep of all documentation.

---

## Behavior

### Step 1: Scan Documentation Files

Read all documentation files to understand current state:

1. **Core Documentation** (`docs/`):
   - `ARCHITECTURE.md`
   - `XSTATE-MIGRATION-PLAN.md`
   - `XSTATE-BEST-PRACTICES.md`
   - `LLM-INTEGRATION.md`
   - `ROADMAP-*.md`
   - `PHASE-*-*.md`

2. **Session Logs** (`sessions/`):
   - All `YYYY-MM-DD_*.md` files

3. **Root Documentation**:
   - `README.md`
   - `CLAUDE.md`

4. **State Machine Docs** (`state-machines/`):
   - `CLAUDE.MD`

### Step 2: Check Against Code State (Read-Only)

Read relevant code files to verify documentation accuracy:

- `src/orchestration/types.ts` - Verify type definitions
- `src/orchestration/machines/plannerMachine.ts` - Verify state machine patterns
- `package.json` - Verify dependencies and scripts
- `tsconfig.json` - Verify TypeScript configuration

### Step 2.5: Scan for Missing TSDoc

**CRITICAL**: During sweep, scan all TypeScript files for exported interfaces, classes, functions, and types that lack TSDoc comments.

1. **Find all TypeScript files**:
   ```bash
   find src/ -name "*.ts" -not -path "*/node_modules/*" -not -path "*/dist/*"
   ```

2. **Scan for exported symbols without TSDoc**:
   - `export interface` without `/**` comment above
   - `export class` without `/**` comment above
   - `export function` without `/**` comment above
   - `export type` without `/**` comment above
   - `export const` (if it's a type or important constant) without `/**` comment

3. **Generate TSDoc** for missing documentation:
   - Use the TSDoc template from `doc-expertise.md`
   - Infer documentation from:
     - Symbol names (e.g., `createPlannerMachine` → "Creates a planner machine")
     - Type signatures (parameters and return types)
     - Context from surrounding code
     - Related documentation

4. **TSDoc Quality Standards**:
   - Every `export` must have `/** */` TSDoc
   - Include `@param` for all parameters
   - Include `@returns` for functions with return values
   - Include `@example` for complex types/functions
   - Include `@remarks` for additional context
   - Use `@link` to reference related types

### Step 2.75: Scan for Technical Debt

**CRITICAL**: During sweep, scan code for hacks, workarounds, and TODO/FIXME comments that represent technical debt.

1. **Search for debt indicators**:
   ```bash
   # Find HACK comments
   grep -r "HACK" src/ --include="*.ts" --include="*.tsx"

   # Find TODO/FIXME comments
   grep -r "TODO\|FIXME" src/ --include="*.ts" --include="*.tsx"

   # Find temporary workarounds
   grep -r "temporary\|workaround\|hack" src/ --include="*.ts" -i
   ```

2. **Identify debt severity**:
   - **🔴 Critical**: Security issues, data loss risks, crashes
   - **🟡 Medium**: Performance issues, maintainability concerns
   - **🟢 Low**: Code quality, nice-to-have improvements

3. **Create debt records** for each untracked item:
   - Use template from `doc-expertise.md`
   - File: `docs/debt/tech-debt-{timestamp}-{id}.md`
   - Include file locations and line numbers

4. **Update debt tracking**:
   - Update `docs/debt/index.md` with new items
   - Update `CLAUDE.md` Technical Debt section
   - Flag relevant phases in roadmap

### Step 3: Identify Outdated Content

Check for:

- **Status Accuracy**: Compare documented status vs actual implementation
- **Date Freshness**: Update "Last Updated" dates to current date
- **Cross-Reference Validity**: Verify all linked files exist
- **Session Log Ordering**: Ensure sessions are sorted chronologically
- **Broken Links**: Check all markdown links

### Step 4: Perform Updates

For each issue found:

1. **Status Badges**: Update ✅/🚧/🔴 based on actual state
2. **Dates**: Set "Last Updated" to current date (YYYY-MM-DD)
3. **Cross-References**: Fix or remove broken links
4. **Session Ordering**: Rename/reorganize if needed
5. **Phase References**: Update to match current migration status

### Step 5: Sort Session Logs

Ensure session logs are in chronological order (newest first):

```bash
ls -t sessions/*.md
```

### Step 5.5: Consolidate Scattered Documentation

**CRITICAL**: Identify and consolidate scattered or duplicate documentation.

1. **Identify Duplicates**:
   - Search for similar topics across all docs
   - Find conflicting information
   - Locate reference material buried in session logs

2. **Consolidate Content**:
   - Choose primary location (prefer `docs/` over `sessions/`)
   - Merge duplicate sections
   - Resolve conflicts (prefer current implementation)
   - Extract reference material from session logs to proper docs

3. **Update Cross-References**:
   - Update all links to point to consolidated location
   - Add "See also" references
   - Remove outdated references

### Step 5.75: Archive Outdated Documentation

**CRITICAL**: Move outdated documentation to archive, don't delete.

1. **Identify Outdated Docs**:
   - Docs superseded by newer versions
   - Docs for deprecated features
   - Duplicate docs after consolidation
   - Session logs older than 6 months (configurable)

2. **Archive Process**:
   - Move to `docs/archive/YYYY-MM-DD-original-name.md`
   - Add archive notice at top:
     ```markdown
     > **⚠️ ARCHIVED**: This document has been archived and is kept for historical reference only.
     > **Archived Date**: YYYY-MM-DD
     > **Superseded By**: [Link to current document]
     > **Reason**: [Why this was archived]
     ```
   - Update `docs/archive/index.md` with entry

3. **Archive Index**:
   ```markdown
   # Archived Documentation

   | Date | Document | Superseded By | Reason |
   |------|----------|---------------|--------|
   | 2026-01-18 | [OLD-ARCHITECTURE](2026-01-18-ARCHITECTURE.md) | [ARCHITECTURE](../ARCHITECTURE.md) | Reorganized |
   ```

### Step 6: Generate Summary

Create a summary of all changes made:

```markdown
## Documentation Sweep Summary - YYYY-MM-DD

### Documentation Files Updated
- [ ] ARCHITECTURE.md - [changes]
- [ ] XSTATE-MIGRATION-PLAN.md - [changes]
- [ ] CLAUDE.md - [changes]

### Code Files Updated (TSDoc Only)
- [ ] src/file1.ts - Added TSDoc to exports
- [ ] src/file2.ts - Added TSDoc to exports

### Consolidation Actions
- Consolidated [count] duplicate sections
- Extracted [count] reference items from session logs
- Resolved [count] conflicting information

### Archival Actions
- Archived [count] outdated documents
- Created archive index entries

### Issues Found
- 🔴 Critical: [count] issues
- 🟡 Medium: [count] issues
- 🟢 Low: [count] issues
- 📝 Missing TSDoc: [count] exports
- 🔄 Duplicates found: [count] items
- ⚠️ Technical Debt: [count] new items found

### Actions Taken
- Updated status badges
- Freshened dates
- Fixed cross-references
- Sorted session logs
- Added TSDoc to [count] exports
- Consolidated scattered documentation
- Archived outdated docs
- Created [count] technical debt records
```

### Step 7: Commit and Push

```bash
git add docs/ sessions/ CLAUDE.md
git commit -m "docs: [summary of changes]"
git push
```

---

## Checks Performed

### 1. Status Accuracy Check
- Compare `XSTATE-MIGRATION-PLAN.md` phase status with actual implementation
- Verify `CLAUDE.md` "Current Status" section
- Check completion badges in roadmap documents

### 2. Date Freshness Check
- Find all "Last Updated" fields
- Update to current date if older than 7 days
- Format: `YYYY-MM-DD`

### 3. Cross-Reference Validity Check
- Extract all markdown links: `\[.*\]\(.*\.md\)`
- Verify target files exist
- Flag broken references

### 4. Session Log Ordering Check
- Parse session filenames for dates
- Sort by date (descending)
- Identify misordered files

### 5. Broken Link Check
- Check all relative paths
- Verify file existence
- Flag unreachable references

---

## Output

### Console Output
```
Documentation Sweep Started...
Scanning documentation files...
Found 25 documentation files
Checking against code state...
Identified 12 issues
Scanning for missing TSDoc...
Found 15 exports without TSDoc
Scanning for technical debt...
Found 3 HACK comments
Found 7 TODO/FIXME comments
Checking for duplicate documentation...
Found 5 duplicate sections
Identifying outdated documentation...
Found 3 docs to archive

Updating documentation...
- Fixed status badges: 3
- Updated dates: 5
- Fixed links: 2
- Sorted sessions: 2
- Added TSDoc to 15 exports
- Created 10 technical debt records
- Consolidated 5 duplicate sections
- Archived 3 outdated documents

Summary: docs/SESSION-YYYY-MM-DD.md
Committed and pushed.
```

### Session Log
Creates `sessions/YYYY-MM-DD_doc-sweep.md` with:
- Full sweep results
- All changes made
- Issues found and resolved

---

## Example Session Log Entry

```markdown
# Documentation Sweep - 2026-01-18

## Executive Summary
- **Status**: Completed
- **Files Scanned**: 27
- **Files Updated**: 8
- **Issues Found**: 15
- **Issues Fixed**: 12

## Changes Made

### ARCHITECTURE.md
- Updated "Last Updated" to 2026-01-18
- Fixed cross-reference to XSTATE-BEST-PRACTICES.md

### XSTATE-MIGRATION-PLAN.md
- Updated Phase 6 status: ✅ Complete
- Added Phase 7 placeholder

### CLAUDE.md
- Updated current status section
- Added reference to ROADMAP-2026-01-18.md

## Outstanding Issues
See `docs/IMPROVEMENTS.md` for 3 remaining issues.
```

---

## Important Notes

- **Read-Only Code Access**: Only reads code for verification, **except TSDoc comments**
- **TSDoc Exception**: TSDoc comments are added to code files (`.ts` files) to document exports
- **Documentation Only**: Writes to `docs/`, `sessions/`, `.claude/` + TSDoc in `.ts` files
- **Auto-Commit**: Always commits and pushes changes
- **Improvements Tracking**: Updates `docs/IMPROVEMENTS.md` with issues found
- **Current Branch**: Always commits to `xstate-migration-gemini`

### Files Modified During Sweep

**Documentation Files** (always):
- `docs/*.md`
- `docs/archive/*.md` (archived docs)
- `sessions/*.md`
- `CLAUDE.md`
- `.claude/*`

**Code Files** (TSDoc only):
- `src/**/*.ts` - Only to add TSDoc comments to exports
- No logic changes, only comments

**New Files Created** (if needed):
- `docs/archive/index.md` (archive index)
- `docs/adr/` (new ADRs if architectural decisions documented)
- `docs/debt/` (new tech debt records if hacks found)
- `docs/debt/index.md` (debt index)

---

**Related Skills**:
- `doc-agent.md` - Main entry point
- `doc-update.md` - Targeted documentation
- `doc-fixes.md` - Improvements tracker
- `doc-expertise.md` - Shared knowledge

**Last Updated**: 2026-01-18
