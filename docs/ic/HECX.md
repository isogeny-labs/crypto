# `hecx` — hyperelliptic discrete logarithms

`hecx` is the front door for the repository's **hyperelliptic Jacobian**
engines.  It accepts a curve over a prime or binary field, verifies it,
estimates the direct degree-one index-calculus cost, and solves
known-answer or planted discrete logarithms when the instance is small
enough.

It is deliberately separate from `icx`: `icx` covers the standardized
elliptic-curve catalog (prime, binary, Koblitz, extension, characteristic
three), while `hecx` covers the Jacobian of a genus-`g` curve

```text
C : y^2 + h(x)y = f(x),   deg f = 2g + 1,   deg h <= g
```

over `F_p` (`h = 0`, odd characteristic) or `F_{2^m}`.

## Quick start

```sh
cargo build --release --bin hecx

# What is supported?
./target/release/hecx list

# Validate a prime-field genus-2 curve and show its Hasse interval.
./target/release/hecx inspect --field prime --p 1019 --genus 2 --f 1,1,2,3,0,1

# Heuristic direct-IC versus rho estimate (no solve, no claim).
./target/release/hecx estimate --field prime --p 1019 --genus 2 --f 1,1,2,3,0,1

# Plant Q = 7*P on an auto-picked factor-base place and recover it.
./target/release/hecx run --field prime --p 1019 --genus 2 --f 1,1,2,3,0,1 \
    --secret 7

# Binary field: C: y^2 + x*y = x^5 + x^2 + 7x + 1 over F_2^10.
./target/release/hecx run --field binary --m 10 --modulus 0,3 --genus 2 \
    --h 0,1 --f 1,0,1,0,7,1 --secret 37 --small-prime-bound 8 --threads 4

# Built-in end-to-end instances.
./target/release/hecx demo --field prime --genus 2 --secret 7
./target/release/hecx demo --field binary --genus 2 --secret 37
```

Every command prints one JSON object.  `run` returns `verified: true` only
after checking `[k]P = Q` in the Jacobian; there is no unchecked answer.

## Measured smokes

These are small known-answer runs, not security claims.  Each `k` was
recovered and re-verified by `[k]P = Q`.

| field | genus | instance | order | IC components | result |
|-------|-------|----------|-------|---------------|--------|
| prime | 2 | `p=1019`, `f=1+x+2x^2+3x^3+x^5`, `k=7` | `273078 = 2·3^3·13·389` | 389 | verified |
| prime | 3 | `p=251`, `f=1+x+x^3+x^7`, `k=11` | `16729494 = 2·3·43·61·1063` | 43, 61, 1063 | verified |
| binary | 2 | `F_2^8`, `h=x`, `f=1+x^2+3x^4+x^5`, `k=37` | `720 = 2^4·3^2·5` | none (BSGS) | verified |
| binary | 2 | `F_2^10`, `h=x`, `f=1+x^2+7x^4+x^5`, `k=37` | `553756 = 2^2·7·19777` | 19777 | verified |
| binary | 3 | `F_2^6`, `h=x`, `f=1+x^2+2x^4+x^7`, `k=11` | `128568 = 2^3·3·11·487` | 487 | verified |
| binary | 4 | `F_2^6`, `h=x`, `f=1+x^2+2x^4+x^9`, `k=11` | `8877568 = 2^9·7·2477` | 2477 | verified |

The binary rows above use `--threads 4` for the relation search.  That is a
correctness-preserving split of the trial budget, not a benchmark claim.

## Parameters

The curve flags are the same for `inspect`, `estimate` and `run`.

| flag | meaning |
|------|---------|
| `--field prime` | `C: y^2 = f(x)` over `F_p` |
| `--field binary` | `C: y^2 + h(x)y = f(x)` over `F_{2^m}` |
| `--p` | prime modulus (decimal or `0x` hex) |
| `--m` | binary field degree |
| `--modulus` | binary modulus low terms, e.g. `0,1,3,4` means `z^8+z^4+z^3+z+1` |
| `--genus` | genus `g`; `deg f` must be `2g+1` |
| `--f` | coefficients of `f`, lowest degree first, decimal or `0x` hex |
| `--h` | coefficients of `h` (binary only), lowest degree first |

Points are given as `--P x,y` and `--Q x,y`.  For binary fields `x` and `y`
are field-element bitmasks.  Omit `--P` to auto-pick a factor-base place;
`--secret k` plants `Q = kP`, while `--Q` supplies an explicit target.

