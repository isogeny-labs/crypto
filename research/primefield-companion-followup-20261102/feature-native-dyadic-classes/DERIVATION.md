# General dyadic quotient classes for the public sign source

T-1824, 2026-11-07. Derived algebra with deterministic native producer checks.

## Domain and representation

Retain the previous source's domain: caller-certified primes p>3 and odd r>=7,
r!=p, gcd(r,p-1)=1, p,r<=u64::MAX/2, nonsingular curve, validated nonzero
r-torsion points and an explicit geometry-lane cap. Runtime receives public
points. Scalar labels and exhaustive group enumeration occur only in checks.
Let q=2^v2(p-1), d=(p-1)/q and pi(z)=z^d. The multiplicative quotient
Fp*/(Fp*)^q has exponent q; pi identifies it with the q-th roots of unity.
Zero is adjoined separately. A class stores one private field representative,
not a q-bit discrete logarithm or a compressed additive field representation.

Multiplication, negation, square and positive powering descend to this quotient.
For nonzero z, [z]^n=[z]^(n mod q) and [z]^-1=[z]^(q-1).
For zero, 0^0=1 and 0^n=0 for n>0; the latter check must precede exponent
reduction. Inverting zero rejects. Combining different field contexts rejects.
There is no class addition, subtraction or public representative accessor.

## Resultant invariant

For a=tb+c with degrees m,n,k and nonzero c,
Res(a,b)=(-1)^(mn) lc(b)^(m-k) Res(b,c).
A constant b contributes b[0]^m; a zero remainder with positive-degree b gives
zero. The existing half-GCD engine continues to compute all coefficients,
quotients and actual remainder degrees in Fp. Only the multiplicative
resultant accumulator is a class. Its leading-coefficient powers may therefore
be reduced modulo q. Binary powering replaces a loop proportional to the
reduced exponent. Existing transition metadata retains actual degrees; it is
not reduced or inferred from truncated leading halves.

## Seed, recurrence and final projection

Use the previous derivation in ../feature-native-dyadic-source/DERIVATION.md.
For beta=W_(r+1)^2 W_2/W_(r+2) and e=(r^2)^-1 mod q, represent beta^-e as
an ordered pair of classes [W_(r+2)^e, (W_(r+1)^2 W_2)^e]. The pair denotes
numerator divided by denominator after projection. Scaled W_n uses this pair
to the power n^2 modulo q and multiplies W_n into its numerator.

Carry pairs through the existing block recurrence. In a step with coordinate
difference delta, current N/D, previous U/V and squared-step A/B, the next
pair is (delta*N^2*A*V, D^2*B*U). Compute delta in Fp before mapping its class.
All remaining products, signs, powers and resultants are multiplicative, so
applying the quotient map commutes with every such step. Required nonzero
terms are checked. The final pair N/D is converted to N*D^(q-1), then pi is
applied exactly once in raw_grid (source inspection, not a measured counter).

The previous projected Ward identity and half-set argument give raw([k]P)=
Legendre_r(k)*raw(P). Calibration stores raw(P); normalization compares to
that value or its negative, avoiding a calibration field inverse. The public
API also retains prepared identity-crossing windows.

## Costs and boundary

Class powers and inverse use O(log q) field multiplications; final projection
uses O(log d). This removes field inversions specifically for seed beta,
seed inversion, final source ratio and calibration. Geometry, cached division
polynomials and polynomial remainder arithmetic retain their own inversions.
The source still builds square-root-sized geometry and polynomial data. This
is a representation change and a general-q implementation, with no timing
comparison or measured net speedup. Runtime classes still occupy field words.
No extension-field transfer, scalar table or discrete-log extractor is added.

## Reproducible checks

- validation-01: initial passing source check, preserved polynomial/generator
  snapshots. Before edits all receipt input hashes were verified.
- validation-02: final source after dropping obsolete copied cfg(test) modules
  with dangling local includes. 114751 field-element pairs; 1792 projected
  root resultants over p=5,7,17,41,97,193,257; exponent-wrap, zero-inverse and
  cross-context controls. Root products use all degrees 1..16 on each field.
- Both source checks: 1108 nonzero points match the prior full dyadic source,
  direct projected half-products, Ward covariance and scalar Euler signs;
  24 identity-crossing windows; 266 wrong-exponent point-function differences.
  Curves are exactly the eight fixtures listed in check.stdout. The five new
  dyadic orders 8,16,32,64,256 retain the previous bounded fixture finder.
- recovery-01: 617 targets across those five groups recover with 5929 source
  calls, zero unresolved. The inherited 63 public comparisons retain 511 calls.
  Workspace-adaptive and ordinary envelopes agree. Additional inherited exact
  trace/envelope/planner checks are retained in stdout.
- normal-build: five source dependencies; no derivative jets or prior source.
  No native build/assertion failure. Warnings retained. No timing study, cloud
  launch, original 4563-case cohort edit or independent signed verdict.

From the repository root, run generate.py and generate-recovery.py in this
directory; then validate.py NEW_OUTPUT_DIR and validate-recovery.py ANOTHER_DIR.
Both scripts compile native Rust with overflow checks and save commands,
compiler identity, dependency hashes and stdout hash. Never overwrite a saved
validation directory. publication.json hashes the deliverable files.

## Sources checked and next construction

KB unavailable in the inherited session; use the local source/half-GCD proofs
and [FLINT's half-GCD documentation](https://flintlib.org/doc/algorithms/polynomials/half-gcd.html)
for field-valued remainder transitions. The page carries a generated-prose
notice; quotient correctness is derived above and checked locally.
[Lauter–Stange 2008, Theorems 3–4](https://arxiv.org/pdf/0803.0728) underlie the
prior Ward/chain normalization. No novelty claim. Next, account field operations
for class pairs versus the full representative route, including geometry and
polynomial inversions, before choosing a runtime route by a cost model.
