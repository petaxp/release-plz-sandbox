# release-plz sandbox

A minimal Rust library for testing release-plz release PRs, version updates,
changelogs, tags, and GitHub releases in a public repository.

This trial uses **git-only mode**. `publish = false` prevents Cargo registry
publishing; no crates.io account, token, or custom repository secret is needed.
It does not test crates.io publishing or build downloadable binary artifacts.

## Release process

1. Merge a change into `main` with a Conventional Commit message, such as
   `fix: correct the greeting` or `feat: add a personalized greeting`.
2. The Release-plz workflow checks the code, releases a merged release PR if
   applicable, and opens or updates the next release PR.
3. Review the proposed version, Cargo.lock, and generated CHANGELOG.md. The
   workflow's `check-release-pr` job validates the proposed release branch.
4. Merge the release PR when ready. Its merge triggers the tag and GitHub
   release. Ordinary commits do not release because `release_always = false`.

No release exists initially. Without a matching tag, release-plz treats the
package as an initial release; review the first proposal rather than expecting
an automatic bump from 0.1.0. Tags use `0.1.0` without a `v` prefix. Subsequent
proposals compare against the last matching tag.

Use `feat:`, `fix:`, and `feat!:` / `BREAKING CHANGE:` messages to explore version
selection. Pre-1.0 increments follow release-plz/Cargo semantics; a feature need
not bump the minor version. Preserve Conventional Commit titles when squashing.

## GitHub settings and CI

The organization and repository must allow **GitHub Actions to create and approve
pull requests** under Settings → Actions → General → Workflow permissions.
The default token can remain read-only; the release jobs explicitly request
Contents and Pull requests permissions as needed. PRs are never auto-approved or
auto-merged by these workflows.

GitHub does not automatically start PR workflows for PRs created by
`GITHUB_TOKEN`. To avoid requiring a personal token, this project explicitly
calls the reusable CI workflow on the generated release PR branch. Find those
results in the **Release-plz** workflow run; they are associated with the main
workflow run, rather than automatically appearing as checks on the PR commit.
For ordinary human-created PRs, the usual CI workflow runs automatically.

For a closer production simulation with required PR checks, configure a
dedicated GitHub App token and use it for the release-pr job so normal PR events
trigger CI. That is optional for this sandbox.

The workflow also supports **Actions → Release-plz → Run workflow** on `main`
for a retry. Re-running an already completed release should not duplicate it.
Disable the Release-plz workflow in Actions to stop the experiment.

## Try the library

```rust
use petaxp_release_plz_sandbox::{farewell, greeting, greeting_for};

assert_eq!(greeting(), "Hello from the release-plz sandbox!");
assert_eq!(greeting_for("Rust"), "Hello, Rust!");
assert_eq!(farewell(), "Goodbye from the release-plz sandbox!");
```

The changelog contains versioned release sections only. Release-plz prepares
those sections in release PRs; it does not maintain an empty `[Unreleased]`
placeholder.

## PR title validation

The **PR title / Validate PR title** check requires Conventional Commit titles,
for example `feat: add a greeting`, `fix: handle empty names`, or
`feat!: change the greeting API`. It runs when a PR opens, its title changes,
new commits arrive, or it reopens, including PRs from forks. The workflow reads
PR metadata only and never checks out or executes PR code.

Use the conventional title as the final squash commit title. For breaking
changes, include `!` in that title. Intermediate commits are not validated by
this check. To enforce it as a merge gate, require **Validate PR title** in the
branch protection or ruleset for `main`; merely adding the workflow does not
block merging failed checks.

## Local validation

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked --all-features
```

release-plz is pinned to 0.4.3; actions use version tags (`v6`, `v0.5`)
and the Rust toolchain action follows `stable`.
See [release-plz configuration](https://release-plz.dev/docs/config),
[GitHub setup](https://release-plz.dev/docs/github/quickstart), and
[token behavior](https://release-plz.dev/docs/github/token).

Temporary PR for checking title validation; this change will not be merged.
