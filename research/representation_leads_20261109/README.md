# Representation leads for binary point decomposition (2026-11-09)

Exact derivations (no timings, no experiment runs) for five representation ideas aimed at the
regime where a per-attempt budget exists on ECC2K-130 (m >= 5; see `REPORT.pdf`). All
computation is native Rust in `derive/`; `plot.py` only draws `results/`.

| lead | representation | outcome here | atlas entry (crypto-autoresearcher) |
|:--|:--|:--|:--|
| R1' | line norm form per merge: mu = e2 (and slope lambda) quadratises the chained S3 system | degree 2 confirmed; same variables and monomial support as a generic quadratisation w = x_a x_b, 30-40% more terms; the trace equations are implied in any chain (census) and known (KR-IC-f5c584) | `LEAD-ICE-quadratised-chain-representation` (open, gated 4-arm cell) |
| R2 | factor-base membership as a Frobenius chain modulo N(X) | not computed; likely inside Huang-Kosters-Yeo 2015 big-field analysis | `LEAD-ICE-subspace-frobenius-chain-mod-n` (blocked on that paper) |
| R3 | odd power-sum coordinates (slot-separable) | S3: j = 1, which is the generic pair table; S4: not expressible for j <= 8 on 6/6 draws, controls pass | `LEAD-ICE-power-sum-coordinates` (closed, scoped) |
| R4 | uncorrelated slot partition for the n = 83, m = 10 gate | prepared as cell G6 in #2679; needs the checkpoint token to run | existing |
| R5 | mixed chain (S3 leaves, norm form at cubic merges) | is the R1' design itself (auxiliaries only on cubic merges) | folded into R1' |

Every number here is an **unverified assertion** (no pcc claim was minted; the
`research-kb` server was unreachable from the producing session, so novelty rests on the atlas
`--match` and one literature search).

## Reproduce

    cd derive
    cargo test --release                 # 9 unit tests (field, group law, identities, controls)
    cargo build --release
    target/release/replead-derive counts   > ../results/counts.jsonl
    target/release/replead-derive census   > ../results/census.jsonl   # exhaustive, n = 11, 13
    target/release/replead-derive powersum > ../results/powersum.jsonl
    cd .. && python3 plot.py && python3 -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf')"

Seeds are fixed in `derive/src/main.rs` (`SEED`); output is deterministic.
