# Canonical oriented fibers without equation history

T-1869, 2026-11-07. Producer derivation and deterministic software evidence; unsigned [pcc:unverified].

## State and ramification normalization

Work over the predecessor's caller-certified native prime F_r. A squarefree split support polynomial G(W) parametrizes quadratic fibers V²−WV+1=0. The runtime obtains these supports from the checked reciprocal phase construction; the helper checks squarefreeness but does not independently enumerate or certify splitting.

Store three polynomials:

- B(W): monic support where both distinct quadratic roots remain;
- S(W): monic support where exactly one root remains;
- v(W): that selected root, reduced modulo S.

B and S are squarefree and coprime. Empty supports use1 and an empty selected-root function uses0. The working mode-evaluation ring also caches G=B S, so the implementation has four persistent vectors, not three. It stores no constraint list.

Initialize S=gcd(G,W²−4), B=G/S, v=W/2 mod S. This assigns the repeated-root fibers W=±2 to the selected component immediately. Without that normalization, equivalent histories could classify a ramified point as both or selected depending on which equation was processed first, violating canonicity.

## Exact update by a+bV=0

First filter existing selected roots:

S_old=gcd(S,a+b v), with v_old=v mod S_old.

On the previously both-root component, split by whether b vanishes:

D=gcd(B,b), B_new=gcd(D,a), Q=B/D.

At a root of D, the equation either preserves both roots (a=0) or eliminates the fiber. On Q, b is invertible and the only possible root is −a/b. It satisfies the quadratic exactly when N=a²+Wab+b² vanishes. Hence

S_new=gcd(Q,N), v_new=−a b^−1 mod S_new.

The old and new selected supports are coprime because the old S and old B were disjoint. Merge their root functions by CRT:

v_out=v_old+S_old*((v_new−v_old)*S_old^−1 mod S_new),
S_out=S_old S_new.

Unit-modulus cases skip the unnecessary inverse/CRT step. Retain B_new,S_out,v_out and refresh the cached support. No division by a possible zero divisor is attempted: b is inverted only on S_new⊆Q, and S_old only modulo its coprime new component. The inverse helper uses an explicit polynomial extended Euclidean loop and verifies the inverse identity.

This is pointwise exact on every split fiber and preserves the disjoint-support and root-function invariants. On selected fibers v_out²−Wv_out+1=0; on B_new both distinct roots satisfy every equation seen so far. Thus the new state replaces the entire equation history. The represented candidate set uniquely determines the two monic supports and the reduced root function, proving order independence and duplicate-update idempotence. This argument includes ramification because it was normalized at initialization.

## Persistent adapter and materialization

CanonicalImage reuses the fixed anchor and scaled-mode rules of PersistentImage: first V=t0Z, later modes at (t/t0)V; the zero candidate is tracked separately. Branch.keep applies the canonical update and refreshes the mode ring's G. No norms or determinants against prior equations are retained or revisited.

At materialization, lift B and S separately under the caller's degree cap and retain the phase condition V^n=u. B needs no additional orientation equation. On S, filter by V−v(W). Union those disjoint factors, then retain the predecessor's total image-degree cap, zero factor, anchor scaling and final public verification. Separate component lifts use the same numeric cap but a different decomposition than lifting their product; do not assume every possible routing decision must match at arbitrary caps. The reported public panels do match.

A separate recover_canonical adapter is published alongside recover_persistent and earlier controls. The four-radius and source-call policies remain unchanged. The canonical core accepts repeated constraints without growing an equation vector; this does not authorize extra radii or cloud work.

## Costs and storage

The history representation stores up to eight pairs plus G. The canonical representation stores B,S,v and cached G. If d=deg(B)+deg(S), their live coefficient lengths are at most3d+4, including normalized empty-polynomial conventions. This is independent of the number of prior equations. Charge all Vec capacities, metadata, allocator overhead, the existing interval context, and temporary polynomial workspaces. The recorded slot sum below is a sum over states, not a peak, byte allocation or process-memory measurement. Some empty/small states can have greater constant overhead.

Each update adds a fixed number of gcds and products: selected substitution, split by b, norm filtering, at most two polynomial modular inversions, and at most one CRT merge. It removes the linear scan of old equations and their repeated coefficient reductions. It does not establish lower total arithmetic for every input.

The inverse kernel is ordinary polynomial extended Euclid using the existing divrem/multiplication engine, with coefficients reduced modulo the modulus at each step. A safe coarse bound is O(d M(d)) polynomial work per inverse, plus divisions/additions and scalar inversions; no half-gcd inverse bound is claimed for this implementation. Its final inverse identity check is charged. Ramification and squarefree checks are charged at initialization. Streamed mode evaluation, source setup, cold-target costs, failed materializations and final history replay retain their prior accounting.

## Native checks

validation01 records optimized Rust with overflow checks, compile/check exit0,64transitive source input hashes and stdout digest.

- 852explicit candidate-membership checks on split fibers over F17,F31,F97, including ramification. Masked equations select different fibers so CRT merging is exercised rather than only empty intersections.
- Three reversed-history controls produce byte-identical (B,S,v) triples.18duplicate equations and96additional zero equations preserve the triple without any history vector. These controls execute36modular inversions and18CRT merges. Six nonsquarefree-support/nonunit-inverse inputs are rejected.
- 2,516retained states across629scalar fixtures and radii2,1,3,4 agree exactly with the prior persistent-history materialization. That prior path's independent dense-state controls also run in this validation. Cached support equals B*S at every recorded state.
- Sum of live coefficient-vector lengths over those2,516states: history75,387; canonical58,119. This includes G and all history-pair coefficients in the reference, and cachedG,B,S,v in the candidate. It excludes capacities, metadata, shared context and transient workspace; it is not a peak or timing result. Reference histories reach8pairs; canonical history-vector count is0.
- Public F257 curve y²=x³+x+7, base(1,3), subgroup order281, mode5: all281targets at cap64 recover with2,881source calls and no route changes versus the persistent reference; cap16 uses3,152calls, no route changes. Both have zero unresolved. At cap16 the canonical adapter preserves275initializations,260later updates,265deferrals and270materializations.
- Inherited persistent, quadratic, reciprocal, context/cache, arithmetic and617-target recovery checks pass. Original4563cohort and prospective protocol unchanged.

No native, generator or render failure. An early output read preceded check-output creation; the existing native process was waited on without a duplicate launch. No cloud/timing trial or fullCargo rerun; historical missing-fixture/deadline limitation remains. Independent signed review is open.

Replay in a fresh directory:

```sh
python3 research/primefield-companion-followup-20261102/feature-native-canonical-fibers/validate-recovery.py /tmp/canonical-fibers-replay-unique
```

## Sources and next construction

Sources checked (KB): MCP absent; scripts/research-kb status reports no executable. Inspected exact persistent/quad code and the scalar polynomial engine. Official FLINT modular and Chinese-remaindering conventions: https://flintlib.org/doc/nmod_poly/index.html, especially coprime CRT prerequisites. CRT and extended Euclid are established methods; no novelty or FLINT-performance claim. The canonical split/update proof and its costs are given above.

Next exploit selected-root components to evaluate later modes directly modulo S, avoiding quadratic-pair operations there, while preserving the both-root component. Alternatively replace the coarse Euclidean inverse with a checked faster inverse. The present diagram/PDF documents the implemented split and update; historical/canonical program graphs are unchanged.
