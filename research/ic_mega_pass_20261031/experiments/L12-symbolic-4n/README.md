# L12: symbolic form of arm A's 4n round-3 falls

**Answer.** The 4 K-valued relations behind L9's 4n falls are now explicit. Their coefficients are polynomials in z and c = a6 over F2
(see below). They come from a 4-twist window, and the certificate uses only F_0, F_1 and F_2 with multipliers at twist positions 0..3.
- **Proven:** these relations and all their Frobenius shifts are in W3, for every n >= 5, every V, and every z·c ≠ 0.
- **Proven in the generic model, not in the real system:** the 4n count, i.e. that the shifts are independent mod W2 and that
  nothing else falls in round 3. The bit-level dumps confirm it per instance at n = 31, 33 and 35.

No security claims. Toy scale, one generator (N1 `lib.system`, arm A). Classification: accounting/explanation of a measured
effect (no exponent change). novelty_unverified: no literature search done.

## Setup
- a_i = a^{2^i}, b_i = b^{2^i}, z_i = z^{2^i}, c_i = c^{2^i}.
- F_j = a_{j+1}b_{j+1} + z_j a_j b_j + z_{j+1}(a_{j+1}+b_{j+1}) + c_j, the Frobenius twists of A's
  F = (ab)^2 + z ab + z^2(a^2+b^2) + a6.
- Over K, macclose's rounds are:
  - W2 = M4 ∩ P≤3, where M4 = span{a^e b^f F_j : wt(e)+wt(f) ≤ 2};
  - W3 = (M4 + Var·W2) ∩ P≤3.
- **Generic Frobenius model Z:**
  - The a_i and b_i are independent.
  - Products add exponents (a_i a_i = a_{i+1}).
  - Bit-degree = binary weight.
  - Engine: `src/zmodel.py` plus `src/kech.c` (dense GF(2^m) echelon, m ≤ 63) for numerics, and `src/sym.py` (PARI, exact over F2(z,c)).

## The relations
Cleared of denominators (`analysis/relations_cleared.json`, F2 coefficients):

- **R1 = c·(RREF row with pivot a0a1b3)**:
  ```
  c z^3·a0a1b3 + (z^8+cz^4)·a1b0b3 + (z^8+cz^2+c^2)·a2b0b3 + (z^16+cz^12+cz^10+c^2z^8+c^2z^6+c^3z^4)·b0b1
  + (z^16+cz^8+c^4)·b0b2 + (z^12+z^10+c^2z^4)·b0b3 + (z^15+cz^9+c^2z^7)·a0b1 + cz^7·a0b2 + z^9·a0b3 + cz^11·a0a1
  + (z^16+cz^12)·a1b0 + (z^16+cz^10+c^2z^8)·a2b0 + (cz^14+c^2z^12+c^2z^10+c^2z^8+c^3z^6+c^4z^4)·b0 + c^2z^9·a0
  ```
- **R2 (pivot a0a2b3), times c**:
  ```
  c·a0a2b3 + z^5·a1b0b3 + (z^5+cz)·a2b0b3 + (z^13+c^2z^5+c^2z^3)·b0b1 + (z^13+cz^9+cz^7+(c^2+c)z^5+c^2z^3+c^3z)·b0b2
  + (z^9+z^7+cz^5)·b0b3 + (z^12+c^2z^4)·a0b1 + (cz^8+cz^4+c^2z^2+c^3)·a0b2 + (z^6+cz^4)·a0b3 + cz^8·a0a2 + z^13·a1b0
  + (z^13+cz^9)·a2b0 + (cz^11+c^2z^9+c^3z^5)·b0 + c^3z^4·a0
  ```
- **R3 (pivot a0a3b1), times z^7**:
  ```
  z^7·a0a3b1 + (z^8+cz^4)·a3b0b1 + (z^8+cz^2+c^2)·a3b0b2 + (z^16+cz^12)·b0b1 + (z^16+cz^10+c^2z^8)·b0b2 + z^15·a0b1
  + (cz^11+c^2z^5+c^3z^3)·a0a1 + cz^7·a0a2 + z^9·a0a3 + (z^16+cz^12+cz^10+c^2z^8+c^2z^6+c^3z^4)·a1b0
  + (z^16+cz^8+cz^6+c^4)·a2b0 + (z^12+z^10+z^8+c^2z^4)·a3b0 + (cz^14+c^2z^12+c^2z^10+c^3z^6+c^4z^4)·b0 + c^2z^9·a0
  ```
- **R4 (pivot a0a3b2), times z^3**:
  ```
  z^3·a0a3b2 + z^4·a3b0b1 + (z^4+c)·a3b0b2 + z^12·b0b1 + (z^12+cz^8)·b0b2 + z^11·a0b2 + (z^11+c^2z^3)·a0a1
  + (z^11+cz^5+c^2z^3)·a0a2 + z^7·a0a3 + (z^12+z^10+c^2z^4)·a1b0 + (z^12+cz^8+cz^6+(c^2+c)z^4+c^2z^2+c^3)·a2b0
  + (z^8+z^6+cz^4)·a3b0 + (cz^10+c^2z^8+c^3z^4)·b0 + (cz^9+c^2z^7)·a0
  ```

