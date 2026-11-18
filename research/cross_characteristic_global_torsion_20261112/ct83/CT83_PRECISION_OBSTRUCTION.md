# CT83 — Exact finite-precision obstruction for local-to-global seven-torsion recognition

**Scientific status:** [pcc:unverified]. A precise information/height gate for the fixed CT80 x-coordinate field; **not** an impossibility result for selective torsion recognition, and **not** an ECDLP complexity bound.

## Setup and explicit counterexample

Let `F(X)=psi_7(X)` for the Cremona 11a1 elliptic curve (CT80). Its degree is `d=24`, its leading coefficient is 7 and its largest absolute coefficient is

`C = 2207441511783053`.

CT80 proves `F` irreducible over Q. Let `L=Q(alpha)` be the degree-24 x-coordinate field, and fix CT80's source place `v|37`, reducing `alpha` to 9. At the binary place `w|2`, reduce `alpha` to a root of

`f12(X)=X^12+X^9+X^5+X^2+1`

in `F_(2^12)`.

**Explicit arithmetic obstruction:** Define `h=(alpha-9)^5`. Then

`red_v(h) == red_v(0) == 0 (mod 37^5)`,

but `red_w(h)=(alpha+1)^5 != 0`. The binary residue of `h` is **51** in CT80's 12-bit polynomial-basis encoding (alpha encoded as 2). The coefficients of `(X-9)^5`, in ascending order, are

`[-59049,32805,-7290,810,-45,1]`.

The global element `h` is nonzero because the degree-5 polynomial is not divisible by the degree-24 irreducible `F`. Its binary residue is nonzero because `f12(1)=1`, so `alpha+1` is a unit in the binary residue field.

Therefore **five 37-adic digits of an arbitrary power-basis global element cannot determine its residue at the binary prime**. This is an explicit, bounded-coefficient failure for a naive local-to-other-prime numeric transfer. It does **not** produce a second seven-torsion root and does not contradict CT82's unique global torsion preimage theorem. Only the power-basis element recognition layer is tested.

## Necessary precision for a uniform bounded box

At the fixed degree-one source place, the reduction of a bounded power-basis element

`c_0+c_1 alpha+...+c_23 alpha^23`, with each `c_i in [-H,H]∩Z`,

has at most `37^N` values modulo `37^N`, whereas the coefficient box contains exactly `(2H+1)^24` distinct global elements (since deg F=24). Consequently, **injectivity across the entire box requires**

```
37^N >= (2H+1)^24 .
```

For `H=59049`, this requires **at least N=78 digits**. This is only a necessary *information-capacity* condition for universal coefficient recovery, not a lower bound on extracting a single torsion root (or on arbitrary compressed/structured representations).

## Constructive sufficient, deliberately conservative precision

There is also an exact sufficient condition once an actual height cap is supplied. For two bounded vectors differing by a nonzero polynomial `g` of degree at most 23 with coefficient bound `|g_i|<=2H`, the integral resultant `Res(F,g)` is nonzero. By the Sylvester determinant and Hadamard's inequality (row 2-norm no greater than row 1-norm),

```
|Res(F,g)| <= ((d+1) C)^(d-1) * (2 d H)^d .
```

One may pad `g` to formal degree `d-1`; the Sylvester determinant then differs from the ordinary resultant by a power of the leading coefficient 7, and the same upper bound applies. If the two global elements agree modulo `37^N` at the fixed place, then the Bezout resultant identity, evaluated at the Hensel root of `F`, implies `37^N | Res(F,g)`. Since 37 does not divide the leading coefficient 7, the same divisibility holds for the padded determinant. If `37^N` exceeds the displayed upper bound, this is impossible for nonzero `g`. Thus a height cap + enough source digits gives **unique** power-basis reconstruction.

For the actual CT80 `d=24`, `C` above and `H=59049`, the conservative certificate requires **N=345** digits. The gap between necessary 78 and sufficient 345 is from loose resultant bounds, not an algorithmic runtime estimate. It should not be construed as a proof that 345 digits are practically required.

## Independent executable evidence

Two standalone verifiers reconstruct `psi_7` independently, check `v_37(F(9))=1`, build the source-local collision polynomial, compute the binary residue using separately coded GF(2^12) polynomial arithmetic, and use exact arbitrary-precision integer inequalities for both precision thresholds.

- Python, Python `-O`, Node, and Node `--no-opt` produce byte-identical canonical JSON.
- SHA-256: `7625287b302b557c7a444ca6a632f92800c17a2d5864b505100f21041cb9ffd5`.
- Exact JSON: `{"binary_witness":51,"coefficient_height":59049,"degree":24,"experiment":"ct83-fixed-prime-precision-obstruction","f9_valuation":1,"information_required_digits":78,"local_collision_digits":5,"local_modulus":69343957,"p":37,"poly_height":2207441511783053,"resultant_sufficient_digits":345}`.

Reproduce from repository root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct83
python3 -B "$D/ct83_precision_gate.py" > /tmp/ct83-py.json
python3 -O -B "$D/ct83_precision_gate.py" > /tmp/ct83-pyopt.json
node "$D/ct83_precision_independent.js" > /tmp/ct83-js.json
node --no-opt "$D/ct83_precision_independent.js" > /tmp/ct83-jsopt.json
cmp /tmp/ct83-py.json /tmp/ct83-pyopt.json
cmp /tmp/ct83-py.json /tmp/ct83-js.json
cmp /tmp/ct83-py.json /tmp/ct83-jsopt.json
printf '%s  %s\n' 7625287b302b557c7a444ca6a632f92800c17a2d5864b505100f21041cb9ffd5 /tmp/ct83-py.json | sha256sum -c
```

## Research decision and next acceptance gate

Continue with input-conditioned **single-torsion-root** global recognition. Require a specified basis and coefficient-height bound, then reconstruct a single root from its unique p-adic lift using a certified integral-relation/LLL or number-field factor-and-embedding algorithm, verify the exact global algebraic torsion equation (not merely local digits), choose the same source and target primes for both encoded inputs, and charge the complete number-field arithmetic and target logarithm costs. If no such bounded-height reconstruction is found, keep CT83 as a conditional precision obstruction and do not promote it to a DLP bound.

**Prior art:** CT80 (global `psi_7` field), CT81 (fixed-place orientation), CT82 (unique global torsion preimage) and standard Hensel/root-lifting, norm/resultant and lattice-reconstruction methods; see Sage's [number-field documentation](https://www.sagemath.org/doc/reference/number_fields/sage/rings/number_field/number_field.html), [p-adic polynomial documentation](https://doc-gitlab.sagemath.org/html/en/reference/polynomial_rings/sage/rings/polynomial/padics/polynomial_padic.html) and MCLF [explicit p-adic embeddings](https://github.com/MCLF/mclf/blob/master/mclf/padic_extensions/fake_padic_embeddings.py). The tools' shared research-KB server and its checkout wrapper are absent in this runtime; exact KB receipt, Lean binding and broader literature audit remain outstanding. No novelty claim on classical methods; CT83 is an instance-specific certificate. No paid compute, benchmark claim, or protected merge.
