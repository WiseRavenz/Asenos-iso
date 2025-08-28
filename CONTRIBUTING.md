# Contributing to Asenos-iso

Thank you for your interest in contributing. This document explains how to report issues, propose changes, and submit pull requests so maintainers can review and accept contributions quickly.

## Quick checklist
- Open an issue first for non-trivial changes.
- Create a small, focused branch: `feature/`, `fix/`, or `chore/`.
- Keep commits descriptive and tests/docs updated.
- Run local checks and document verification steps in the PR.

## Reporting issues
- Search existing issues to avoid duplicates.
- Provide a reproducible test case, environment (OS + shell), and relevant logs or outputs.

## Pull request workflow
1. Fork and branch from the repository default branch.
2. Make small, focused commits. Prefer one logical change per PR.
3. Rebase onto the latest default branch before opening the PR.
4. In the PR description explain why the change is needed, what changed, and how you tested it. Link related issues (e.g. `Fixes #123`).

## Branching & commit messages
- Branch naming: `feature/<short-desc>`, `fix/<short-desc>`, `chore/<short-desc>`.
- Commit message format:

  Short summary (<=72 chars)

  Optional detailed description explaining reasoning and edge cases.

- Include `Signed-off-by:` lines only if requested by maintainers.

## Scripting and style
- Prefer POSIX-compatible shell in `scripts/` when practical; repo tooling often runs in `zsh`/`bash` environments.
- Add `set -euo pipefail` to scripts that must fail fast.
- Lint shell scripts with `shellcheck` where practical.

## Building and testing locally
- Read `scripts/build.sh` and `scripts/run_archiso.sh` headers for build prerequisites.
- For quick checks, run shell checks and dry-run the modified scripts in a disposable environment or container.
- For full ISO/build verification, use the provided build script inside a VM or container to avoid polluting your host.

## Review checklist for contributors
- Is the change small and focused?
- Are commits grouped logically and messages clear?
- Are configuration changes documented inline?
- Are scripts linted and error-handling present where needed?
- Are docs/README/CHANGELOG updated if the change affects users?

## Licensing & sign-off
By contributing you confirm you have the right to submit the code under this project's `LICENSE`.

If a DCO or a specific sign-off is required by maintainers, include a `Signed-off-by: Your Name <you@example.com>` line in your commits.

## Code of conduct
Treat others with respect. If you see problematic behavior, open an issue or contact the project maintainers.

## Need help getting started?
Open an issue describing what you want to work on and ask for a `good first issue` or `help wanted` label; maintainers will help you pick a small task.

Thanks for contributing to Asenos-iso — your improvements make the project better for everyone.
