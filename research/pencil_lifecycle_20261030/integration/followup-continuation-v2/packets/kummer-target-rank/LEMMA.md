# Signed duplicate coefficient lemma

Let J be an abelian group, G have exact odd prime order r, Q=kG, and D_i=a_iG+b_iQ, with labels in F_r. Let canonical(D) choose one of {D,-D} deterministically. Choose sigma_i in {+1,-1} so sigma_i D_i=C; at D_i=-D_i use a fixed convention. Suppose two accepted signed divisors have the same canonical C and belong to the same curve, generator, target and modulus identity.

Set alpha_i=sigma_i*a_i and beta_i=sigma_i*b_i mod r. Then

  (alpha_2-alpha_1)G+(beta_2-beta_1)Q=0,
  Delta_alpha+Delta_beta*k=0 mod r.

Proof: subtract the two representations of C. Replace Q by kG. Exact order r identifies the coefficient annihilating G with zero modulo r. If Delta_beta!=0, its inverse exists because r is prime, and k=-Delta_alpha/Delta_beta mod r. If Delta_beta=0, validity forces Delta_alpha=0; this duplicate provides the zero equation. A nonzero Delta_alpha with zero Delta_beta is an invariant failure, not a target value. Verify any candidate by kG=Q before returning. No assumption about individual factor-base point orders is needed for this duplicate lemma.

An equal geometric key alone does not determine whether the coefficient equation is zero. It is therefore incorrect to infer that all duplicates lose no information. Opposite signs must normalize BOTH coefficients before comparison; comparing raw (a,b) is insufficient.

# Target-rank statement for graph equations

For a consistent formal system A*x=a+b*k over F_r, a vector y with y^T*A=0 yields 0=y^T*a+(y^T*b)k. It determines k exactly when y^T*b!=0. A graph cycle describes a dependence candidate; its target coefficient still matters. A balanced signed cycle may have y^T*b=0 and yield no target information. An unbalanced signed cycle can fix a component-root expression while A has full row rank, producing no eliminated target equation yet.

A signed incidence matrix of a loop-free forest has independent rows: a leaf column appears in exactly one incident row with coefficient+1 or-1, forcing that coefficient in every left-kernel vector to vanish; remove the edge and induct. Thus the deduplicated114-edge forest has zero left nullspace in its unanchored free-node model. This says nothing about excluded repeated rows with different labels. More general anchored systems require including the anchor equations.

The formal graph statement assumes a consistent F_r node-value model. To derive that model from arbitrary Jacobian point divisors, one needs an appropriate group homomorphism taking G to1, or another justified projection/subgroup argument. ord(G)=r alone does not prove all prime divisors have scalar coordinates in F_r. This packet does not strengthen the historical source's graph assumptions; the duplicate lemma avoids that additional node-coordinate requirement.

# Source-faithful prospective insertion

Current source uses relation_seen:HashSet and `if !relation_seen.insert(d.min(j.neg(d)))` at src/pruned.rs:521, before graph.edge. At that same accepted degree2/split gate, replace only the set's value storage with the first normalized label pair. On first occurrence, retain that pair and perform existing graph insertion. On duplicate, calculate the normalized coefficient difference, record its status, verify/return any nonzero-coefficient candidate, then suppress the graph edge exactly as before. Do not overwrite the first labels before comparison. Namespace is target-local as in the original collector; do not combine different archived targets' labels.

This is a proposed information-preserving deduplication interface, not an implemented or measured collector improvement. Historical existing general-coordinate/split scheduling stays as is for this scoped change. Retaining degree1 relations would be another explicit change. The old HashSet semantics remain in all sealed evidence.

Per accepted divisor, sign normalization needs at most two scalar negations. Per duplicate, two modular subtractions; if informative, one inversion, one multiplication and one negation, then actual kG=Q verification. Stored label payload is two u64 values per canonical key, excluding map allocation/metadata overhead. Existing Ring arithmetic must satisfy canonical operands and r<=2^63 for the add/sub word bound; inverse/multiply accounting and verification costs remain charged. No timing estimate is inferred.

# Future deterministic geometric fixture, not executed

Take a retained recovered split divisor D=aG+bQ from the original63-state packet. A second occurrence with labels (a+k,b-1) is algebraically the same D, and the normalized difference is target-bearing. The fixture must openly use its known planted k to construct that second label. Feed the same signed divisor (and separately its negation with negated labels) through a prospective duplicate handler; require recovery-neighbour and group checks as before. This tests geometric representation/label plumbing but does not show a naturally generated second walk occurrence or add a graph cycle. Do not count the planted label construction as new relation-search evidence. A future source-faithful trajectory test must obtain both label representations from its own initialized/propagated states.

Primary precedent: Handbook of Applied Cryptography, chapter3, section3.6.3 and Algorithm3.60, https://cacr.uwaterloo.ca/hac/about/chap3.pdf. The coefficient-difference mechanism is standard labelled-collision algebra. Signed recovery precedent: Chung–Costello–Smith, sections2,6,7, https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf. No novelty, geometric-cycle or performance claim is made.
