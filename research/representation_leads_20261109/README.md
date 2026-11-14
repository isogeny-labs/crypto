# Representation leads for binary point decomposition (2026-11-09)

Exact derivations (no timings, no experiment runs) for five representation ideas aimed at the
regime where a per-attempt budget exists on ECC2K-130 (m >= 5; see `REPORT.pdf`). All
computation is native Rust in `derive/`; `plot.py` only draws `results/`.

| lead | representation | outcome here | atlas entry (crypto-autoresearcher) |
|:--|:--|:--|:--|
| R1' | line norm form per merge: mu = e2 (and slope lambda) quadratises the chained S3 system | degree 2 confirmed; same variables and monomial support as a generic quadratisation w = x_a x_b, 30-40% more terms; the trace equations are implied in any chain (census) and known (Lenstra-Kosters 2013, KR-IC-f5c584); `PROTOCOL.md` frozen 2026-11-14 with instrument and pinned systems | `LEAD-ICE-quadratised-chain-representation` (open, gated on the checkpoint token) |
| R2 | factor-base membership as a Frobenius chain modulo N(X) | closed 2026-11-14: dominated inside Huang-Kosters-Yang-Yeo (arXiv:1505.02532, Thm 1.1 and Sec. 4.4): subspace constraints are already weight 1 there, and the chain only adds l m big-field variables | `LEAD-ICE-subspace-frobenius-chain-mod-n` (closed) |
| R3 | odd power-sum coordinates (slot-separable) | S3: j = 1, which is the generic pair table; S4: not expressible for j <= 8 on 6/6 draws, controls pass | `LEAD-ICE-power-sum-coordinates` (closed, scoped) |
| R4 | uncorrelated slot partition for the n = 83, m = 10 gate | prepared as cell G6 in #2679; needs the checkpoint token to run | existing |
| R5 | mixed chain (S3 leaves, norm form at cubic merges) | is the R1' design itself (auxiliaries only on cubic merges) | folded into R1' |

Every number here is an **unverified assertion** (no pcc claim was minted; the
`research-kb` server was unreachable from the producing session, so novelty rests on the atlas
`--match` and one literature search).

## Reproduce

    cd derive
    cargo test --release                 # 11 unit tests (field, group law, identities, controls)
    cargo build --release
    target/release/replead-derive counts   > ../results/counts.jsonl
    target/release/replead-derive census   > ../results/census.jsonl   # exhaustive, n = 11, 13
    target/release/replead-derive powersum > ../results/powersum.jsonl
    target/release/replead-derive export   > ../results/systems.jsonl   # PROTOCOL.md cells (sha256 pinned there; not committed)
    cd .. && python3 plot.py && python3 -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf')"

Seeds are fixed in `derive/src/main.rs` (`SEED`); output is deterministic.
