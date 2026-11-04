# Round 7: twisted-Edwards node relations in the glued tree (arms E1, E1S, E2)

2026-11-04 · follow-up to [`../round5/README.md`](../round5/README.md) · curve search
[`curve.sing`](curve.sing) · generator [`gen7.sing`](gen7.sing) (native Singular,
planted check before every write) · instances [`instances7.txt`](instances7.txt) ·
input hashes [`SYSTEMS.sha256`](SYSTEMS.sha256) · logs [`logs/`](logs/) · records
[`rows.tsv`](rows.tsv)

**Standing:** every result line below is an unverified assertion [pcc:unverified].

## Prediction (written 2026-11-04T05:46:22Z, after generating the systems, before any msolve run)

Derived while writing the generator: eliminating u1, u2, u3 from the Edwards law gives
(v1²v2² − 1/d) · S3E, and S3E = d·v1²v2²v3² − d·(v1²v2² + v1²v3² + v2²v3²) + a·(v1²+v2²+v3²)
− 2(a−d)·v1v2v3 − a. It has degree 2 in each v_i (as the Weierstrass S3 has in each x_i)
and total degree 6 (Weierstrass: 4). In the cherry coordinates it becomes
S3Esym(s, q, X) of total degree 4, the same as the Weierstrass S3sym. At the root
(v_R fixed) S3E(X1, X2, v_R) also has total degree 4. So E1 has the same variables, the
same equation count and the same input degrees as T, cell by cell.

1. **E1 (v-only tree): D_E1 = D_T** in every cell: 6 at m = 3, t = 4, 5; 7 at m = 4, t = 4;
   m = 4, t = 5 not decided within 1,200 s. The node relation is still a degree-2-per-point
   2-to-2 correspondence, and THEORY.md's coupling argument does not depend on the model.
   E1 should be **wider** than T: the base v ∈ g·μ_(2^t) is closed under v → −v (adding the
   2-torsion point (0, −1)), so each planted relation has 4× (m = 3) and 8× (m = 4) more
   F̄_p-solutions. Predicted quotient dimensions 12 (m = 3) and 48 (m = 4), against T's 3 and 6.
2. **E1S (separators fixed): D = 4** in every cell, the same floor as S.
3. **E2 (full coordinates, per-leaf chains, node law cleared of denominators): D_E2 ≥ D_T + 1**
   (≥ 7 at m = 3, t = 4), with much wider matrices: N = 17/24 at t = 4 against T's 13/18,
   the node equations have total degree 5, and the leaves lose the cherry symmetry
   (like arm C, which had D 6 / ≥ 7 at m = 3). Quotient dimension 24 at m = 3 (3! orderings
   × 4 even 2-torsion shifts) if it finishes.
4. Overall: no Edwards arm lowers D below the Weierstrass T reference. The question is
   answered "no" unless E1 or E2 has D < D_T in some cell.
