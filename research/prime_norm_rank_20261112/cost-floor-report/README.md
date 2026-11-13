# Single-target batch floor in the declared m8 entropy-charge model

2026-11-13. Model-accounting correction, unsigned producer standing
[pcc:unverified].

The incoming PKM report projects a gain at m8 if D(8,3)=9, using slope1/8 and
a dense entropy charge. Its balanced total number of solver calls has a
negative exponent. Enforcing at least one whole solve for a single target
removes that gain **within the same declared charge model**, even if a solve
outputs every useful relation it can. This does not lower-bound an actual
sparse solver, close PKM, or resolve existence of a sub-exponential algorithm
[pcc:unverified].

## Exact model and result

Let p>1 be the normalized group-size parameter, b>=1 the base size, and

    H2(s) = -(s log(s)+(1-s)log(1-s))/log(2),
    theta8 = 8 H2(1/8).

Lean derives the exact formula and bounds

    theta8 = 24-7 log(7)/log(2),   4<theta8<5.

The bounds follow from the exact integer inequalities
2^19 < 7^7 < 2^20. The incoming unclamped exponent and balanced-call exponent
satisfy

    2/5 < 2/(9-theta8) < 1/2,
    1-7/(9-theta8) < 0.

Thus the apparent gain is paired with a fractional whole-attack solve count.
For the optimistic all-output model, use

    W(p,b,c) = max(1,p/b^7)*c + b^2,
    c >= b^4.

The declared entropy charge c=b^theta8 satisfies that inequality. The exact
Lean conclusion, for every p>1,b>=1, is

    p < W(p,b,b^theta8)^2.

This is a normalized cost-model statement; no wall time, GAE, measured solver
exponent or universal cryptographic lower bound is asserted [pcc:unverified].

## Proof

Write t for total charged work. The floor and yield charges imply

    b^4 <= t,   p <= t*b^3,   t>=0.

If b>1, then

    p < p*b <= (t*b^3)*b = t*b^4 <= t*t.

If b=1, the yield charge gives p<=t; p>1 implies p<t^2. The actual work
expression supplies both charges. The central universal ring identity is

    (t*b^3-p)*b + (t-b^4)*t = t^2-p*b.

No integer rounding is hidden in max(1,...): this is already an optimistic
real-valued lower proxy for whole calls. A ceiling, more than one independent
masked target, output processing or deficient relation rank can increase the
charged work. Relating any such charge to actual solver cost requires evidence
outside this theorem [pcc:unverified].

## What this changes in the experiment decision

The D8,3 cell remains useful as a PC-degree diagnostic. Its result alone is
not decisive for the advertised single-target gain under this entropy model.
A positive sparse solver might cost less than the declared dense charge; that
possibility is not excluded. The next experiment should measure actual solver
work, witness/output extraction and useful independent relations, with the
single-target call floor present before parameter selection. See
[NEXT_TEST.md](NEXT_TEST.md) [pcc:unverified].

A fixed exponent below1/2 would still be exponential in the input bit length.
The user's sub-exponential objective needs a uniform growing-parameter work
bound, with all generation, sign handling and relation processing charged.
The current norm/quotient proofs establish certificate components, not that
public-input finder.

## Bounded source audit, not a blanket closure

Source snapshot: isogeny-labs/crypto@5965fa03b767f286bee51d849bf7275666a92d8d.
The following are source observations and limits, not independently replayed
numerical results [pcc:unverified].

