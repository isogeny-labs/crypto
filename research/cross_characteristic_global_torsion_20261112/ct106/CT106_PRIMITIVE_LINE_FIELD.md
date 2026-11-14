# CT106 — A single primitive generator of the eleven-torsion line field

**2026-11-14. Scientific standing: [pcc:unverified].** Exact finite-field computations and a conditional-on-CT102/CT103/CT105 mathematical argument. No new ECDLP solver or measured speedup.

## Theorem: a concrete primitive global invariant

Fix the 11a1 curve `E: y²+y=x³−x²−10x−20`. Let `T=(alpha,Y)` be CT102's oriented nonzero eleven-torsion point, `K=Q(T)` (degree 120) and `M=K^Aut_Q(K)` (degree 12 by CT105). Define

```
s(T) = x(T)+x([2]T)+x([3]T)+x([4]T)+x([5]T).
```

This is invariant under all `(Z/11Z)^×`: the units permute the five pairs `{±1},...,{±5}`, and x([-m]T)=x([m]T). Thus s∈M and `[Q(s):Q]≤12`.

**CT106 proves equality unconditionally for the fixed number field:**

```
Q(s)=M,     [Q(s):Q]=12,
L=Q(x(T)),  [L:M]=5,  Gal(L/M)≅C5,
[K:L]=2,    Gal(K/M)≅C10.
```

The degree-12 proof is an explicit modular certificate, not an uncomputed norm assertion:

1. Reconstruct the classical degree-60 `psi11` by the CT101 recurrence. Over F13 it is irreducible: `X^(13^60)=X (mod psi11)` and `gcd(psi11,X^(13^d)-X)=1` for `d=12,20,30` (Rabin test). The monic scaling by 11 is valid at 13.
2. Compute `x([m]T)=phi_m(alpha)/psi_m(alpha)^2` in `A=F13[X]/(psi11)` for m=1,..,5, using `phi_m=X psi_m²−psi_(m+1)psi_(m−1)`. All denominators are invertible in A, checked by actual inverses. Hence the invariant s has well-defined reduction in F_(13^60).
3. The Frobenius orbit of that residue has *exactly* 12 members: `s^(13^12)=s`, while `s^(13^d)≠s` at every `1≤d<12`. The polynomial of its conjugates is

```
q13(Z) = Z^12+6Z^11+12Z^10+2Z^9+6Z^8+8Z^7
         +4Z^6+3Z^5+Z^3+11Z+2  ∈ F13[Z].
```

This degree-12 polynomial is irreducible over F13 by the verified orbit. Because all the x-coordinates are prime-to-13 torsion coordinates at good reduction, s is integral at the chosen prime above 13. Any characteristic-zero polynomial annihilating s reduces to a polynomial vanishing on the residue, so `[Q(s):Q]≥12`. Together with invariance this proves `Q(s)=M`. **The displayed q13 is a modular polynomial; it is NOT a characteristic-zero defining polynomial of M.**

L/M is the cyclic degree-five subextension of K/M fixed by negation; the polynomial

```
H_T(Z)=prod_(m=1..5) (Z−x([m]T)) ∈ M[Z]
```

has degree five, is irreducible over M and defines L/M. Its coefficients are symmetric invariants; no numerical scalar from a *public input* is used in constructing the invariants.

## Constructive residues and exact orientation-loss control

At CT103's fixed source place (X,Y)=(19,22) over F59, the entire cyclic subgroup has five distinct x-roots 19,20,22,43,46. Direct polynomial multiplication gives

```
H_59(Z) = Z^5 +27Z^4+31Z^3+45Z^2+19Z+27.
s mod 59 = 32.
```

Every one of the **ten** oriented nonzero rational source points produces exactly the same H_59. Thus **M and H cannot identify the oriented source scalar**; passing to M has deliberately forgotten the C10 torsor coordinate. This is a rigorous obstruction to an M-only recognizer, not a lower bound for all global lifting algorithms.

At CT103's fixed binary residue `t=X mod g`, `g=0x130027`, CT101's tower realizes `F_(2^20)` with x-coordinates of the same five cyclic subgroup points. In its polynomial basis:

```
H_2(Z) = Z^5 + 0x10116 Z^4 + Z^3 + 0x10117 Z^2
        + 0x1358 Z + 0x73383.
s mod 2 = 0x10116.
```

All H_2 coefficients are fixed by Frobenius^4; at least one has minimal degree four. In fact `0x10116` satisfies **`s^4+s+1=0`** over F2, and has degree four. The five roots form a single orbit under `x↦x^16` of length five. Hence H_2 is irreducible over the actual F16 residue of M and the binary x-extension has relative degree five. Orientation still requires the quadratic extension F_(2^40), in agreement with CT103/105.

## Independent implementation and reproduction

- `ct106_primitive_line_field.py`: independent integer division-polynomial recurrence, modular-13 Fermat inversion, Rabin irreducibility, degree-12 Frobenius/characteristic polynomial, source subgroup checks, and binary quintic arithmetic.
- `ct106_primitive_line_field_independent.js`: independently reduces the recurrence modulo13 from the start, uses polynomial extended Euclid for denominator inversion, and separately implements binary and source arithmetic.
- Both programs check the exact q13 coefficients, source H coefficients, binary H coefficients, F16 minimal polynomial and orbit length five. The producer's `check` guards remain active under `python -O`.

From repository root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct106
python3 -B "$D/ct106_primitive_line_field.py" >/tmp/ct106-py.json
python3 -O -B "$D/ct106_primitive_line_field.py" >/tmp/ct106-pyopt.json
node "$D/ct106_primitive_line_field_independent.js" >/tmp/ct106-js.json
node --no-opt "$D/ct106_primitive_line_field_independent.js" >/tmp/ct106-jsopt.json
cmp /tmp/ct106-py.json /tmp/ct106-pyopt.json
cmp /tmp/ct106-py.json /tmp/ct106-js.json
cmp /tmp/ct106-py.json /tmp/ct106-jsopt.json
printf '%s  %s\n' e2cac923b8426e70a4a62162e7d7d1cdf5b3f9807e405034288e5969d36fb882 /tmp/ct106-py.json | sha256sum -c
```

The four local runs matched byte-for-byte (digest above); exact-head Actions and independent PCC scientific certification are separate gates.

## Missing work and falsification controls

There is **not** yet a characteristic-zero degree-12 minimal polynomial with evaluated coefficient heights, a relative quintic written over that power basis, or a local-to-global root selection into the *fixed* degree-120 field that then specializes at the fixed target prime. The source F59 roots themselves can be distinguished by public x, but choosing the corresponding *global* root without enumerating known scalar multiplications remains the hard online gate. A subgroup polynomial without orientation is not a DLP-preserving evaluation algorithm.

Next: compute exact `minpoly_Q(s)` and `H_T∈Q(s)[Z]`, certify integrality/denominator units at both primes, then solve the quintic root-labelling problem from Hensel coordinates using exact global recognition without scalar-indexed lookup. Price the cold degree-12 model, degree-five factorization/root recognition, degree-two orientation, target F2^40 group ops, target DLP, and reference rho/IC per CT104 protocol before any performance claim.

Prior work: CT101–105 in this repository, [Magma division-polynomial documentation](https://docs.magma-maths.org/ArithmeticGeometry/EllipticCurves/function_field.html) and [Sage number-field torsion documentation](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_number_field.html). Research-KB MCP/checkout wrapper unavailable in this runtime. No novelty claim. No paid compute, protected merge, or benchmark promotion.
