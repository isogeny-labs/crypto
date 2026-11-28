#set page(margin: 0.42in)
#set text(font: "DejaVu Sans", size: 7.4pt)
#set heading(numbering: "1.")

= Semantic relation-rank bound for hashed prime-field decomposition
#text(fill: rgb("475569"))[2026-11-18 · local kernel checks pass · PCC standing unverified · no performance run]

== Result

Split $m$ hashed factor-base slots after $k$. The semantic matrix $A_S$ has
partial tuples as rows and columns, and entry one exactly when their sums form
the target. If $D_S(R)$ is the number of complete decompositions, then

$ "rank"_QQ(A_S) <= "nonzero rows" <= "nonzero entries" = D_S(R). $

Claim `PCC-20261118-4717cb` binds the generic finite-relation theorem. Its
local proof verifier passes; signed PCC standing is unverified.

#figure(
  image("figures/semantic-rank-split.svg", width: 64%),
  caption: [The semantic obstruction and the separate low-level BTNIR obligation. Dashed means no rank-equivalence theorem is known.],
)

== Source-threshold bound

Use Bochum v0.2 notation $n=r-1$, $rho=tau/s$, and $L=n rho$. At most
$n^(m-1)$ nonidentity tuples sum to one target. Independent slot hashes accept
each tuple with probability $rho^m$, so

$ "E"_S[D_S(R)] <= n^(m-1) rho^m = L^m/n. $

The source selects $tau = "ceil"((s/n) "max"(r^(1/m), 10m))$. In the stated
regime put $q=r^(1/m) >= 10m$. Cofactor one, Hasse's bound and $s>=2p$ give
$n<s$, hence $L<q+1$. For $r>=100$,

$ "E"_S[D_S(R)] < (q+1)^m/(r-1)
  = (r/(r-1))(1+1/q)^m
  <= (100/99) "exp"(1/10) < 2. $

Thus, for integer $K>0$,

$ "E"_S["rank"_QQ(A_S)] < 2, quad
  "Pr"_S["rank"_QQ(A_S) >= K] <= 2/K. $

An all-seed integer rank floor has $K<2$. More generally,
$"Pr"["rank" >= 2^(c b)] <= 2^(1-c b)$, so an exponential floor does not hold
with nonnegligible probability for a uniformly sampled source seed. An
exceptional selected seed sequence is not excluded. Claim
`PCC-20261118-e6722e` binds the finite uniform-average implication. Its local
proof verifier passes; signed PCC standing is unverified.

== Scope and verification

This applies to the encoding-independent slot relation. E1's BTNIR census
flattens low-level arithmetic wires and may have higher rank. No transfer to
nonlinear, randomized, approximate or separator-avoiding algorithms is made.

Lean 4.35.0 / Mathlib `cfc55fc…` imports
`Matrix.rank_le_card_of_support_subset` as stated and checks five new theorems:
row-support containment, the witness injection, the semantic bound, finite
Markov counting and the uniform-average endpoint. The hash expectation is the
specialization of Bochum Lemmas 9 and 13 written above, not a Lean probability
model.

Primary source: Akshima et al., Bochum v0.2, revision `1ee44e56`, Definition 4,
Lemmas 9–13, Theorem 4 and Remark 2. Full paths, proof map, evidence limits and
KB-unavailable receipt are in `SEMANTIC_RANK_BOUND.md`.
