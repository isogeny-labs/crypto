# CT89 — Constructive 2-maximal order and explicit F16 × F16 residue algebra

**Status: [pcc:unverified].** An exact fixed-instance arithmetic implementation in the CT86–88 degree-eight projective seven-torsion field `F=Q(s)`, **not** a global-point recognizer or faster ECDLP algorithm.

## New result: explicit 2-maximal integral basis

CT88 established that `g(s)=0`, `deg g=8`, `v₂(disc(g))=16` and, using prime-to-two good-reduction unramifiedness, the 2-primary index of `Z[s]` in the maximal order is `2^8`.

Define the following eight rational-polynomial elements (the coefficient `1/4` is literal inside the number field, **not** reduction modulo 2):

```text
b0 = 1
b1 = s
b2 = s^2
b3 = s^3
b4 = (1+3s+s^4)/4
b5 = (s+3s^2+s^5)/4
b6 = (s^2+3s^3+s^6)/4
b7 = (1+3s+s^3+s^7)/4
```

Let `B=Z b0+⋯+Z b7`. The two independent exact verifiers check **all 36 unordered basis products** and express each product uniquely as an **integer** linear combination of these eight elements, using `g(s)=0`. Since `b0=1` and the lattice is closed under multiplication, `B` is an order of algebraic integers (not just a Q-vector-space basis).

The power-basis-to-B matrix has determinant `1/256`. Therefore

```text
disc(B) = disc(g) / 256^2
v₂(disc(B)) = 16−2·8 = 0.
```

Consequently `B` is **2-maximal**, i.e. it agrees with the full ring of integers after localization at 2. No claim is made that it is maximal at other primes.

## Explicit binary decomposition and field selection

Reduce the verified integer multiplication table modulo 2. Both independent verifiers calculate the Frobenius-square endomorphism `Fr:a↦a²` of the 8-dimensional F2-algebra `B/2B` and certify:

| Computed invariant | Exact value |
|---|---:|
| Rank of Fr | 8 / 8 |
| dim Fix(Fr) | 2 |
| dim Fix(Fr²) | 4 |
| dim Fix(Fr⁴) | 8 |
| Idempotent coefficient masks in the ordered basis `b0,…,b7` | `0, 1, 240, 241` |

The Frobenius is injective, so the finite algebra is reduced and therefore is a product of finite fields. These fixed-space dimensions, or equivalently the two nontrivial complementary idempotents, establish an explicit algebra isomorphism **up to choice of residue-field bases**:

```text
B/2B ≅ F_(2^4) × F_(2^4).
```

The two nontrivial idempotents can be represented by

```text
e  = b4+b5+b6+b7  (mod 2),  mask 240
1+e                        mask 241.
```

These cut out the two degree-four prime reductions of `F` above 2. **This repairs CT88's nonreduced power-basis quotient** `Z[s]/2=(F2[S]/(S^4+S+1)^2)`. Choosing one idempotent alone does *not* pick the fixed binary embedding of the full oriented point field K; further compatible relative-cubic and ordinate embeddings remain essential.

### Discovery and independent validation

An initial local SymPy-based 2-saturation search enlarged the power basis by the nilradical of the Frobenius action modulo 2. Its order-index trajectory was `1 → 2^4 → 2^8`, with successive Frobenius nilradical dimensions `4, 4, 0`. That search was used to *discover* the above basis, but neither committed certifier depends on SymPy, HNF or a pre-computed multiplication table.

- Python: independent Fraction-based Gauss–Jordan basis inverse and exact rational polynomial reduction.
- JavaScript: independent BigInt polynomial reduction and *explicit triangular* recovery of integral structure constants (no rational Gauss–Jordan).
- Both implement their own GF(2) rank/Frobenius calculation and enumerate all 256 residue-algebra vectors to identify idempotents.
- All four **local** modes (Python, Python -O, Node, Node --no-opt) agreed byte-for-byte at SHA-256 `c7ff3a7388db0d05797b758d707e0cf2bdbaa77c3a4a6d0eb55c1a4ebb3d8a1b`. Exact-head CI and scientific PCC are separate and must be inspected.

Reproduce from the repo root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct89
python3 -B "$D/ct89_basis.py" > /tmp/ct89-py.json
python3 -O -B "$D/ct89_basis.py" > /tmp/ct89-opt.json
node "$D/ct89_basis_independent.js" > /tmp/ct89-js.json
node --no-opt "$D/ct89_basis_independent.js" > /tmp/ct89-jsopt.json
cmp /tmp/ct89-py.json /tmp/ct89-opt.json
cmp /tmp/ct89-py.json /tmp/ct89-js.json
cmp /tmp/ct89-py.json /tmp/ct89-jsopt.json
printf '%s  %s\n' c7ff3a7388db0d05797b758d707e0cf2bdbaa77c3a4a6d0eb55c1a4ebb3d8a1b /tmp/ct89-py.json | sha256sum -c
```

## Research implications and next gate

An actual 2-integral arithmetic representation is now available for CT87's degree-eight subgroup field. It avoids the invalid operation of dividing CT87's rational relative-cubic coefficient polynomials by even denominators in F2. The next gate is to use the idempotents to implement **two explicit F16 residue projections** of the verified integral order, choose the one compatible with CT80's binary place, and rewrite CT87's relative cubic into a locally integral basis before selecting a single global oriented root of the public encoded Q at the fixed source place.

The complete cold field/ring construction, p-adic recognition, relative factorization and target DLP costs still need matched rho/IC timing and growing-order tests. **Neither the 2-maximal basis nor the residue algebra provides scalar orientation by itself.** [pcc:unverified]

Internal provenance: CT80–88; root [crypto #3877](https://github.com/isogeny-labs/crypto/pull/3877), stacked [#4074](https://github.com/isogeny-labs/crypto/pull/4074), cryptanalysis [#1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523). External methods consulted: [Sage residue fields](https://doc.sagemath.org/html/en/reference/finite_rings/sage/rings/finite_rings/residue_field.html), [PARI maximal orders](https://pari.math.u-bordeaux.fr/dochtml/html-stable/General_number_fields.html) and classical order discriminants. Shared research-KB MCP and checkout wrapper unavailable in this execution; no comprehensive novelty claim. No cloud work, protected merges, PCC waiver or performance claim.
