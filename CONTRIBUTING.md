# Contributing

## Commit conventions

All commits follow [Conventional Commits](https://www.conventionalcommits.org/).

```
<type>(<scope>): <subject>

[optional body]

[optional footer]
```

### Types

| Type | When to use |
|------|-------------|
| `feat` | New feature or capability |
| `fix` | Bug fix |
| `refactor` | Code change that is neither feat nor fix |
| `perf` | Performance improvement |
| `test` | Adding or updating tests |
| `docs` | Documentation only |
| `chore` | Build, deps, tooling, CI — no production code |
| `style` | Formatting, lints — no logic change |

### Scopes

Match the crate or area changed:

| Scope | Area |
|-------|------|
| `workspace` | Root `Cargo.toml`, workspace-level changes |
| `rss` | `crates/rss` |
| `redpanda` | `crates/redpanda` |
| `config` | `crates/config` |
| `proto` | `proto/` |
| `docker` | `docker-compose.yml` |
| `docs` | `knowledge/`, `AGENTS.md`, `CONTRIBUTING.md` |

### Rules

- Subject: imperative, lowercase, no period — `add rss fetcher crate` not `Added RSS fetcher crate.`
- Keep subject ≤ 72 chars
- One logical change per commit (atomic)
- Breaking changes: append `!` after scope — `feat(config)!: rename broker field`

### Examples

```
feat(rss): add empty google news rss fetcher crate
chore(workspace): add .gitignore for rust project
fix(redpanda): handle missing guid on produce
docs(proto): add news_item schema with dedup decision
refactor(config)!: rename RedpandaSettings brokers field
```
