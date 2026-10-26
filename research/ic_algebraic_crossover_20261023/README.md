# Algebraic decomposition and quartic IC pursuit

Owner-requested campaign, begun 2026-10-23; evidence snapshot 2026-10-26.
**The campaign is partial.** The finite results below do not establish a
bounded degree law, global guess minimum, complete-price IC improvement or
measured rho crossover. [PROTOCOL.md](PROTOCOL.md) preserves the original
scope, amendments, limits and deviations.

## Additional qualified query and wide-core controls

The [qualified-query report](reports/run-qualified-queries-20261025/report.pdf)
adds all 2,048 draws at (m,n,ell)=(3,22,8),(3,25,9),(4,17,6),(4,18,6),
four families and 128 draws each. Explicit a=0, full-degree-b curves have
prime subgroups with cofactor 4,16,8,12. Public hash/lift/cofactor-clear
targets are paired across families. All finite-chain counts replay; all
ordered signed point counts, bases and witnesses agree with independent
BinaryCurve arithmetic. There are 1,591 Boolean-SAT queries and 161 with
actual subgroup decompositions, all within the Boolean-SAT class. These
different contracts are retained; no population or full-price claim follows.
[Native summary](pipeline-receipts/qualified-queries-20261025/summary.json).

The predeclared selection yields 55 negative and 24 positive controls,
retaining nine/eight quota shortages. All 158 zero-guess degree-two/three
attempts reach nonrefuting fixed points; all ranks/verdicts agree with the
separate HKY engine. [Report](reports/run-qualified-closures-20261025/report.pdf),
[native accounting](pipeline-receipts/qualified-closure-replay-20261025/summary.json),
[archive reproduction](verify-qualified.sh). Independent cost recount,
degree four, complete guess minima and full query prices remain open.

The [wide-core report](reports/run-wide-core-audit-20261025/report.pdf)
checks standalone m=16/17/18 arithmetic, exact group orders, high scalar and
coordinate bits, serialization and independent replays at extension degrees
64/68/72. The completed wide-core wave passed 4,634 library tests, zero
failures, 119 ignored, on its frozen source. The narrow-constructor
port described below separately passed 4,635 library tests, zero failures,
119 ignored, on its recorded source. Later merged-source checks remain separate. This is
not full wider ecbench/IC support. [Protocol](WIDE_GROUP_AUDIT_PROTOCOL.md),
[reproduction](verify-wide-core.sh), [remaining integration](../../problems/quartic-binary-ic-wide-scalars.md).

The [constructor profile](reports/run-wide-constructor-profiles-20261025/report.pdf)
records 9,130,856 versus 575,951 simulated whole-process instructions, with
identical returned field. A/A, repeated A/B, external fastest-tool comparison,
native runtime and complete-IC pricing remain missing. The
[proof map](lean/README.md) has six checked Lean algebraic dependencies;
the finite-field specialization/irreducibility bridge is not formalized.
Identity IDC1h0e27b483c3989bcd certifies only the integer expansion.

The separate [narrow-constructor comparison](reports/run-narrow-constructor-20261026/report.pdf)
records 8,774,220 versus 570,927 simulated whole-process Ir with identical
stdout and seven identical even/odd field/product controls. Source c754e7111
passes 4,635 library tests, its affected example tests and scoped clippy.
This remains instruction attribution with missing native A/A/A-B/external
performance qualification. [Native audit](pipeline-receipts/publication-followup-audit-v3-20261026/output.json),
[reproduction](verify-publication-followup.sh), [protocol](NARROW_CONSTRUCTOR_EXCLUSION_PROTOCOL.md).

