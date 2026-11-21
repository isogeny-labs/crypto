# Prior art for the multiplicative-subgroup factor base and SP-04

Read 2026-11-15. Read from the papers' text; nothing re-derived.

## Petit, Kosters, Messeng, PKC 2016 (PKM)

"Algebraic approaches for the ECDLP over prime fields", LNCS 9615, pp. 3-18,
DOI 10.1007/978-3-662-49387-8_1; author PDF
christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf.

- Factor base {(x, y) in E(F_p) : L(x) = 0} with L a composition of low-degree maps.
- Section 3.3 (p - 1 with a large smooth factor): V is the subgroup G of F_p^* of smooth order prod p_i ~ p^(1/m), or a coset aG. L_j(x) = x^(p_j) and L_n'(x) = 1 - x^(p_n'). This is a multiplicative-subgroup (roots of unity) factor base, written as a tower of power maps. They note that NIST P-224 fits this setting.
- Section 3.4 (arbitrary p): V is the x-coordinates of a coset of a smooth-order subgroup H of an auxiliary curve E'/F_p. L is built from the Velu factors of the isogeny with kernel H.
- Section 4 experiments: m = 2 (P = P1 + P2), Magma. Table 4 (p - 1 subgroups) and Table 5 (isogeny kernel) behave alike. Degree of regularity 4-6 for n1 <= 11.

Consequences for this repository:

- The multiplicative-subgroup factor base tested in `sym_m4/` (arm B: X^n = 1) is not new; it is PKM section 3.3.
- Two things here are not in PKM: the symmetrized m = 4 formulation (e3^n = 1 plus Newton power-sum constraints) and the head-to-head with a dense factor base at matched size. The result was no advantage [pcc:unverified].
- An isogeny-defined factor base is PKM section 3.4. SP-04 differs: it pulls S3 back along an isogeny to the target curve, not to an unrelated auxiliary curve. It is still in the same family. The repository already cites the "PKM 2-isogeny tower" (`PROPOSAL.md`).

## Amadori, Pintore, Sala, Finite Fields Appl. 51 (2018) 168-182 (APS)

IACR ePrint 2017/609; revised PDF on iris.unitn.it.

- The factor base is built from random points R = uP + vQ until it has s elements, with s^m ~ q. One relation P1 + ... + Pm = O among them solves the DLP, so there is a single Groebner basis per attempt instead of about s. [pcc:unverified]
- V is the x-coordinates of those random points, encoded densely as f(z) = prod (z - v). It is partitioned into m disjoint sets f_i (after Galbraith-Gebregiyorgis), which deliberately breaks symmetry. There is no symmetrization, no multiplicative subgroup and no isogeny.
- For m = 4 and 5 they split S4/S5 through an auxiliary variable Y (S3(X1,X2,Y), S3(X3,X4,Y)).
- Experiments for p of 11 to 22 bits and m in {3, 4, 5}. They make no claim against Pollard rho ("more work is needed").

Consequences: APS is not prior art for the subgroup factor base. Its random-point factor base avoids the yield loss we measured, because every element of V is an x-coordinate. That loss (about half of mu_n is not x-coordinates) is the main reason arm B lost [pcc:unverified]. The split-variable S4 encoding is an untested alternative to our dense S4 and to symmetrization.
