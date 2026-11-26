# Protocol: operational pair-image degree gate

**Frozen on 2026-11-25 before implementing the certificate or observing any
pair-image polynomial.**

Study id: `EXP-C25519-PAIRIMAGE-GATE-001`. Parent revision: `eba99df211`.
The certificate will be native Rust in Isogeny Labs `crypto`. No JavaScript is
used.

## 1. Question and boundary

`EXP-C25519-PAIRTRACE-001` showed that submitting the complete quadratized
2+1 circuit to ordinary-grevlex F4 is untenable even at `d=7`. The surviving
idea is to split operationally: construct the abscissa image of an unordered
factor-base pair, construct the target-conditioned image of the lone root,
and intersect the two images.

This protocol asks the cheapest prerequisite question. Does the pair image on
the fixed Curve25519-family instance have collisions or repeated algebraic
roots that reduce it below its generic degree `d^2`? It is an exact algebraic
certificate, not a performance benchmark or an ECDLP claim.

## 2. Fixed instance and polynomial

Use the existing short-Weierstrass instance over

```text
p=1,048,573, A=63,700, B=872,826,
x(w)=349,833+3w, w in mu_d, d in {7,9,19}.
```

For each unordered pair `i<=j`, write the standard Semaev polynomial as a
polynomial in the intermediate abscissa `U`:

```text
S3(x_i,x_j,U) = a_ij U^2 + b_ij U + c_ij,
a_ij = (x_i-x_j)^2,
b_ij = -2*((x_i+x_j)*(x_i*x_j+A)+2B),
c_ij = (x_i*x_j-A)^2-4B*(x_i+x_j).
```

For `i<j`, retain the quadratic. For `i=j`, require `a_ii=0` and retain the
nonzero linear polynomial. Define the unnormalized pair-image polynomial

```text
H_d(U) = product_(0<=i<=j<d) S3(x_i,x_j,U).
```

The formal degree is at most
`2*d*(d-1)/2+d=d^2`. The certificate must not divide by, drop, or squarefree
any factor before measuring the original product.

## 3. Exact certificate

For each `d`:

1. construct `mu_d` from the repository primitive-root routine and check all
   roots are distinct and satisfy `w^d=1`;
2. construct every pair factor and record counts of quadratic, linear,
   constant, and zero factors;
3. multiply all factors over `F_p`, record the exact degree and coefficient
   SHA-256, and verify every factor divides the product;
4. compute `gcd(H_d,H_d')`, recording its degree; independently multiply the
   monic trace blocks indexed by the canonical pair trace
   `c=q+q^-1`, and require equality with monic `H_d`;
5. require the `c=2` block to have degree `d`, every other trace block to have
   degree `2d`, `(d+1)/2` blocks total, and their degrees to sum to `d^2`;
6. run internal unit controls for polynomial multiply/divide/gcd, repeated
   factors, and deliberate trace-block corruption.

The machine output is one JSON object with schema
`curve25519.pair-image-degree-gate/v1`. A second run must be byte-identical.

## 4. Decision rule

- If any zero/constant factor, degree loss, repeated factor, or block mismatch
  occurs, retain the exact exception as a possible compression lead and stop
  for derivation before performance code.
- If `deg(H_d)=d^2` and `gcd(H_d,H_d')=1` at all three orders, close explicit
  pair-image construction and lookup as an exponent-moving lead. It has
  `Theta(d^2)` distinct algebraic support on these instances and is the same
  structural object as the known pair-root index/generic pair-sum MITM.
- A successor may open only with a proved arithmetic upper bound `o(d^2)` for
  a target-conditioned common-root test that does not materialize `H_d`, does
  not enumerate `Theta(d^2)` pairs, and returns a recoverable witness. Merely
  storing `H_d` factored into `Theta(d)` trace blocks of degree `Theta(d)` does
  not pass.

This finite gate cannot prove an asymptotic lower bound. Relation probability,
collection, filtering, linear algebra, target recovery, and total `r^o(1)`
work remain outside it. Results remain `[pcc:unverified]` without independent
validation.
