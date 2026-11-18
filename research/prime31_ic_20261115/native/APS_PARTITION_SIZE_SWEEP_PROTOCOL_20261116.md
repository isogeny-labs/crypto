# R31-APS-SIZE: Rust-only one-relation IC partition-size sweep (2026-11-16)

**Standing:** preregistered matched-source development experiment. Results from a new CI run must be frozen separately; do **not** infer results from the prior 12/12 T=1536 source. **[pcc:unverified]**.

## Aim and exact experimental arms

The completed Rust APS-style one-relation prime31 solver on [#4374](https://github.com/isogeny-labs/crypto/pull/4374) recovered 12/12 public targets at T=1536, with a 268.610ms cold single-Q observation and 2.271s total across heldouts in the frozen prior CI. Its finite-list pair S3 quadratic search is generic meet-in-the-middle equivalent, despite being an actual complete one-relation index-calculus method. This sweep is a cost optimization **within that already established published-control methodology**, not a novel non-generic attack or a result against optimized external APS.

The next Rust mode `--size-sweep` uses the **same 12 independently generated public prime31 Q** as the prior heldout panel (caller-only known scalar used after recovery), and runs the unmodified algebraic S3 extraction semantics over the following preselected partition sizes and bounded rounds:

| Points per labelled partition T | Maximum rounds per target |
|---:|---:|
| 768 | 12 |
| 1024 | 8 |
| 1536 | 3 |
| 2048 | 3 |
| 3072 | 2 |

Each round rebuilds its 3T public labelled points `aG+bQ` and charges this construction, all S3 field quadratic/sqrt queries, every miss, sign recovery and independent scalar replay. Each arm logs per-target success/failure, total paid in-process wall, visited pairs and actual attempts. No known k is passed into the algebraic solver. Rejected duplicate-abscissa and infinity points are replaced by newly generated public points, as in the frozen previous implementation. Failure to recover before the cap remains a *failure within the registered budget*, not a negative ECDLP existence result. At nonzero failure rates, conditional successful-only times are not comparable complete costs.

**Model to test, not assume:** With three independent random point lists of size T, each signed triplet corresponds to four choices modulo global simultaneous negation. The heuristic relation rate is `lambda ~ 4*T^3/n` and heuristic one-round success `1-exp(-lambda)` before degeneracies and input dependencies. This suggests an increasing T may reduce pair queries to first hit, while raising costly labelled source setup. The PRNG's modular reductions and random walk are deterministic, so Poisson is a guide only. The measured *entire cold cost per verified target*, including retries and zero-return outcomes, decides the improvement.

## Reproduction

    cargo +1.100.0 test --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation
    cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_aps_one_relation -- --size-sweep

The existing workflow `.github/workflows/prime31-s4-split-20261116.yml` runs this step on the exact PR source in Rust release mode. No Python/C++ execution. The previous `--one`, `--compare`, `--holdouts` paths remain with T=1536 and must continue passing. The `--size-sweep` loop's per-arm totals are descriptive same-host development observations, without randomized A/A ordering, isolation, uncertainty interval, external tool equivalence or independently verified PCC. **Do not declare an absolute speed frontier on a single run.**

## Previous leads checked and next scientific direction

Consulted AGENTS.md/current log, #4188 (original natural m4 S3 quotient, B31 squarefree K), #4373 (chained S3 m8 censored), #4374 (complete published APS m3 one relation), #4228 (certified quotient) and current #4349 (selected-root lazy compact with recent native evidence; no admitted wide-prime m8 oracle). Checked papers PKM PKC 2016 DOI 10.1007/978-3-662-49387-8_1 and APS FFA 2018 DOI 10.1016/j.ffa.2018.01.009. The shared KB tools and repo wrapper are unavailable from this connector/container runtime; no exhaustive novelty clearance.

A separate new source-exact **Rust CI success** at [run 43788581330](https://github.com/isogeny-labs/crypto/actions/runs/43788581330), head `fa6f90d7337ad16475f139b140484a827dfb1bb6`, established the H31/K961 rational quotient **semantic check**: 40/40 library tests, 1/1 quotient bridge test, 31 admitted of 64 Dickson x, 465 S3 quadratics +31 linear factors, degree K=961, and all 961 signed pair-coordinate checks. [Frozen output](M8_RATIONAL_QUOTIENT_BRIDGE_CI_20261116.json). This is a construction-equivalence result, *not* target-conditioned m8 F4 roots or a scalar solve.

The mathematical next step remains original-ideal-equivalent selected-root elimination of the **target-dependent S5 equation in four K-root variables** (or equivalent compact balanced m8) without expanding all `961^4=852891037441` support tuples. Exact rational solutions, identity/cancellation branches, full-y signed replay and complete roots are required on small B first, followed by B31 natural targets. The APS size-sweep can improve a validated complete-control implementation while the m8 lead stays blocked by this algebraic integration problem.

**Acceptance and falsification:** A smaller T is only better if its *unconditional paid* cold cost per recovered heldout target improves without introducing unacceptable missed-case probability; source-exact Rust tests must compile and retain all failures. To claim a non-generic frontier improvement, the distinct m8 method must produce actual natural target-dependent algebraic relations faster than the strongest completed PKM/APS oracle and same-ideal native/external F4/CAS, not merely smaller partial matrix counts.


## Refreshed certified-root lead disposition

The latest checked [#4349](https://github.com/isogeny-labs/crypto/pull/4349) report (head `9c31116010eafc3882769655e58354e4318ac436`) now provides a **negative complete-cost** result on its three registered six-variable exploratory systems: lazy compact certification was 10.88–11.24× faster than *expanded certificate* worker calls, but with shared mature-reference setup/checking charged it was only 2.43–2.99× faster than expanded, and **optimized native F4 remained faster than compact**. Its source-specific L0 producer evidence remains PCC-unverified, and it does not cover prime31 m8 or a cold ECDLP. Consequently, #4349's constructor must not be treated as the default fastest oracle simply because it produces shorter certificates. The appropriate next test is a genuinely different small-B target-conditioned m8 ideal where the original F4 work dominates and a certified quotient actually finishes, with completed same-input F4 and mature-CAS controls. Stop at that gate if it loses; do not uncritically transplant a 15× headline into prime31.
