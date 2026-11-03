# Four additional ECDLP experiments: geometry gates

2026-10-19. This is a second lead search after the [five-lead follow-up](../five_leads_e2e_20261019/REPORT.md). “Additional” means a distinct experiment for this workstream, not new mathematical priority. All numbers here are exact finite-field **geometry gates** or eligibility checks. They are not end-to-end DLP measurements, instruction counts, or speedups. The earlier 1,260-target genus-2 incumbent and source negation-rho results remain the comparison baseline.

| Lead | Distinct mechanism | First gate run here | Next discriminating test |
| --- | --- | --- | --- |
| Q1. Quartic point-incidence sieve | Replace repeated line-intersection factoring with a point-pair/line-incidence table on a plane quartic | Exact four-point line census on both published Tian equations | After an executable elliptic-to-quartic map exists, compare complete one-target harvest, linear algebra, setup and verification against Diem-style line testing and source rho |
| Q2. Dynamic small-prime base for quartic relations | Change which rational points count as small primes as the large-prime graph develops | Fixed-base eligible-line populations at three sizes, on the same two equations | Freeze a target-blind promotion rule on disjoint training curves; charge promotions, graph work, duplicate lines and rank updates against fixed bases and rho |
| G1. Genus-2 pencil sieve | Sweep the functions `y-v(x)-λu(x)` for a transferred divisor instead of testing only whether its current quadratic `u` splits | Geometric `F_257` census, followed by 100 target-labelled walks and a 20-target linear solve on an admissible `F_739` source fixture | Instrument the full collector and compare complete one-target cost with the existing two-point collector and source rho |
| T1. Direct trace-zero index calculus | Attack the trace-zero subvariety of an elliptic curve defined over a cubic extension's base field, without constructing a Jacobian quotient | Exact `j`-invariant gate rejects the published Tian cubic fixture | Freeze a separate base-defined `E/F_q` cohort with prime-order trace-zero subgroups; solve and price its decomposition systems and matched Frobenius-aware rho |

## Q1 and Q2: plane-quartic line geometry

Vitse and Wallet's [point-incidence sieve](https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf) replaces many polynomial splitting tests by incidence arithmetic. For a smooth quartic, a nonvertical line with four distinct affine rational intersections gives a principal divisor relation against a fixed reference line. This first gate reuses the two published equations whose smoothness was checked in the five-lead study. It enumerates every affine rational point, groups point-slope incidences, independently recovers each line from point pairs, and re-enumerates polynomial roots for every four-point line. Tangent multiplicities, vertical lines and intersections at infinity are outside this census.

| Equation | Affine points | Complete four-point lines | Lines with at least two fixed small points: 1/8 · 1/4 · 1/2 base |
| --- | ---: | ---: | ---: |
| Tian `F_523` | 522 | 11,090 | 818 · 2,833 · 7,610 |
| Tian `F_257` | 250 | 2,451 | 190 · 634 · 1,687 |

The fixed bases are the first floor(`fraction × affine points`) under the SHA-256 point ranking in [the script](quartic_line_gate.py). The full `k=0…4` line histograms, small-point histograms, first witnesses and SHA-256 digest of the ordered complete lines are in [the receipt](line_gate_results.jsonl). These counts establish that the incidence relation pool is nonempty and that a fixed small base discards many full lines. They do not establish a faster collector or a valid transfer from the source elliptic curve. In particular, the cited paper explicitly says its quartic **double-large-prime** sieve does not provide a significant speedup in that setting; its roughly threefold result is for different Diem–Kochinke methods and is not applied to these rows.

