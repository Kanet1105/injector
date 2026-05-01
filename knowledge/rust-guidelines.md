# Rust Idioms

Reference for idiomatic Rust used in this project.
Sources: Apollo GraphQL Rust Best Practices handbook + Rust Book + ecosystem conventions.

---

## Ownership & Borrowing

- Prefer `&T` over `.clone()` unless ownership transfer is required
- Prefer `&str` over `String`, `&[T]` over `Vec<T>` in fn params
- Small `Copy` types (≤ 24 bytes, no heap) — pass by value
- `Cow<'_, T>` when ownership is conditional
- If a caller needs ownership, make the fn take `T` — don't clone inside

```rust
// ✅
fn log(msg: &str) { ... }

// ❌ — forces allocation on caller
fn log(msg: String) { ... }
```

---

## Error Handling

- Libraries: `thiserror` for typed errors, never `anyhow`
- Binaries: `anyhow` acceptable for ergonomics
- No `unwrap()`/`expect()` outside tests
- Use `?` to propagate, `map_err`/`inspect_err` to transform/log
- Async errors must be `Send + Sync + 'static` across `.await` boundaries
- `let Ok(x) = ... else { return Err(...) }` for early returns when Err value is irrelevant

```rust
// ✅ library error
#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("parse error: {0}")]
    Parse(String),
}

// ✅ propagate + log
some_call()
    .inspect_err(|e| tracing::error!("context: {e}"))
    .map_err(FetchError::from)?;
```

---

## Option / Result combinators

- Prefer `_else` variants to avoid eager allocation: `unwrap_or_else`, `ok_or_else`, `map_or_else`
- `.ok()` / `.ok_or()` / `.ok_or_else()` to convert between Result ↔ Option
- `match` when pattern matching on inner types or transforming shape
- `if let` / `let ... else` for early exits

```rust
// ❌ allocates even on Ok path
x.ok_or(format!("missing value for {key}"))

// ✅
x.ok_or_else(|| format!("missing value for {key}"))
```

---

## Iterators

- Prefer iterators over manual loops for transformations
- `for` loops when you need early `break`/`continue` or side-effects dominate
- Never `.collect()` just to iterate again — chain instead
- `.iter()` over `.into_iter()` when inner type is `Copy`
- `.cloned()` / `.copied()` at chain end, not `.map(|x| x.clone())` mid-chain
- Iterators are **lazy** — nothing runs until a consumer (`.collect`, `.sum`, `.for_each`)

```rust
// ✅
let titles: Vec<_> = items.iter()
    .filter(|i| i.pub_date > cutoff)
    .map(|i| i.title.as_str())
    .collect();

// ❌ — intermediate allocation for no reason
let filtered: Vec<_> = items.iter().filter(...).collect();
let titles: Vec<_> = filtered.iter().map(...).collect();
```

---

## Static vs Dynamic Dispatch

- Default: generics / `impl Trait` — zero runtime cost, monomorphized at compile time
- `dyn Trait` only when: heterogeneous collections, plugin architecture, or type must be erased
- `&dyn Trait` over `Box<dyn Trait>` when ownership not needed
- `Arc<dyn Trait>` for shared across threads
- Don't box internally in structs — box at API boundaries only

```rust
// ✅ static — preferred
fn process(feed: impl FeedSource) { ... }

// ✅ dynamic — justified: mixed source types at runtime
fn process(feeds: Vec<Box<dyn FeedSource>>) { ... }
```

---

## Type State Pattern

Encode valid states in the type system to prevent invalid operations at compile time.
Use when a type has a lifecycle with distinct phases (e.g. connected/disconnected, built/unbuilt).

```rust
struct Poller<State> { _state: PhantomData<State> }
struct Idle;
struct Running;

impl Poller<Idle> {
    fn start(self) -> Poller<Running> { ... }
}
impl Poller<Running> {
    fn poll(&self) -> FeedItem { ... }
}
// Poller<Idle>.poll() — compile error ✅
```

---

## Imports

Order: `std` → external crates → workspace crates → `super::` / `crate::`.

```toml
# rustfmt.toml
reorder_imports = true
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
```

---

## Comments

- `//` = **why**, not what. Link to ADR/issue when relevant.
- `///` = public API docs (tested by `cargo test --doc`)
- No wall-of-text comments — split into functions instead
- TODOs reference a tracked issue: `// TODO(#42): support Atom feeds`

---

## Linting

Run: `cargo clippy --all-targets --all-features -- -D warnings`

Key lints to watch:
- `redundant_clone` — unnecessary clone
- `large_enum_variant` — box the large variant
- `needless_collect` — premature collection

Use `#[expect(clippy::lint_name)]` with a justification comment over bare `#[allow(...)]`.
