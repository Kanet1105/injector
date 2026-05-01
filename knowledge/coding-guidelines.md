# Coding Guidelines

Applies to all languages in this project.

---

## Before Writing Anything

- State assumptions explicitly. If uncertain, ask — don't guess silently
- If multiple interpretations exist, surface them before picking one
- If a simpler approach exists, say so and push back
- Turn vague tasks into verifiable goals before touching code:
  `"fix the bug" → write a test that reproduces it, then make it pass`
- For multi-step tasks, state a brief plan with a verify step for each

---

## Simplicity

- Minimum code that solves the problem. Nothing speculative
- No abstractions for single-use code
- No flexibility or configurability that wasn't asked for
- If you wrote 200 lines and it could be 50, rewrite it
- Functions do one thing. If you need a comment to explain "step 1 / step 2" — split

---

## Changes

- Touch only what the task requires
- Don't improve adjacent code, formatting, or comments unless asked
- Don't refactor things that aren't broken
- Match existing style even if you'd do it differently
- If you notice unrelated dead code, mention it — don't delete it
- Every changed line should trace directly to the request
- One logical change per commit (see `CONTRIBUTING.md`)

---

## Naming

- Names explain intent, not type: `pub_date_unix` not `date_i64`
- Booleans read as assertions: `is_empty`, `has_guid`, `should_skip`
- No abbreviations unless universal (`url`, `id`, `rss`, `http`)

---

## Errors & Failures

- Every error is handled, propagated, or logged with context — never silently discarded
- No bare `let _ = fallible()` without a comment explaining why
- Design for failure paths first, happy path second

---

## Testing

Tests are not optional. Every non-trivial piece of logic ships with tests.

- Write the test before or immediately after the function — not "later"
- Tests are living documentation — they say what the code does and what it protects against
- Test behaviour, not implementation. Don't test private internals; test the observable contract
- If something is hard to test, the design is probably wrong — fix the design

### Structure — Arrange / Act / Assert

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

Multiple assertions are fine when they describe one logical outcome, not multiple unrelated things.

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

## TODOs

Always attach an issue or reason — no bare `// TODO`:

```rust
// TODO(#12): retry on transient HTTP errors
// TODO: blocked on proto compilation strategy decision
```
