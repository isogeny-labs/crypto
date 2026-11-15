# Shared norm algebraic oracle: bounded software qualification

Frozen before implementation and execution of the new component, 2026-11-15.
The owner requests a complete end-to-end alternative to the tensor route.
Completion of this bounded qualification does not discharge the growing-family
subexponential cost/yield requirement.

Question: can the existing shared norm/remainder equations replace the explicit
p(p-1) coefficient census in the cold mixed-target IC path, with independently
checked rational point extraction? The existing F17 m3,t4 system has quotient
size 28 > 17, so one F17-valued separating linear form cannot cover all points.
The current one-linear-form extractor in PR #4091 reports unknown on collided
kernels. Use successive coordinate multiplication operators on the dual
quotient algebra, exact characteristic polynomials and full finite-field roots,
then simultaneous eigenspaces. No projected Krylov sequence is a completeness
certificate. This is standard finite-algebra machinery, not a novelty claim.

Sources checked: PKM 2016 sections 3.1–3.4 (primary PDF); the one-shot norm
continuation and its initial-elimination/selective-oracle notes; the retained
unbound CAS receipt; PR #4091's native extraction code. KB unavailable (no
MCP tools or CLI backend). Debian msolve 0.7.5-1 and FLINT 3.1.3-2 are the
existing pinned backend; downloaded packages must match packages.json hashes.
No backend source, existing gate, frozen evidence or historical output changes.

Before code: the changed component is rational extraction and the adapter from
public curve/target/base to an unknown polynomial solve. The existing native
point arithmetic, canonical polynomial parser and rank-free recovery are reused.
Missing prerequisites: native backend installation, exact staircase/reduction,
coordinate eigenspace splitting and independent model replay.

Qualification fixtures and acceptance:

1. A rectangular F5 grid of 25 points (> characteristic), a nonreduced one-point
   quotient, an extension-only quotient, and a unit ideal. Extract the exact
   rational point sets. Reject caps and malformed/noncommuting presentations;
   never call an incomplete or failed solve UNSAT.
2. The retained unbound F17,m3,t4 system/GB: extract all 28 complete rational
   assignments, evaluate every original equation, recover its curve points,
   and compare complete models with the independent native point oracle.
3. Generate fresh m3 remainder systems for the existing 90 affine-target/domain
   controls (p=17, a=b=2; (t,c)=(1,1),(2,1),(3,1),(3,16),(4,1)). Compare exact
   complete rational model sets, not only their counts. No witness assignments,
   factor-base logs, target scalars or point-sum tables enter the candidate.
4. Cold mixed-target IC on the fixed order-19 fixture: base x^8=1, P=(5,1),
   Q supplied as coordinates only. Draw a,b, ask the algebraic oracle about R
   alone, independently re-add every returned decomposition, collect B+1 rows,
   compute a left dependency from C only, reject zero denominators and verify
   recovered scalars. Harness verifies all 18 nonzero targets outside the solver.
5. Preserve all unsuccessful attempts, unit-ideal/unknown statuses, backend and
   extraction invocations, failures and caps. A default F17 linear-form
   parametrization failure is not nonexistence. No elapsed-time comparison or
   claimed exponent is collected in these software tests.

Budgets: one thread per backend call, 20s wall/2GiB address-space cap per fixed
fixture; at most 90 complete-model controls and 18 cold IC controls; staircase
cap 128, polynomial-reduction cap and per-point verification. Stop qualification
on the first disagreement and preserve the system/output. Larger arity or a
scientific growth panel requires a separate frozen protocol and the existing
EKS/checkpoint/admission gates. No scientific benchmark/cloud job is launched
by this software-qualification protocol.

The oracle returns independently verified relations or unknown. A returned GB
is a backend computation, not an independent source-ideal provenance proof.
Completeness relative to the input ideal requires that proof or the independent
complete-set reference on these bounded fixtures. No general good-target yield,
solving-degree bound or subexponential theorem is assumed from these controls.
