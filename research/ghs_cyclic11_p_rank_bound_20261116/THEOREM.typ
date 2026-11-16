#set page(paper: "a4", margin: (x: 18mm, y: 18mm))
#set text(font: "New Computer Modern", size: 10pt)
#show link: underline

= A p-rank-zero bound for cyclic-11 three-branch covers

_Research report, 16 November 2026. PCC-20261116-4818d8 has unverified standing._

== Statement

For every prime $p$ with $p mod 11 in {3,4,5,9}$, every smooth geometric cyclic degree-11 cover of the projective line with exactly three geometric branch points has genus five and p-rank zero. This includes every geometric branch position and twist. For any degree-five extension $K/k$ in such a characteristic, the Jacobian of such a carrier cannot receive a nonzero algebraic transfer from an ordinary elliptic curve over $K$. [pcc:unverified]

== Derivation and finite certificate

After algebraic closure, move the three branch points to $0,1,infinity$ and write $y^11=x^a(x-1)^b$, with $c=-a-b mod 11$ and all $a,b,c$ nonzero. Riemann–Hurwitz gives genus five. Lemma 2.1 of Ozman–Pries–Weir gives the signature `s_i = -1 + sum_{e in {a,b,c}} <-i*e/11>`, where brackets mean fractional part. [pcc:unverified]

Order five modulo 11 partitions the ten nontrivial characters into $R={1,3,4,5,9}$ and $N={2,6,7,8,10}$. Bouw's p-rank upper bound, restated as Theorem 2.3 by Ozman–Pries–Weir, is $f_p(C) <= 5 min_(i in R) s_i + 5 min_(i in N) s_i$. `OrbitBound.lean` checks all 90 valid exponent pairs: in every row, each orbit has a zero signature entry. Its conditional endpoint `zero_rank_if_bouw_bound` uses that finite certificate and a supplied upper-bound hypothesis to conclude zero. The finite declarations are axiom-free; the endpoint uses standard `propext` and `Quot.sound`. The normalization, signature formula and p-rank theorem remain cited external inputs. [pcc:unverified]

#image("orbit.svg", width: 100%)

The consequence for ordinary elliptic sources uses the fact that an ordinary Weil restriction has only ordinary positive-dimensional abelian images, whereas the target Jacobian has p-rank zero. Supersingular source factors, other branch counts, and noncyclic covers remain outside this statement. [pcc:unverified]

== Prior work and scope

This is a finite specialization of published signature and p-rank bounds. The exact order-five, degree-11 GHS exclusion is new to this project; publication priority was not established by the targeted search. The F3 and F5 rational Kummer point-count screens are separate finite results, not a general supersingular exclusion. [pcc:unverified]

Primary source: #link("https://doi.org/10.1017/nmj.2022.12")[Ozman–Pries–Weir (2022), Lemma 2.1 and Theorem 2.3]. The complete 90-row receipt is `ORBIT_BOUND.json`; finite Lean source is `OrbitBound.lean`; readable proof and limitations are in `THEOREM.md`.
