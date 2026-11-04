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

## Construction

- **Curve** [pcc:unverified]. `curve.sing` fixes a = 1 and tries nonsquare d = 2, 3, …. It
  counts points on the birational Montgomery form B·y² = x³ + A·x² + x (A = 2(a+d)/(a−d),
  B = 4/(a−d)) with a table of squares and finite differences. d = 5 and d = 10 give #E/4
  composite. **d = 11 gives #E = 787,492 = 4 · 196,873, with 196,873 prime.** Since a is a
  square and d is not, the addition law is complete over F_p. `gen7.sing` checks the order
  independently: Q₀ = (−240076, 2) has 4Q₀ ≠ O and 196,873 · 4Q₀ = O, and 4 · 196,873 is the
  only multiple of 196,873 in the Hasse interval.
- **Base** v = g·w, w ∈ μ_(2^t), g = 2. Points with u² = (1−v²)/(1−11v²) a nonzero square:
  14 of 16 at t = 4 and 24 of 32 at t = 5. Each cell plants m points with pairwise distinct v²
  (so v_i ≠ ±v_j) and random u-signs, using an LCG with seed 0xED7A1104. R is the sum.
  Records are in `instances7.txt`.
- **S3E** comes from `eliminate` of u1, u2, u3 over the two curve equations and the cleared
  addition law. The eliminant factors as (v1²v2² − 1/d) · S3E. The first factor is the
  denominator locus, which has no F_p points because d is a nonsquare. S3E equals the closed
  form given under Prediction (checked in Singular).
  S3Esym(s, q, X) = d(q² − s²)X² + 2d·qX² + X² + s² − d·q² − 2(a−d)·qX − 2q − 1, where s
  enters only as s². Both have total degree 4.
- **E1:** the same variables and remainder towers as arm T, with S3 replaced by S3E.
  **E1S:** E1 plus X_c fixed to the planted values. **E2:** leaves (u_i, w_i), each with its
  curve equation (degree 4) and a per-leaf Kummer chain. Each node (U_c, V_c) gets
  U(1 + d·u u′v v′) = u v′ + u′v and V(1 − d·u u′v v′) = v v′ − a·u u′ (degree 5). At the
  root the node equals R = (u_R, v_R).
- Every system passed the planted check in Singular before it was written (`logs/gen7.out`).
- **Engine:** msolve 0.6.5, `-g 2 -t 1 -v 2`, `ulimit -v 2097152`, `timeout 1200`. Runs went
  one at a time (`../run.sh DIR 1`) on a 4-core host that was shared with another agent's
  2-thread msolve. **Same-host T references:** `../gen.sing` regenerated round 1's T
  systems, and they were rerun under the same caps right after (`logs/wref/`).

## Result [pcc:unverified]

| arm | m | t | N | input deg | D (≥: capped) | widest | sum rows·cols | wall s | qdim | exit |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|---|
| E1  | 3 | 4 | 13 | 4 | **6** | 6,971 | 1.63e8 | 1.8 | 12 | 0 |
| T (ref, same host) | 3 | 4 | 13 | 4 | 6 | 7,077 | 1.51e8 | 3.7 | 3 | 0 |
| E2  | 3 | 4 | 17 | 5 | **6** | 60,804 | 6.98e9 | 36.2 | 24 | 0 |
| E1S | 3 | 4 | 13 | 4 | 4 | 149 | 7.2e4 | 0.04 | 2 | 0 |
| E1  | 3 | 5 | 16 | 4 | **6** | 24,792 | 2.95e9 | 120 | 12 | 0 |
| T (ref, same host) | 3 | 5 | 16 | 4 | 6 | 23,720 | 3.15e9 | 587 | 3 | 0 |
| E2  | 3 | 5 | 20 | 5 | ≥ 6 | ≥ 61,087 | — | 102 | — | 139 (2 GiB) |
| E1S | 3 | 5 | 16 | 4 | 4 | 347 | 4.8e5 | 0.04 | 2 | 0 |
| E1  | 4 | 4 | 18 | 4 | **7** | 21,795 | 2.11e9 | **60** | 48 | 0 |
| T (ref, same host) | 4 | 4 | 18 | 4 | ≥ 7 | ≥ 25,742 | ≥ 1.53e9 | 634 | — | 139 (2 GiB) |
| T (round 1, 6 GiB) | 4 | 4 | 18 | 4 | 7 | 25,742 | 3.44e9 | 697 | 6 | 0 |
| E2  | 4 | 4 | 24 | 5 | ≥ 6 | ≥ 40,640 | — | 58 | — | 139 (2 GiB) |
| E1S | 4 | 4 | 18 | 4 | 4 | 297 | 2.9e5 | 0.01 | 4 | 0 |
| E1  | 4 | 5 | 22 | 4 | ≥ 7 | ≥ 91,279 | — | 61 | — | 139 (2 GiB) |
| E2  | 4 | 5 | 28 | 5 | ≥ 6 | ≥ 49,661 | — | 62 | — | 139 (2 GiB) |
| E1S | 4 | 5 | 22 | 4 | 4 | 693 | 1.9e6 | 0.06 | 4 | 0 |

