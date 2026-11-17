# Factoring normalization seed dependence — 2026-11-03

## Derived identity and domain

For the existing supported prime-field source, fix the public point Q, its
geometry grid, and partition (w,k,s,e). Work with nonzero canonical seed c and
nondegenerate recurrence operands. Geometry and division-polynomial values are
independent of c. Every seed-dependent operation in the source is multiplication,
powering or division, so the quotient is a monomial in c times its value at1:

    R(c) = c^E R(1).

This statement concerns the ratio represented by numerator/denominator pairs;
their individual common scales need not agree. Zero exclusions are unchanged by
multiplying by nonzero seed powers. The calibrated sign predicate is unchanged.
No elliptic-group homomorphism or characteristic-changing map is asserted here.

Let t=(r+1)/2. The initial value u has seed exponent t^2. Both initial recurrence
states (+u and -u) have this same exponent; negation does not change it. The baby
recurrence adds2 to its second difference, so its j-th exponent is

    b_j = t^2 + j(j+1),  j=0,...,w.

After the baby loop the center recurrence starts from b_w and b_(w-1). Its
square factor has exponent 2(2w)^2=8w^2. Hence center i has exponent

    a_i = t^2+w(w+1)+2wi+4w^2 i(i+1),  i=0,...,k-1.

These formulas track the actual source's recurrence boundaries. Substituting
ordinary squared scalar indices without these boundaries would not prove the
same identity.

Define G(n)=n(n-1)(n+1)/3=sum_{j=0}^{n-1} j(j+1). Then

    B = w*t^2 + G(w),
    A = k*(t^2+w(w+1)) + w*k*(k-1) + 4*w^2*G(k),
    H = s*t^2 + G(s),
    E = 2w*A + 2k*B + 2s*(s+e)^2 + 2H + e*t^2.

The last three terms encode the partial tail and singleton; when s=0 its terms
vanish. All signs and resultants have seed exponent0. Induction on each
multiplicative recurrence followed by these product sums proves the identity.

## Native construction

For c=N/D, the new consumer evaluates seed-free residual [U,V] and returns

    [U*N^E, V*D^E].

The constructor precomputes E modulo p-1. This reduction uses nonzero field
operands and Fermat's identity. E is public and depends only on p,r,partition.
G(n) is computed by dividing an integer factor by3 before modular products;
3 is not assumed invertible modulo p-1. Products reduce each factor before u128
multiplication; under the existing p<=u64::MAX/2 domain this cannot overflow.
The supported partition has k>=1. There is no division by3 in the field.

The seed-free residual is the preserved scalar ratio computation specialized at
c=1: scaled division values are just psi_n(Q), and seed squared is1. Product
trees, resultants, partial-tail and singleton factors remain. Its internal
rational recurrence denominators remain; only the seed-dependent propagation
has been factored out. The original scalar raw-parts function and recovery
source are byte-identical. The previous homogeneous consumer is test-only.
Prepared seed-pair generation, storage, geometry and direct phase calls remain.

This replaces seed-dependent powers throughout the consumer with two final
powers and two products. It also removes propagation of seed denominators
through the consumer's recurrence and tail factors. Exact whole-source costs
have not been measured; integer exponent setup, residual pair arithmetic and
the final powers must all be included in the next matched accounting candidate.
This does not remove the source's size-dependent geometry or decoder program.

## Independently checked software evidence

One optimized native test build and one30-second-capped invocation pass all192
methods, none filtered. New checks include2545 comparisons of the closed formula
with a separate iterative symbolic recurrence (509 odd r values from7 through1023
and five exponent moduli), and144 exact ratio cross-products against the original
scalar source with independently rescaled pairs. These cover three saved curves,
four nonidentity public points each, four seed values and three scales. The old
pair consumer's sign/error result also agrees. Invalid zero pairs are rejected.
Existing small-field partial/singleton, identity crossing, rescaling, arithmetic
and all eight full recovery receipt checks remain active.

Checked exponents modulo p-1 are312 for p353/r359,484 for p509/r479, and556 for
p701/r683. These are exact construction outputs, not performance measurements.
The source lock, raw compiler/test logs, receipt and control.json preserve the
checks. No comparative panel or cloud job ran; the original4563-case cohort and
all prior panel identities remain unchanged.

## Prior work and scope of search

Read the existing pair-seed derivation, scalar raw-parts code and native source
recurrences before deriving this specialization. EFD documents standard
homogeneous representations: https://www.hyperelliptic.org/EFD/g1p/index.html.
Mathlib's current EDS documentation states the standard elliptic recurrences:
https://leanprover-community.github.io/mathlib4_docs/Mathlib/NumberTheory/EllipticDivisibilitySequence.html.
Both read2026-11-03. Neither is cited as proving this program-specific exponent.
The related paper https://eprint.iacr.org/2008/444.pdf was located, but its full
text returned403, so no claim is attributed to its unread contents. No novelty
claim for monomial factoring or homogeneous arithmetic is made.

Next instrument this candidate against the preserved paired consumer, counting
all residual arithmetic and final powers. Keep the current preflight/panel
identities intact; any new comparison needs its own binding and frozen protocol.
