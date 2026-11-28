# CT331 — irreducible two-torsion gate for longer 2-power chains (candidate written argument)

**Scope and standing (2026-11-23 UTC).** This is a source-linked **candidate proof and falsifiable structural filter**, not a Lean-kernel theorem, native run, signed PCC claim, or ECDLP performance result. [pcc:unverified] It extends the *scope investigated* in [CT13](../chained_jacobian_transfers_20261110/CT13_DESCENT_OBSTRUCTION.md), [CT50](https://github.com/isogeny-labs/cryptanalysis/blob/f9c7b49e770212c24e29ca5c87e202e2aa6e8493/experiments/chained-transfer-reference/ct50/CT50_BIDUAL_RETURN.md), and [two-hop PR #4992](https://github.com/isogeny-labs/crypto/pull/4992) to arbitrary-length **2-primary isogenies, provided the source mod-2 Galois representation is irreducible**. Do not identify this with a new general Jacobian transfer impossibility theorem.

## Exact candidate theorem (finite-field version)

Let $k=\mathbb{F}_q$ have odd characteristic and let $A/k$ be an abelian variety of dimension $g\ge 1$. Suppose that the geometric group $A[2](\bar{k})\cong(\mathbb{Z}/2\mathbb{Z})^{2g}$ is an **irreducible** $\mathbb{F}_2[\operatorname{Gal}(\bar{k}/k)]$-module. Then any finite Galois-stable $2$-primary subgroup $H\subset A(\bar{k})$ has the form

$$H=A[2^r](\bar{k}) \qquad\text{for some integer }r\ge0.$$

Here $r=0$ denotes the trivial subgroup. Consequently any $k$-defined isogeny $f:A\to B$ of degree a power of two, with $B/k$ an abelian variety, factors as $u\circ[2^r]$, where $u:A\xrightarrow{\sim}_k B$ is an isomorphism and $\deg f=2^{2gr}$. In particular a rational 2-power isogeny cannot reach a new $k$-isomorphism class under the irreducibility hypothesis.

**Proposed elementary argument (to formalize independently).**

1. Since the characteristic is odd, $A[2^n]$ is finite étale; its geometric torsion points and Galois action classify these finite subgroup schemes. Any nonzero finite $2$-group $H$ has nonzero $H[2]$, a Galois-stable subgroup of $A[2]$. Irreducibility forces $H[2]=A[2]$.
2. Induct on $|H|$. The stable subgroup $[2]H$ has smaller order. If it is zero, $H=A[2]$. Otherwise induction gives $[2]H=A[2^{r-1}]$ for an integer $r\ge2$. Since $\ker[2]=A[2]\subset H$, every preimage of any element of $[2]H$ differs from a member of $H$ by an element of $A[2]$. Thus $H=[2]^{-1}([2]H)=A[2^r]$.
3. A $k$-defined isogeny with power-of-two degree has finite étale Galois-stable kernel, so it has one of these full layers as kernel. The universal property of the quotient $A\to A/A[2^r]$ and equality of separable degrees factor $f$ as $u\circ[2^r]$ with $\deg u=1$; the induced $k$-morphism $u$ is an isomorphism. **This geometric quotient-to-isomorphism step is not covered by the explicit Bool finite Lean model in #4992.**

The argument does **not** assume every geometric intermediate kernel is rational: only the **composite morphism** is $k$-defined, so its total kernel is stable. A rational endpoint equation alone is insufficient.

## Concrete genus-2 Frobenius gate: 5+1 root type

For a squarefree genus-2 hyperelliptic sextic over $\mathbb{F}_q$ whose six geometric Weierstrass roots have Frobenius cycle type $(5)(1)$, $J[2](\bar{k})$ has 15 nonzero classes represented by unordered pairs of roots (equivalently even subsets modulo complement). Under the five-cycle, the 15 pairs form **three orbits of length five**: five fixed-root/5-cycle pairs, five cyclic-adjacent pairs within the 5-cycle, and five cyclic-nonadjacent pairs. Frobenius has order five and no nonzero fixed vector on this four-dimensional $\mathbb{F}_2$-module. Since $\operatorname{ord}_5(2)=4$, the polynomial $X^4+X^3+X^2+X+1$ is irreducible over $\mathbb{F}_2$; hence this representation is irreducible.

**Conditional consequence for Richelot chains.** If $J$ has Frobenius type $(5)(1)$ and a composite of $h$ geometric $(2,2)$-isogenies actually descends over $\mathbb{F}_q$, its degree is $4^h$ but the candidate theorem forces $4^h=16^r$. Thus $h$ is even, $r=h/2$, and the descended composite must be $u\circ[2^{h/2}]$. For odd $h$, no such rational composite exists. For even $h$, the destination is $k$-isomorphic to $J$. This conditional all-length filter is **specific to mod-2 irreducibility** and does not rule out 3+3 or 6-cycle chains, odd-degree transfers, mixed-degree correspondences, norm/trace constructions, higher-dimensional destinations, or different (possibly faster) computational models of an isomorphic variety.

**Sharp scope controls.** In the already qualified #4992 finite population, $(5)(1)$ has no stable first Lagrangians and the two-hop stable-kernel incidence is dual only. By contrast $(3)(3)$ has three stable first Lagrangians and a reducible mod-2 module; 6-cycle has one stable first kernel. They cannot inherit the all-length conclusion. Even the absence of rational two-torsion alone does **not** imply irreducibility (the $(3)(3)$ example).

## Reproducible qualification plan (unexecuted)

- **Mathematics:** independently check the induction in a generic finite Galois module, the full inverse-image identity, the finite étale step, the quotient factorization, and the ((5)(1)) cycle representation. Formalize generic subgroup and kernel statements in the compatible **shared** Mathlib/crypto-lean group-instance owner, not by silently treating #4992's custom Bool model as a generic instance.
- **Rust structural controls:** on the *existing* #4992 representation only after its owner clears use, check the three length-five orbits on all 15 nonzero classes; enumerate admissible Frobenius-stable (2)-subgroups in bounded mod-4 and mod-8 toy models and reject the deliberately substituted ((3)(3)) or ((3,1,1,1)) reducible-action controls. Freeze source, finite universe, negative mutants and resource limits **before** executing. No existing C/D numerical population is to be rerun.
- **End-to-end relevance:** when vetting a proposed rational chain out of a ((5)(1)) source, compute its total kernel/degree and morphism descent first. If it reduces to multiplication up to isomorphism, skip an expensive new-destination IC run and record this structural negative; keep ((3)(3)), 6-cycle, odd-degree maps and genuine higher-genus prime-field transfers on the candidate queue. Any measured speed improvement still requires complete cold transfer/solve/recovery/setup/LA/memory and the strongest matched rho, BSGS and IC references.
- **Validation:** an independent Rust implementation and Lean source/instance assembly, actual checksums/commands/whole outputs, mutation controls, CI budget, and PCC signing remain **not done**. No source, timing, theorem verification, or success claim may be inferred from this note.

## Sources and custody

- [Magma Richelot documentation](https://magma.maths.usyd.edu.au/magma/handbook/text/1615): geometric kernel/dual and double-step definitions; does not prove this all-length extension by itself.
- [Castryck–Decru–Smith (2020), Section 6](https://doi.org/10.1515/jmc-2019-0021): double-step types only, not this arbitrary-length statement.
- [CT50 source-bound dual discussion](https://github.com/isogeny-labs/cryptanalysis/blob/f9c7b49e770212c24e29ca5c87e202e2aa6e8493/experiments/chained-transfer-reference/ct50/CT50_BIDUAL_RETURN.md), [#4992 SOURCES_AND_SCOPE](https://github.com/isogeny-labs/crypto/blob/8475b8a2a6cc88a738f09f8b4d0ca15ff505d8fa/research/chained_twohop_descent_20261123/SOURCES_AND_SCOPE.md), [#4992 RESULT](https://github.com/isogeny-labs/crypto/blob/8475b8a2a6cc88a738f09f8b4d0ca15ff505d8fa/research/chained_twohop_descent_20261123/RESULT.md): earlier source and bounded finite controls. Source revisions #4992 `8475b8a2a6cc88a738f09f8b4d0ca15ff505d8fa`; #4997 starting `4bd489756c31787adc056012982e1ac2af502ae6`.
- Shared KB/conductor/native subagent slots are not exposed in this runtime, and the local repo wrapper is unavailable; source search of crypto, cryptanalysis and crypto-autoresearcher plus primary handbook was used instead. This is **not** a complete novelty clearance. No locked VLT, paid EKS job, six-object original fixture or norm-reuse-owned files were accessed. No extra CI workflow or automation was created. [pcc:unverified]

## Handoff / role accounting

Coordinator performed source/ownership/PR/queue checks and authored this one new, disjoint research note. Mathematics package: written all-length conditional proof as above; **not independently checked by a native worker**. Rust package: **deferred** because its existing norm-reuse owner/reservation must not be duplicated and no native Rust execution is available here. Validation package: a frozen-to-design, **unexecuted** public test/negative-control matrix above. No two workers were counted as independent validators. Parent #4990/#4992 custody/CI/instance obligations remain, as do original 695/701-record norm reuse, C169/C13a/C13b/N2_actual/D, strongest same-host complete-solver pricing and signed PCC. [pcc:unverified]
