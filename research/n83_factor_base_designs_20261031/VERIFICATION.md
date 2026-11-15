# Development verification, 2026-10-31

Source baseline: `48c8a4767d2d1eed6a7b379f74410bc729239380`.
Toolchain: Rust `1.94.1 (e408947bf 2026-03-25)`, Cargo
`1.94.1 (29ea6fb6a 2026-03-24)`, Linux x86-64.

Commands use `CARGO_HOME=/workspace/n83-cargo`,
`RUSTUP_HOME=/workspace/n83-rustup` and `/workspace/n83-cargo/bin/cargo`.
They are builds and unit tests of a standalone crate. No research experiment
or comparative timing ran.

| Check | Observed outcome |
| --- | --- |
| `cargo test --manifest-path research/n83_factor_base_designs_20261031/native/Cargo.toml` | 7 passed, 0 failed |
| `cargo test --release --locked --offline --manifest-path research/n83_factor_base_designs_20261031/native/Cargo.toml` | 7 passed, 0 failed after formatting/lint fixes |
| `cargo clippy --locked --offline --manifest-path research/n83_factor_base_designs_20261031/native/Cargo.toml -- -D warnings` | Passed on final source |
| `cargo fmt --check --manifest-path research/n83_factor_base_designs_20261031/native/Cargo.toml` | Passed |
| `cargo build --release --locked --offline --manifest-path research/n83_factor_base_designs_20261031/native/Cargo.toml` | Passed |
| `git diff --check` | Passed before staging; recheck staged files before commit |
| Typst report compile | Passed, Typst 0.15.0; 3 A4 pages |
| PDF render/inspection | All 3 pages inspected; diagram font reduced and final first page reinspected |
| Full root-library tests | Not run; this change does not modify the root crate, and no push was attempted |
| Main CI / external-CAS timing / remote execution | Not run |

Unit-test coverage: a degree-three nonlinear UNSAT fixture with upper/lower
identity replay; corrupt upper and lower certificates; SAT and capped-UNSAT
distinction; constant contradictions and empty systems; bad input binding,
degree and monomial masks; explicit resource-cap errors; all 256 pairs of
two-variable Boolean generators compared with complete truth tables.

## Preserved development failures and access checks

- Initial full-history clone was interrupted after downloading gigabytes;
  recovered with a blobless shallow sparse checkout. The first attempted
  rename found that Git had already cleaned the interrupted directory.
- Rustup initially rejected a custom installer basename. Renaming the
  downloaded executable to its expected `rustup-init` name fixed installation.
- First strict Clippy pass reported `needless_range_loop` and
  `manual_is_multiple_of`. Both were corrected, then release tests and
  Clippy passed. No assertion or gate was disabled.
- Primary ePrint 2004/031 PDF request returned HTTP 403. Semaev's
  arXiv:1504.01175v1 PDF was downloaded and its relevant sections read.
- `conductor check`: command not found. Its documented missing-installation
  behavior is fail-open. Study paths remain isolated.
- `gh auth status`: the injected GH_TOKEN is invalid. No PR or remote branch
  was created and no token value was printed.
- First checkpoint gate refused the absent resume file. After preparing every
  required section, the next gate reported: `no way to upload: no AWS profile
  'ecc-challenges-new', no CHECKPOINT_UPLOAD_TOKEN` and no token file. It then
  reported `cannot upload, so nothing was checkpointed`. No experiment launched.

## SHA-256 bindings

| Artifact | SHA-256 |
| --- | --- |
| PROTOCOL.md | `e0cb779cd2a8c1b180c4c05505bef33a3421acfe501fa2cd8648b83ea80f00d8` |
| native/degree_certificate.rs | `91dd3546fc5d5bc9ab9a9eb15c7ecd80f5c442b47f26cefea217a4d98d86a2bf` |
| native/Cargo.lock | `0be2202e8b662b705761d79d65488d229a1f9dcdb834d2a28f790819e94fa689` |
| Local release executable | `cd67933268d312125fefabcc722920e2d6fc86b6575a665274305f7234aa5303` |
| REPORT.pdf | `4655696fd341fe53133963a9da8ae9722d32643e49a732101cccbe60aee6b441` |

The executable is build output and is not committed. Rebuild it from the
locked native crate. These bindings identify development artifacts; none is a
factor-base experiment certificate.
