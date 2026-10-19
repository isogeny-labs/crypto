# Three native factor-base oracle leads — frozen before execution

The user authorized pursuing all three proposed constructions: asymmetric
factor bases, selector-based membership equations, and torsion-quotient
coordinates. This is a new bounded experiment, not execution or promotion
of any draft backlog card. All earlier protocols, records, archives,
registry/alias/store entries and installed byte-bound executables remain
unchanged. No S3, paid-host launch, scalar-log search or historical Python.

## Questions and scope

1. Does a size-ordered three-branch asymmetric decomposition procedure find
   the same original point decompositions as a symmetric union-domain
   reference, with different successful F4 work?
2. Does an exact guarded selector encoding of that same union reduce
   successful F4 work compared with its single product-membership equation?
3. Can a torsion quotient find a decomposition and lift it to an actual
   original-curve point sum, and how much counted F4 work does that require
   relative to the direct lifted-base description?

Success in this diagnostic means valid execution, exact input-domain
semantics, every emitted witness verified, and the complete bounded panel
reported. A lower row-reduction count on complete matched successful
calls is a solver-component observation, not a complete cost improvement.
Failures, unresolved calls and resource stops are retained. No candidate
can be declared a total-cost winner from this experiment.

The existing best verified full IC pipeline is not replaced. The
references here are exact finite-domain S3 decomposition procedures, not
the best whole IC pipeline. A complete improvement still owes the frozen
regression/holdout contract, all cold phases, symmetric native pricing and
matched external references under AGENTS.md. The WDSat full-pipeline suite
does not directly encode this initial bounded union-membership/quotient
experiment; this panel is explicitly a stage diagnostic, not a substitute
for that acceptance suite.

## Frozen inputs and cases

`plan.json` is the complete machine-readable design: seed 20261019,
12 training targets, 12 disjoint holdout targets, and up to 4 positive
controls per case. The four input inventories are byte-exact copies of
the prior chart screen; `inputs/models.json` preserves its registry rows.
They identify the exact models:

- `icv1-f2m7-tm13-ac10a42c`, Koblitz a=1, n=7, r=71, h=2;
- `icv1-f2m13-t181-515ee569`, Koblitz a=0, n=13, r=2003, h=4.

Retain their ICV1, EC1, field polynomial, subgroup and generator records.
The CLI checks its constructed curve slug, r and h against these inputs.

The six cases, fixed without solver measurements, are:

- n7-small: first 2 nonzero coordinate-chart abscissae and first 1 GP
  abscissa, in numerical order, with both signs;
- n7-full: both complete old coordinate-d3 and GP-d3 inventories;
- n13-small: first 2 main-chart abscissae and first 1 auxiliary abscissa;
- n13-graft: the complete old B033 main and B027 auxiliary inventories;
- n7-torsion and n13-torsion: C consists of ±G and ±[2]G. Construct its
  complete two-element-kernel preimage B as described below.

Selecting a prefix defines a new explicitly smaller case; it never
replaces either complete case. Exclude x=0 from chart inputs (the affine
two-torsion point). Deduplicate exact signed points. Do not replace a
union by its linear span: the span parameterizes variables, and exact
membership equations still restrict its roots to the intended inventory.

## Encodings and references

For each side, a deterministic binary echelon basis spans its exact
abscissae. Variable bits parameterize this span. Field multiplication is
expanded in the recorded polynomial basis and Boolean monomials are
squarefree modulo the explicitly included field equations z²+z=0.

Use binary S3 with b=1:

    (xy)^2 + t^2(x+y)^2 + txy + 1 = 0.

Descend its field coefficients to F2. A point witness is still mandatory:
an algebraic S3 root alone is not a decomposition.

Chart-case arms on identical original targets and the identical union U:

- product-membership reference: g_U(x)=g_U(y)=0, where
  g_D(X)=product over the distinct abscissae d in D of (X+d);
- asymmetric-branches: A+A, A+B, B+B, sorted by signed inventory size
  product (stable order breaks ties); stop only after a verified point
  decomposition, retaining/charging every earlier call. A+B includes its
  transposed representations by commutativity; there is no omitted branch;
- selector-membership: each side has its own Boolean selector s and
  guards (1+s)g_A(X)=0 and s*g_B(X)=0. This is exactly A union B,
  not a larger span or a relaxation.

Each positive control also repeats the reference as a same-binary A/A
count/outcome control. This is not an isolated wall-time A/A experiment.

## Torsion map and lifting

For these F2-defined b=1 curves, T=(0,1), and the existing dual-Frobenius
map is psi=Frobenius^(n-1) composed with [2]. Its finite x-coordinate is
x+1/x for x nonzero. Kernel points map to O and are handled separately.
This uses the prior exact-base quotient derivation, not a new theorem.

For each Q in C, obtain a subgroup preimage

    P = [(r+1)/2] Frobenius(Q),

and retain both P and P+T. Verify psi of each preimage equals Q directly.
No assumption that every point in 2E belongs to the chosen prime subgroup
is used: every C point and original target is checked against r. Degree13
has cofactor4, explicitly distinct from the older cofactor2 proof fixture.

Compare direct S3 decomposition on B+B against S3 on C+C at psi(R).
Lift each returned quotient point through the recorded B inventory. If
the lifted sum differs from R by T, translate one summand by T. Require
both returned points in B and their exact original sum equal R.
An unresolved quotient solve is unresolved, not an inferred decomposition.

## Targets, truth and relation rank

