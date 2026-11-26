# Ideas 68–100: bounded execution

2026-11-24. Draft PR #5165. This continues the item assessment while keeping elliptic-curve relations distinct from integer smoothness in `F_p^×`. Numerical and execution statements are producer analyses `[pcc:unverified]` unless a signed verdict is cited.

## Inputs, hand derivation, and attempted native replay

The repository's `src/cryptanalysis/large_prime_filter.rs` already handles arbitrary multi-large-prime rows over the subgroup field. Its tests cover single and double large primes, a peeled hypergraph component, arbitrary coefficients, fill rejection, and exact source-row replay. The collector must additionally verify the original curve-group identity. A graph cycle without coefficients is not a row dependency: `[[1,1,0],[0,1,1],[1,0,1]]` has determinant 2 and full rank modulo 5 despite a triangle. For #76, target pinning is exactly target-basis-vector membership in the augmented row span. The prior `workspace/docs/notes/graph-theory-ic-20261121.md` checks the two-summand frame-matroid stopping rule on 900 generated streams. Its GT-01 structured-Gauss comparison reports linear algebra at 0.03–0.14% of counted cost on its prime cells. Even eliminating that stage gives an Amdahl ceiling of `1/(1-0.0014)=1.001402` for those cells (#84); larger matrices require separate measurement.

Native command attempted: `RUSTC=/Users/adamburan/.rustup/toolchains/1.99.1-aarch64-apple-darwin/bin/rustc CARGO_TARGET_DIR=/Volumes/SSD990-2/crypto/target /Users/adamburan/.cargo/bin/cargo +1.99.1 test --lib large_prime_filter::tests:: -- --nocapture`. Homebrew Cargo/rustc 1.93.1 failed dependency MSRV checks before the explicit compiler path was set. The corrected build reached the library test target but stopped on 30 missing `include_bytes!`/`include_str!` fixtures in this sparse checkout, including `gpu/certicom/emulator.cpp` and `research/chained_transfer_next_20261115/select_only.tsv`. There is no new native test pass. No unrelated fixtures were materialized.

Hand-checkable modular control, without an executable research claim: over `F_101`, let the column order be `(x0,x1,x2,L10,L11,L12)` and assign values `(3,5,12,10,9,7)`. The rows `(1,0,0,1,-1,0)`, `(0,1,0,0,1,-1)`, `(0,0,1,-1,0,1)` have respective right-hand sides 4, 7, 9. Their sum is `(1,1,1,0,0,0)` with right-hand side 20; each large column cancels. The three source rows have distinct pivots in `x0,x1,x2`, so rank 3; the completed small row alone has rank 1. Adding `x0=3` and `x1=5` gives three independent small rows and determines `x2=12`. Omitting `x1=5` leaves rank 2 and `x2` free. A disjoint two-column component with rows `(1,1)` and `(1,-1)` has determinant -2, so rank 2 modulo 101 and whole block rank `3+2=5`. This deliberately separated matrix checks algebraic cancellation, rank, and target coverage but is not an EC point-relation dataset or a performance result.

Four proposed raw sources were checked: `benchmarks/records/20261113T1855Z-ic-filter-layout-ab.json`, `20261114T2243Z-ic-scan-vfilter-ab.json`, `20261115T2132Z-ic-filter-double-ab.json`, and `20261116T2046Z-budgeted-early-rank-relations-20261116.json`. The first three are aggregate filter-layout A/B records with S3 archives, no inline relations; their old local `/dev/shm` or `/tmp/claude` archive paths are unavailable. The fourth is a symbolic rank/degree panel with 19 locally absent archives, not a large-prime relation stream. None as mounted provides immutable partial rows, subgroup modulus, source witnesses, target coefficients, and timestamps for the requested exact EC replay. Their workload identities were not combined.

For #96, the historical `workspace/docs/notes/graph-theory-ic-20261121/probe_summary.csv` (SHA-256 `8c531484925862e447f8597802cf8f54b218b74a48ca55e061502f5a26dec8c5`) reports 24-repeat, seed-777002 prime `m=2` sweeps `[pcc:unverified]`. Their best recorded frozen sizes are B16 at order 65003 (`S=13.460`) and B32 at order 261673 (`S=12.839`); old B40/B63 controls record `S=19.620/20.389`. These are prior measurements, not a new adaptive-policy result. No time-ordered relation stream or target-descent comparison is available from this aggregate CSV.

Protocol error: before inspecting `AGENTS.md` §Implementation language, this workstream used Python to parse the historical CSV and to check the synthetic matrix. That section forbids Python for cryptographic arithmetic, linear algebra, replay, and research result generation. The exploratory calculated transfer-policy ratios and scripted matrix output are excluded as evidence. The displayed matrix is retained solely as a direct, hand-checkable derivation. No further experiment was run after the S3 checkpoint gate was reported as 403.

## Every item: result or specific missing input