A subgroup order can be supplied with `--order`.  When it is omitted the
driver finds the order of `P` by baby-step giant-step inside the Hasse
interval, then factors it (trial division plus Pollard rho).  A known
factorisation can be passed as `--factors 'q1^e1,q2^e2,...'`.

## What the engine actually does

### Factor base

The factor base is the set of degree-one places, one representative per
`±` pair, represented by the divisor class `[P] - [∞]` in Mumford form.
In odd characteristic the base is about `p/2` places; in characteristic two
an abscissa can carry zero, one, or two points:

- if `h(x) = 0`, the equation is `y^2 = f(x)` and the point is unique
  (ramified);
- otherwise substitute `y = h(x)z` to obtain the Artin–Schreier equation
  `z^2 + z = f(x)/h(x)^2`.  It has two roots exactly when the absolute
  trace of the right-hand side is zero, and none otherwise.

The full base is materialised for small fields.  Above that, `hecx` uses
Gaudry's reduced base of size `q^{2/(g+1)}` (clamped to a practical
materialisation budget).

### Smoothness

A reduced divisor `(u, v)` is smooth over the factor base exactly when
`u` splits into linear factors over the field.  The binary engine decides
this with the characteristic-two Cantor–Zassenhaus split:

1. compute the squarefree radical of `u` (with the characteristic-two
   recursion, where even-multiplicity factors have zero derivative);
2. compute `gcd(radical, x^{2^m} + x)` by repeated Frobenius squaring
   modulo the radical;
3. split the resulting squarefree product by the absolute trace
   `Tr(a) = a + a^2 + ... + a^{2^{m-1}}`, recursing on the two gcds;
4. attach multiplicities by dividing each linear factor out of `u`.

The prime-field engine uses the same Cantor–Zassenhaus idea with the
quadratic-character split `a^{(p-1)/2}`.

### Relation collection

The default **factor-base walk** starts at `D_1 + D_2`, steps by a
random factor-base place (one Jacobian addition, and the step's unknown
logarithm is recorded and subtracted when the divisor decomposes), and
occasionally steps by `D_1` or `D_2` to move the known coefficients.  A
random fresh-start mode is available for comparison.

The binary engine can split the trial budget over `--threads` independent
walks; each worker returns a private relation list and the rows are
concatenated before linear algebra.

### Linear algebra and Pohlig–Hellman

Each relation is

```text
Σ c_i log F_i − b log D_2 ≡ a  (mod N).
```

The default solver is sparse Markowitz-lite elimination; dense Gaussian
elimination is retained as a reference.  A composite order is handled by
Pohlig–Hellman: the driver solves each prime-power component with
baby-step giant-step when the prime is small and with index calculus when
it is large, then reconstructs the scalar by CRT.  The final scalar is
always verified.

## Scope and honest limits

- **Models:** odd-degree (`deg f = 2g+1`, one point at infinity) only.
  Even-degree (two-infinity) models are rejected explicitly.
- **Prime fields:** checked arithmetic supports `p` up to one word
  (`2^64`) on the deterministic path; wider primes are accepted through a
  cached Miller–Rabin check.  Wide fields use the sampled reduced base and
  normally need an explicit `--P` and `--order`, because the full-base
  builder enumerates `F_p`.
- **Binary fields:** the field arithmetic supports `m` up to 4096; a full
  degree-one base is practical to about `m = 20`.  Above that the reduced
  base is sampled.
- **Genus:** the arithmetic and the relation structure are genus-generic,
  but the cost grows with `g`; the full-base relation stage is `O(q·g!)`.
- **No sub-exponential claim:** direct degree-one index calculus on a
  hyperelliptic Jacobian is `O(q)` at fixed genus (or `O(q^{2/(g+1)})`
  with the reduced base), not a break of deployed curves.  The estimator
  is a heuristic for this algorithm only; it does not model Weil descent,
  summation polynomials, or family-specific attacks.
- **No security claim:** every run is a toy or known-answer instance.
  `verified: true` means `[k]P = Q`, nothing more.

## Tests

The engine tests live in

- `src/cryptanalysis/binary_hyperelliptic_index_calculus.rs` —
  Artin–Schreier point construction, trace splitting, multiplicities,
  factor-base points on the curve, and an end-to-end DLP;
- `src/cryptanalysis/hec_dlp.rs` — BSGS, Pohlig–Hellman, CRT, and
  factorisation;
- `src/prime_hyperelliptic/curve.rs` — checked construction, including a
  wide prime and a wide composite rejection.

Run them with:

```sh
cargo test --lib binary_hyperelliptic_index_calculus
cargo test --lib hec_dlp
cargo test --lib prime_hyperelliptic
```