Before solving, enumerate actual point sums to retain exact nonidentity
prime-subgroup support. The first up to four supported targets, sorted by
point coordinates, are conditioned positive controls. They are not random
yield measurements or held-out confirmations.

For train/holdout, use the CLI's frozen `derive_u64` domain
`factor-base-leads/<case>/<phase>/v1` on seed, trial index, rejection
counter; reduce to 1..r-1. Reject a scalar already drawn in either phase.
All arms receive exactly the same targets. Holdout is disjoint from train,
but the finite existing bases were selected using earlier studies, so no
claim of an untouched discovery population or statistical significance.

Independent finite-inventory truth is audit work, recorded separately
from the online solver. It cannot supply a solver's witness. F4 roots
must satisfy every original equation, decode to exact inventory
abscissae, and yield a verified point sum after sign selection/lifting.

For training only, form signed relation rows on the SAME nonzero [h]
projections of union-B points, including for lifted quotient answers.
Collapse points with equal signed [h] projections and remove zero columns;
otherwise kernel translates could create artificial independent unknowns.
Retain both original and projected column counts. Record rank gains modulo r
without solving any point logarithms. Holdout and controls cannot add rank.
These row ranks do not guarantee completion or useful end-to-end LA;
all later pipeline phases remain obligations.

## Resource envelope and accounting

Unmodified native `f4_fp::solve`, grevlex, max degree4, one Rayon thread
per subprocess, 750ms cooperative budget PER solver call. Hard subprocess
limit 2000ms per permitted branch (asymmetric at most three branches).
One million observed matrix cells, 1024MiB Linux address-space limit,
parent-death termination on Linux. The mechanical whole-command timeout
is 2400s. A matrix event can trigger termination after observation;
the OS memory limit remains a separate guard. No retries or budget changes.

Preserve systems, job specifications, all roots, worker events/stderr,
failed/unresolved results and seals. Count F2 row-reduction multiplications
through the existing counter, encoding term products, extraction pair
checks, root counts, rank and actual lift-map evaluations. Partial killed-call
counters are explicitly incomplete, never zero. No wall/native timing
comparison: elapsed times are used only for resource limits.

F4's symbolic preprocessing, allocations, hashing, lookup, polynomial
assembly, target generation, map evaluation and rank costs are not fully
counted/calibrated. List these gaps for EVERY arm. The symmetric native
pricing band 0, 0.1 and 1 therefore has no complete total for any arm;
all total-cost rankings stay unknown. F4-only ratios, if shown, use only
complete matched calls, name their denominator and exclusions, and cannot
be called a speedup. No rho or full-DLP S value is manufactured.

Magma F4/F5, msolve/FGb and applicable Singular engines are the external
solver references to assess before a fastest claim. Their executables
are not present in this managed workspace at setup. This run cannot
establish fastest implementation or whole-pipeline cost. Exported system
JSON permits a later matched external comparison; no published external
timing from another input/host is substituted.

## Execution and preservation

Build/test/lint CLI and independent native audit before committing the
source/protocol/input lock. Bind those source files, root Cargo files,
inputs, this protocol, plan, and both built executables in the director's
card. Use the repository director -> verbatim mechanical -> director
assessment relay once, no retries, no worker choosing parameters.

Card ID CARD-oracle-leads-20261019-1; experiment ID
factor_base_oracle_leads_20261019; bus executor-mechanical-oracle-leads;
background true. Parent creates ignored `runs/` before dispatch.
Write scope only the new study tree. The only research commands are:

1. `./research/factor_base_oracle_leads_20261019/factor-base-sweep compare-leads --plan research/factor_base_oracle_leads_20261019/plan.json --out research/factor_base_oracle_leads_20261019/runs/experiment`
   timeout2400s, success0, stdout/stderr cards/run.stdout and run.stderr;
2. same binary `verify --run research/factor_base_oracle_leads_20261019/runs/experiment`
   timeout120s, success0, stdout/stderr cards/verify.stdout and verify.stderr;
3. `./research/factor_base_oracle_leads_20261019/audit/target/release/factor-base-oracle-audit research/factor_base_oracle_leads_20261019/runs/experiment research/factor_base_oracle_leads_20261019/inputs/models.json`
   timeout120s, success0, stdout runs/independent-audit.json, stderr cards/audit.stderr.

Receipt cards/CARD-oracle-leads-20261019-1.receipt.json. No research output
may exist at dispatch. Preserve setup failures before this lock separately.
Subsequent result reports are outside the sealed experiment directory.
Raw trees land as deterministic GNU-tar/xz with hashes and extracted-tree
byte-identical repacking. Reports, SVG/PNG, visually inspected PDF,
benchmark browsing copies, source-linked requirement status and PR update
are mandatory. Existing draft cards are not changed or executed.

## Interpretation limits and exit conditions

Report every case/arm/phase and actual completion count, including losses,
resource stops and any A/A discrepancy. The independent native audit
rechecks subgroup membership, every returned original witness, finite
support, transported targets, emitted roots and exact point memberships
using separate polynomial-field arithmetic on this same host. It is not
an independent-host certificate or a second F4 implementation.

No certified degree of regularity/refutation degree, proof of lower
successful solving degree, asymptotic result, wide83/130 support,
significance, recovered scalar, accepted native speed or end-to-end win.
Formalization status remains explicit; Lean/Lake are unavailable at setup.
Finite map tests do not stand in for a Lean theorem or an IDC1 proof.
The next action follows the observed successful-work and completeness
results, keeping incomplete pricing/proof obligations open.
