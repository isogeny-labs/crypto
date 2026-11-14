# CT90 — Two explicit binary CRT projections and selection of CT80's fixed target prime

**[pcc:unverified].** This is an exact fixed-instance computation in the CT86–89 11a1 order-seven subgroup field. It repairs the binary residue-place ambiguity and gives a usable locally integral reduction of the CT87 relative-cubic coefficients. It **does not** reconstruct an arbitrary oriented global torsion point or solve the source ECDLP.

## Precise statement

Let F=Q(s) have CT86's degree-eight polynomial
`g(s)=s^8−8s^7−840s^6−29862s^5−39690s^4−1696464s^3−6423347s^2+3828478s−2102915`.
In the 2-maximal Z-order B with basis b0,…,b7 from CT89, the two nontrivial idempotents in B/2B are e0=b4+b5+b6+b7 (mask **240**) and e1=1+e0 (mask **241**). The certified algebra is B/2B≅F16×F16, with residue-generator polynomial f(U)=U^4+U+1.

For e=e0,e1, define an explicit ring homomorphism ρ_e into F2[U]/f(U) by first multiplying the input by e and expanding uniquely in the F2-basis **e, e·s, e·s², e·s³**. Send e·s to U. Both maps are surjective; together they give a CRT isomorphism.

The computed images of the integral basis b0,…,b7 are (four-bit coefficients of powers 1,U,U²,U³):

| basis element | ρ_240 | ρ_241 |
|---|---:|---:|
| b0 | 1 | 1 |
| b1=s | 2 | 2 |
| b2=s² | 4 | 4 |
| b3=s³ | 8 | 8 |
| b4 | 6 | 5 |
| b5 | 12 | 10 |
| b6 | 11 | 7 |
| b7 | 0 | 8 |

In particular **both maps send s to the same U**. Therefore CT86's target value s=1545 in its polynomial-basis model F_(2^12) is not sufficient to select one of the two primes over 2. This is a specific non-injectivity of the *nonmaximal order's* primitive-element reduction, not a failure of the number field or a general barrier to field transfers.

## Select the actual binary prime with CT87's integral symmetric coefficients

Recall the subgroup cubic
`Z³−sZ²+e₂ Z−e₃`, with rational-power-basis coefficients `e₂=C₂(s)/q₂`, `e₃=C₃(s)/q₃` exactly as recorded in CT87. The denominators have even factors, so **directly evaluating C_i(s)/q_i modulo 2 is invalid**.

Convert these rational elements to CT89's B-basis first, cancel 2-adic denominator factors, and *only then* reduce the integral-at-two coordinates modulo 2. Two independent implementations find:

| invariant | B/2B coefficient mask | ρ_240 | ρ_241 | target values after U→1545 in F_(2^12) |
|---|---:|---:|---:|---|
| e₂ | 4 | 4=U² | 4=U² | 1906, 1906 |
| e₃ | 254 | 15=1+U+U²+U³ | 14=U+U²+U³ | **1261, 1260** |

The independently checked CT86/CT80 target subgroup has **(s,e₂,e₃)=(1545,1906,1261)** in F_(2^12). Hence, for this fixed choice U=1545, **e0 (mask 240)** is the compatible binary prime; selecting e1 gives e₃=1260 and fails the known target cubic. This is the negative control. The target orientation remains unselected: this only fixes the degree-four subgroup-field prime, not the relative-cubic root, ordinate, source-to-global point recognition or DLP scalar.

The invariant e₂ and e₃ are integral at 2 because their B coordinates have odd denominators after exact cancellation; this is independently checked, rather than inferred from a formally invalid division in characteristic two. The Python verifier uses exact Fraction Gauss–Jordan change-of-basis and polynomial reduction modulo g; the independently authored JavaScript uses BigInt reduction with a separately derived triangular basis inverse and numerator valuation tests. Both independently rebuild all 36 basis products, the two maps, and the CT87 coefficient residues.

## Exhaustive algebra certificate and local replay

Each verifier reconstructs the 8-dimensional B/2B multiplication and checks **all 65,536 ordered pairs** (a,b) for multiplication preservation by *both* maps; addition preservation is checked over the same 256² ordered pairs. Both check complementary idempotents, every image of the eight basis elements and the target positive/negative selection.

Run from repository root:

```sh
D=research/cross_characteristic_global_torsion_20261112/ct90
python3 -B "$D/ct90_binary_projections.py" > /tmp/ct90-py.json
python3 -O -B "$D/ct90_binary_projections.py" > /tmp/ct90-opt.json
node "$D/ct90_binary_projections_independent.js" > /tmp/ct90-js.json
node --no-opt "$D/ct90_binary_projections_independent.js" > /tmp/ct90-jsopt.json
cmp /tmp/ct90-py.json /tmp/ct90-opt.json
cmp /tmp/ct90-py.json /tmp/ct90-js.json
cmp /tmp/ct90-py.json /tmp/ct90-jsopt.json
printf '%s  %s\n' ab40a0b47b997abb6e6cbf5f6854a6eb10bdcbf15da2776a0850c798f87f6417 /tmp/ct90-py.json | sha256sum -c
```

Both implementations were executed locally with normal/alternative optimization modes, and their complete JSON outputs matched byte-for-byte with the pinned SHA-256 shown above. CI exact-head and independent scientific PCC are additional gates; **not claimed complete until independently inspected**. No paid EKS job was necessary for this bounded exact-integer certificate.

## Limits, next step and prior work

- CT90 constructs **subgroup-field** binary maps only. Global oriented point-field extension still has relative degrees three and two. Do not confuse target-place selection using an already known CT80 witness with arbitrary-point recognition from public encoded source Q.
- Next: lift CT87's integral cubic to the chosen degree-four target residue, factor all three roots in the compatible F_(2^12) field, recover the ordinate root in F_(2^24), and check the fixed-source/fixed-target orientation and all seven public toy cases. Then implement **single-root** point recognition at the fixed source place without scalar enumeration, with a certified root-height bound. Charge full cold costs before any speedup claim.
- Internal references: CT80, CT86–89, [crypto root PR #3877](https://github.com/isogeny-labs/crypto/pull/3877), [stacked PR #4074](https://github.com/isogeny-labs/crypto/pull/4074), [cryptanalysis PR #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523).
- Classical number-field prior art: [Sage number-field residue fields](https://doc.sagemath.org/html/en/reference/finite_rings/sage/rings/finite_rings/residue_field.html) and [Sage orders](https://doc.sagemath.org/html/en/reference/number_fields/sage/rings/number_field/order.html) for exactly the warning that a primitive-element order may not model the maximal-order residue algebra. No method novelty claim. The shared research-KB MCP and repository-local wrapper are unavailable here; survey incomplete. [pcc:unverified].
