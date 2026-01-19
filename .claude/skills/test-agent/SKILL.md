# Test Agent

> **Purpose**: Run and verify AI-Coreutils tests
> **Expertise**: Rust testing, benchmarks, verification

---

## Core Principle: Test Everything

**CRITICAL**: All changes must be tested:
- Unit tests for functions
- Integration tests for utilities
- Benchmarks for performance-critical code

---

## Invocation

```
skill: "test-agent" --scope "<scope>"
```

### Examples

```bash
# Run all tests
skill: "test-agent" --scope "all"

# Test specific module
skill: "test-agent" --scope "memory"

# Run benchmarks
skill: "test-agent" --scope "bench"

# Test specific utility
skill: "test-agent" --scope "ai-ls"
```

---

## Behavior

### Step 1: Run Tests
```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_name
```

### Step 2: Verify Results
- Check test output for failures
- Verify benchmarks meet targets
- Note any warnings

### Step 3: Report
- Document test results
- Update ralph.yml if task depends on tests
- Create session log with results

---

## Test Categories

### Unit Tests
Located in `src/*.rs` files:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_function() {
        assert_eq!(result, expected);
    }
}
```

### Integration Tests
Located in `tests/*.rs` files:
```rust
use ai_coreutils::memory::SafeMemoryAccess;

#[test]
fn test_memory_access() {
    // Test across module boundaries
}
```

### Benchmarks
Located in `benches/*.rs` files:
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_function(c: &mut Criterion) {
    c.bench_function("function", |b| {
        b.iter(|| function(black_box(input)));
    });
}
```

---

## Performance Targets

### Memory Access
- Files > 10MB: Use memory mapping
- Files < 10MB: Can use standard I/O
- Memory mapping should be 10x faster for large files

### JSONL Output
- Overhead: Negligible compared to I/O
- Must handle large result sets efficiently

---

## Common Issues

### Test Failures
1. Check error messages
2. Review recent changes
3. Verify test assumptions

### Benchmark Regressions
1. Check for algorithmic changes
2. Verify compilation mode (debug vs release)
3. Check system load

---

## Output Format

```
Test Agent: [scope]
====================
Running tests...
Test result: ok. X passed in Ys

Benchmark results:
- memory_access: Zms
- jsonl_output: Wms

All tests PASSED ✅
```

---

**Last Updated**: 2026-01-19
