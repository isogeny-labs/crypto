# Persistent oriented fibers across sign-mode radii

T-1867, 2026-11-07. Producer derivation and deterministic software evidence; unsigned [pcc:unverified].

## Fixed anchor and scaled mode evaluation

Retain the reciprocal construction's caller-certified prime scalar field F_r, native bound, m≥3 and4m dividingr−1; n=(r−1)/(4m). On the first completed radius b0, fix t0=b0^(−m), V=t0 Z and W=V+V^−1. Each phase u has a squarefree compressed branch G_u(W), and a retained pair equation V^n−u=0. The first C0 constraint is encoded in G_u. Apply the first C2 pair, then retain this state even if lifting exceeds its cap.

For a later radius b, t=b^(−m), the new normalized image is tZ=(t/t0)V. Therefore evaluate F_e((t/t0)V), keeping the same W coordinate and quadratic relation V²−WV+1. Balanced streaming changes only the leaf power from V to (t/t0)V. The first state is never rescaled or rebuilt; the new constraints are expressed in the anchor coordinate.

At every later radius, sequentially apply C0=A F0((t/t0)V)−f0 and C2=f1² F2((t/t0)V)−A f2 F1((t/t0)V)², A=m b^((r−1)/2). Reduce all retained coefficient pairs modulo the surviving G after every update. Empty branches are skipped. Zero survives only if it satisfies both scalar constraints at every radius, using the predecessor's exact constant coefficients.

## Common orientation across a history

A branch retains pair equations P_j=a_j(W)+b_j(W)V. On each quadratic fiber, they share a root if and only if every norm N(P_j)=a_j²+Wa_j b_j+b_j² vanishes and every pairwise determinant a_i b_j−b_i a_j vanishes.

Proof: if some b_j≠0, all determinants force every linear form to vanish at the same candidate V=−a_j/b_j, and its norm guarantees that candidate is a quadratic root. If all b_j=0, the norms force every a_j=0; any fiber root works. Necessity follows by evaluation. The argument includes repeated roots and identically zero equations. Since the branch G is squarefree and its fibers split over F_r, gcd with these conditions gives exactly the projected common candidate set.

Inductively the old norms/determinants already vanish on the current G. To append P, take gcd with N(P) and its determinant against every prior equation; then reduce all old pairs and P modulo the new G. Comparing only against the phase would be insufficient on fibers where the phase vanishes identically: two later equations could select opposite orientations. The tests contain exactly that trap.

The original four-radius bound permits at most eight retained equations: initial phase plus first C2, then C0 and C2 for each of three further radii. A ninth equation and a fifth radius are explicitly rejected. No unbounded constraint history or target table is introduced.

## Deferred materialization and recovery adapter

Materialization is an attempted read of the retained algebraic state, not an update. It performs the predecessor's capped homogeneous lift and phase gcd, then filters against every retained pair. This last step is required because projection guarantees the existence of a common orientation but does not by itself discard other lifted orientations. It unions the disjoint phase factors, includes zero when retained, substitutes V=t0 Z and normalizes. The result equals the original image-domain intersection of all observed mode constraints.

A cap failure retains G, pair histories and the zero flag. Further radii can shrink the state, after which the same cap can succeed. Polynomial engine counters may advance during a failed materialization; mathematical state does not change. Initial H/D_n vectors are discarded after branch construction, since subsequent orientation lifting only needs r and n.

A separate StreamModes::recover_persistent adapter implements this path. Existing recover_traced and all predecessor controls remain available. Unsupported parameters or a failed bounded public quartic-root search select the predecessor before any source call. For eligible inputs it collects the same public signs and uses the same call, radius, extra-sign and final public-verification rules. On a lift cap it continues with another radius. Once a state materializes, existing phase recovery, pullback and history replay resume.

This adapter uses the existing caller's pullback-degree cap C for temporary homogeneous lifts and floor(C/m) for the materialized image degree, ensuring its subsequent pullback stays within C. B continues to bound coefficient chunks. These are distinct limits: the older first-radius helper used B=7 for its temporary lift. Therefore do not compare its40/275completion count to this adapter as if the lift envelope were identical. The new public comparisons use the same C for persistent and isolated adapters. Behavior at other caps may change routing or terminate at existing limits; no universal same-route claim is made.

