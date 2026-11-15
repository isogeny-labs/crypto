# Reproduction of the 2026-11-12 lead runs from the integrated source

Two of the producing agents edited source after their recorded runs: clippy
renames and a refactor of the table print, which they reported as having no
effect on output. The integration session rebuilt from the final tree (branch
`claude/ecdlp-chained-transfers-8cpiek`, restarted from `main` after #3981) and
reran each binary under its own bench record. Every result is `[pcc:unverified]`.

| run | record | rerun | outcome |
|---|---|---|---|
| v3 (`chained_transfer_bench`) | `20261112T0341Z-chained-prime-transfer-v3` | full | `results_v3.json` and `certs_v3.jsonl` byte-identical; `verify_certs` 16/16 |
| amortized (`amortized_bench`) | `20261112T0402Z-chained-amortized-ic` | p = 211, 419, 821, 1619, 3203 | every non-timing field identical; only wall-clock fields differ |
| tower_large | `20261112T0421Z-chained-tower-large-p` | p = 251, 503 | every non-timing field identical; the HCurve cross-check passes |

Gate before integration: `cargo build --release` of the four binaries, `cargo
test --release --lib chained_transfer` (19 passed), `cargo clippy --release
--lib` plus the four binaries with `-D warnings` (clean), and `rustfmt --check`
on every touched file (clean).
