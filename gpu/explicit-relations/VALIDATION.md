# Source validation — 2026-10-29

This is a source-only handoff. No executable, CUDA kernel, Cargo test,
benchmark, image build or EKS job was run for this change.

| Check | Observed outcome |
| --- | --- |
| `rustfmt --check --edition 2021 --config skip_children=true` on changed Rust sources and the retained binary-basis fix | Passed |
| `make -C gpu/explicit-relations syntax` | Passed: `g++ -std=c++17 -Wall -Wextra -Werror -Wno-unknown-pragmas -fsyntax-only service.cpp` |
| `clang-format --dry-run -Werror` on the four native source files | Passed, clang-format 23.1.3 |
| `git diff --cached --check` | Passed |
| Existing curve-name checker's `lint_diff` applied to staged additions against `origin/main` | Passed, zero problems |
| Full `python3 scripts/check_curve_names.py --diff origin/main` | Blocked: sparse checkout lacks tracked `.agents/skills/audit-runs/SKILL.md` |
| Rust type checking, Cargo tests and Clippy | Not run; user requested source-only delivery without local plug-in compilation |
| Host executable self-tests, Rust service integration and certificate replay | Written, not run |
| CUDA compilation, GPU differential tests and Compute Sanitizer | Written/documented, not run; no local nvcc or NVIDIA tool available |
| Performance comparison | Not run; no speedup or asymptotic claim |

The C++ command checks the host compilation path only. It does not compile the
`ER_CUDA` branch, validate CUDA barriers, link an executable or run arithmetic.
Rustfmt parses syntax but does not type-check. These checks do not establish
that the change passes CI or is safe to merge.

An initial C++ syntax pass found three misleading-indentation warnings; these
were corrected and the final pass succeeded. An initial whole-file scoped
curve-name scan included unchanged legacy labels in existing modules; the
table records the repository's actual added-line check instead. The full-tree
gate remains pending, without exemptions or source changes to the checker.

Run the build and device checks listed in [README.md](README.md) and the full
repository checks in [the handoff](../../docs/ecbench/explicit-bases.md) on a
builder with adequate disk. Any EKS image must be built from the resulting
`crypto/main` revision and retain its commit and image digest. GPU work remains
a standalone service; its row diagnostics are not CPU GAE measurements.
