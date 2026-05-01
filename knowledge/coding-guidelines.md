# Coding Guidelines

Applies to all languages in this project.

---

## Testing

We don't enforce strict TDD but tests are not optional. Every non-trivial piece of logic ships with tests.

### Philosophy

- Write the test before or immediately after the function — not "later"
- Tests are living documentation. A good test suite tells you what the code does and what it protects against
- Test behaviour, not implementation. Don't test private internals; test the observable contract
- If something is hard to test, the design is probably wrong — fix the design

### Structure — Arrange / Act / Assert

Every test follows this shape:

```rust
#[test]
fn parse_should_return_error_on_empty_input() {
    // Arrange
    let input = "";

    // Act
    let result = parse(input);

    // Assert
    assert!(result.is_err());
}
```

### Naming

`<subject>_should_<expected>_when_<condition>`

```
parse_should_return_error_when_input_empty
poller_should_skip_duplicate_guid
feed_item_should_map_pub_date_to_unix_timestamp
```

### One assertion per test (where practical)

Prefer focused tests. Multiple assertions are fine when they describe one logical outcome, not multiple unrelated things.

### Test categories

| Type | What | Location |
|------|------|----------|
| Unit | Single fn/struct in isolation, no I/O | `#[cfg(test)]` mod in same file |
| Integration | Multiple components wired together | `tests/` dir at crate root |
| Doc tests | Public API usage examples | `///` doc comments |

### What must have tests

- All parsing / deserialization logic
- All error paths
- All data transformations (e.g. `FeedItem → NewsItem`)
- Any function with a non-obvious edge case
- Anything that has broken before

### What doesn't need tests

- Trivial getters/setters
- Direct delegation with no logic
- Main entrypoints (test the pieces they wire instead)

---

## General

### Small functions

Functions do one thing. If you need a comment to explain "step 1 / step 2 / step 3" — split.

### Naming

- Names explain intent, not type: `pub_date_unix` not `date_i64`
- Booleans read as assertions: `is_empty`, `has_guid`, `should_skip`
- No abbreviations unless universal (`url`, `id`, `rss`, `http`)

### No silent failures

Every error is either handled, propagated with `?`, or logged with context before being swallowed. Never:

```rust
let _ = some_fallible_call(); // ❌ silent discard
```

### TODOs

Always attach an issue or a short reason:

```rust
// TODO(#12): retry on transient HTTP errors
// TODO: blocked on proto compilation strategy decision
```

No bare `// TODO` with no context.

### Keep diffs reviewable

One logical change per commit (see `CONTRIBUTING.md`). PRs should be readable top to bottom.
