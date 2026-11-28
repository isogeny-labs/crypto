# Bounded binary quadratic root records

Design recorded 2026-10-28 before implementation, data generation, or native
validation. Base source: `d71ec2ef5ab9b7f2a038f61c941af10297a775ad`.
Conductor T-743 reserves only the new root module and this research directory.
The parent owns module registration, dataset integration, and centralized builds.

## Scope

Implement a standalone native quadratic-root module for polynomial-basis
GF(2^n), 3 <= n <= 61, polynomial degree at most two. This covers the requested
n=23..31 and 47..61 field arithmetic; it does not add higher-m elimination.
The eventual dataset integration remains the existing binary/Koblitz root
adapter at m=2, on the exact public curve, twist, target and factor-base domain.
Preserve legacy APIs. This is software implementation, not an experiment or
a completed corpus, root-coverage measurement, or fall-degree result.

## Algorithm and exposed states

Preserve the existing quadratic/Artin--Schreier method: constants and linear
equations, pure-square Frobenius roots, and the generic substitution
X=(B/A)z with z^2+z=AC/B^2. Emit primitive field operations, explicit
Itoh--Tsujii inverse-power states, normalization, trace and odd-degree
half-trace states, even-degree GF(2) basis images/pivots/reductions, extracted
roots and terminal distinct roots. The even branch uses Gaussian elimination,
not brute-force field enumeration. A zero polynomial has an explicit all-field
result; it never becomes an empty complete root set.

Strict typed records bind the degree, full monic irreducible polynomial,
canonical coefficient words and limits. Reject high coefficient bits instead
of allowing field constructors to mask them. Validate the modulus, including
irreducibility. The producer uses the existing generic F2m arithmetic for
individual additions/multiplications/squares; traced inversion exposes its
individual operations so work bounds can stop inside it.

## Bounds, durability and accounting

Bound native work and event count with hard maxima. Work counts selected field
add/mul/square calls plus explicitly counted GF(2) pivot bit tests and word
XORs. Inverse calls are informational, not an extra charge. A matrix reduction
has two word XORs. These mixed primitive units are an admission bound, not
machine instructions or performance-normalized field work. Input validation,
serialization, independent replay, loop/control overhead, and future adapter
coefficient setup/group lifts are outside this module's work counter.

Only sink-acknowledged events enter the durable prefix. A failed sink stops
immediately; retain the attempted unacknowledged event and charge its executed
work. Event exhaustion occurs before computing its next event's work. Report
work/event/sink stops as incomplete, with no complete root-set claim. Completed
events can later use the existing live-artifact sink, but a durable event prefix
is not a resumable in-memory solver checkpoint. Parent integration also bounds
serialized bytes and the aggregate work over root calls/lifts.

## Independent root certificate

Use separate portable carryless polynomial arithmetic, without invoking the
producer root solver, its inversion schedule, or its Artin--Schreier solver.
For each completed nonzero f, recompute the monic gcd(f, X^(2^n)+X) by n
modular squarings and compare it with the product of distinct reported
(X+root) factors. Handle zero/nonzero constants explicitly. This verifies the
complete set of distinct field roots, not multiplicities, a PDP witness,
scalar recovery, or complete execution cost.

Check every supplied event identity and its recorded tally, with bounded
ordering/state checks. Event omission can leave the root-set certificate valid;
therefore execution_trace_complete_verified and total_operation_count_verified
remain false and are enforced by the checker. A completed root certificate
must not promote a capped call or an incomplete finite-domain search.

## Validation and integration obligations

Native unit tests will cover exhaustive small odd/even-field quadratics;
constant/zero/linear/pure-square and trace-one cases; constructed roots at every
requested field degree; canonical/modulus/roots/events/counter/flag tampering;
and exact work/event/sink boundaries, including failure at the terminal event.
Output and primitive-count compatibility with the legacy method can be checked
without treating legacy agreement alone as independent correctness evidence.

No Cargo command or cloud job belongs to this standalone task. Parent validation
must register the module, execute its tests and strict lint, integrate frozen
public-domain/budget/outcome bindings and consumer certificate references, and
run the required release library suite before publication. Actual cloud
artifacts, worker IAM, durable quota authority, and the existing USD100--200
total budget (maximum USD200) remain separate unchanged obligations.

## Sources checked

- Repository `src/cryptanalysis/binary_semaev.rs`,
  `src/binary_ecc/{f2m,operation_tally}.rs`,
  `src/cryptanalysis/prime_root_trace.rs`, and the existing dataset root,
  live-sink, outcome-binding and export code. The prior accounting correction
  includes batched inversion squares; this module uses explicit bounded steps.
- Shoup, *A Computational Introduction to Number Theory and Algebra*, version2,
  [sections19.4 and20.4.1](https://www.shoup.net/ntb/ntb-v2_0.pdf): finite-field
  trace and the distinct-linear-factor gcd characterization. The checker uses
  that characterization independently of the producer's quadratic algorithm.
- [SEC1 version2, section2.3.9](https://www.secg.org/sec1-v2.pdf): canonical
  polynomial-basis bit-to-integer representation.
- The repository's Semaev S3 derivation and reference to ePrint2004/031 were
  reviewed. A fresh request for the primary PDF returned HTTP403; its full
  text was not newly inspected. No novelty or speed claim is made.

## Authorized dataset integration, 2026-10-28

After the standalone increment, the parent handed off `src/ml_dataset` and
module registration under renewed T-743 ownership and exclusive centralized
Cargo ownership. The nine native tests passed at registration commit
`aea52933390a63e38cdebe7c81d1baff1b059a45`; strict lint and integration follow.

Replace only the binary/Koblitz m=2 root adapter with the bounded producer.
Bind canonical JSON digests of the complete public instance, complete actual
factor base and exact CaseConfig, plus the explicit field/modulus, target,
ordered fixed-x calls, recomputed S3 coefficients, and work prefixes. Never
read private-witness data. Preserve legacy APIs and legacy artifacts without
the new certificate markers. Higher-m remains explicitly unsupported.

Charge the five-operation binary S3 recipe, native mixed primitive work,
six-operation Horner checks, and actual group lift additions against one
aggregate cap. Bound native events and serialized event bytes across all calls;
independent replay recomputes both aggregates, verifies the sole recognized
byte-cap sink stop, and rejects unjustified omitted calls or lifts. Live sink
I/O failure aborts immediately and preserves its previously fsynced prefix.
Selected native states and per-call reports become live artifacts. An all-field
zero polynomial enumerates only actual factor-base x values under the same cap.

Certificate, outcome and training-export reference bindings enforce false
execution-completeness and total-cost flags and no scalar or ECDLP completion
claim. Tests cover both binary families and twists, odd/even fields, cap
boundaries, exact public/config/domain/field binding, tampered aggregates,
sealed outcome labels and live-prefix persistence. These are bounded software
fixtures, not new scientific data or coverage measurements. Persistence uses
the existing canonical S3 source/evidence path; no cloud scientific execution,
new budget, public push or PR is authorized by this increment.
