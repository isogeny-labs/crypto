# Results from all four research tracks

The new work produces a certified forced two-step example, rules out several
simple arithmetic predictors of coverage, strengthens image recovery, and adds
line and double-large-prime target solvers. All recorded source answers pass
independent verification. The tested complete attacks remain slower than
source-group Pollard rho.

## 1. Coverage: distinguish a total step from an extra step

Over F_(157³), with z^3 + z + 152=0, the prepared curve

    y^2 = x^3 + (72*z^2+69*z+148)*x + (102*z^2+50*z+40)

has prime order **r=3868159**, is ordinary and non-subfield, and satisfies
the torsion gate. All twelve target Jacobians independently have the wrong
order **3871629**. Exact N1/N2 counts, actual
divisor orders, and unique multiples in a Weil interval certify those orders.

There are **2 nonprepared neighboring source curves**, each with a unique
rational degree-three isogeny leading to an isomorphic all-obstructed prepared
model. Therefore at least **two degree-three steps in total** are needed from
these sources within this construction. A usable two-step target is exhibited.
This refutes the total-one-step sufficient condition for arbitrary gated inputs.

One of the forced sources is

    y^2 = x^3 + (120*z^2+98*z+86)*x + (101*z^2+50*z+144)

Its unique degree-three kernel polynomial is
`x + 127*z^2 + 51*z + 84`. Its codomain is isomorphic over
the cubic field to the all-obstructed model above.

It does **not** refute one additional step after torsion preparation. Every
zero-coverage center found in the new searches has an eligible usable neighbor.
The universal extra-step assertion remains open.

The broad unrestricted-coefficient search tested **2384 distinct prepared
Frobenius orbits** within its time budget and screened 1 zero-coverage centers.
The deliberately biased scalar-nine-torsion stratum tested **163 orbits** and screened
0 zero-coverage centers. The known p=109 component closes after **26** eligible
orbits. The datasets can overlap and are not density estimates.

Evidence: [forced two-step certificate](results/forced_two_step_audit.json),
[explicit divisor witnesses](results/explicit_forced_two_step_witnesses.json),
[broad search](results/neighborhood_random.json),
[nine-torsion stratum](results/neighborhood_random_nine.json), and
[closed component](results/neighborhood_component.json).

## 2. Structure: the endomorphism ring does not decide coverage

Two curves in the p=109, r=1293169 isogeny class have the **same geometric
endomorphism discriminant -21195**, scalar -I Frobenius on E[9], and the same
distance from the prepared boundary. One has zero usable kernels and the
other has usable kernels with an independently certified target order r.

The proof uses exact division-polynomial root counts: forty for E[9] and 121
for E[27]. It is cross-checked with exact Hilbert class polynomials. Thus a
criterion using only these arithmetic data cannot classify coverage.

All **36** roots of H_(-21195) lie in the cubic field and form **12 Frobenius
orbits**. Exactly **one orbit** in this complete finite screen has zero coverage.
A **degree-five isogeny** repairs it while preserving both the three-torsion
type and the endomorphism order; the repaired target order is independently
certified. This motivates obstruction-guided walks using small degrees coprime
to three as well as degree three. General isogeny preprocessing is established
prior art, so this specialization is not asserted novel.

Within this fixed CM-order audit, the exceptional orbit is exactly
**j³+55j²+74j+107=0 over F_109**. The degree-five repair leaves the closed
prepared degree-three component pictured below. This provides a concrete way
to change the obstruction while keeping the endomorphism order and torsion
type. The cubic identifies a finite exceptional orbit, not a general criterion
over varying fields.

![The closed prepared component](results/four_track_component.svg)

Evidence: [structural comparison](results/structure_audit.json),
[endomorphism proof data](results/endomorphism_audit.json), and
[complete CM screen and degree-five repair](results/cm_coverage.json).

## 3. Transfer: actual fallback and exceptional-case handling

The reusable solver now composes a bounded isogeny walk, screens obstructions,
inverts source images through base-field polynomial systems, recovers actual
closed-place divisors, and aligns signs by addition in the target Jacobian.

It handles trivial targets, evaluation poles, empty affine inversion fibers,
coincident x-coordinates, and closed places whose y-coordinate is not generated
by x. Seven targeted edge checks pass. A real p=19 benchmark encounters an
empty inversion fiber and succeeds after a common-source-multiplier retry.

Two larger complete examples exercise the fallback: the p=109 prepared
counterexample is repaired automatically in one step, and a p=157 unprepared
source is transferred through two actual degree-three isogenies. The recovered
source scalars are **1729** and **271828**, respectively. Neither is an input
to the attack API.