The [interrupted quartic report](reports/run-partial-quartics-20261026/report.pdf)
preserves m14's 17/24 records (11 successful rho answers, five IC timeouts,
one interrupted/crashed IC attempt) and m15's 15/24 records (ten successful
rho answers, five IC timeouts). Seven/nine registered executions are missing.
The m14 original audit reproduces all 11 successful records but fails the
complete-session gate. A new BinaryCurve audit verifies all 11/ten scalars
against their exact recorded generators and targets. Legacy coordinates are
packed tower elements and require the explicit polynomial-basis conversion;
two initial verifier failures are preserved. Both factories' generators agree
after conversion on these two checked models. No complete IC cost or IC/rho
ratio is available. [Small complete raw archive](evidence/runs-partial-quartics-complete-20261026.tar.xz),
[independent partial replay](verify-publication-followup.sh), [audit protocol](FOLLOWUP_PUBLICATION_AUDIT_PROTOCOL.md).

The [versioned curve-identity adapter](curve-identity-adapter.v1.json)
binds all 18 reference/twist, four qualified-query and three wide-core models
to canonical global aliases/full UIDs using the repository's metadata-only
reference. Original EC1-shaped labels use producer-specific preimages and
remain source/provisional labels; they must not be silently used as canonical
global aliases. Exact records, hash-to-target inputs and measurements are
unchanged. [Metadata reproduction](adapt-curve-identities.py).

The original 116-attempt balanced campaign ended at 17:53:53 UTC on
2026-10-25. With eight earlier draw-0/k3 branches, all 124 attempts are
accounted for: 78 refute, 45 saturate without refuting and one times out.
The m14/m15 continuation and dependent HKY queue stopped across the
workspace restart; no research runner is currently live. The independent
HKY recovery is prepared as two real124-case arms (248fresh processes); its
auditor6tests and20otherexampletargets35tests pass. No fresh case has run yet. Original L2 full-panel specs and larger
host shards remain unchanged and unexecuted. The USD 100 extra-cloud ceiling
includes replay; no additional VM has launched and workspace billing is
unmeasured.