Exit 139 means msolve failed to enlarge its hash table at the 2 GiB address-space cap, and the
run segfaulted. In those rows D and the width are lower bounds: the degree being reduced when
the run died, and the widest matrix reached so far. Wall times are evidence of ordering only.
The host was shared, though E1 and the T references ran under identical caps one after the other.

**Prediction check.**
1. D_E1 = D_T in every comparable cell: confirmed (6, 6, 7). The quotient dimensions 12 and
   48 are also confirmed.
2. D_E1S = 4: confirmed.
3. D_E2 ≥ D_T + 1: **falsified at m = 3, t = 4**, where D_E2 = 6 = D_T, though 8.6× wider and
   about 10× slower. The other E2 cells hit the memory cap.
4. No Edwards arm lowered D: confirmed.

## Reading [pcc:unverified]

- **Answer: no.** The Edwards node relation does not lower the F4 solving degree. In the
  v-only tree (E1), S3E has degree 2 per point and, in cherry coordinates, total degree 4,
  the same as Weierstrass S3sym. D is identical cell by cell: 6 at m = 3 (t = 4, 5) and 7 at
  m = 4, t = 4. Fixing the separators gives the same floor, D = 4. So the excess degree is
  still caused entirely by the 2-to-2 coupling, as rounds 2–5 found for x-only Weierstrass.
- **"Degree 2 in each point's coordinates" does not lower the degree either.** The full
  (u, v) tree (E2) has bidegree-(2,2) addition-law nodes, but they cost an extra variable
  per leaf and total degree 5. It reaches the same D = 6 at m = 3, t = 4, with 8.6× wider
  matrices and 10× the time. It cannot finish within 2 GiB at larger cells.
- **What Edwards does change is the constant.** E1's matrices are about as wide as T's, but
  sparser: 2.4–3.3 % against 4–6 % in the largest degree-6 rounds. S3E is even in each
  variable apart from the single v1v2v3 term. On the same host and caps, E1 is 4.9× faster
  than T at m = 3, t = 5 (120 s vs 587 s). At m = 4, t = 4, E1 finished in 60 s, while T ran
  out of 2 GiB at 634 s; the 6 GiB round-1 T run took 697 s, with 1.6× the summed matrix
  size. This holds even though E1 has 4× (m = 3) and 8× (m = 4) more F̄_p solutions, because
  the base is closed under adding the 2-torsion point (0, −1).
- **Status of that gain.** It is a measured constant-factor gain on the solver stage at fixed
  D. It is not a degree or slope improvement, and no pipeline is priced. As a candidate
  improvement it needs uncontended timing and a second target before it counts.
- **Next lever.** Quotient the 2-torsion action, which E1 does not do (FGHR-style
  invariants: v_i², v_i·v_j, or s² in each cherry, since S3Esym depends on s only through
  s²). This would remove the 4×/8× solution redundancy, and possibly some degree, at no cost
  in input degree.

## Deviations

- `.resume/RESUME.md` and the S3 checkpoint were not written, because this task allowed
  writes only under `round7_edwards/` and the scratch directory. The prediction above, with
  its UTC timestamp, is the pre-registration.
- No graph or PDF (research-visuals) yet. The task asked only for this README.
- Under the 2 GiB cap (a task constraint), msolve's hash table overflowed in 5 cells: E2 at
  m3t5, m4t4 and m4t5; E1 at m4t5; and the T reference at m4t4. These are lower bounds, not
  timeouts. No run reached 1,200 s.
- One target per cell. The E instances are not the Weierstrass instances (different curve),
  so the cells match in (m, t) and the targets are random, not identical.
