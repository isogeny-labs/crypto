# L12 PROTOCOL — symbolic form of A's 4n round-3 falls (L9)
Written 2026-11-02 BEFORE any L12 computation. No security claims. Toy scale, one generator (N1 lib.system arm A).

## Setup (what is being modelled)
- Arm A: K=F_{2^n}, a,b in a k-dim F2-subspace V (k=(n+1)/2), F(a,b) = (ab)^2 + z ab + z^2 (a^2+b^2) + c (c=a6).
  Write a_i = a^{2^i}, b_i = b^{2^i} (K-valued F2-linear forms), z_i = z^{2^i}, c_i = c^{2^i},
  F_j = F^{2^j} = a_{j+1}b_{j+1} + z_j a_j b_j + z_{j+1}(a_{j+1}+b_{j+1}) + c_j.
  A K-monomial a^e b^f has bit-degree wt(e)+wt(f) (binary weight); products add exponents (a_i a_i = a_{i+1}).
- Closure (macclose, D=4, round-based), restated over K (tensoring with K is exact):
  W2 (x) K = M4 ∩ P<=3 with M4 = span_K{ a^e b^f F_j : wt(e)+wt(f) <= 2 };
  W3 (x) K = (M4 + Var·W2) ∩ P<=3 with Var = span_K{a_i, b_i}.
  (Derivation: round 1 = x·f, round 2 = x·y·f, round 3 = x·(round-2 falls).)
- Generic Frobenius model Z(L,p): all a_i,b_i (i in a finite integer range of length L+2p around the window) treated as
  independent (true in the real system for any <= k consecutive twists), no wrap-around, no subspace relations.
  Hypothesis H1 (locality): window-{0..3} parts of W2(x)K and W3(x)K are certified inside a bounded padded range.

## Pre-registered predictions
- P1 (model fidelity): with L9's own K, z, a6 (n=31 s2), the Z-model with some padding p<=3 gives
  dim_K(W2(x)K ∩ Win{0..3}) = 37 and dim_K(W3(x)K ∩ Win{0..3}) = 41, and the 4 new K-RREF relations (cubic-first order of
  L9 newrel.py) coincide coefficient-by-coefficient with analysis/newrel_n31_A_s2.json.
- P2 (V-independence / genericity): the same 37/41 for random z,c over the n=33 and n=35 fields and for a random
  unrelated field; window dims do not depend on n.
- P3 (translation count): for windows {0..L-1}, dim_K(W3∩Win) - dim_K(W2∩Win) = 4(L-3) for L=4..7
  (the new relations are exactly the shifts tau^j R_i; nothing else new locally). With H2 (no wrap-around
  interaction in the cyclic Z/n, n large) this gives 4n F2-dimensions.
- P4 (certificate shape): each R_i = sum_v v·h_v + m, h_v round-2 falls, m in M4, with all twist indices in {0..4}
  (window {0..3} plus one carry slot); i.e. a 'Frobenius window' of 5 positions certifies.
- P5 (symbolic): the R_i (suitable basis) have coefficients in F2(z) (denominators powers/products of z_i only)
  and are affine-linear in c_j. Exploratory; may fail.
- Numerical check: the symbolic R_i, evaluated with each instance's z,a6 and expanded to bits, have all trace
  components in W3 and none in W2 (L9 macdump dumps, re-run at n=31 s2, n=33 s0, n=35 s0), spanning exactly 4n mod W2.
- Kill: P1 failing means the generic model is not a faithful local model (report exact dims); P3 failing (other than 4(L-3))
  means the 4n count is not a pure translation count.

## Budget
<= 3 threads; single runs < 20 CPU-min; dumps under $TMPDIR, deleted after; outputs < 200 MB.