Q2 is a separate policy question motivated by [Laine and Lauter's time–memory work](https://eprint.iacr.org/2014/346): whether promoting a carefully selected point while the graph grows reaches a useful target dependence with lower *charged total cost* than a frozen small base. A fixed-base population is only the prerequisite. An adaptive selection must be trained separately; an oracle that chooses points after seeing all target relations would invalidate the comparison. Since the present quartic transfer maps and DLP solver are still missing, both Q1 and Q2 remain **partial**.

## G1: genus-2 pencil geometry

The hyperelliptic sieve described by [Vitse and Wallet](https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf) gives a new relation-generation arm to test with the executable Scholten genus-2 transfer. The original [geometry script](genus2_pencil_gate.py) uses the same quintic equation `12Y²=12+100t+299t²+380t³+187t⁴+30t⁵` over `F_257`. It checks squarefreeness, chooses 20 disjoint pairs of rational points using a fixed hash rank, forms each monic Mumford `u` and interpolant `v`, and exhausts all 257 values of `λ`. Every quotient of `(v+λu)²-h` by `u` has zero remainder. Every three-distinct-root quotient is rechecked on the curve.

This yields **1,055 complete three-root quotients out of 5,140 pencil members**, with 50–57 per divisor. Per-divisor counts, root histograms, pair identities and relation-list digests are in [the receipt](pencil_gate_result.json). **Correction:** `p=257` is `1 mod 4`, while the implemented `Scholten::new` transfer requires `p=3 mod 4`. Thus the `F_257` result is only standalone genus-2 geometry; the earlier wording incorrectly implied that this exact field supported the executable source transfer. The geometric receipt remains valid and unchanged.

The follow-up [native gate](native/target_labelled_pencil.rs) uses the first public exact-order fixture of `order-genus-16.json`: `p=739`, source subgroup order `r=34,147`, and its recorded generator. It constructs 20 public known-answer targets `Q=kG` and five labelled source walks `aG+bQ` per target; it transfers each walk through `tau`. All 100 transferred divisors have degree two. For every `λ∈F_739`, it divides `(v+λu)²-h` by `u`, enumerates the cubic's rational roots, and checks **every complete relation** as `D+R₁+R₂+R₃=0` in the actual Jacobian. The first relation of each walk also passes the dual norm check back on the source elliptic curve.

| Exact `F_739` gate | Count |
| --- | ---: |
| Target-labelled walks / pencil members | 100 / 73,900 |
| Complete, Jacobian-verified three-point relations | 12,233 |
| Walks with a split two-root `u` | 46 |
| Point-driven sieve probes / ordinary + exceptional denominators | 73,710 / 37,105 + 95 |
| Batched inversions / batch multiplications | 100 / 111,600 |
| Candidate `λ` values / accepted `λ` values | 12,277 / 12,233 |
| Targets recovered by online three-point elimination | 20 / 20 |
| Relations inserted before recovery, across targets | 7,244 (357–366 each) |
| Modular row updates / inversions, across targets | 9,907,551 / 7,208 |

The point-driven sieve evaluates `λ=(y-v(x))/u(x)` at rational curve points, batching all ordinary and exceptional nonzero denominators once per walk. It explicitly handles roots of `u`, including three repeated-root `u` walks. Its set of accepted `λ` values agrees **exactly on every walk** with the exhaustive 739-member scan. The current native gate still uses that exhaustive scan to feed the solver, so its runtime is not a measurement of a sieve-driven collector.

For each target, an online modular solver projects all 739 rational curve points by cofactor 16 and verifies that the projected points are killed by `r`. It identifies the two signs above an `x` coordinate, inserts complete three-point relations with their public source labels, and performs row elimination until a target scalar is determined. Every recovered scalar is checked against the source generator. The first solve occurs in walk 3 for 13 targets and walk 4 for seven. The 7,244 inserted relations have ranks 355–362 per target at recovery; only 1–7 per target are dependent at that point. Thus most complete pencil quotients are redundant for a given target, even though all are valid relations.

The [JSONL receipt](target_labelled_pencil_v1.jsonl) keeps each walk's labels, divisor, counts and first relation witness, plus each target's ordered `(walk, λ)` keys inserted into elimination. The separate [Python auditor](audit_target_labelled_pencil.py) recomputes all aggregate counters, independently checks the polynomial identity and curve points for the **100 saved first witnesses**, reconstructs the **7,244 inserted rows** from those keys, and reproduces all 20 scalar recoveries and elimination counters. Its [receipt](target_labelled_pencil_audit_v1.json) records the scope. The Jacobian checks on all 12,233 relations, the cofactor subgroup checks and the dual norms remain native producer checks.

The matched [source negation-rho control](native/source_rho_controls.rs) solves the same 20 targets with fresh seeds and a 128,700-step cap. Its [receipt](source_rho_controls_v1.jsonl) records 38–294 walk steps per target (median 129.5), 600–882 elliptic additions (median 709), and 566–848 base-field inversions (median 675), including rho initialization. The original exhaustive gate above is a correctness reference. The following run prices a separate early-stop collector.

### G1 early-stop collector and matched controls

The `--price` mode of the [native gate](native/target_labelled_pencil.rs) reuses a target-independent rational-point table, generates only the three or four walks needed by each target, and feeds each point-driven sieve's root witnesses into online elimination. A degree-three quotient with three distinct witnessed roots needs no full `x=0…p−1` root scan: the gate verifies each witnessed root in the quotient and each resulting relation in the Jacobian. The exhaustive gate independently confirms that **all** complete root sets agree with these witnesses. The early-stop run includes source walk generation and transfer, sieve work, rejected candidates, Jacobian checks, modular elimination, and final source verification. It records field operations inside the existing group backend and direct prime-field and scalar operations in the sieve and matrix code. All 20 targets solve in each of three rounds, using **67 walks, 7,244 verified relations, and 27 rejected candidates per round**. The relation keys and operation counters are deterministic across rounds.

| Diagnostic on this `p=739`, `r=34,147` fixture | Pencil online | Two-point v2 full call | Source negation rho full call |
| --- | ---: | ---: | ---: |
| Median wall time over 60 one-target calls, ms | 6.073 | 2.955 | 0.536 |
| Backend multiplications across 20 targets, one round | 1,714,478 | 1,851,170 | 235,324 |
| Backend inversions across 20 targets, one round | 82,937 | 48,392 | 13,820 |
| Additional direct prime-field multiplications | 903,030 | not separately instrumented | not separately instrumented |
| Additional direct scalar multiplications | 10,066,147 | native scalar counters in receipt | native scalar counters in receipt |

Pencil's reusable setup, which includes constructing the destination, enumerating 739 rational curve points and checking their cofactor projections, took **51.9 ms** in the saved run and is recorded separately. Its median online phases were 0.078 ms walk generation, 0.263 ms sieve, 3.082 ms candidate verification, 2.593 ms elimination, and 0.008 ms recovery check. The [priced pencil receipt](priced_pencil_v1.jsonl), [two-point control receipt](two_point_controls_v1.jsonl), and [three-round paired control receipt](paired_price_controls_v1.jsonl) retain each target's counts. The [priced auditor](audit_priced_pencil.py) confirms the pencil solve witness, phase-sum counters, cross-round determinism, and equality of each paired control's operation counters with its frozen single-run receipt; its [summary](priced_pencil_audit_v1.json) records the timing medians.

A frozen [full-root-scan control](priced_pencil_scan_control_v1.jsonl) runs the same 60 solves, but evaluates each candidate quotient at all 739 field elements. Carrying the sieve's root witnesses into candidate verification reduces direct prime-field multiplications from **22,396,106 to 903,030** across the 20-target round while preserving every recovered scalar, relation count, and elimination counter. The saved wall medians were 10.620 ms for full scans and 6.073 ms for witness buckets; timing variability makes the exact wall delta diagnostic. This is a verified **candidate-verification stage reduction** within the pencil prototype, not an end-to-end advantage over the incumbent.

These wall times are **host diagnostics**, not isolated performance claims. Pencil's clock excludes reusable setup, while the existing two-point API's full-call clock includes destination construction; this does not make the wall rows a primary online comparison under the repository's protocol. The direct modular operations are exposed separately rather than silently folded into the backend `M/S/I` totals. The current prototype loses to both controls in these diagnostics. G1 is a **complete small-fixture solve and cost-accounting prototype**, with isolated matched online intervals and a larger-size cohort still open.

## T1: trace-zero applicability

The [trace-zero index-calculus construction](https://arxiv.org/abs/1405.1059) applies to `E/F_q` and the kernel of its extension trace. This is a different input family from Tian's cubic curve over `F_{523^3}`. The [gate](trace_zero_gate.py) uses the exact Tian coefficients `a=(140,92,82)`, `b=(243,339,359)` in `F_523[w]/(w³+2)` and computes
`j = 1728·4a³/(4a³+27b²) = (413,1,0)`. Its Frobenius image is `(413,462,0)`, so **this specific source has no elliptic model over `F_523`**. The arithmetic identity for `j` is rechecked in [the receipt](trace_zero_gate_result.json). This is a rigorous negative gate for one published fixture, not a rejection of trace-zero index calculus on base-defined curves. The next cohort must be chosen independently and compared with whole-curve index calculus and a matched rho using available Frobenius symmetries. T1 is **not attempted beyond eligibility**.

## Reproduce and limits

Run from the repository root:

```sh
python3 research/more_leads_20261019/quartic_line_gate.py > /tmp/line_gate_results.jsonl
python3 research/more_leads_20261019/genus2_pencil_gate.py > /tmp/pencil_gate_result.json
python3 research/more_leads_20261019/trace_zero_gate.py > /tmp/trace_zero_gate_result.json
cargo run --release --offline --manifest-path research/more_leads_20261019/native/Cargo.toml --bin target-labelled-pencil > /tmp/target_labelled_pencil_v1.jsonl
cargo run --release --offline --manifest-path research/more_leads_20261019/native/Cargo.toml --bin source-rho-controls > /tmp/source_rho_controls_v1.jsonl
cargo run --release --offline --manifest-path research/more_leads_20261019/native/Cargo.toml --bin two-point-controls > /tmp/two_point_controls_v1.jsonl
cargo run --release --offline --manifest-path research/more_leads_20261019/native/Cargo.toml --bin target-labelled-pencil -- --price > /tmp/priced_pencil_v1.jsonl
cargo run --release --offline --manifest-path research/more_leads_20261019/native/Cargo.toml --bin target-labelled-pencil -- --price-scan > /tmp/priced_pencil_scan_control_v1.jsonl
cargo run --release --offline --manifest-path research/more_leads_20261019/native/Cargo.toml --bin paired-price-controls > /tmp/paired_price_controls_v1.jsonl
python3 research/more_leads_20261019/audit_target_labelled_pencil.py > /tmp/target_labelled_pencil_audit_v1.json
python3 research/more_leads_20261019/audit_priced_pencil.py > /tmp/priced_pencil_audit_v1.json
cmp /tmp/line_gate_results.jsonl research/more_leads_20261019/line_gate_results.jsonl
cmp /tmp/pencil_gate_result.json research/more_leads_20261019/pencil_gate_result.json
cmp /tmp/trace_zero_gate_result.json research/more_leads_20261019/trace_zero_gate_result.json
cmp /tmp/target_labelled_pencil_v1.jsonl research/more_leads_20261019/target_labelled_pencil_v1.jsonl
cmp /tmp/source_rho_controls_v1.jsonl research/more_leads_20261019/source_rho_controls_v1.jsonl
cmp /tmp/two_point_controls_v1.jsonl research/more_leads_20261019/two_point_controls_v1.jsonl
cmp /tmp/priced_pencil_audit_v1.json research/more_leads_20261019/priced_pencil_audit_v1.json
cmp /tmp/target_labelled_pencil_audit_v1.json research/more_leads_20261019/target_labelled_pencil_audit_v1.json
```

The Python scripts use the standard library; the native binaries compile the repository's `three_leads.rs` source through their own Cargo manifest. All inputs are public published or synthetic equations and points. Quartic coefficients match [`tian_quartic_smoothness.rs`](../../src/bin/tian_quartic_smoothness.rs); the Scholten quintic matches the [transfer certificate](../three_leads_e2e_20261018/TRANSFER.md). The exhaustive native gate checks every Jacobian identity and one dual norm per walk; the separate Python auditor checks the retained first witness per walk, all inserted elimination rows and all reported totals. No quartic map, quartic Jacobian relation matrix, trace-zero decomposition solver, or isolated matched online comparison was produced. Those gaps remain acceptance gates before any IC or rho advantage claim.
