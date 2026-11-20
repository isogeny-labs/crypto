# Redundant unknown-index augmentation: pre-code protocol

The user's objective remains an index-calculus scaling advantage for actual
Curve25519. This stage implements a representation candidate and checks its
root correspondence. It does not run a solver, time a comparison, sample
targets or replace the original huge-base objective with a small control.
Existing direct-elimination-v1/NEXT.md remains the cost-comparison gate.

## Sources checked before selecting the construction

- Existing registered source review PCC-20261116-3aeb9e, the frozen v2
  membership/target compiler and direct-elimination-v1/NEXT.md: retain the
  actual original ideal and all map, inverse, distinct-column and target
  guards; do not infer work from quadratic degree or variable count.
- Menezes, van Oorschot, Vanstone, Handbook of Applied Cryptography,
  https://cacr.uwaterloo.ca/hac/about/chap2.pdf, Facts2.170/2.172/2.173 and
  Fact2.213: finite cyclic orders, power subgroups and finite-field units.
- Same authors, https://cacr.uwaterloo.ca/hac/about/chap14.pdf,
  Algorithm14.9 and Algorithms14.76/14.79: subtraction borrows and ordinary
  binary exponentiation. Boolean range circuitry below is an elementary
  fixed-bound encoding, not a new exponentiation or branching method.
- KB unavailable: no loaded kb_status/search_knowledge tools; current-main
  scripts/research-kb wrapper was read and its sibling backend is not
  executable in this runtime. Direct primary texts and pinned owned files
  supply the fallback. No novelty inference follows from the unavailable KB.

## Constructor and correspondence contract

New isolated Rust crate tools/torus-indexed-circuit; old compiled source,
source archives, claims, reports, original criteria and native outputs stay
unchanged. Compile the original Circuit/TargetCircuit first, copy its entire
variable/equation arrays as literal prefixes, then APPEND constraints.

External mathematical contracts: p is an odd prime and Fp2=Fp[sqrt2] is a
field. Runtime admission checks5..512bit odd p, Euler nonsquare2, canonical
norm-one h, and exact orderM=a*q via closure and h^(M/r)!=1 for each prime
factor r ofM. Factoring uses ordinary trial division of a and q separately;
both are positive and at mostu32::MAX, so this constructor is bounded but
does NOT admit an arbitrary growing family. Original k/no-wrap and curve
contracts remain in force. Euler/order arithmetic is conditional on p being
prime; it is not a primality certificate. Variable/equation caps16384/32768;
arity comes from the original2..16 target interface.

For each original member:

1. Reuse its unknown Boolean selector bits x=j-1 and slack bits. Bind them
   through the pinned original member layout: public first_selector/width,
   interleaved selector/slack slots, and checked shifted original equations.
   This is a version-bound adapter, not an arbitrary-compiler metadata theorem.
2. Append unknown Boolean t bits, width bitlength(q-1). For q1 this is the
   empty encoding of0. No known j/t/sign/point/witness is a compiler input.
3. Add deterministic subtraction-borrow constraints for existing slack
   (k-1)-x. Incoming/final borrow0; for constant bit0 outgoing=x+b-x*b,
   for constant bit1 outgoing=x*b. The slack digit equals c-x-b+2*b_next.
4. Enforce t<=q-1 by a most-significant-bit prefix-equality circuit.
   Starting equal=1, a bound bit0 forbids equal*t_bit; prefix equality
   updates with t_bit for bound1 and1-t_bit for bound0. Do not allocate
   the unused last prefix state. All gates have degree at most2.
5. Compute h^j from h and reused x bits with named constant-product Fp2
   gates, then multiply by the factors of (h^a)^t. Append BOTH coordinate
   equalities to the original z. Keep every original power/map/group guard.
6. Publish input-role metadata for reused selector bits, new t bits,
   original branch/raw-v and derived original/new coordinates. This stage
   provides no new branching solver and charges no index work as free.

Deterministic ordering per member: t bits LSB first; slack-borrow states
LSB first; range-prefix states MSB first; j product gates LSB first;
t product gates LSB first; both final coordinate equations. Members ascend.

Conditional correspondence argument to document/check, not a new complexity
theorem: an original canonical field root gives z^q=h^(q*j). Thus
(z/h^j)^q=1. The order-q element h^a enumerates all qth roots in the
multiplicative group of Fp2, giving a UNIQUE t in0..q-1. All appended gate
values are then determined. Conversely a new root retains every original
equation literally, and its original prefix must replay. Boolean range
and borrow constraints are necessary for this argument. A field/group/native
compiler soundness formalization is not supplied by a producer assertion.

## Fixed development controls, before code or native output

- Complete Fp101 pair enumeration of original norm/power membership for
  four declared norm-one fixtures: h(17,12),a17,q3,k8;
  h(84,89),a51,q2,k2; h(17,12),a3,q17,k1;
  h=(17,12)^3,a17,q1,k1. For every original accepted pair, check every
  representable t bit string against the new system and exact h^(j+a*t).
  Preserve a complete comparison trace. Do not call this exhaustive over
  arbitrary auxiliary field assignments; derived gates supply that argument.
- On unchanged p101/A95/q3/k8 target fixtures, inspect all24 raw columns,
  exclude the two original identity images, keep raw aliases distinct,
  and check all ordered signed distinct-raw pairs against all13 original
  eightfold-image targets. New and original decisions and independent row
  replay must agree. Retain complete comparison trace and actual counts.
- On the ORIGINAL actual a60824497/q213156431/k20633 six-member fixture,
  compile the original unchanged1059/1089 target system and append the new
  constraints. Extend the SAME known original six-member witness using its
  fixture labels only in the qualification harness. Replay original decoded
  row/group sum. This is extension checking, not root discovery.
- Mutate every actual augmented variable once, truncate assignments and
  change the target. Require the original/new check to reject. Exercise
  every representable toy overflow t, malformed field/order/norm/spec/arity,
  q1/k1 zero-width, q power-of-two and non-power-of-two range cases.
  For order controls, h^3 under the old a17/q3 spec must be refused; closure
  alone is insufficient. Retain failures without fixture/budget relaxation.
- Preserve complete original/augmented actual systems, all canonical roots
  needed for replay, public role metadata, source/binary pre-call hashes,
  logs and a new versioned source qualifier. Existing source archives are
  referenced compositionally, never rewritten to fit new inputs.

Acceptance: literal original prefix retention, no wrong root/projection,
one valid t extension per accepted member under admitted field/order
contracts, rejected malformed/range/mutation controls, and original signed
row replay. Any mismatch rejects this implementation at its frozen version.
An admission refusal is not route disproof. No solver invocation, performance
comparison, growing-family exponent, useful-yield or full-base claim here.

Before later cost comparison, finish the accounting/dependency-propagation
bridge on identical public targets. Never infer cheaper solving from extra
constraints, role metadata, smaller degree or the availability of labels.
Future solver/campaign work needs a separate committed protocol/admission;
no campaign or paid job is scheduled by this construction qualification.