The owner approved publication on 2026-10-26, and the results summary is now
on [private PR 2550](https://github.com/isogeny-labs/crypto/pull/2550).
The repository moved with the same ID. Remote head `ac16317e5` and main
`942caf7e4` are merged locally as `69cf0253e`; all 65 inventoried evidence
files remain byte-identical. The owner explicitly authorized S3 delivery on
2026-10-26. The standard artifact tool published all 41 additional files
(7,508,437 bytes) as 30 content objects in 32 genuine manifests; every file's
actual remote bytes pass SHA-256 and size readback, with previous bindings
preserved. The official resume checkpoint also passes actual archive/meta/resume
readback. [Delivery receipt](pipeline-receipts/artifact-publication-20261026/remote-byte-replay.json).
No Git storage exception or CI relay was used. Source integration and final-head
validation/push are still in progress. The earlier integrated library gate fails
on [unchanged upstream SAT syntax](../../problems/sat-progress-phase-base-syntax.md);
main `e8fcadf62` is incorporated in `d3c9f02c7` and fixes that syntax. The full
release command reached its 2,700-second watchdog with 4,869 named passes,
121 ignores and one unfinished unchanged JV test; it is not a full-suite pass.
[Runtime record](../../problems/jv-nonweak-end-to-end-library-watchdog.md).
Earlier test passes belong to their recorded frozen sources. There are 31 inspected reports,
34 pages. The [v2 metadata adapter](curve-identity-adapter.v2.json) preserves
all 25 v1 identities and adds the two independently checked partial models.
[Initial publication snapshot](PR_UPDATE_20261026.md),
[updated published summary](PR_UPDATE_FINAL_20261026.md),
[publication packet](PUBLISH_REVIEW.md),
[continuation gates](CONTINUATION_READINESS_20261026.md).

Hydrate the stored study/report bytes with
`python3 tools/artifacts/artifacts.py pull research/ic_algebraic_crossover_20261023 benchmarks/20261025-ic-algebraic-stage`.
Use a writable `ARTIFACTS_CACHE` when the default home cache is read-only.
The completed pending plans are historical inventories, not replacements for
the real `.artifacts.json` bindings.

## Measured structural result

The native symmetric formulation has fewer recorded solver word operations
in 263 of 304 completed paired queries at closure degree 2. At closure
degrees 3 and 4 it has more in every completed pair. Smaller product spaces
alone did not yield a uniform gain when stacked with extra closure.

| Closure cap | Raw rows | Complete extractions, all forms | Completed direct/symmetric pairs | Symmetric fewer words | Symmetric/direct word ratio |
|---|---:|---:|---:|---:|---:|
| 2 | 1,152 | 768 | 304 | 263 | 0.04200896--5.12320089 |
| 3 | 1,152 | 891 | 304 | 0 | 1.34249804--178.51670155 |
| 4 | 1,152 | 923 | 304 | 0 | 13.37332864--16,500.11984829 |

![Recorded solver work on the same small queries](reports/local-structural-20261025/closure-ratios.svg)

Fields n=13,19,23,31, dimensions ell=2,3,4; prefix, geometric,
trace-zero geometric and random bases; shared public subgroup targets.
Duplicate prefix replicates remain in raw records and are excluded from
the primary comparison. Each panel has 336 unique queries; 32 unpaired
or incomplete cases remain unknown. The direct hybrid solver raises its
effective extraction degree to the input degree, up to six: these are
**closure-cap ablations, not equal-solving-degree comparisons**.

Independent direct point enumeration found zero mismatches. Across the
304 completed symmetric queries in each panel, 8,100 algebraic roots
produced four verified x-multisets; 8,096 roots failed splitting or signed
lifting. Construction, splitting and physical conversion are not priced
by these solver-word ratios. Shared-host wall times earn L0 and are diagnostic.

The [results PDF](reports/local-structural-20261025/report.pdf) accompanies
the [degree-2](reports/run-structural-d2-20261023/report.pdf),
[degree-3](reports/run-structural-d3-20261023/report.pdf) and
[degree-4](reports/run-structural-d4-20261023/report.pdf) run reports.

## Guessing and exact controls

All eight assignments of the balanced three-set `[7,15,23]` on original
ell=8 draw 0 saturate degree four without refuting. Branches 0--3 have
rank 129,221; branches 4--7 have rank 129,473; all have 136,698 columns.
The recorded total charges every branch: **4,738,566,310,210 solver word
operations and 5,982,980 closure products**. This rejects this set on this
draw; it is not a minimum over all variable subsets. [Run report](reports/run-guess-k3-draw0-20261023/report.pdf).

The upstream PR already reports branch-0 failure on draws 0,3,5 and
all-eight refutation on draw 6. Our draw-0 replay agrees and fills the
other seven branches; it does not claim priority. The original four-set's
all-64-branch result is still upstream, unmerged evidence, not independently
replayed here. Our 28 k=0/1/2 attempts hit a column cap and remain inconclusive.
Eight earlier three-guess attempts hit the library's default internal cap;
they are retained and excluded from the successful process counts.

A [separate curve-membership audit](reports/run-fixture-group-audit-20261025/report.pdf)
checks every y root by substitution and an independent trace criterion.
Original UNSAT draw 5 and SAT draw 4 are not liftable on this campaign's E0
reference curve; draw 0 is liftable. These x-only fixtures do not specify
the full curve coefficient a. S3 at b=1 is independent of a, so nonliftability
on E0 does not imply invalidity on every such curve or twist. This first audit leaves
target/base subgroup membership unknown. The [native receipt](pipeline-receipts/fixture-group-audit-20261025/receipt.json)
retains both positive and negative outcomes.

The [fresh-target audit](reports/run-fresh-target-relevance-20261025/report.pdf)
extends this reference-model check to all 1,152 frozen targets. SAT draws
split into 154 E0-liftable and 176 nonliftable; UNSAT draws into 434 and 388.
Every y-root satisfies the curve equation and an independent trace criterion
agrees for every target; a native rerun is byte-identical. These remain
uniform field-x Boolean fixtures, not established ordinary subgroup queries.
Their exact inputs, all outcomes and the first audit's unknown subgroup membership are retained
in the [native receipt](pipeline-receipts/fresh-target-relevance-20261025/receipt.json).

The [declared-subgroup follow-up](reports/run-fixture-subgroups-enumerated-20261025/report.pdf)
checks all 1,159 original/fresh target x-values on E0 and an explicitly chosen
trace-one-a twist in each exact field. It binds each model, largest odd prime
r, full order/factorization and a verified G with an unregistered EC1 record
identity. **105 targets lie in the declared cyclic subgroups and 1,054 lie
outside**, with no remaining unknowns for these models; 69 of the members
are Boolean SAT and 36 UNSAT. This is model-specific membership, not a
replacement for the original uniform field-x law or a verified relation.

For the original n=22 inputs, E0 has r=991 and cofactor 4232; the chosen
twist has r=2,097,349 and cofactor 2. All seven E0 factor bases have zero
abscissae supporting r-subgroup points. Original draw 6 has zero on both
models, and its target is outside E0's r-subgroup. Each of the three selected
SAT witnesses gives two signed curve relations on its matching model, but
zero relations whose summands all belong to the declared subgroup. Other
Boolean witnesses and other subgroups are not exhausted by this diagnostic.
These are strict original-point membership facts, not a universal
obstruction to IC. A separately frozen [cofactor audit](reports/run-cofactor-projection-20261025/report.pdf)
checks the projected alternative: every original base has nonzero projected
points hP on both models. One canonical target sign for each of the three
selected SAT witnesses gives three valid relations at nonzero hR, despite
zero relations with all original summands in G. Draw 6 has 254 distinct
nonzero E0 projected points and a nonzero projected target that is not a
direct base hit. Projected x-coordinates need not remain in V; a projected
method would retain original preimages for its oracle. The original fixtures
still supply no complete target/preimage/cofactor-component law, factor-base
logs, scalar recovery or full cost. The strict-G contract of the separate
1,591/161 panel remains distinct. [Native receipt](pipeline-receipts/cofactor-projection-20261025/receipt.json),
[protocol](COFACTOR_PROJECTION_PROTOCOL.md), [reproduction](verify-completed-guess.sh).
The prior subgroup PDFs and receipts remain unchanged.

The [first subgroup receipt](pipeline-receipts/fixture-subgroups-20261025/receipt.json)
conservatively retained 152 unknowns where gcd(r,cofactor)>1. A declared
[amendment](SUBGROUP_RELEVANCE_AMENDMENT.md) resolves those by enumerating
the actual 11-point cyclic subgroup, with an r-torsion-outside-G control.
Both versions independently trace-check lifts, annihilate points by the full
order, replay byte-identically and retain all original inputs. The
[amended native receipt](pipeline-receipts/fixture-subgroups-enumerated-20261025/receipt.json)
and [verifier](verify-relevance.sh) preserve that distinction.

The [200,000-column continuation](GUESS_CAP200K_PROTOCOL.md) has ended.
The [completed report](reports/run-completed-balanced-guess-20261025/report.pdf)
charges every one of its 116 new attempts and the eight earlier k3 branches.
For prefixes of [7,15,23,24] the first all-branch refuting k is **4,4,4,0**
on draws 0,3,5,6. Every smaller prefix on draws 0,3,5 has a nonrefuting
branch; draw 6 already refutes without guesses. All 64 k4 branches refute,
charging **23,078,681,663,212 solver-word operations** in total. The passing
sets cost 6,542,971,204,156 / 6,266,204,429,585 / 6,095,932,567,609 /
1,548,633,051,630 words respectively. Including all failed smaller prefixes
through success gives 21,448,879,433,676 / 21,064,467,554,811 /
22,191,811,097,962 / 1,548,633,051,630 words. The one timeout is draw 6,
k1, branch 1; its complete cost remains null, so the full 124-attempt cost
is unknown. It does not affect the zero-guess minimum on that draw.

These are minima within this registered balanced-prefix family at degree
four, not over arbitrary variable subsets or orderings. Native raw accounting
and the prefix summary reproduce byte-identically from the complete archive;
independent HKY verdict replay has not run; its queue was interrupted by the workspace restart.
Independent word recount, construction, splitting/lifting, verification and
physical conversion remain open. [Summary](pipeline-receipts/guess-completion-20261025-01/summary.json),
[reproduction](verify-completed-guess.sh), [replay protocol](COMPLETED_GUESS_REPLAY_PROTOCOL.md).
The earlier 140k cap refusals and original draw-6 pilot remain preserved.

Exact independent fixture counting reproduces `0,6,6,0,18,0,0`, with
checked Boolean witnesses for all three SAT controls. Small controls
check 2,304 assignments, 373 corresponding symmetric roots and 2,880 signed
lifts, including repeated roots. The campaign-local u128 chain builder
matches the library's equations for m=3--6; sparse and parallel fixed points
match dense controls. These are finite correctness checks.

First-two-summand exchange is checked on all 308 coordinate equations
of the original fixtures. [IDC1ha27a77e72057f9cb](proofs/s3-swap.identity.json)
and its [receipt](proofs/guess-swap.receipt.json) relate degree verdicts for
`[7,23,24]` and `[15,23,24]`, not their costs; no formal-proof pointer is set.

## Complete DLP and quartic status

[PIPELINE.md](PIPELINE.md) documents the native ecbench adapters and their
accounting limits. The n=13 smoke verified all ten arms/targets and replayed
all ten with equal deterministic counters. The n=23 session verified nine
records and timed out once; its independent replay is separate. The n=7
session exhausted all ten attempts, including rho. K0 n=17 was refused by
the constructor. These failures remain evidence, not zero-cost solves.

An [exhaustive n=7 support audit](pipeline-receipts/structured-n7-support.json)
checks all 64 ordered signed-base triples: none sum into its prime subgroup.
All 29 subgroup points produce only three canonical representatives, none
passing the configured four-bit distinguished-point predicate. These exact
finite support failures explain the two exhausted methods; they do not
generalize to larger fields or retune the frozen session.

The [corrected n=13 report](reports/run-structured-n13-20261025-01-pricing-correction/report.pdf) and
[corrected n=23 report](reports/run-structured-n23-20261025-01-pricing-correction/report.pdf) quote the
native cost bands. All nine verified n=23 records independently replayed;
the incomplete direct arm is excluded from the complete-arm plot. The
[setup-refusal report](reports/run-pipeline-setup-refusals-20261025/report.pdf)
preserves the n=17 refusal and the later sparse-input setup failures.

**Pricing correction:** our earlier prose incorrectly called these S
values wall-priced. Completed n=13/n=23 solvers use fixed pinned word
rates; the measured n=7 solver term is removed. The
[native pricing audit](pipeline-receipts/pricing-audit-20261025/receipt.json)
rebuilds the available IC charges from counts and fixed prices. Raw S
remains an incomplete fixed-price estimate. The native band at n=13 gives mean S for
direct/symmetric/pair-table/strong-rho of
`19.573/19.473/54.285/14.330` at weight 0,
`7290.842/2883.805/58.245/14.552` at 0.1, and
`72732.257/28662.791/93.889/16.553` at 1.
Uncounted construction and split arithmetic remain missing at every weight.
There is no robust complete-price gain over the existing references.

The [pricing-audit PDF](reports/run-pricing-audit-20261025/report.pdf)
documents the native reconstruction and its byte-identical reproduction.
The [quartic smoke](reports/run-quartic-smoke-20261025-02/report.pdf) has
24/24 verified records at m=6,8,10,12 and 24/24 identical independent
deterministic replays, with zero audit problems. Canonical `ecbench band`
refuses its constructed curve slugs because the virtual calibration rows
are absent from the global table. The audit reproduces those deterministic
count conventions without changing the table and separately labels an
additional scenario that varies counted Fq solver work. Neither scenario
is hardware calibration or an algorithmic gain.

Ten [n13 instruction profiles](reports/run-instruction-profiles-n13-20261025-01/report.pdf)
reproduce all recovered scalars and integer native counters. The two-target
whole-solve Callgrind counts are about 0.38--0.39 million for strong rho,
0.97--3.44 billion for direct S4, 0.52--1.89 billion for symmetric S4 and
28.6--29.8 million for generic MITM. Both rho repeats have identical counts.
These are implementation instructions under Valgrind's allocator; real
allocator work and physical throughput are absent. IC online intervals lack
markers and remain null. Four rho rows have one-ULP serialized price
differences under the explicitly recorded tolerance. This diagnostic does
not repair complete native-price accounting or establish a runtime speedup.

The [N=58/60/100 probes](reports/run-hky-probes-20261025/report.pdf) all
saturate without refuting at degrees two and three. Dense N=58/60 allocate
unsuccessfully at degree four under the local limit; sparse N=100 reaches
25 million entries after admitting an implicit 4,087,976-column universe.
All three degree-four verdicts remain unknown. These are fixed Boolean
fixtures over n=10 or 15, not curves with 58--100-bit subgroup security.

Quartic IC remains separate over GF((2^m)^4), requiring its proper
intermediate field. The historical single-target m=14 gap near 4.3 times
measured rho and crossover extrapolation near subgroup size 2^68 remain
historical, distinct claims. The registered native ecbench path still refuses
m=16--18. A separate u32/u128 arithmetic core now passes independent field,
group-order, scalar and encoding controls; full wide stage integration and
the registered complete ladder remain outstanding.
[Width problem](../../problems/quartic-binary-ic-wide-scalars.md).

## Same-input external correctness

The [Singular correctness report](reports/run-external-cas-20261025-01/report.pdf)
covers 22 frozen equation systems, including two constructed positive
controls. All 44 `std`/`slimgb` commands finish. Native reconstruction and
complete Boolean enumeration agree with every external zero set and every
complete native extraction. All original generators reduce to zero against
the returned bases. The strengthened independent validator also checks the
exact exported ideals and reproduces the original receipt byte for byte.

This is a correctness baseline at L0, with explicit Boolean field equations,
single-threaded 60-second/1-GiB external limits. A complete basis computation
and root extraction have different output contracts, so no speed ratio is
claimed. Magma, msolve, FGb, fastest-tool and hardware comparisons remain
open. [Protocol](EXTERNAL_CAS_PROTOCOL.md),
[receipt](pipeline-receipts/external-cas-strengthened-20261025/replay.json),
[reproduction](verify-external-cas.sh); the separate hash-checked raw archive
preserves all generated scripts, package hashes, outputs and statuses.

## Requirement status

| Requirement | Status | Evidence or remaining gap |
|---|---|---|
| A1: independent structural replay | partial | exact original fixtures and small equations/roots checked; upstream large fall-degree panel not fully replayed |
| A2: native structured bases and symmetric formulation | verified, finite pilot | Rust implementations, all four families, identity/splitting/repeated-root controls; limits in PIPELINE.md |
| A3: combine formulations and closure | verified, finite stage panel | three matched panels above; no whole-query performance conclusion |
| B1: ell=8 minimum guesses | partial | all 124 registered attempts terminated; balanced-prefix minima 4/4/4/0; one retained timeout; independent HKY replay unexecuted after restart, arbitrary sets, ordinary-query interpretation and full prices remain |
| B2: ell=9 and four summands | partial | 2,048 additional qualified queries, independently recounted points and D2/D3 controls; original 1,152 fixtures preserved; larger guessing/degree-four shards unexecuted |
| B3: N=58/60/100 continuation | partial | degree-two/three nonrefuting fixed points on three fixtures; all degree-four results resource-capped; independent large fixed-point certificates remain |
| C1: quartic independent replay | partial | m=6/8/10/12 smoke has 24 verified and identically replayed records; additional m14/m15 runs interrupted; 11/ten rho answers independently verified, no completed IC; original L2 full panels outstanding |
| C2: m=14--18 ladder | partial, integration/compute blocked | standalone m16--18 core/order/encoding controls and independent replay pass; full wide stage adapters, eight-target IC/rho ladder and complete prices remain |
| C3: complete method and full native price | partial | n13/n23 answer replay and ten n13 instruction profiles; construction/split counters, native allocator/physical costs and qualified comparison missing |
| D1: external/hardware controls | partial | all 44 same-input Singular correctness checks agree on 22 cases; fastest-tool performance and actual hardware comparisons remain |
| D2: durable evidence and PR | partial, summary published | owner approved publication; PR 2550 summary is updated; source/evidence delivery awaits storage access or an owner exception and integrated-source gates |

[Cost/scaling problem](../../problems/ic-low-degree-complete-cost-and-scaling.md)
and the four `XP-20261023-*` backlog cards preserve unresolved obligations.
Completed documentation or a passing PR does not complete those experiments.

## Reproduction and provenance

[report-data.json](report-data.json) is independently recomputed by
[native Rust](../../examples/ic_campaign_report_data.rs) directly from raw
rows and passed point-oracle audits; derived summary ratios are not inputs.
The [evidence archive](evidence/runs-local-20261023.tar.xz) preserves all
33 original local run directories, command JSON, hashes, source revisions,
stdout/stderr, exits and host metadata. Run:

```sh
bash research/ic_algebraic_crossover_20261023/verify-recorded.sh
```

The verifier extracts into a fresh temporary directory and byte-compares
the complete recomputation. The plotting Python only renders recorded
native values; it executes no experiments or numerical research.

Native experiment source was built at d67f6085; adapter smoke source was
pushed through a5ff7036. Individual receipts pin exact binaries and source
files. A later source-only parity lint fix is distinct from the recorded
instrument binary. Source-pinned inputs and their manifest remain under
`inputs/`. The first timer-less setup and jq argument failure are retained.
The original structural outer shell ended with a syntax error after all
96 native commands succeeded; editing its running script caused that tail
error. Subsequent panels used the preserved frozen runner copies.

Full library tests passed: 4,487 passed, 119 ignored. Native instrument:
nine tests; ecbench adapters: two tests. Initial sandbox socket failures
and the network-enabled successful rerun are both retained. Strict clippy
is blocked by two unchanged base lints; the scoped check passes with only
those command-line exceptions, without committed lint allowances.

No value is promoted to the IC leaderboard, progress ratio series or
admitted benchmark table: the complete-price and validation gates remain
unmet. Their historical figures remain unchanged; the scoreboard receives
a separate finite-stage panel. Visual copies in `benchmarks/` are expressly
unadmitted evidence, not a new admitted benchmark row.

The owner's total additional cloud-compute budget is **USD 100**, including
replay ([BUDGET.json](BUDGET.json)). No paid cloud job has been launched in
this snapshot. Local runs and a read-only GitHub Actions access probe are
separate; prepared shard plans are not a queued cloud allocation.
The [completed probe](access/ci-probe/status.json) authenticated but hit an
explicit identity-policy denial of `ssm:GetParameter` for `/ephemeral/config`
in us-east-1 ([masked log](access/ci-probe/failure.txt)). The prescribed
launcher cannot proceed with that identity. No IAM policy was changed.

Sources: [fall degree PR 2214](https://github.com/aburan28/crypto/pull/2214),
[guessing PR 2429](https://github.com/aburan28/crypto/pull/2429),
[cryptanalysis structured bases](https://github.com/aburan28/cryptanalysis/tree/claude/cryptanalysis-repo-setup-p59kek/experiments/pdp-degree-heuristics),
[historical quartic study](../gaudry_quartic_d4_char2_20261015/README.md).
