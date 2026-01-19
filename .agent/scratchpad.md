# AI-Coreutils Scratchpad

## What Was Accomplished (2026-01-19)

### Completed Tasks
1. **Repository Setup**
   - Initialized git repository
   - Created GitHub repo: https://github.com/kgd32/ai-coreutils
   - Pushed initial commit

2. **Skills System Created**
   - `.claude/doc-config.yml` - Auto-documentation configuration
   - `.claude/skills/auto-doc.md` - Auto-invoked documentation skill
   - `.claude/skills/dev-agent.md` - Development work skill
   - `.claude/skills/doc-agent.md` - Manual documentation skill
   - `.claude/skills/test-agent.md` - Test execution skill
   - `.claude/skills/phase-agent.md` - Phase tracking skill
   - `.claude/skills/README.md` - Skills index

3. **Documentation Updated**
   - Added auto-documentation policy to prompt.md and CLAUDE.md
   - Added gnu-core-utils.md as blueprint reference
   - All files reference gnu-core-utils.md as primary specification

4. **ai-ls Implementation Complete**
   - Full directory traversal with walkdir
   - File metadata extraction (size, permissions, modified time)
   - Sorting options (-t, -S, -r, -R)
   - Hidden file handling (-a)
   - Human-readable sizes (-h)
   - Long format output (-l)
   - Cross-platform support (cfg_attr for Unix-specific code)

5. **Error Handling Enhanced**
   - Added WalkDir error variant to AiCoreutilsError
   - Fixed test lifetime issues

## Current Project State

### Phase 1 (MVP) Status
- ✅ setup-project - Complete
- ✅ implement-memory-layer - Complete
- ✅ implement-jsonl-output - Complete
- ✅ **implement-ai-ls - Complete** (just finished)
- ⏳ implement-ai-cat - Next task (todo)
- ⏳ implement-ai-grep - Next task (todo)

### Test Status
- ✅ All tests passing (5 passed)
- ✅ Project compiles successfully
- ✅ ai-ls works with all specified options

### Blockers
None currently

## Next Task: implement-ai-cat

### Requirements (from gnu-core-utils.md)
**Key Options**:
- `-n`: Number all output lines
- `-b`: Number non-blank lines
- `-A`: Show all characters (including non-printing)
- `-E`: Show end of lines ($)
- `-T`: Show tabs as ^I
- `-s`: Squeeze multiple blank lines

**Implementation Priority**:
1. File reading with memmap2
2. Line numbering options
3. Memory pointer access option
4. JSONL output with metadata
5. Cross-platform compatibility

### Dependencies Satisfied
- implement-memory-layer: ✅ Complete
- implement-jsonl-output: ✅ Complete
- Ready to start

## Technical Notes

### File Structure
```
src/
├── lib.rs              # Library entry point
├── error.rs            # Error types (with WalkDir variant added)
├── jsonl.rs            # JSONL output formatter
├── memory.rs           # Memory access with SafeMemoryAccess
├── fs_utils.rs         # File system utilities
└── bin/
    ├── ai-ls.rs        ✅ Complete
    ├── ai-cat.rs       🚧 Next task
    └── ai-grep.rs      🚧 Third task
```

### Working Patterns Added
1. **cfg_attr for platform-specific code**
2. **Clone struct fields when ownership is needed**
3. **Dereference file references for memmap2**
4. **Use temp file for memory-mapped vec data**

## Commit Messages
```
feat: initial AI-Coreutils implementation

- Project setup with Cargo.toml and proper structure
- Error handling with thiserror
- JSONL output formatter
- SafeMemoryAccess with memmap2
- Stub binaries for ai-ls, ai-cat, ai-grep
- Auto-documentation system (auto-doc skill)
- Skills system for agent coordination
- gnu-core-utils.md as blueprint
- Integration tests and benchmarks
- All Phase 1 dependencies complete

Phase 1 (MVP): Setup Complete
```

```
feat: implement ai-ls with full functionality

- Implement full directory traversal with walkdir
- Add file metadata extraction (size, permissions, modified time)
- Implement sorting options (-t, -S, -r, -R)
- Add hidden file handling (-a)
- Add human-readable sizes (-h)
- Add long format output (-l)
- Add WalkDir error variant to error types
- Fix test lifetime issues
- All tests passing (5/5)

Phase 1 (MVP): ai-ls complete
Next: ai-cat and ai-grep
```

## Git Repository
**URL**: https://github.com/kgd32/ai-coreutils
**Branch**: master
**Last Commit**: feat: implement ai-ls with full functionality

## Performance Targets
- Memory mapping: Not yet benchmarked
- JSONL overhead: Not yet measured
- Directory traversal: Uses walkdir (efficient for large directories)

## Remaining Phase 1 Tasks
1. implement-ai-cat
2. implement-ai-grep
3. implement-remaining-utils

## Next Agent Should
1. Read gnu-core-utils.md cat specification
2. Implement ai-cat with memmap2 integration
3. Test and verify functionality
4. Update documentation and commit
5. Move to ai-grep when complete

## Important Reminders
- Always reference gnu-core-utils.md as blueprint
- Auto-documentation is ALWAYS enabled (no manual invocation needed)
- Update ralph.yml task status before starting
- Commit after each task completion
