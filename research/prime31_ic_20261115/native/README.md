# Native algebraic IC continuation

Standing: **producer-local development checks, [pcc:unverified]**. Claim record:
`PCC-20261115-d73efd`. This is an implementation and prerequisite audit, with
the requested full prime31 cost experiment still blocked. No speedup or novelty
claim is made. This extends [crypto #4188](https://github.com/isogeny-labs/crypto/pull/4188).

## What is implemented

The focused Cargo crate links the **actual existing Rust source** for
`src/cryptanalysis/f4_fp_tower.rs` and `sig_fp_tower.rs` by `#[path]`.
It does not copy them, replace them with Python, or depend on the C++ pair table.
The source revision used for the initial development tests was
`c6b1819d157173ca3e19f6aeed523a77b40d9381`; the new adapter source and lockfile
are in this PR. The linked modules' own tests run in the focused crate too.

The implemented pipeline is a **two-summand algebraic PKM/Dickson baseline**:

1. Check a prime-order full-group certificate on the supplied curve and generator.
2. Build a fully split fibre of the composed map `x -> x^2-2`. Keep actual EC
   points and one canonical ordinate per abscissa; signs remain in the relations.
3. For a public point `R`, put **both** `S3(x1,x2,x(R))=0` and the factor-base
   membership constraints into the existing quadratic quotient-ring F4 engine.
4. Extract rational points of the resulting basis, using linear consequences
   when available and actual square-root branching otherwise. Record every
   extraction node; no scan over the entire prime field, pair table or rho
   fallback exists. Extraction can still branch exponentially and is charged.
5. Lift ordinates and replay every signed relation in the EC group. Reject
   nonliftable algebraic roots. Handle identity/inverse-pair exceptions explicitly.
6. Generate natural `R=[a]G+[b]Q` relation queries, eliminate modulo the prime
   group order, and return a scalar only after `[k]G=Q`.

The solver takes **public target coordinates**, not a target scalar. Its call
cap and F4 degree/time/memory bounds can censor an attempt. `undetermined_f4`
and `undetermined_extraction` are separate from a completed no-relation call.
The CLI exits 2 on a censored run and never substitutes a generic DLP algorithm.
This is a bounded implementation, not a guaranteed completion within its caps.

## Exact requested prime31 fixture

Reuse the ordinary cofactor-one fixture from the existing study:

| quantity | exact value |
|---|---|
| field prime | 2147483647 = 2^31-1 |
| curve | y^2 = x^3 + 847514846 x + 2087444724 |
| full prime group order | 2147446831 |
| generator | (625037715, 2100627251) |
| target interface | affine public coordinates, checked on the curve |
| identity | same exact model/generator as `../curve_certificate.py`; no invented registry slug |

The native certificate checks primality, nonsingularity, nonzero generator,
`[n]G=O` and `2n` above the Hasse upper bound. The historical certificate also
screens embedding degrees through 1000; the adapter does not silently count that
historical screen as new native evidence. It does not use a smooth EC group order,
an anomalous curve, or a supersingular source.

## Development checks and their limits

`development-tests.txt` records the final focused release test run;
`development-provenance.json` binds source, lockfile, output and binary hashes.
Its 40 tests
include 37 inherited F4/signature-engine tests and three adapter tests:

- Prime31 certificate and positive two-summand queries, including diagonal
  decompositions, compared with a small exhaustive **test-only** relation oracle.
- Four independent fixed prime31 public multiples, identity handling, an invalid
  generator, invalid depth, and an extraction-limit control. Positive and negative
  relation sets must equal the exhaustive test control.
- A complete algebraic IC solve for one public target on the historical ordinary
  `p=1031, a=447, b=997, n=991` toy fixture, plus a prime31 zero-call censorship test.

The toy known scalar is constructed outside `solve` and used only to assert the
returned answer. The toy control does not discharge a complete prime31 solve.
The four prime31 public oracle calls are not full ECDLP recoveries or a yield
estimate. The exhaustive test helper is absent from the production oracle path.
No independently signed PCC, Lean proof, cross-host replay, benchmark interval,
strongest-rho comparison, external F4 comparison or exponent fit is claimed.

Development reproduction (unit tests are not a benchmark):

```sh
cargo test --locked --release --lib --manifest-path research/prime31_ic_20261115/native/Cargo.toml
cargo clippy --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --lib --bin prime31-algebraic-ic -- -D warnings
```

The full repository release-lib command was also attempted. After materializing
the named missing modules, Cargo path dependencies and compile-time fixtures,
the complete-input `rustc` process was terminated by **signal 9 (SIGKILL)**.
`workspace-resource-stop.txt` retains the output. The kill cause is unconfirmed;
no memory diagnosis or full-workspace test pass is asserted. Claim
`PCC-20261115-3d5e1d` is unverified. The focused 40-test run is a separate check.
Earlier sparse-input failures are retained in `workspace-test-attempt*.txt`.
[pcc:unverified]

## Accounting and execution status

The adapter emits setup and total arithmetic counters, F4 elimination muladds,
extraction nodes, calls, completed misses, undetermined calls, accepted rows and
rank. **These counters are incomplete:** symbolic preprocessing, allocation,
hashing, ring normalisation, basis scans and some field work inside the shared
engine remain unpriced. Setup counters are a subset of total counters, not an
additional charge. Inversion counts accompany their exponentiation multiplications
and must not be double-priced. No `S`, full GAE cost or cross-method ranking is emitted.

The requested full prime31 run was **not launched**. This runtime lacks EKS tools,
an authenticated AWS client, GitHub CLI authentication and a connector operation
for workflow dispatch. The checkpoint gate also fails: there is no available
S3 upload transport. Installing Rust enabled local development tests; it did not
establish cloud execution or budget authority. No paid resources were launched.

`EXPERIMENTS.md` freezes the next discriminating work, pending execution admission.
Do not turn a development CLI invocation into a benchmark by changing its label.
Before a research run, complete and upload the resume checkpoint, publish the
exact source/protocol, check live budget/capacity, and extend ecbench accounting.

## Previous leads checked and what changes the next action

| source / lead | observed status in the cited source | consequence |
|---|---|---|
| #4188, `../BATCHED_INDEX_CALCULUS_FOLLOWUP.md` | C++ complete prime31 IC with signed pair tables; nine producer-local complete recoveries; slower than its BSGS control | Preserve as generic-decomposition control; do not rebrand it as an algebraic oracle |
| `research/prime_leads_chase_20261113/CONTINUATION_PROTOCOL.md`, #4077 | Whole-call sparse-PKM gate, m=8; optimistic gamma<3.5 for full useful output, <3 for one output | Require useful independent relation rank and all failed calls; degree alone is insufficient |
| `research/prime_ic_lead_heatmap_20261114/TABLES.md` | Bounded negative/cost results for small-root, residual-walk, digit and transfer presentations | No repeat of the same presentation without a changed mechanism |
| `research/pkm_dickson_torus_20261015/README.md` | Nonsplit torus already implemented and tested; solver-degree data match split controls in its measured cells | Reuse construction; no novelty or automatic degree benefit from p+1 smoothness |
| Rust `f4_fp_tower.rs` | `MAX_FREE=4`; a chained m-summand presentation uses m-2 free abscissae | This specific packed engine cannot encode m=8 directly; extend/audit representation or use the existing generic exponent-vector F4 engine |
| #4188 norm-rank follow-ups | k=8 rank-four examples and signed-support/cold-cost controls already exist | A determinant corank is not yet a target-positive cheaper solver; test usable witnesses and selector cost |
| #4228 direct paired-S3 cubic construction | PR reports certificates and degree diagnostics, not full cold speed | Test its certified preprocessing inside a complete natural-positive oracle, charging construction and extraction |
| #3877 chained transfers and #4015 cross-characteristic work | Complete cheaper source-to-destination solving route not supplied by the material read | Require scalar-blind public maps and complete destination cost; do not use transfer existence as measured improvement |

KB unavailable: no loaded `kb_status`/`search_knowledge`, wrapper exits 127.
Fallback used GitHub cross-repository code/PR search, the heat map and primary
files above. This is a bounded prior-work review, not proof that all leads have
been searched. No Lean-backed claim is introduced.

Primary literature read: [Petit-Kosters-Messeng, PKC 2016](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
Algorithm 1 and Sections 3.1-3.4; [Diem 2011](https://www.math.uni-leipzig.de/~diem/preprints/ind-cal-ell-curves.pdf),
Sections 1-2. PKM's total cost explicitly depends on the unestablished polynomial
solver cost; Diem's extension-field regime does not establish this prime-field
comparison. Direct full-text requests to Semaev 2004/031 and APS 2017/609 were
HTTP 403 in the browsing tool; their contents are not claimed to have been read.

## Completion against this request

| requirement | state | evidence or remaining dependency |
|---|---|---|
| Reuse Rust | implemented; development-tested, unverified | Actual F4/signature sources linked by path; focused release suite |
| Inspect previous leads | partial bounded review | Source table above; KB unavailable; no exhaustive novelty claim |
| Algebraic IC path | implemented at m=2, unverified | Membership and summation equations solved together; native signed extraction |
| Full prime31 ECDLP comparison | blocked and unperformed | Execution/checkpoint admission plus complete ecbench accounting and matched rho |
| Establish cheaper complete solver | unresolved | No measured cost advantage or full prime31 scalar recovery in this continuation |
| New experiments | specified, unrun | EXP-R31-A/B/C in EXPERIMENTS.md |
| GitHub check-in | this existing PR | Code, lockfile, development log, report, experiment specification and PCC record |

Diagram: [pipeline.svg](pipeline.svg). Printable report: [REPORT.pdf](REPORT.pdf).
No leaderboard, scoreboard, timeline or browser measurement row changes: this
continuation adds no eligible cost measurement or registry change. [pcc:unverified]


## New exact prime31 two-summand yield preflight (2026-11-15) [pcc:unverified]

[Mathematical gate, prior-lead assessment and proposed experiments](TWO_SUM_YIELD_GATE.md) · [Rust-native executable replay](examples/prime31_two_sum_gate.rs). This deterministic development preflight imports this crate's actual `Base`, `Arithmetic`, and `PRIME31`; it **does not call F4** and must not be cited as a solver performance measurement. An independent producer-local modular arithmetic implementation counted actual curve-liftable base sizes **B=135,256,509** at Dickson depths 8,9,10 and exact distinct signed two-point target supports **36,451; 131,067; 518,085**. The Rust executable asserts these expected results; exact-head GitHub Actions replay is pending. A uniform fixed-base query has exact positive probability support/n, and a restricted full-(B+1)-rank collector has an idealized calls floor in the report; this is not a lower bound on early scalar-solving cycles or target-adaptive algebraic methods.

The cross-repository survey also found [#4275](https://github.com/isogeny-labs/crypto/pull/4275): on its measured planted-positive systems, dense-membership polynomial extraction outperforms the prior tower solver while enumeration remains less expensive. Prioritize a **prospective natural-positive dense-versus-tower Rust A/B** (same public inputs, failed calls, useful rank, complete costs) before expensive m8 extension. This remains a proposed test, not a frozen protocol or a demonstrated cold prime31 advantage.


## 2026-11-15 natural-positive m=8 4+4 control

[Detailed report and cost boundary](M8_SPLIT_CONTROL.md) · [Rust-native source](examples/prime31_m8_split_ic.rs) · [frozen public relation transcript](M8_SPLIT_CONTROL.results.json) · [independently authored exact Python replay](replay_m8_split_control.py).

This new m=8 control reuses this crate's actual `Arithmetic`, `Base::build`, curve certificate, and existing ordinary cofactor-one prime-order p=2^31−1 fixture. It obtains 32 independent affine rows in 32 unknowns from 124 natural masked targets, then recovers all basis logs and the public Q scalar. The underlying 4+4 witness finder is still **generic group meet-in-the-middle**, not an F4 sparse-PKM attack: it does not establish the required non-generic speedup. Producer-only arithmetic observations total 3,827,503 inclusive group additions, with missing field/linear-algebra charges. Current GitHub-head Rust compilation and independent PCC replay are still pending. [pcc:unverified]

The next implementation step is to replace the generic 4+4 hash join with the existing Rust dense/tower polynomial oracles (notably `src/cryptanalysis/pkm_smooth_tower.rs`) on the same unplanted natural targets, preserving per-call failed/censored outcomes, projective-unique relation rank, complete costs and optimized-rho control. The current focused F4 tower's `MAX_FREE=4` cannot directly represent all six chained m=8 free coordinates; do not treat the new generic control as evidence that this prerequisite is solved.

## 2026-11-15 balanced m8 solution-orbit audit [pcc:unverified]

[Exact ordered-root/permutation result and symmetry-quotient experiment](M8_ORDERED_ROOT_SYMMETRY_GATE.md): on the registered ordinary prime31 curve the first two distinct-index m8 witnesses for the same public target induce **at least 80,640 distinct labeled S4/S3 roots**, grouped into 560 orbits under the manifest 144-element block relabeling symmetry. This is producer-local mathematics/replay, not new F4 witness extraction, independent PCC validation, or a cheaper ECDLP solver. Reproduce using the *existing Rust* Arithmetic and Base with `cargo +1.100.0 run --locked --release --manifest-path research/prime31_ic_20261115/native/Cargo.toml --example prime31_m8_permutation_orbit_gate`; the development workflow now contains that step. Treat symmetry reduction as a prospective staged F4 experiment against the frozen natural-positive/negative masks and complete cold-cost gate.
