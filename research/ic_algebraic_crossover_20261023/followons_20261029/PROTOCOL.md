# Three follow-ons: component development and outstanding pilots

Registered 29 October 2026 before executing the new component checks.
This is an additive continuation of the three proposals published in PR2655.
The earlier scientific source, raw records, workload, caps and original absolute
deadline (2026-10-29T20:02:21Z) are unchanged. Extra-cloud total remains USD100.
No historical job is inspected, inferred live or relaunched.

## Scope

Build a standalone native Rust certificate/verification crate, independent of
the old review worktree's library. Local Cargo tests are software correctness
checks, not performance experiments or substitutes for the three full pilots.
No timing, speedup, complete query cost, minimum guessing number or crossover
is inferred from these checks. Every such cost stays null.

1. Constraint transfer: implement the actual binary b=1 S4 symmetric relaxation,
   exact Boolean substitution, and degree-bounded closure carrying source
   multipliers. Check the resultant/chain witness and all assignments of fixed
   small controls. Reject a corrupted certificate. Record pullback degree and
   support, without treating them as a query-cost win. The full pilot still
   compares direct-chain, symmetric, and transfer arms, signed lifting,
   physical/subgroup verification, duplicate removal and held-out queries.
2. Branch sharing: intersect the exact degree-bounded residual row spaces and
   replay a membership certificate separately in every leaf. Check all branch
   outcomes against the original equations, with positive and corrupted controls.
   Equal rank alone never certifies sharing. The full pilot must charge
   preprocessing, independent/shared solves, every leaf, certificate replay,
   lifting and memory, on a new complete branch set; original ell8 replay is
   untouched.
3. Cofactor quotient: implement binary-curve arithmetic and an independent
   finite-kernel Vélu evaluator. Small fixed E0 controls over F32/F64/F128
   verify the full map, target curve, kernel, fibers and homomorphism. The
   existing F(2^22) fixture is an additional component regression, with its
   recorded order treated as input. Test the concrete endomorphism candidate
   [23](-1-Frob)^3 against Vélu; no isogeny-yield gain is assumed. The full pilot
   still needs an algebraic implicit relation oracle, distinct projected bases,
   signed lifting, independent relation rank and complete cold pricing.

The solver/constraint arms belong to the algebraic track. Enumeration and group
sums used to verify them are correctness references in the generic decomposition
track (bounded by sqrt(r)); they are never reported as algebraic IC progress.

## Controls and limits

Fixed binary fields use moduli x5+x2+1, x6+x+1, x7+x+1 and the historical
x22+x+1. Curve E0 is y2+xy=x3+1. The symmetric correspondence control uses
V=<1,z>, three summands, and target abscissae 0 and 1. Tests must include
repeated roots, non-liftable algebraic assignments, and nonzero relations.
All closure rows retain exact multipliers. A complete certificate must replay
coefficient-for-coefficient; root agreement alone is insufficient.

Native compilation and fixed software tests have a 900-second watchdog and
one test thread. No performance isolation or A/A claim is made. Cloud pilots
must use Iso Lab EKS, immutable tested source/image identity, independent
replay and the original budget/deadline. No pool/identity/quota change and no
substitute deployed fixture. Before any experiment launch, update the complete
private resume and pass the official S3 checkpoint gate.

## Sources checked

- Published proposals: ../reports/new-leads-20261028/REPORT.md.
- Existing implementations: examples/ic_algebraic_followup.rs;
  src/cryptanalysis/binary_semaev_s4.rs; examples/ic_projected_query_law.rs.
- Kosters--Yeo, arXiv:1503.08001v3, section4: first fall does not bound solving
  degree; Huang--Kosters--Yang--Yeo, arXiv:1505.02532v2, hypotheses and ECDLP
  limitation: no uniform-degree conclusion from zero-dimensional results.
- Galbraith, Mathematics of Public Key Cryptography, chapter25: kernel
  quotient isogenies; source-access gap from the original report is retained.
- Atlas147 leads, review24October: hybrid-guess-determine-l7,
  cofactor-torsion-quotients, isogeny-transported-relation-yield; current scoped
  match queries29October. Existing closures and outstanding atlas delta remain.
- Current main AGENTS tracks rule29October; report-evidence, research-visuals,
  isolab-eks and its access/submission contracts.

## Stop and promotion

An invalid source multiplier, changed solution set, missing leaf or failed map
verification blocks that component. Preserve failures. A sound component is not
a useful solver or speedup. Promote only after the corresponding full pilot
lowers complete verified-query/branch-set cost against its frozen baseline,
including all failed attempts, lifting, verification and rank work. Native-time
claims additionally require the specified EKS admission and matched external
tools. Negative and inconclusive outcomes remain reportable.

## Versioned component amendment, 29 October after attempt1

Retain the original two controls and their raw records unchanged. Add a fixed
prime-subgroup positive control over F32: choose the first nonidentity [4]P
from the deterministic point order, take V=<1,x([4]P)> and target [3]([4]P).
Require at least one signed relation with all three legs in the order11 group.
This corrects the initial controls' absence of prime-subgroup positive relations.
The new component record is a successor, not a replacement performance run.

Select low-degree pullbacks by the kernel of projection onto all terms above
degree4, including linear combinations; filtering displayed basis rows alone
is incomplete. Add a cancellation control that must recover x+y from rows
xyz+x and xyz+y. Add an equal-rank/zero-intersection control. For sharing,
also test elimination of literally common input rows before finishing leaves;
charge its compilation/intersection/certificate overhead in the still-pending
pilot. A row-space intersection found after independent solves is diagnostic,
not an accelerator. No time/cost claim is added by these component amendments.

Final proof amendment: add conditional full-kernel and subgroup-injectivity
lemmas using the complementary-map identity, a separately certified h-kernel
cover, and its checked annihilation. These hypotheses remain explicit; no
elliptic-curve geometry or ambient-order proof is smuggled into the theorem.
