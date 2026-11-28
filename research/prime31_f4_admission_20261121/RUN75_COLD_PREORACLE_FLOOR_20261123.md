# Run75 — architecture-specific cold-cost feasibility floor (2026-11-23 UTC)

## What is now established
An independent read/recalculation of the frozen [Run72 original-Rust CI data](RUN72_RESULTS.json) establishes a **negative engineering bound for the existing K=2/3/4 architecture**, before any new root-extractor timing is included:

> With the original Run72 setup plus public witness extraction unchanged, making the entire remaining relation-walk phase free would still **not** beat the matched shared BSGS control for 32 unknown logs in any of the nine measurements.

This is a *conditional component-floor statement*, not a lower bound for all index-calculus algorithms, factor-base choices, alternative implementations, zero-setup amortized pipelines or public-only algebraic extractors.

Original Run72 receipt: [native Rust Actions #45077314037](https://github.com/isogeny-labs/crypto/actions/runs/45077314037), source commit `783d2a928a548d48be9f2e0e6e7be9b0cf60141b`, 9/9 K/seed cells and 41/41 original-library tests for each job. Curve `E/F_2147483647 : y²=x³+847514846x+2087444724`, `n=2147446831`, cofactor one, B=677, 88 homogeneous rows, **GENERIC** 916,469-key pair dictionary, 32 public unknown targets per seed. Logs and target certifications remain valid *generic controls*, not qualifying non-generic ECDLP improvements. Recompute these nine pure-cost inequalities with the checked-in original-crate Rust replay `examples/prime31_run75_cost_floor.rs`.

## Results directly calculated from the nine frozen Run72 cells
`pre=original cold setup_ms+witness_ms`. `BSGS=matched setup_ms+online_32_ms`. `pre/BSGS>1` means relation work alone cannot be the crossover. The final column is the **minimum required reduction in this pre-stage** even if the entire relation stage miraculously becomes free; in reality a nonzero relation stage requires more.

| Cohort seed | K | Complete cold candidate / BSGS | Fixed pre / BSGS | Minimum pre reduction with free relations |
|---|---:|---:|---:|---:|
| 0x2c6478eb19d | 2 | 7.736x | 1.844x | 45.77% |
| 0x2c6478eb19d | 3 | 5.775x | 2.388x | 58.13% |
| 0x2c6478eb19d | 4 | 5.032x | 2.358x | 57.59% |
| 0x68b677a1d1 | 2 | 7.109x | 2.142x | 53.31% |
| 0x68b677a1d1 | 3 | 5.575x | 2.497x | 59.95% |
| 0x68b677a1d1 | 4 | 5.402x | 2.752x | 63.66% |
| 0x9d3a6f5c127 | 2 | 5.023x | 1.790x | 44.12% |
| 0x9d3a6f5c127 | 3 | 4.851x | 2.084x | 52.02% |
| 0x9d3a6f5c127 | 4 | 4.828x | 2.316x | 56.83% |

All 9 fixed-pre floors exceed matched BSGS. For the three K=4 cohorts, the baseline spends **573.592 / 647.688 / 623.517 ms** in setup plus witness construction before relation work; the corresponding BSGS 32-target totals are **243.237 / 235.344 / 269.195 ms**. Therefore the most optimistic fixed-pipeline K4 relation-optimization-only limit is at least **2.32–2.75x slower** than BSGS. The already measured complete-cold K4 is **4.83–5.40x slower**. The matched BSGS is not the strongest optimized negation-map rho or index-calculus baseline, so beating this weak control would only be a necessary, not sufficient, gate.

## Immediate lead triage

**P0 — genuinely public-only nonenumerative relation oracle.** Existing #4692, #4812, #4822 own RR9/Horner/F3 and exact root extraction. Replace the generic pair/triple table *and* the expensive witness phase, not merely the public kG walk. Keep separate positive/negative root verification; no extrapolation from algebraic circuit input degrees.

**P1 — independently measured Prime31 m=4 chained S3 / radical quotient.** [#4924 native F4 CI](https://github.com/isogeny-labs/crypto/actions/runs/45086381347) now passes Rust small-stage pilot. The degree-B representation is sometimes faster on a planted m=2 instance, but m=4 symmetric stage doubles F4 field operations and the quadratic circuit is slower at B=4. This is a stage observation, not a whole solver.

**P2 — rank-to-cost and multi-target amortization.** K3/4 target-certificate difference rows produce authentic labelled extra rank; explore distinct public targets T=1/32/128/1024 on matched resources only after public non-generic extraction succeeds. Recalculate this floor because new architecture/precompute sharing can change which costs are fixed.

**Quarantine — m2 micro-optimization alone.** For B677, at most `4*binom(B+1,2)=918012` signed pair sums among `n=2147446831` group elements. Under uniformly distributed independent probe targets, success probability is at most 0.00042749, corresponding to at least about 2,339 attempted probes per success. This random-query rate and the observed Run72 fixed pre-cost both disfavor a cold m2-only speedup. It is **not** an impossibility theorem for adaptive algebraic algorithms.

## Explicit next experiment and acceptance gates

1. Start from the currently existing original Rust `f4_fp`, Semaev, curve, authentic-RHS and DSU/rank code; do **not** create an independent replacement EC engine.
2. Run the already registered public-only `46+18` held-out algebraic-root extraction panel under a bounded wall/RSS allowance, instrumenting all preparatory witness generation and pair dictionary access. The candidate MUST not inspect generic pair/triple tables or harness logarithms.
3. Accept root witnesses only after signed curve addition, honest public RHS rank increment and independent `[x]G=Q` checks. Reject all unsupported timeout/UNSAT claims.
4. Compare 1/32/128/1024 target cold/batch fully charged time and RAM against strongest project index calculus, source-linked BSGS and optimized negation rho on the same machine. Preserve both failures and successes.
5. **Pass:** statistically reproducible total-cost improvement (including setup, unsuccessful attempts and all log recovery) AND a genuinely algebraic/non-generic relation oracle. Until then the complete solver advantage is not established.

### Reproduction and limits
```bash
cargo test --locked --release --example prime31_run75_cost_floor
cargo run --locked --release --example prime31_run75_cost_floor
```

This new Rust diagnostic was committed but **was not natively compiled in the current analysis environment**; the numerical inequalities above were independently calculated from source-frozen JSON. The older full Run72 Rust solver and CI were native and successful. PCC/signature verification remains pending.
