# Collector transition and exceptional-branch source audit

Mapped eight collector transitions and eight dispatch regions to existing component evidence. The next central invariant is class-labelled reachable state: decode(d)=aP+bQ. Polynomial/Mumford validity alone does not provide this invariant. No component proof was repeated, no scientific jobs ran, and no repository files changed.

Source aliases: collector = retained projective_state_collector.rs; law = retained projective_law_gate.rs; three_leads = current repository src/cryptanalysis/three_leads.rs. Exact hashes are in status.json.

## Transition obligations

### transfer
collector:132-161. Existing evidence: normalization-input-guards, normalization-inverse-support, monic-remainder-zero-test; new residual Acceptance reported by parent.
Bind actual source transfer to class homomorphism, nonzero normalized leading scale, canonical words and nonzero weights. Existing j.valid checks prove representation acceptance, not class-image equality.

### initialization and restart
collector:173-198,221-228,344-352. Existing evidence: word-add11-validity-review, word-doubling-review, mixed-dispatch-word-review, sameu-word-review.
Prove scalar loop and add dispatch preserve class labels; initialize d=aP+bQ and each jump=A_iP+B_iQ. Preserve on both restart sites.

### candidate transition
collector:242-252. Existing evidence: same component addition proofs.
For each of TWO candidates derive nd=d+jump and na=a+A,nb=b+B modulo r. Relation insertion can happen for both candidates although only best advances. Needs class-sum semantics and [r]P=[r]Q=0.

### identity return
collector:253-257. Existing evidence: identity representation and word canonicality components.
Prove zero class implies na+nb*k=0 mod r via injective generator scalar map; justify prime r or unit nb before inverse.

### split normalization
collector:258-275; law:582-589. Existing evidence: weighted decode and normalization packets.
Prove split_possible discriminant scaling reflection for nonzero weights; normalized u roots and repeated-root multiplicity; curve validity preserved. Native asserts ordinate equation only after roots, not complete j.valid here.

### signed physical relation
collector:288-320. Existing evidence: relation-adapter-review, negation and Mumford validity components.
Bind effective Mumford divisor to signed point classes, including repeated roots and y=0. Canonical min with negation is only duplicate key; na,nb retain original sign. Define canonical node ordinate and identity anchor0.

### graph insertion
collector:321-334; three_leads:1558-1712. Existing evidence: relation-adapter-review and target-information review scopes.
Prove find potentials/path compression, union-by-size reversal, fixed anchors, same-root positive/negative cycles and constraint extraction. Need homomorphism h:J->Z/r with h(P)=1,h(Q)=k; nodes need not individually be r-torsion if h exists.

### continuation selection
collector:337-354. Existing evidence: no complete induction found in inspected packets.
Commit selected candidate together with its matching na,nb; graph remains sound after relations from unselected candidate; duplicate-induced restart resets labels coherently.

## Ordered exceptional branches

### identity and1+1
law:612-615. Existing evidence: word-add11-validity-review; word-doubling-review.
Complete ordered inverse/equal/distinct assembly and class-sum semantics, rather than redo component validity.

### 1+2 /2+1
law:616-622,828-878. Existing evidence: mixed-dispatch-word-review.
Reviewed source-order canonical validity and None exclusions available under original premises; supply reachable-state premises and class-sum theorem.

### same-u inverse
law:925-927. Existing evidence: sameu guard/word packets.
Inverse precedence and identity class sum; do not equate arbitrary record equality with class equality.

### same-u equal/double22
law:928-930,727-826. Existing evidence: degree22-guards-review and bzero producer packets.
Ordered bzero/szero identity exclusion established; bzero support geometry does not prove valuations/class sum. Generic double22 and all output branches still require composed validity/class/weight closure.

### same-u mixed
law:931-951. Existing evidence: sameu-composition-review, sameu-word-review.
Zero-slope exclusion, extraction/doubling validity established with global smoothness; supply constructor smoothness binding and class sum.

### distinct support B=0
law:958-999. Existing evidence: degree22 guards; mixed dispatcher components.
shared_zero_q_delta exclusion available. Need original-input proof of extracted p1..p4 validity/nonzero weights; mismatch exclusion; sequential p2+2p1 then p4 addition. Both explicit first secondary None and returned second None need exclusion.

### B!=0 C=0
law:1000-1023. Existing evidence: no full composed result identified in inspected packets.
Prove literal degree-one payload, canonicality/nonzero weights, normalized validity and class sum.

### B!=0 C!=0
law:1025;636-726. Existing evidence: common formula source present.
Bind common helper guards to outer values and prove payload/canonicality/weights/validity/class sum.

## Consequences for the next proof

First specify a generic transition theorem over a group with label map, then instantiate only after actual addition class-sum semantics are established. This avoids re-proving the already reviewed word and validity components. The signed graph may use a homomorphism into the scalar field; requiring every factor-base point to be r-torsion is stronger than necessary. Existence and normalization of that homomorphism still need proof for the actual transferred subgroup.

add_projective calls expect on both Option-valued mixed and degree-two additions. Excluding None on reachable states is therefore a native panic-freedom condition. The source comment describing secondary shared-support exceptions may be older than the reviewed complete mixed dispatcher; use actual calls and premises, not that comment as proof of reachable failure.

Stored-state equality and HashSet equality need not recognize all equal projective classes. This affects walk scheduling/deduplication and cost, while label soundness can remain valid; avoid silently assuming a canonical projective representation.

All scope labels above concern source composition, not compiled runtime equivalence, timing, independent replay of every imported proof, or full cohort completion. The new residual root remains untouched.