## Charged work and storage

For s≤8 retained equations and branch degree d, an appended equation requires its norm, s determinants, gcd filtering and reduction of all stored pairs. Charge the full mode streams, balanced pair products, quartic-root search, first-mode transform, source setup, all failed lift attempts and final history replay. This is an exact representation change; elapsed time and total arithmetic dominance were not measured.

A branch retains G plus two degree-below-d coefficient vectors for each equation: at most (1+2s)d live coefficient slots, up to constants for normalized zero polynomials. There are at most four branches, each with d≤n. A conservative retained bound is O(sn), with s≤8, plus the existing interval context, metadata and zero/anchor fields. Charge actual Vec capacities and allocator overhead. Balanced mode evaluation retains its predecessor temporary bound O(d log(L+1)+B); scalar polynomial kernel workspaces remain charged separately. The cap C is not a process byte ceiling, and retaining histories extends their lifetime until recovery returns.

## Exact checks and failures

validation02 compiles and checks successfully with optimized Rust and overflow checking; its receipt pins60transitive inputs. The generator pins its predecessor and outputs.

- 2,516 states: all629scalar fixtures across (r,m)=(17,4),(41,5),(97,3),(193,3),(281,5), after each of radii2,1,3,4. Materialized polynomials agree coefficient-for-coefficient with the original dense intersection at every step. Fixtures use independent dense mode coefficients to construct observations; these known scalars do not enter runtime recovery.
- At the same small limits (temporary lift7,image degree7),626materializations defer;497fixture histories later fit after additional constraints. A large-cap read after every small-cap attempt confirms retained state is unchanged. Maximum retained history is8;629fifth-radius requests and21ninth-equation requests are rejected.
- 21degenerate-phase traps: the phase is zero on a split fiber, then two later equations select opposite roots. The all-prior determinant update correctly eliminates the branch.
- Public F257 curve y²=x³+x+7, base(1,3), order281, mode5, all281targets: at caller cap64/image cap12, persistent and isolated adapters each use2,881source calls, with zero unresolved and zero route changes.275states initialize and materialize at the first radius; no later updates are needed.
- Same public panel at caller cap16/image cap3: each adapter uses3,152source calls, zero unresolved and zero route changes. Persistent state records275initializations,260later-radius updates,265lift deferrals and270materializations. Endpoint returns account for trajectories that do not materialize after initialization.
- All inherited quadratic, reciprocal, context, arithmetic and617-target recovery controls pass. Original4563cohort and prospective trace protocol are unchanged.

validation01 compiled successfully and passed all2,516state checks, then stopped at a checker assertion incorrectly requiring later updates and deferrals in the cap64public panel. That expectation was not a correctness requirement and was contradicted by this panel. The failed receipt/stdout/stderr and exact failed checker snapshot are retained, with source-snapshot.json mapping its original pinned hash. The corrected checker reports actual counts and separately tests cap16. No production source changed between those native attempts. No render failure, cloud/timing trial or fullCargo rerun; historical missing-fixture/deadline limitation remains. Independent signed review remains open.

Replay current validation in a fresh directory:

```sh
python3 research/primefield-companion-followup-20261102/feature-native-persistent-fibers/validate-recovery.py /tmp/persistent-fibers-replay-unique
```

## Sources and next construction

Sources checked (KB): MCP absent; scripts/research-kb status reports no executable. Inspected exact quadratic pair algebra and recovery adapter. Primary sources: Bhargava-Zieve reciprocal fibers/Dickson definition, https://websites.umich.edu/~zieve/papers/manjul.pdf; official FLINT modular conventions, https://flintlib.org/doc/nmod_poly/index.html. No novelty or FLINT-performance claim. The multi-equation orientation lemma is proved above and checked by degenerate controls and dense state equality.

Next compress the retained constraint history into a canonical oriented state, or eliminate repeated per-branch mode evaluation. Either construction must preserve the common-root invariant and charge source and cold-target setup. The new diagram/PDF depicts only this implemented persistent adapter; historical/canonical research graphs remain unchanged.