Every monomial contains a0 or b0. None contains a0·b0, and there is no constant term. The exact K-RREF over F2(z,c), which is L9's
normalization, is in `analysis/sym_rref.json`.

## Status of the 4n count
1. **Inclusion (rigorous).**
   - Send a_i ↦ a^{2^(i mod n)}. This is a ring homomorphism from the model to the real Boolean functions ⊗ K, and it does not raise
     degree.
   - It therefore maps model-M4 into M4⊗K and model-W2 into the real W2⊗K. Each R_i's certificate (10 rows of the form
     g·F_j or v·h, with h in W2; `analysis/sym_rref_cert.json`) maps to a real certificate.
   - The lcm of all certificate denominators is **c·z^7**. So for every n ≥ 5, every V, and every z·c ≠ 0, R_i ∈ W3⊗K.
     Applying Frobenius to the identity puts τ^j R_i in W3⊗K too, so all trace components of the R_i lie in W3.
2. **Count in the model (exact over F2(z,c)).**
   - With window {0..L-1}: dim(W3∩Win) − dim(W2∩Win) is 0, 4, 8, 12 for L = 3, 4, 5, 6.
   - W2 dims follow 4L^2 − 7L + 1: 16, 37, 66, 103, and numerically 148 and 201 for L = 7 and 8.
   - The new space is spanned exactly by the shifts τ^j R_i, j = 0..L−4, and they are independent. This was checked
     numerically for L ≤ 7 at n = 31, 33 and 35 (`analysis/p3_shifts.json`).
   - So in the model the round-3 quotient is free of rank 4 per shift position. That is 4n per orbit, if nothing interacts
     across the cycle.
3. **Gap (not proven).** For the real system it remains to show:
   - (a) the n·4 trace components of the R_i are independent modulo the real W2;
   - (b) no other round-3 falls exist.
   Both depend on the real W2, whose n(3N−5) plateau is itself unexplained (L13). The extra relations to worry about come from
   V's subspace polynomial (length k+1) and from the cyclic wrap-around.
   Per-instance verification on fresh macdump dumps (L9 engine unchanged; sha256 of macdump.c starts 400655e0):

   | n | s | dim W2 | dim W3 | new | span of R_i trace components mod W2 | each R_i: in W3 / in W2 |
   |---|---|---|---|---|---|---|
   | 31 | 2 | 2821 | 2945 | 124 | **124** | 31/31, 0/31 |
   | 33 | 0 | 3201 | 3333 | 132 | **132** | 33/33, 0/33 |
   | 35 | 0 | 3605 | 3745 | 140 | **140** | 35/35, 0/35 |

   Since span = new, W3 = W2 + span(Tr R_i) exactly at these three instances.
   The gap is real: at n = 29 the same relations are present (L9) but 82 further falls occur, and at n = 27 count-forced falls
   swamp them. So (b) needs n large relative to the degree-4 rank. This is not a pure genericity condition.

## Pre-registration scorecard (PROTOCOL.md)
- **P1 held.**
  - Window dims 37 and 41 already with padding p = 0.
  - The 4 relations are identical, coefficient for coefficient, to L9's `newrel_n31_A_s2` and `newrel_n31_A_s3`.
- **P2 held.** 37/41, and the symbolic formulas match the numeric model, on 15 draws at n = 29, 31, 33, 35 and 37
  (`analysis/sym_vs_numeric.json`).
  - Control: independent coefficients (z_j and c_j unlinked) give 39/73. The Frobenius coupling of the coefficients is essential.
  - c = 0 keeps 37/41 but changes the RREF.
- **P3 held.**
  - 4(L−3) numerically for L = 3..8, at padding p ≤ 2 for L ≤ 6 and p ≤ 1 for L = 7 and 8.
  - Exact over F2(z,c) for L ≤ 6 at p = 0.
  - Padding p = 1 exact over F2(z,c): NOT done. The run was stopped after about 25 min wall (budget). Padding invariance is numeric only (one (z,a6) at n=31, p <= 2).
- **P4 held, in a stronger form than predicted:** the certificates use only positions 0..3 and no carry slot.
- **P5 failed.** The RREF has denominators c and z^k. Cleared, the coefficients are polynomials in z and c but are not affine in
  the c_j (c^3 = c_0·c_1 occurs).

## Files
- `src/kech.c` is the GF(2^m) echelon engine.
- `src/zmodel.py` is the numeric model, `src/sym.py` the exact model (PARI; args: out lo hi wlo whi [cert]), and `src/evalsym.py`
  evaluates the symbolic relations at an instance.
- Checks: `src/cmpl9.py` and `src/chk_sym_vs_num.py` (against L9 and the numeric model), `src/p3.py` and `src/shifts.py` (the
  translation count), and `src/dumpchk.py` (bit-level, on the dumps).
- `src/run_dumps.sh` re-runs the dumps. They are deleted after use.
- `raw/macdump_*.json` holds the round logs, and `analysis/*` the outputs.
- CPU used: macdump about 10 CPU-min total; everything else under 10 min. At most 3 concurrent processes.
