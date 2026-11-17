# Native signed collision bridge: source construction

Status: derived algebra and unexecuted source candidate. No compiler, native trajectory, trial, instrumentation measurement or cohort was run. The candidate is packet-only; the production module is unchanged. Independent source review is a separate publication step. Proof-carrying-code status is unverified.

## Concrete connection

The actual two-choice collector's duplicate branch currently records the first candidate attempt and suppresses its canonical divisor with a BTreeSet. Existing archived duplicate adapters store signed labels, but that behavior is not in this native branch. The proposed opt-in `solve_genus2_signed_collisions_observed` retains the first normalized label pair at the actual insertion point and evaluates each later signed duplicate before the existing choice/restart logic. Legacy `solve_genus2_observed` forwards false to the common implementation and retains its original event schema. Runtime equivalence of the false path has not been executed.

The new path uses no known target scalar. All accepted returned scalars must satisfy source_curve.mul(generator, candidate) == target, including existing identity/graph returns. A versioned signed-collision-v2 event schema distinguishes this changed policy. No existing consumer is asserted to accept that schema.

## Assumptions and collision derivation

Let C be the source elliptic-curve group, G a point of exact odd prime order r, Q=kG, and tau a group homomorphism into the destination Jacobian J with P=tau(G) nonzero. Then P has exact order r and T=tau(Q)=kP. Native primality checking occurs in Ring::new; the new path also bounds r <= 2^63, checks the reconstructed source field/model identity, canonical source point coordinates, source membership, G nonzero and rG=0. The source code's transfer and group-law interpretation remain mathematical premises; those global proofs are not established here. The premise Q in <G> is not inferred from rQ=0.

For a candidate divisor D with a valid label D=aP+bT, let epsilon=+1 if D is its selected canonical representative min(D,-D), otherwise epsilon=-1. Its normalized labels are A=epsilon*a and B=epsilon*b modulo r. Both labels receive the sign. Therefore canonical(D)=AP+BT. Equal canonical representatives with first labels (A0,B0) and current labels (A1,B1) satisfy (A1-A0)P+(B1-B0)T=0. Since T=kP and P has exact order r, Delta_a+Delta_b*k=0 in F_r.

If Delta_b != 0, prime r makes Delta_b invertible and candidate=-Delta_a/Delta_b. If Delta_b=0 and Delta_a=0, this pair contributes no new scalar equation and is retained as redundant. If Delta_b=0 and Delta_a!=0, the assumed valid-label/context invariant has been contradicted: the candidate records an invariant-conflict diagnostic and fails rather than inverting zero. The candidate is additionally checked by actual source kG=Q; even a failure of the upstream label premise cannot produce an accepted incorrect scalar through this branch, assuming the source multiplication/equality implementation is correct. This check is not an independent arithmetic implementation.

The derivation is established collision algebra, not a new DLP algorithm: Handbook of Applied Cryptography section 3.6.3, equations 3.3-3.4 and Algorithm 3.60 derive a scalar from equal group elements with tracked coefficients over a prime-order group. The present source connection adds the geometric sign normalization and native first-occurrence invariant. Reference consulted: https://cacr.uwaterloo.ca/hac/about/chap3.pdf, pages 106-107 (PDF pages 20-21).

## Source-ordered invariant

Within one invocation, the source generator, target, transferred P,T and modulus are fixed. At each native processing boundary the first-label map has exactly the same eligible canonical keys as the existing seen set, and each key's value is the normalized pair and attempt of its earliest processed occurrence. Stored pairs never change. This is a source-level inductive argument, not a compiled theorem.

Initialization: both containers are empty. New eligible key: seen.insert succeeds, the candidate normalizes both labels and uses the BTreeMap vacant-entry case to insert exactly once. The occupied-entry case reports a structural conflict without overwriting the stored pair. Duplicate key: seen.insert fails, a lookup must find the original pair; no map entry or graph edge is inserted. Non-split candidates and identity candidates bypass the same duplicate eligibility gate as before. Existing degree-one eligibility is preserved; no new degree-one/Kummer graph rule is introduced.

A restart changes the current walk D,a,b but not the fixed target context or retained map. The new label is rebuilt from the same P,T, preserving each stored entry's meaning. For the two prospective jumps, each nd,na,nb is constructed from the common current base and that jump's own labels; both are processed in native candidate order even when the first is not selected as the next walk state. Thus the earliest processed occurrence, not the eventual chosen state, is retained. An accepted collision breaks the actual outer walk immediately, and no second candidate or selection event is fabricated afterward.

The initial-label, jump, addition and restart equations require correct underlying group operations. The existing signed-label/scheduler/restart fixture checks exercise those equations on saved controls, but this universal induction retains them as explicit premises. The direct source verification gates every successful return in the new mode.

## RHS conventions and scope

The native graph edge keeps su*xu+sv*xv=a+b*k, hence its zero-equation row is [node coefficients,-b,-a]. The new duplicate equation is Delta_a+Delta_b*k=0, hence its target-only row is [Delta_b,Delta_a] with no nuisance coefficients. It is not inserted into the native signed graph; it is a separate scalar-return path. In particular, normalized duplicate labels do not replace the raw labels or edge signs for first-occurrence graph insertion. The pencil complement convention [A,b,+a] remains unrelated and unchanged.

Existing 64 signed streams, 6,900 edge rows, their witness certificates and first-information results remain immutable. They do not measure this new collision policy or its event stream. No natural informative collision rate, timing, cost ratio or new target count follows from the construction.

## Cost obligations and early returns

`Scholten::short_curve` clones `s.e.f`; Field::clone shares Rc<Cell<Counts>>. Source verification therefore enters the existing counts() = s.e.f.counts()+s.j.f.counts() closure, before the relations_and_signed_elimination phase snapshot. New-mode source generator/order checks occur inside walk_initialization. Ring normalization/subtraction/inversion/multiplication use the existing counted Ring. Symbolic increments and unpriced map counters are listed separately in costs.csv; none is a measured total.

The new map's 24 value bytes per entry count only three u64 values. Keys, BTree nodes, capacity, allocator bookkeeping, comparisons, serialization and observer memory are excluded. No physical-memory or complete-cost claim uses that number. It uses checked multiplication rather than saturation. Existing common preflight `inst.curve.valid(target)` uses the instance field outside the destination counts closure; this pre-existing boundary also needs complete accounting before a comparison.

On a successful duplicate, source verification runs before the loop break and existing phase/counter/outcome finalization still runs. Failed source verification, inconsistent zero-denominator labels, observer errors or count overflow return Err and may leave a diagnostic prefix without a terminal event. Those failures require outer receipts and full attempted-cost accounting; missing phase costs remain unknown, never zero. A future registered method must price map/observer work, setup, all attempts, failures and final source verification, and keep matched references and the frozen 1,260-target scope.

## Next validation gates

Independent source review of this exact candidate and patch, then a prospectively authorized finite validation plan with positive and negative signed duplicates, source-target mismatch, zero denominator, first-pair immutability, two-candidate ordering, restart continuity, observer failure and default-path equivalence. No tests have been run or assigned observed results here. Before any cloud build: publish exact source, configuration, fixture/input manifest, seeds and commands and retain GitHub readback; bind finite funding, CPU/RAM, concurrency, deadline and retry controls. The expired campaign is not renewed by this packet.
