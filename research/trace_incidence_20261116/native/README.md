# Native auxiliary-curve root finder

This implementation follows the supplied paper's complete root-finding path:
an auxiliary curve and quadratic twist, a polynomial lift, multiplication
isogenies evaluated by division polynomials, Sylow separation, and root
recovery. It includes a source-coordinate adaptation that avoids canonical
divisor reconstruction during separation. Development checks are unsigned.
[pcc:unverified]

## Build and run

Requires C++17, FLINT development headers/library, GMP and MPFR. On a machine
with system packages:

```sh
make -C research/trace_incidence_20261116/native
cd research/trace_incidence_20261116/native
build/rootfinder pullback paper83.datum paper83.poly
build/rootfinder trace paper83.datum paper83.poly
build/rootfinder dense paper83.datum paper83.poly
build/rootfinder pullback paper83.datum paper83.poly --two-isogeny
build/paper_example
```

For the development environment, build with
`FLINT_PREFIX=/workspace/native-deps/flint/usr`. The Debian package was
3.1.3-2; its public version macro reports3.1.0. Link MPFR before GMP when using
the extracted development package's static MPFR archive.

The datum format is whitespace-separated: `p a b beta N Ntw`, then each
factorization as `count (prime exponent)...`, then four basis records
`infinity x y exact_order`. The first two basis records belong to the curve,
the last two to its twist. The twist coefficients are derived and the datum
is checked before use. The polynomial format is `degree c0 ... c_degree`.
Coefficients are constant-first. Inputs must be monic, split and squarefree;
production checks this algebraically. Domain: prime3<p<2^63. [pcc:unverified]

No CLI timer is supplied. The counts are development diagnostics, not GAE,
cycles or a performance comparison. The optional two-isogeny path is an
explicitly counted **exhaustive toy construction**, refused above p257.
It constructs a complete codomain datum and re-embeds the original polynomial.
It does not invert an isogeny or treat generator images as a full basis.

## Components

- `api.hpp`, `translation.cpp`: point arithmetic and full canonical divisor
  translation, trace and dense modes, collisions, infinity and exceptions.
  Divisor support is a set: actual points in different components are
  disjoint; opposite points at the same x can occupy separate components.
  The infinity flag records the single possible supported point at infinity.
- `paper.hpp`, `paper.cpp`: Definition1.2 datum validation and Algorithms
  1,2,4,5,6, with division-free normalized division-polynomial recurrences and
  balanced CRT. General datum construction is not supplied.
- `pullback.hpp`, `pullback.cpp`: original source variable retained through
  recursion; see [the derivation](../reports/PULLBACK.md).
- `isogeny.hpp`, `isogeny.cpp`: classical degree2 Velu map, twist compatibility
  and counted toy datum construction.
- `incidence.hpp`, `incidence.cpp`: complete norm/GCD selector and signed
  mixed-label witness recovery on the original target curve. The root backend
  can use known coordinates, literal trace/dense, or source pullback. Shared
  constant/linear shortcuts apply to all backends. [pcc:unverified]

`RootBackend::Pullback` is now connected to actual incidence output, rather
than multiplying a stand-alone kernel ratio into unrelated solver timings.
The existing Horner norm still performs O(B^2) degree-B quotient products.
This integration supplies no improved bound for that norm. [pcc:unverified]

## Checks and receipts

```sh
bash check-translation.sh
bash check-paper.sh
bash check-integration.sh
bash check-cli.sh
bash check-edge.sh
```

Each launcher builds a separate qualifier, makes it mode0555, refuses to
replace its pre-call record, and keeps distinct source/build/pre/post hashes.
They collect no performance timings. Test-only enumeration and validation
scalars stay in the auditor; production gets polynomial coefficients and
the auxiliary datum. [pcc:unverified]

Current receipts:

The native incidence panel uses p83/p97, B2/3/4 and ten seeds per combination.
It is a correctness panel, separate from the historical growing-B timing
experiment; it supplies no new scaling measurement. [pcc:unverified]

- `audit-data/002-result.json`:37,556 independent group/scalar checks,
  16,500 division-polynomial checks,4,690 translations,190 literal root-finder
  calls and95 source-pullback calls; includes characteristic-p indices,
  f=X^p-X, malformed data, twist scaling and mutation controls.
- `audit-data/004-result.json`:60 natural labelled-pool cases,42 hits/18
  misses,244 complete incidence calls across four backends,81 nonlinear
  root calls, an uninformative-first regression and5,184 isogeny homomorphism
  pairs. Target scalars are rechecked on the original curve.
- `audit-data/005-example.json`: the exact degree10 polynomial from the
  paper, all three methods, with structural counts. Both literal methods call
  translation73 times; source pullback calls it zero times. Linear early
  stopping also differs, so these counts do not isolate one speed factor.
- `audit-data/005-cli-result.json`:six CLI root calls, the fixed structural
  example and a rejected repeated-root input. [pcc:unverified]
- `audit-data/007-edge-result.json`:an auxiliary curve over F5 of order5,
  and its twist of order7, exercise actual rational characteristic-p torsion.
  The unchanged independent checker passes13 additional pullback root calls,
  26 literal root calls and375 division-polynomial point checks. The auxiliary
  anomaly is a correctness edge case, not a target-hardness benchmark.
  [pcc:unverified]

Call001 predates the balanced-CRT/source-pullback changes and remains historical.
Call003 has passing child output but an enclosing-shell status anomaly;
call004 repeats the same integration fixtures with an isolated launcher and
an explicit child exit-status file. No rerun is counted as an additional
independent natural sample. [pcc:unverified]

The current independent verifier was authored separately from the production
kernel. It remains a small-field executable audit, not a formal proof,
cross-host benchmark, PCC-signed verdict or exhaustive literature review.

## Auxiliary isogeny cost

For the paper's curve y²=x³+31x+21 over F83, the rational2-isogeny with kernel
generated by(32,0) produces y²=x³+37x+74. Curve/twist orders stay72 and96.
The images of the supplied generators have order product36, so they do not
form a codomain basis. Explicit completion in the checked toy constructor
produces cyclic bases72×1 and96×1 and scans13,778 coordinate pairs. The
constructor's point/order/basis work is also recorded. [pcc:unverified]

Cyclic groups permit ell representatives per level instead of ell², as in
the paper's Remark5.9. Their exponent can also increase, adding levels: no
runtime gain is inferred from cyclicity alone. Isogenies preserve the orders
and therefore do not improve the smoothness parameter Xi. [pcc:unverified]

The auxiliary curve is used for field-root extraction. Known mixed labels
remain on the original target curve; no DLP homomorphism to a smooth group,
free structured factor-base labels, or security transfer is assumed.

Sources: supplied Spaenlehauer paper Definition1.2, Algorithms1–6,
Remark5.4,§§1.3,6.1,8.1,8.4; existing repository Velu formulas in
`src/isogeny/velu.rs` and `src/bin/isoclass_rho/isogeny.rs`; fixed-source
coset precedent in Grenet–van der Hoeven–Lecerf2015 Algorithm2.
KB review is unavailable in this runtime; exact fallback/limits are retained
in `pullback_review.json`. No priority or fastest-external-tool claim.
