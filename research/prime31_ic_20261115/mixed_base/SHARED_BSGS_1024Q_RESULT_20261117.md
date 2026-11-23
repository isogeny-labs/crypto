# Prime31: shared-precomputation generic BSGS **refutes** the apparent IC performance frontier

2026-11-17 UTC. **Source-specific Rust experiment completed successfully** (one GitHub-hosted runner, one public 1024-Q target cohort) on actual ordinary cofactor-one p=2^31−1, group order n=2147446831. This is a *correctness-checked producer-development experiment* [pcc:unverified], not a signed independent PCC, isolated repeated hardware comparison, new non-generic attack or largest-scale cryptanalytic result.

## Outcome and acceptance

- [Successful exact-source Rust CI run #44031618724](https://github.com/isogeny-labs/crypto/actions/runs/44031618724) on source SHA `df661dee03446583f232998c95eb326f4dab5ae1` (the frozen comparator source + the sole compile-time batch counter binding repair), Rust 1.100.0. Original native library **40/40 tests**, original reusable IC **3/3 tests**, source-exact appended BSGS/Rho example **23/23 tests**.
- **1024/1024** original IC unknown-scalar recoveries, **1024/1024 for EACH of three generic BSGS table sizes**, and **1024/1024 for EACH of the original native unfolded and folded rho32 arms**; rho A/A 64/64. Identical public Q throughout, and known caller k never crosses into the BSGS solver.
- [Raw full 1024-Q records, artifact #14305472259](https://github.com/isogeny-labs/crypto/actions/runs/44031618724/artifacts/14305472259): includes IC output, three BSGS arms and two rho arms; original ZIP SHA256 `dcf190695bfd9b939f5316b180f01db2beb20c243beed3f312923b33b4326cb2`.
- Independent arithmetic readback used separately reimplemented original-field affine point addition and scalar multiplication, not the producer's Rust routines or an alternative discrete-log solver: verified every `[k]G=Q` in all six output cohorts, 61 recovered basis logs and the modular equations for 61 setup rows and 1024 target relations. Details are in [SHARED_BSGS_1024Q_INDEPENDENT_AUDIT_20261117.json](SHARED_BSGS_1024Q_INDEPENDENT_AUDIT_20261117.json). This is **not** independent cryptographic signature/PCC certification.

## Fully charged same-runner batch, all 1024 identical Q

| Complete Rust solver | Once-per-curve setup | 1024 target work | Full batch | After-setup process RSS |
| --- | ---: | ---: | ---: | ---: |
| Mixed 4+4 IC (61 independent factor-base logs) | 1315.438 ms | 906.035 ms | **2221.474 ms** | Not recorded |
| Original unfolded rho32 | – | 2246.126 ms | 2246.126 ms | Not recorded |
| Original folded rho32 | – | 4360.294 ms | 4360.294 ms | Not recorded |
| Shared BSGS m=46341 | 2.878 ms | 985.153 ms | **988.031 ms** | 12,288 KiB |
| Shared BSGS m=262144 | 22.912 ms | 193.299 ms | **216.211 ms** | 27,504 KiB |
| **Shared BSGS m=1048576** | **103.840 ms** | **66.274 ms** | **170.113 ms** | **67,924 KiB** |

Memory is a sampled process RSS *after setup* on a serial 3-arm run, NOT isolated maximum RSS; IC memory was not sampled. The BSGS hash capacities were 57,344 / 458,752 / 1,835,008 entries; raw baby entries m−1. Input-generation and verification policies are the same source-specific workflow for every arm; timestamps are unisolated GitHub Actions monotonic-clock wall measurements. Rho's original method had an in-solver scalar replay. No hardware cost external validity beyond this scope is asserted.

### Meaning of these numbers

- The pre-registered million-entry *generic* BSGS arm cost **0.07658×** the complete IC batch (**13.06× faster**) and **0.07574×** the original unfolded rho32 batch (**13.20× faster**), with all 1024 complete.
- Even the conventional square-root sized shared BSGS table cost **0.44476×** the complete IC batch. The advantage is classical table reuse, not a non-generic mathematical breakthrough.
- With the *current* same-run observations, IC/unfolded rho32 = **0.98902**, only approximately **1.10%** lower, vs 0.88971 / 11.03% in the prior [merged PR #4513](https://github.com/isogeny-labs/crypto/pull/4513) run. **This directly demonstrates run-to-run timing variability** and limits the earlier IC/rho claim. Do not mix absolute times across runs.
- A 20,000-replicate independent paired-target bootstrap (fixed charged setup, identical 1024 Q, deterministic bootstrap seed 20261117) estimates 95% percentiles for BSGS/IC at m=46341: [0.4261,0.4638], m=262144: [0.0938,0.1009], m=1048576: [0.0745,0.0786]. These intervals only resample targets; they omit run-to-run hardware, clock, setup variability, arm order and system memory effects. They do **not** imply external validation or optimum-complexity proof.

### Protocol custody and failed precursor

[SHARED_BSGS_1024Q_PROTOCOL_20261117.md](SHARED_BSGS_1024Q_PROTOCOL_20261117.md) froze the 1024 public Q, m sizes, complete-cost criteria and original immutable source before BSGS implementation. The first source-specific run [#44031421068](https://github.com/isogeny-labs/crypto/actions/runs/44031421068) passed 40 lib and 3 IC tests but failed *before any measurement* with E0425 `baby_batches` instead of the existing local `batches`. Repair only changed the constructor to `baby_batches: batches`. Failed runs are retained, not treated as benchmarks. The passing run is pinned to the corrected source SHA.

## New mathematical lead and next experiments

The essential cryptanalytic frontier is now sharper. The previously reported complete IC improvement **does not survive** comparison against classical *equally amortized* precomputed generic BSGS, in this instance. The IC relation oracle is a generic group 4+4 meet-in-the-middle representation; a relative rho-only advantage cannot establish non-generic structure.

Next, test genuinely nonenumerative **target-conditioned RR9/S9 factor-polynomial extraction** from public residual `R=aG+bQ` without enumerating signed four-sum supports or recovering any secret scalar. Reuse PR #4466 Hermite and norm code and #4228/#4349 Rust quotient/subresultant/rootfinding routines. First confirm B4/B8 exact positive and negative solution fibres (including Dickson31 negative controls), then B31 1024 held-out Q with all elimination, rootfinding, relation collection, matrix solving, failures and memory charged against this exact generic BSGS frontier.

An engineering sublead is rank/relation trade-off using mixed factor bases of size B=28,29,30 versus B=31 with identical public targets and signed transcript replay, plus batch-Jacobian complement enumeration to reduce inversions. These are **generic optimizations**, and must be clearly separated from a novel non-generic attack. The no-improvement Rust buffer-reuse and linear-chord experiments #4484/#4486, autoresearcher SA negative leads #3301/#3305 and unchanged anchored/tower F4 and Vieta trials are not repeated. A memory- and precomputation-matched Bernstein–Lange rho and mature external IC comparisons remain outstanding; no leaderboard promotion.

**Conclusion:** The new fully charged benchmark has been implemented, checked and independently arithmetically replayed, revealing a strong *generic* counterexample to our prior rho-only frontier comparison. A cheaper genuinely non-generic prime-field ECDLP solver is **not** established by this run. [pcc:unverified].