| Source | Scope that must be retained |
|---|---|
| L1 PROTOCOL/AMENDMENTS/D83_VERDICT | Completed level8 is the producer's recorded D>=9 evidence. Level9 is censored. Entering a degree10 F4 round is a different trace claim; it does not replace the frozen completed-closure criterion. |
| L1 scripts/gen.py | The random/planted branches enforce rational targets. This concern was checked and cleared; do not report invalid targets. The code hash matches the committed manifest. |
| L1 MANIFEST and directory listings | Manifest names closure/src/main.rs and closure/Cargo.toml, but closure/ is absent from this committed study listing. Several normal ladder run paths also have no committed parent directory. This is a bounded replay-artifact gap; no repository-wide absence claim. |
| L2 REPORT | Its truncated nu_s(T) values are explicitly upper estimates; failure to find a low-weight element is not an all-degree absence proof. Its scope excludes non-lattice finders. Original full protocol/code audit remains incomplete. |
| L3-check CHECK | The review separates unsplit degree bounds from split encodings and full-space column counts from actual solver work. Its one-output correction motivates checking the whole-batch floor even when all outputs are allowed. |
| L4-check CHECK | The review identifies success-conditioning, mask scope and post-hoc filter issues. Historical scratch durability comments are not assumed current after the Git commit. No numerical height audit was repeated here. |
| L5 REPORT | The group-isomorphism reduction and point-ordered widths do not exclude arbitrary algebraic or interleaved representations. Shared-VM timings were not replayed or adopted as calibrated comparisons. |

No inherited Python research was executed. A broad git archive request began a
promisor fetch and was stopped; subsequent reads used only named GitHub content
endpoints. The source review is bounded and does not claim exhaustion of all
repositories or primary papers [pcc:unverified].

## Formal map and receipts

Source: [PrimeICCostFloor.lean](../lean/PrimeICCostFloor.lean), frozen
commitd42ef1874. Twelve exact interpretive producer bindings are in
[cost-floor-evidence](../cost-floor-evidence/) [pcc:unverified].

| Declaration | Statement |
|---|---|
| surplusIdentity | Universal ring balance |
| floorsNoBeat | Explicit two charges imply p<t^2 |
| workCallFloor, workYieldFloor, workNoBeat | Actual defined max-one-call model supplies charges and conclusion |
| thetaEightFormula, thetaEightGtFour, thetaEightLtFive | Exact entropy formula and bounds |
| unclampedExponentBounds | Incoming exponent lies between2/5 and1/2 |
| unclampedCallExponentNegative | Incoming whole-attack call exponent is negative |
| entropyChargeFloor, entropyWorkNoBeat | Actual defined entropy charge and corrected model conclusion |

Lean4.35.0 with unchanged pinned CryptoLean/Mathlib. Target build003 passes2074
jobs. Full tracked-source audit covers155 declarations with zero failing and
zero additional trusted axioms. Fresh import-inclusive replay completed with exit0; its attributable
completion receipt is retained. Only standard propext/Classical.choice/Quot.sound are
admitted; no signed PCC verdict is claimed [pcc:unverified].

Build001 exposed real-vs-natural power rewriting and even-power positivity
proof issues; build002 corrects them, build003 adds the exact projection/call
bounds. All failure/success logs are retained. Earlier control/matrix/functional
source declaration hashes are unchanged [pcc:unverified].

Native record **IDC1h0f32be38dee24607** checks only surplusIdentity over any
CommRing at32 statement-derived and32 extra-stream points through the unchanged
maintained checker. It pins the exact universal theorem source, does not
execute Lean and does not prove inequalities/logarithms or solver-cost claims.
Its adapter's22 inherited tests pass. No timing result is inferred
[pcc:unverified].

## Sources and resources

Primary source read: Galbraith, Mathematics of Public Key Cryptography,
15.8.1–15.8.2 (author-hosted book already pinned by SHA-256
ca75ee62b96daa0d46faaa2d4dc28205683455ad3a2a48586b9b39299c25022d),
https://www.math.auckland.ac.nz/~sgal018/crypto-book/main.pdf .
Incoming L1 protocol/amendments and L3 checker supply the exact cost formulas
reviewed here. Pinned Mathlib logarithm, real power and inequality APIs were
read before implementation. Review source receipts identify exact paths/hash.

KB MCP absent; wrapper no executable. Four fallback queries covered PC/F4
bound direction, fractional/full-rank accounting, success/mask conditioning and
classical summation-polynomial decomposition. No priority claim for elementary
batch accounting. Conductor unavailable. No new paid job or experimental
runtime/scaling campaign; monthly isolab budget exhausted. No benchmark,
leaderboard or ecdb change. Canonical scoreboards and historical study figures
gain no measured result and remain unchanged.

Editable [DOT](floor.dot), [SVG](floor.svg), [diagram PDF](floor.pdf),
[report PDF](report.pdf) and [LaTeX](report.tex) accompany this report.
