# CT342 frozen protocol — odd-primary transfer and 2-primary cokernel audit

Registered 2026-11-23 UTC before CT342 source implementation or execution. Existing lane: draft crypto PR #5045, branch `automation/ct341-f103-full-jacobian-dual-census-20261123`, source head at registration `d1636e7d66685c9028583a123b1766f8e21b8430`. [pcc:unverified]

## Scope and sources
Bounded, **untimed** finite-group audit on CT341's fixed public split F103 example, not an original 695/701-record norm-point run, not a Rust/Cantor replacement and not a claim of ordinary elliptic ECDLP improvement. CT341's frozen Python source SHA256 `4793c969e2c563ff55323e4c78c19affdda838445652cc277942da7afcc31086` and output SHA256 `ecc449a98eeb3d1554329d27e61ef45fa3aa408760ce2aeb76e84c060f5ac514` remain immutable; do not rerun its full 9200-class population.

Internal primary context: CT340_RESULT.md and CT341_RESULT.md at the above source head. External: Diem, *Families of elliptic curves with genus 2 covers of degree 2* (https://arxiv.org/abs/math/0312413); Magma Richelot handbook (https://magma.maths.usyd.edu.au/magma/handbook/text/1615). Research-KB MCP/CLI and Conductor are unavailable in this runtime; no novelty or independent-validation standing claimed.

## Question and mathematical obligations
Let `A=J(C)(F103)`, `B=(Eplus x Eminus)(F103)`, `Phi:A→B`, `Psi:B→A`, with CT340 dual identities `Psi Phi=[2]_A`, `Phi Psi=[2]_B`, and CT341 finite counts `#A=#B=9200`, `#ker(Phi)=#ker(Psi)=4`. Independently determine the complete abelian-group invariants of Eplus and Eminus using explicit F103 rational-point enumeration and their *own* elliptic group laws in a separately written Node.js implementation. Require exact 92 and 100 counts, all three nonidentity rational 2-torsion points on each factor, and classify the 5-primary part of Eminus by point-order tests (not by the group size alone).

Derive and write a finite-group lemma: any two homomorphisms of finite abelian groups with both dual identities `[2]` induce inverse isomorphisms on every odd-primary subgroup (inverse of Phi is `[2^{-1}] Psi` on that subgroup). Both cokernels are killed by 2. In this F103 example, #A_2=#B_2=16, and all six branch points make J(C)[2](F103) a 4-dimensional F2-vector space. From the elliptic 2-torsion data, both A_2 and B_2 are (Z/2)^4; since both kernels have size four, both maps on 2-primary parts have rank two and cokernel (Z/2)^2. In particular, the 2-primary information loss is genuine, but odd-order DLP recovery is exact conditional on correctly computing Phi/Psi.

## Required run and negative controls
- New standalone `ct342_primary_structure.cjs` only; no import of old Python/Node producers, no EKS/paid compute, no timed benchmark. Use exact mod103 elliptic Weierstrass arithmetic for both models, enumerate rational points and test group identities, individual point orders and primary populations.
- Assert each factor's claimed order, group exponent, 2-primary structure and 5-primary structure against independently computed point-order histograms. Check at least one nontrivial 23-primary and every 5-primary order, and that a predicted incorrect 5-primary group structure is rejected.
- The program must fail nonzero under deliberately wrong elliptic-coefficient, wrong point order and wrong primary-structure expectations. Do not use Node's disabled-assert fallback; throw explicit errors.
- Produce sorted canonical JSON with schema `ct342-primary-structure/v1`; run standard Node twice and compare outputs. Preserve all errors, no signed PCC or full Jac map verification from this self-check.
- Final report must state exact group structures, derivation of odd-order inverse and rank-two mod2 obstruction, commands/source hashes, limits and ownership. Keep original matched norm reuse pricing `C169/C13a/C13b/N2_actual/D` and cold rho/BSGS/IC reserved; do not duplicate native F3/E7 or legacy CT341 full-population work.

## Ownership and decision gate
One coordinator is authorized for this disjoint finite-group audit; no native subagent slots or Rust compiler were exposed on entry. Mathematics, Node producer and postrun review are coordinator roles, **not** independent worker attestations. If the bounded audit fails, log the negative and do not adjust a frozen result silently. No merge or CI rerun/waiver; PR #5045 stays draft while independent Rust/PCC gates remain open. [pcc:unverified]