| # | Bounded result and remaining discriminating input |
|---:|---|
| 68 | Existing exact block Wiedemann/Lanczos is the control. No matrix+RHS export in selected graph summary for BP residual/rank replay. |
| 69 | Counterexample: integers 2 and 7 coincide modulo 5, but only 2 is 2-smooth. Residue-only compressed measurements cannot recover the lift's factorization. |
| 70 | Polar channel needs explicit transition probabilities and a prime-to-channel map; compare held-out independent rank at matched size and cost. |
| 71 | CVP needs an exact short-vector-to-EC-point-relation map and a costed CVP solver; neither specified. |
| 72 | `8=1000₂` is 3-smooth, `10=1010₂` is not; Hamming distance one gives no deterministic smoothness guarantee. |
| 73 | Reed–Solomon recovery requires stored parity records and an erasure model absent from selected relation summaries; compare storage/IO to duplication. |
| 74 | BCH needs a defined sieve-codeword and a no-false-negative syndrome; no encoding specified. |
| 75 | Existing native coefficient-aware hypergraph filter and source replay are the baseline. New EC partial-row corpus is the blocker for a matched variant. |
| 76 | Exact target-row-span criterion and prior 900-stream frame-matroid check. Missing source rows prevent new rank-preserving filter comparison. |
| 77 | On a three-vertex path, max-cut cuts both interaction edges, maximizing cross-part traffic in this example. Need actual matrix traffic to claim benefit. |
| 78 | No tree traversal to certified EC decomposition map specified; require yield per total cost against existing candidate generator. |
| 79 | No pre-oracle features, held-out labels, or costed graph-message model supplied; require certified yield rather than classifier accuracy. |
| 80 | Prior exact counterexample: de Bruijn windows `00,01,11,10` map to `0,1,3,2`, then `0,1,0,2` modulo 3. |
| 81 | Sperner needs a boundary labeling and a fully labeled simplex to point-relation implication; neither specified. |
| 82 | Matching on known factors occurs after their factorization; need an available pre-oracle incidence graph and zero-false-negative proof. |
| 83 | Prior exact counterexample: `[2]` has real rank 1 and rank 0 over `F₂`; real SVD does not preserve modular rank. |
| 84 | Prior prime-cell LA share yields <=1.001402 total-cost ceiling there. Missing larger exact matrix and stage costs for a new block test. |
| 85 | ADMM real residual is not a finite-field certificate; require exact `Ax=b` replay and convergence/cost against block Wiedemann. |
| 86 | Real Arnoldi eigenvalues do not certify finite-field kernel; require modular Krylov and original-row checks. |
| 87 | Chebyshev filtering assumes ordered real spectrum; define an exact finite-field polynomial and compare matvec count. |
| 88 | Repository already has finite-field block Lanczos with breakdown retries and original-row verification; Jacobi variant needs same checks. |
| 89 | Nonlinear feature lift needs proof that it preserves and recovers the original modular kernel; no map supplied. |
| 90 | FMM needs an exact compressible off-diagonal kernel; none identified in generic sparse relation rows. |
| 91 | Dual NFS/FFS needs a homomorphism carrying EC point relations to a common logarithm group; none specified. |
| 92 | Approximate holographic proposals must pass exact modular and curve-group checks; cost false candidates and retrieval. |
| 93 | `E(F_p)⊂E(F_{p^k})` remains an EC group; a field DLP transfer needs a suitable pairing and embedding degree. |
| 94 | Residue slices are testable for integer sieving, but require a no-false-negative map to EC point decomposition for this scope. |
| 95 | Short lattice vector needs a certified EC relation implication and priced search; lattice/map unspecified. |
| 96 | Historical fixed-size values above. The attempted Python transfer replay is excluded by the implementation-language rule. A genuine adaptive test needs native code, time-ordered rows, and complete setup/collection/solve/descent costs. |
| 97 | Selected CSV has aggregate trials, rows and cost but no interval order or yield trajectory; a PID policy cannot be replayed from it. |
| 98 | Dropping one column from equation `x+y=1` changes its solution set unless its value is known or exact elimination retains a witness. Need rank and descent coverage. |
| 99 | No cheap stationary potential correlated with EC decomposition specified; require mixing/effective samples and full-cost yield. |
| 100 | Two identical balanced binary columns each have maximal marginal entropy but add only one rank dimension. Require held-out conditional rank gain. |

The repository inventory `docs/ic/ECDLP_RESEARCH_TECHNIQUES.md` and Semaev's prime-field EC summation-polynomial paper (https://eprint.iacr.org/2004/031.pdf) ground the domain and existing baseline. KB revision `752f945c662745959e4516205d027b5e` was ready but last refreshed 2026-11-22, before this worktree; owning files were read directly. The next valid replay input is a frozen full/partial EC relation export with group-identity witnesses, subgroup modulus, source IDs, target coefficients, time order, and full stage costs.
