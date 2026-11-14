# CT119 — Invariant-field-only root-selection obstruction

**Date:** 2026-11-14. **Standing:** [pcc:unverified]. Fixed-example mathematical obstruction and independent finite-field control, not a DLP speedup or general complexity lower bound.

## Exact scope and theorem

Use the CT105/CT112 established tower for the 11a1 curve with an eleven-torsion point T:

- `M=Q(S) ⊂ L=Q(x(T)) ⊂ K=Q(T)`, of degrees `12,60,120` over Q;
- `S=Σ_(m=1)^5 x([m]T)`;
- `H(Z)=∏_(m=1)^5 (Z-x([m]T))` is irreducible of degree five over M. Its five roots are permuted transitively by `Gal(L/M)=C5`.

These are inherited premises with their certificates in CT105, CT112 and CT113; CT119 does **not** reprove the global number-field tower. [pcc:unverified]

**Invariant-only root-exclusion theorem.** Given an encoded public source point `Q=(r,y) ∈ E(F59)[11]`, identify `r,y` with their canonical integer representatives. Any defined rational expression in `S,r,y` with rational coefficients, evaluated exclusively using field arithmetic in `M`, lies in `M`. It cannot equal a requested `x([m]T)`, because each such x-coordinate has degree five over M. The same conclusion holds for a deterministic algorithm using only arithmetic and branch decisions on values in M, provided it never introduces a non-invariant element of L.

**Binary specialization.** CT110–113's three binary primes of M each have residue field `F16`; every integral element of M reduces into F16 there. The corresponding relative quintic is irreducible of degree five over F16 at **each** prime. Consequently no target root is the reduction of an M-element: target x requires an extension of residue degree five, namely `F_(2^20)`. The oriented ordinate requires `F_(2^40)` per CT115.

**Proof:** Field closure proves the first assertion. Irreducibility of H over M rules out all five roots lying in M. The integral local ring of M maps to its residue field F16; the irreducible quintic has no root in F16. No runtime/cryptographic lower bound follows from these field degrees. [pcc:unverified]

**Important exception:** The full source-place homomorphism `rho59:L→F59`, actual root expressions in L, or an algorithm that selects a factor/root in a larger field supplies *non-invariant data*. With those, the stated theorem does not apply. An unordered quintic's coefficients cannot by themselves pick a labelled root, but the source place can be used to break C5 symmetry if its construction/selection is exposed and paid for. This is not a general impossibility theorem for cross-characteristic computational transfers.

## Independent, fixed-place arithmetic checks

At source 59, the relative quintic has ascending coefficient vector `[27,19,45,31,27,1]`, and **five distinct roots** `[19,20,43,46,22]` in F59.

At the three binary primes, the corresponding monic quintics over `F16=F2[s]/(s^4+s+1)` have ascending vectors:

| Prime | Coefficients |
|---|---|
| U=1 (fixed CT103 prime) | `[9,6,3,1,2,1]` |
| U=s²+s | `[4,10,15,7,2,1]` |
| U=(s²+s)² | `[15,15,13,6,2,1]` |

**Two different irreducibility algorithms** are provided. Python checks Rabin's criterion `gcd(H,Z^16−Z)=1` and `Z^(16^5)=Z mod H`. Independently written JavaScript exhaustively excludes every F16 linear factor and all 256 monic quadratic factors; any reducible degree-five polynomial has a factor of degree ≤2. Both reject a deliberately reducible constant-term mutation.

At the fixed binary prime `0x130027`, the frozen five target x-coordinate field elements are bitmasks `[2,17,4956,65536,4697]` in `F_(2^20)`, not integer residues. Multiplying their linear factors gives ascending binary coefficient masks `[471939,4952,65815,1,65814,1]`. Their sum `s` is bitmask 65814, satisfies `s^4+s+1=0`, and embedding the first F16 quintic via that s reproduces the same product. Frobenius-16 starting at field element 2 cycles exactly through `[2,65536,4697,17,4956]`, a period-five orbit.

All four developer-local Python/Python -O/Node/Node --no-opt executions produced the same JSON, SHA256 `9fc71f62a723e00f8fb2ce21987e6b121174e5fd098d26ff59486c356308e961`. These are arithmetic controls using CT115–118 frozen fixture roots; no online arbitrary-input DLP construction or full cost measurement is claimed. [pcc:unverified]

```bash
D=research/cross_characteristic_global_torsion_20261112/ct119
python3 -B "$D/ct119_invariant_gate.py" >/tmp/ct119-py.json
python3 -O -B "$D/ct119_invariant_gate.py" >/tmp/ct119-pyo.json
node "$D/ct119_invariant_gate_independent.js" >/tmp/ct119-js.json
node --no-opt "$D/ct119_invariant_gate_independent.js" >/tmp/ct119-jso.json
cmp /tmp/ct119-py.json /tmp/ct119-pyo.json
cmp /tmp/ct119-py.json /tmp/ct119-js.json
cmp /tmp/ct119-py.json /tmp/ct119-jso.json
printf '%s  %s\n' 9fc71f62a723e00f8fb2ce21987e6b121174e5fd098d26ff59486c356308e961 /tmp/ct119-py.json | sha256sum -c
```

## Sources, concurrency, open gate

Reviewed root [#3877](https://github.com/isogeny-labs/crypto/pull/3877), CT105/110–113/115–118, [cryptanalysis #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523) and [crypto-autoresearcher #3282](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3282). Related classical background: [Masser–Wüstholz, *Galois Properties of Division Fields of Elliptic Curves* (1993)](https://doi.org/10.1112/blms/25.3.247) and [Sage division-field implementation](https://github.com/sagemath/sage/blob/develop/src/sage/schemes/elliptic_curves/ell_field.py). This is an application of standard fixed-field theory to the CT tower, not a novelty claim.

**KB unavailable:** neither the shared research-KB tool nor repository wrapper was accessible here. Conductor, project-wide `cargo test --release --lib`, independent signed PCC and matched cold rho/index-calculus timings were also unavailable/not run; these gates remain open. No EKS or paid resources launched.

**Next nonduplicate experiment:** construct a genuinely source-prime-labelled **non-invariant** element/root in L from a public input Q without computing or precompiling its projective discrete logarithm. Measure field construction, factorization, source embedding, root selection, binary DLP and sign resolution against matched optimized source rho and existing cold IC. A search that enumerates all five roots then matches source residues is an O(5) projective enumeration control, not a demonstrated advantage. Cantor/Jacobian/cover work remains with the concurrent owners.