This is still a generic-tangent-chart implementation with bounded retries.
Universal handling of all exceptional geometric inputs remains unfinished.
See [the edge-case record](results/transfer_edge_cases.json),
[p=109 complete run](results/repaired_pipeline.json), and
[p=157 complete run](results/forced_source_pipeline.json).

## 4. Target solvers and complete timing

Split line sections supply exact relations with the canonical class retained
as an unknown. The double-large-prime implementation builds a graph on points
outside a small factor base and substitutes rooted-tree expressions before
linear algebra. A translate walk supplies anchoring relations efficiently.
These are implementations of known geometric index-calculus ideas.

| p | r | Target solver | 3-isogeny steps | Target log | Complete transfer and log | Source rho median |
|---|---:|---|---:|---:|---:|---:|
| 7 | 373 | line | 1 | 4.05 s | 44.06 s | 0.0069 s |
| 13 | 2137 | line | 0 | 12.45 s | 46.49 s | 0.0093 s |
| 19 | 6871 | line | 1 | 9.11 s | 41.41 s | 0.0118 s |
| 31 | 29641 | line | 0 | 30.88 s | 96.23 s | 0.0227 s |
| 7 | 373 | large-prime | 1 | 3.67 s | 65.48 s | 0.0105 s |
| 13 | 2137 | large-prime | 0 | 4.41 s | 63.10 s | 0.0145 s |
| 19 | 6871 | large-prime | 1 | 7.04 s | 77.40 s | 0.0165 s |
| 31 | 29641 | large-prime | 0 | 5.41 s | 59.04 s | 0.0249 s |
| 109 | 1293169 | line | 1 | 37.40 s | 73.88 s | 0.2744 s |
| 157 | 3868159 | large-prime | 2 | 10.87 s | 91.43 s | 1.0352 s |

The p=157 large-prime run uses a matrix of rank **39**, covering **157 of 158**
rational curve points, instead of assigning a separate unknown to every point.
Its final answer is checked both in the target Jacobian and on the source
elliptic curve. The basic smooth-divisor solver is separately measured on the
same target instances where a comparison is archived.

Every row is a single complete pipeline observation; the rho column is the
median of five seeded full-subgroup runs. Pipeline time includes construction,
section setup, inversion, and target DLP, and excludes independent controls.
Some jobs ran concurrently on four cores. These timings show a bottleneck
reduction, not a controlled scaling estimate. **No complete attack beats source
Pollard rho here. No new exponent or cryptographic-sized DLP is claimed.**

Raw results: [line solver](results/four_track_benchmarks.json),
[large-prime solver](results/four_track_large-prime_benchmarks.json),
[independent certificate checks](results/four_track_validation.json), and
[machine-readable summary and code hashes](results/four_track_summary.json).

## Reproduce and remaining work

With SageMath 10.6, from the suite root:

```sh
sage -python research_lead/obstruction_neighborhoods.py --mode component
sage -python research_lead/obstruction_neighborhoods.py --mode random --count 3500 --seconds 1200
sage -python research_lead/obstruction_neighborhoods.py --mode random --target-nine --count 250 --seconds 300 --seed 20261116
sage -python research_lead/forced_two_step_audit.py
sage -python research_lead/explicit_counterexample_witnesses.py --audit forced_two_step_audit.json --output explicit_forced_two_step_witnesses.json
sage -python research_lead/structure_audit.py
sage -python research_lead/endomorphism_audit.py
sage -python research_lead/cm_coverage.py
sage -python research_lead/transfer_edge_cases.py
sage -python research_lead/transfer_pipeline.py --primes 7,13,19,31 --compare-basic
sage -python research_lead/transfer_pipeline.py --primes 7,13,19,31 --solver large-prime --compare-basic
sage -python research_lead/repaired_pipeline.py
sage -python research_lead/forced_source_pipeline.py
sage -python research_lead/validate_four_track.py
python3 research_lead/four_track_report.py
```

Wall-clock-bounded searches will not reproduce identical case counts on every
machine. Seeds and actual accepted inputs are saved. Benchmark files resume
completed cases; rename an existing output file to force fresh measurements.

The main mathematical question is now the **extra-step** bound, with total-step
coverage already refuted. A useful theoretical investigation is to express the
simultaneous obstruction signs as square classes of modular functions and look
for dependencies that forbid or permit an all-negative neighborhood. This is a
proposed direction, not an established calculation or theorem.

Comprehensive chart coverage, sparse large-scale implementations, isolated
repeated performance measurements, and novelty verification also remain open.
The derivations and limitations are in [FOUR_TRACK_METHOD.md](FOUR_TRACK_METHOD.md).
