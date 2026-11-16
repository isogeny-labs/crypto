# APS follow-up: four solver and encoding leads

2026-10-28. Four bounded native implementations and correctness checks
completed; performance validation remains open. Full release suite:
5,002 passed, 0 failed, 122 ignored. Focused checks: 20 APS, 7 CLI,
63 F4-related and 15 GLV tests; seven new Lean lemmas pass both pinned gates.
Baseline: `6187036c049b6de2b1521b014f575952e4f1705c` (APS source unchanged
since [PR #2614](https://github.com/isogeny-labs/crypto/pull/2614)).

| Requested lead | Deliverable and acceptance criterion | Status |
| --- | --- | --- |
| 1. Stop on an informative relation | Stream equation-checked roots through point/sign/label checks; stop only after nonzero B and dG=Q; keep exhaustive mode, deadlines and unsuccessful branches | Verified on fixed public fixtures; performance open |
| 2. Membership-aware roots | Carry exact coordinate domains through substitution; factor unrestricted univariates with a deadline; never restrict auxiliary or symmetric variables to coordinate domains | Exact-set controls pass; performance open |
| 3. Two-parameter line divisor | Require H=X^3-t^2 X^2+(a-2tc)X+b-c^2 to divide the squarefree base polynomial; compare complete distinct-coordinate relation sets with point addition and the existing unordered encoding | Exact-set controls and recovery pass; performance open |
| 4. Remove uninformative orbit components | Certify propagated j=0 labels before excluding horizontal lines; reject independent relabeling; retain the B=0 recovery guard and prove/test preservation of informative relations | Finite oracle, F4 and refusal controls pass; performance open |

This extends the [earlier study](../aps_extensions_20261026/README.md).
The comparison baseline is its exhaustive, full-field substitution path on
the same labeled bases. Correctness tests are development checks with fixed
public fixtures; they are not a frozen performance panel. No end-to-end or
external-solver speed ratio is inferred from variable counts or solver counters.

## Sources checked before implementation

- APS, revised author manuscript, [§§3.1–3.4](https://iris.unitn.it/retrieve/e3835194-60e8-72ef-e053-3705fe0ad821/Amadori%2C%20Pintore%2C%20Sala%20-%20On%20DLP%20for%20prime-field%20EC%20%28rev%29.pdf): single labeled zero-sum relation and exact membership constraints. In particular, the authors would accept one suitable relation; complete enumeration is a validation tool.
- Milne, [Elliptic Curves, second edition, I §3](https://www.jmilne.org/math/Books/EC2.pdf#page=30), pp.25–28: chord law on the smooth cubic. Substitution of y=tX+c gives the displayed monic intersection polynomial. This is standard curve geometry, not a novelty claim.
- Existing native [`glv_invariant_base`](../../src/cryptanalysis/glv_invariant_base.rs): reuse counted gcd(X^p-X,f) / Cantor–Zassenhaus rational-root extraction, adding cooperative cancellation rather than a second arithmetic implementation.
- [`factor_base_designs_20261015`](../factor_base_designs_20261015/README.md), design 11 and `pencil_norm_factor`: prior norm-divisibility presentations already exist, with cost-unfavourable measured cases. Reducing variable count does not settle elimination cost.
- [Earlier source map](../aps_extensions_20261026/SOURCES.md): APS, Petit–Kosters–Messeng and Galbraith–Gebregiyorgis were checked; the Kudo et al. full-text gap persists. No exact reproduction or novelty claim for that follow-up.
- Sibling atlas `fd387705a71cd2f4f3a896af827d3676e0a07659`: ran `lead_atlas.py --brief` and `--match` before the local survey. Relevant existing leads are `LEAD-ICE-rootfind-pairtable-oracle`, `LEAD-STR-special-j-ic-semaev`, and the earlier study's atlas handoff. Their scoped negative results and complete-cost obligations remain intact.

## Mathematical scope

Work over a prime field p>3 on a nonsingular short Weierstrass curve.
The base has distinct rational, liftable abscissae. A monic cubic H dividing
F_V=product(X-v) has three distinct roots in V. For the line polynomial,
the points (x,tx+c) lie on the curve and sum to O. Conversely a signed
zero-sum triple with distinct abscissae lies on a finite-slope line, so it
appears in this presentation. The parameters (t,c) and (-t,-c) represent
the same abscissae. No division by t is used, so horizontal lines are kept
unless the separate label certificate authorizes their exclusion.

The pruning equation adds an inverse variable and can make elimination
harder. Solver diagnostics retain visited coordinate sets and aggregate
branch/root counts; they are not a standalone replay transcript or a complete
cost model. The earlier accounting and transcript-error gaps remain open.

For j=0, a horizontal intersection with three distinct roots is precisely
an order-three automorphism orbit. It is uninformative when both labels
are propagated by a checked subgroup eigenvalue. Geometry alone does not
prove this for arbitrary labels: independently labeled points on those same
abscissae can recover a logarithm. The pruning API must check its own label
assumptions and fail closed.

The [proof map](PROOF_MAP.md) records seven checked core-only Lean lemmas
and the full theorem's remaining bridges. The native line identity
certificate [line-identity.json](line-identity.json), `IDC1h0abc57c4d1e81eb0`,
was issued at seed 20261028 with 32 points and checked at a separate seed
9101 with 32 further points. The fixture runner's artifact test reads that
shipped file, checks its statement fingerprint, and runs the native checker.
This is not an independent EC replay certificate or a full formalization.

The [validation receipts](validation/README.md), [diagram](pipeline.svg),
and inspected two-page [report PDF](report.pdf) record the implementation
and its limits. The general EC chord-law bridge and correspondence between
Rust construction and Lean remain separate obligations from coefficient
identities. No leaderboard, performance chart or canonical curve graph
receives a new value; the study has no qualifying measured gain or curve route.

## Native interfaces and use

The [module](../../src/cryptanalysis/aps_single_relation.rs) keeps
`solve_system` exhaustive, adds `solve_system_reference` for the legacy
scan, and exposes `find_relation` for verified early stopping across system,
hybrid and cover searches. `solve_dlp_with_mode` adds the retry-route choice;
the legacy `solve_dlp` remains exhaustive. `Encoding::Line3` selects the
unpruned line presentation; `build_pruned_line_system` requires checked
orbit/label data. Field and sample caps remain unchanged.

The [fixture runner](../../examples/aps_single_relation_check.rs) exposes
`line3`, `orbit-line3`, `orbit-pruned-line3` and `--first-informative`:

```sh
cargo run --release --locked --example aps_single_relation_check -- solve \
  --arm line3 --first-informative --output /tmp/aps-line.json
cargo run --release --locked --example aps_single_relation_check -- solve \
  --fixture toy103-j0 --subset-size 1 --arm orbit-pruned-line3 \
  --first-informative --output /tmp/aps-orbit-line.json
cargo run --release --locked --example aps_single_relation_check -- line-certificate \
  --output /tmp/aps-line-identity.json
```

These commands are fixed public-fixture/certificate checks, not a performance
panel. Transcript `complete`, `stopped_early` and `timed_out` remain distinct;
a verified first witness reports partial enumeration. The runner still lacks
the independent replay/export and complete accounting required below.

## Performance work retained

The existing [complete-cost card](../../backlog/XP-20261026-aps-complete-cost-panel/README.md)
and [problem record](../../problems/aps-single-relation-complete-cost.md)
still require full phase accounting, independent replay, frozen inputs and
analysis, checkpoint upload, controlled EKS execution and an external solver.
No performance experiment has been launched for this follow-up.

Publication leaves the shared `problems/README.md` and daily log unchanged.
Automatic approval review rejected uploading those full cross-project files
even after checking their already-public baselines and APS-only diffs. Their
updates remain pending explicit publication approval. The APS problem record,
links above and extracted project-only [handoff](HANDOFF.md) are included.
