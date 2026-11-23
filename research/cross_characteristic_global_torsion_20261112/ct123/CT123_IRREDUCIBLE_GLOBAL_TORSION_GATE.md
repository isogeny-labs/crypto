# CT123 — Degree-60 global torsion persists after CT122 ordinary binary retargeting

**2026-11-14; scientific standing [pcc:unverified].** An exact polynomial/finite-field certificate for both integer CRT lifts in CT122. This is not a global torsion recognizer or a DLP-solving speedup.

## Preregistered question and sources checked

CT122 [#4179](https://github.com/isogeny-labs/crypto/pull/4179) keeps the exact source curve over F59 while reducing the maximal binary 11-torsion point field degree from 40 to 10 (and x-field degree from 20 to 10). Does the characteristic-zero torsion field also become smaller? Prior CT105, CT114 and CT117–119 show that local projectors, interpolation and invariant-only selectors do not supply a table-free global torsion root recognizer. Read root [#3877](https://github.com/isogeny-labs/crypto/pull/3877), newest `docs/session-logs/2026-11-14.md`, CT122, active [cryptanalysis #1523](https://github.com/isogeny-labs/cryptanalysis/pull/1523) and [autoresearcher #3282](https://github.com/isogeny-labs/crypto-autoresearcher/pull/3282); preserved sibling work. Standard division-polynomial recurrence cross-checked against [Sage's official general Weierstrass/division polynomial documentation](https://doc.sagemath.org/html/en/reference/arithmetic_curves/sage/schemes/elliptic_curves/ell_generic.html), including its explicit source-model psi4/psi2 example. **KB unavailable:** neither research-KB MCP nor repository checkout-local wrapper was available in this runtime; this fallback is not evidence of novelty or absence. Conductor/native checkout and EKS Spot credentials were likewise not available; these bounded local tests did not launch paid compute.

We use the actual integer Weierstrass curves `E: (0,-1,1,-10,-20)`, `E+: (59,-1,60,-10,39)` and `E-: (59,58,60,-10,39)` with tuple order `(a1,a2,a3,a4,a6)`. All reduce to the *same* source at 59, but have different binary reductions. Their 11-division polynomials have 61 integer coefficients, degree 60 and leading coefficient 11.

## Explicit certificate (independent integer and modular implementations)

Use the universal invariants `b2=a1²+4a2`, `b4=2a4+a1a3`, `b6=a3²+4a6`, `b8=a1²a6+4a2a6−a1a3a4+a2a3²−a4²`, `C=4X³+b2X²+2b4X+b6`. Write `u=psi3=3X⁴+b2X³+3b4X²+3b6X+b8` and `v=psi4/psi2=2X⁶+b2X⁵+5b4X⁴+10b6X³+10b8X²+(b2b8−b4b6)X+(b4b8−b6²)`. The standard recurrence is:

```text
a5  = v*C^2 - u^3
a6  = u*(a5-v^2)               # psi6/psi2
a7  = a5*u^3 - v^3*C^2
psi11 = a7*a5^3 - v*a6^3*C^2
```

Both programs calculate *all integer coefficients* and independently execute finite-field Frobenius powering and polynomial gcd. If `f=psi11`, put `g_p(k)=deg gcd(f mod p, X^(p^k)-X)`. Every tested reduction is squarefree by `gcd(f,f')=1`.

| Model | `g_2(1,2,5,10,20)` | `g_59(1,5)` | Irreducible witness |
|---|---|---|---|
| E | (0,0,0,0,60) | (5,60) | not attempted |
| E+ | (0,0,10,60,60) | (5,60) | mod **73** |
| E- | (0,0,10,60,60) | (5,60) | mod **29** |

Thus the complete squarefree binary factor signatures are **(20,20,20)** for the original curve and **(5,5,10,10,10,10,10)** for either ordinary lift. All three 59-reductions have **five linear factors and eleven quintics**, with source rational torsion x-roots `[19,20,22,43,46]`. This is consistent with CT122 and certifies actual division-polynomial fiber decomposition.

For E+ at p=73 and E- at p=29, the scripts establish the exact congruence `X^(p^60)=X mod f` and independently compute

```text
deg gcd(f, X^(p^12)-X) = 0
deg gcd(f, X^(p^20)-X) = 0
deg gcd(f, X^(p^30)-X) = 0
deg gcd(f, X^(p^60)-X) = 60
```

As the prime divisors of 60 are 2, 3 and 5, Rabin's irreducibility criterion proves each `f mod p` irreducible at the stated auxiliary prime. The leading coefficient is 11 (nonzero modulo 73 and 29), and the integer polynomials are primitive (coefficient gcd 1); Gauss's lemma therefore proves **both new 11-division polynomials irreducible over Q**. Hence every nonzero geometric 11-torsion point `T` on E+ or E- satisfies

```text
[Q(x(T)):Q] = 60.
```

This **rules out a smaller global x-field for these two particular integer lifts**, even though their binary residue x-degrees are 5 or 10. It does not supply an unconditional algorithmic complexity lower bound, and the degree of the oriented y-field has not been proved here.

Exact integer polynomial fingerprints, comma-separated high-to-low decimal coefficients: E `b7634bd6ed0d84bd2479e608e1f76a47a902e307c4ba1742032bcc0061094d9b` (largest 40 digits), E+ `9f5123f411082ef5d5a2ee85e51632e01b830dfe289462a36898e79bfe185891` (87 digits), E- `3ad73b5144eacd04d63bf6f5e1cff18f680515dc953a6513f22a2405ed6e3dcb` (91 digits).

## Reproducibility

Self-contained Python standard-library arithmetic and separately written Node.js BigInt arithmetic returned *byte-identical* canonical JSON in all four local normal/optimized modes, digest `c64b7d232db0749206f36c58efc0fe0e3a1c13bc1d1096835766581cec686854`. Development cross-check with SymPy 1.14 factorization matched the finite-field degree signatures and both irreducible witnesses, but CI does not depend on SymPy.

```bash
D=research/cross_characteristic_global_torsion_20261112/ct123
python3 -B "$D/ct123_division_certificate.py" >/tmp/ct123-py.json
python3 -O -B "$D/ct123_division_certificate.py" >/tmp/ct123-pyo.json
node "$D/ct123_independent.js" >/tmp/ct123-js.json
node --no-opt "$D/ct123_independent.js" >/tmp/ct123-jso.json
cmp /tmp/ct123-py.json /tmp/ct123-pyo.json
cmp /tmp/ct123-py.json /tmp/ct123-js.json
cmp /tmp/ct123-py.json /tmp/ct123-jso.json
printf '%s  %s\n' c64b7d232db0749206f36c58efc0fe0e3a1c13bc1d1096835766581cec686854 /tmp/ct123-py.json | sha256sum -c
```

[Exact-head CI](../../../../.github/workflows/ct123-irreducible-division-field.yml) is deliberately scoped to this new evidence and is distinct from independent signed scientific PCC, full `cargo test --release --lib`, independent reviews and protected main-merge policy. Any failures must be retained and corrected, not waived.

## Next mathematical gate

Build integral presentations of the **new** irreducible degree-60 fields, factor 59 and 2, and find the unique source-prime-labelled conjugate for *arbitrary* public `Q` without scalar-labelled advice or a disguised source DLP. Then independently check pinned binary specialization and measure full cold setup/recognition/destination solving/sign/verification against matched source rho and existing index calculus. The current negative finding means CT122's binary representation improvement **does not shrink the global x-field**. This does not rule out a computationally efficient global recognition algorithm. No paid job, protected merge, benchmark admission or ECDLP speedup is claimed.
