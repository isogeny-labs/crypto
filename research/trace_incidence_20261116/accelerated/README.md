# Native acceleration candidates

The [local runtime measurement](../measure_20261117/README.md) is complete.
At B128 the new Bloom pair path gives a5.27× median paired improvement over the
prior pair control; composition gives4.99× over same-generator Horner. The new
pair path remains3.06× the registered rho cost. These are provisional local
measurements, separate from the old Runpod data. [pcc:unverified]

This sibling extends the incidence and pool-generation stage; the historical,
native and closed Runpod sources remain frozen. The candidates are implemented
for public synthetic prime-field inputs with prime p,r below 2^62.
`PROTOCOL.json` defines the parent Runpod test; the linked local amendment and
explicit user exception define the completed local run.

The baseline is the existing native quotient/Horner/GCD solver with actual
random mixed-labelled pools. The strongest existing generic control is the
frozen streamed batched pair search. These solve different intermediate tasks:
the algebraic solver returns the complete third-pool support polynomial, while
the fastest generic control can stop at one informative relation.

## Changes and their cost

`composition_gcd` interpolates the actual third-pool ordinates Y3 modulo F3.
For each first-pool point (x,y), it forms the two x coordinates of its signed
sums with the third-pool points in the split algebra Fp[Z]/F3. Their values are

    t± = (Z^3+aZ+b+y^2 ± 2yY3)/(Z-x)^2 - x - Z.

The norm factor is F2(t+) F2(t-). A final GCD with F3 yields exactly the same
monic support as the original quadratic-algebra norm. Disjoint coordinates
make every denominator a unit. Synthetic division constructs its inverse with
the correct minus sign; fast multipoint evaluation and one shared scalar
inversion supply all F3(x) inverses. Polynomial construction, interpolation,
preinverse setup, validation and witness recovery are charged.

FLINT's generic composition routine uses Horner when deg(F2)>=deg(F3), which
includes the equal-pool case. The new code writes F2(T)=c0+T*tail(T), calls the
explicit preinverted Brent–Kung routine on the lower-degree tail, then performs
one product/add. Unequal pool sizes use valid outer blocks; dimension1 has a
scalar path. For equal B>1 the implementation makes 2B Brent–Kung calls and
7B explicit quotient products, versus the original 2B²+10B products. This is
an operation ledger, not a timing or new modular-composition bound.

With M(B) polynomial multiplication and C(B) modular composition, the norm
work is O(B C(B)+B M(B)+B²+M(B) log B) field operations, plus validation and
recovery. C(B) and FLINT's matrix workspace are substantial. The abstract bound
does not establish an ECDLP exponent or practical crossover.
These source-derived operation costs remain unsigned assertions. [pcc:unverified]

`rust/src/lib.rs` reuses the existing `prime_fast` Montgomery arithmetic. It
prepares a joint two-bit fixed-window table for the public P and Q, then batches
affine additions across all pending labels. All table setup is per target and
charged. The C++ MT19937 stream draws exactly the remaining pool deficit,
filters in the original order and repeats only the deficit. It preserves every
accepted label/point and every rejected draw. A Straus control separates the
existing backend from the new batching schedule. Operation counters
cover explicit batches and exceptional doubles; they do not count all internal
scalar-multiplication field operations. Complete timers include those operations.

The Rust pair engine streams both signed sums with one inversion per row,
uses Montgomery-coordinate lookups, and recovers the public scalar from actual
mixed labels. Optional Bloom filtering precedes exact hashmap lookup; no
probabilistic acceptance occurs. Full-support mode continues after finding a
witness and builds the entire GCD support polynomial with a product tree.
First-witness mode stops early and **returns no GCD**. Pair working storage is
O(B), with no B² pair table. These are generic pair-enumeration optimizations.

## Prior work and scope

Sources checked (KB): unavailable (`scripts/research-kb status` exits127; no
catalog MCP tools loaded). The fallback review used:

- [Pinned cross-repository assessment](../historical/repo_controls/repository_assessment.json):
  existing crypto/cryptanalysis quotient/GCD, labelled recovery and batched pairs.
- [Historical composition candidate](../historical/incidence/compose_candidate.py)
  and [incidence report](../historical/incidence/REPORT.md): the split-coordinate
  norm construction already exists. Its old Python timings are historical only.
- [Existing Rust backend](../../../src/cryptanalysis/prime_fast.rs): Jacobian
  Straus multiplication, Montgomery arithmetic and batched affine additions.
- [FLINT official composition API](https://flintlib.org/doc/nmod_poly/compose_mod.html):
  generic Horner fallback, Brent–Kung degree restriction and preinverse contract.
  The [v3.1.3 implementation](https://github.com/flintlib/flint/blob/v3.1.3/src/nmod_poly/compose_mod.c)
  confirms the degree-dependent fallback on the installed version.
- [FLINT multipoint API](https://flintlib.org/doc/nmod_poly/multipoint.html):
  fast interpolation and multipoint evaluation.

This is a native implementation and reproducible comparison, with no priority
claim. It does not combine a structured PKM/isogeny factor base with free mixed
labels. Auxiliary curve, twist and isogeny root-finding controls remain in the
closed experiment. Their timing ratios do not apply to this norm or to pool
generation; all new labels are explicitly constructed on the target curve.

## Build and check

Requires Rust1.100, C++17, FLINT3, GMP, MPFR and nlohmann-json headers.
The shared Rust library additionally needs OpenSSL development libraries.

```sh
make -C research/trace_incidence_20261116/accelerated
make -C research/trace_incidence_20261116/accelerated check
```

Set `FLINT_PREFIX`, `JSON_INCLUDE`, `CARGO` and `CARGO_TARGET_DIR` for nonstandard
installations. The copied read-only qualifiers, separate non-overwritten build,
pre-call and post-call manifests and raw no-timer output live in a newly created
temporary check directory. The audit compares all support coefficients, actual
witnesses and labels; first-witness output is never treated as full support.
The timed benchmark rejects validation-secret files and regenerates pools.

The eleven protocol arms include generator/backend ablations, complete-support
and first-witness tasks, Bloom off/on, A/A and matched rho. `job.sh` requires a
fresh source/protocol-bound successful S3 receipt; it allocates no infrastructure.
The frozen native analyzer charges misses/censoring, with additional full-GCD
and generation-accounting checks. Runpod proxy access and the previously failed
artifact credentials must work before a new paid run. Old admission receipts
cannot admit this source.
The source manifest pins kernels, fixtures, launch scripts and protocol; the
enclosing Git revision and committed Cargo lockfiles pin the shared Rust build.
The development pre/post manifests also directly pin the standalone lockfile.

## Development checks

The copied no-timer qualifier records302 full-support cases,17856 exact generated
points/labels,52 rejected draws and eight malformed-input cases. The21 crossed
driver checks and two Rust unit tests passed; the native analyzer rejected both
mutated-support and mutated-generation controls. These are producer assertions
registered as PCC-20261116-a777e8, with unsigned standing. [pcc:unverified]

Raw development output and distinct pre/post manifests are in `development/`.
No new candidate timings or speed ratios exist. The existing closed Runpod
results remain separate and unchanged.

Pre-data launch correction: use `launch-v2.sh`, `bootstrap-v2.sh`, `job-v2.sh`,
`SOURCES_v2.sha256` and the `trace-incidence-accelerated-checkpoint-v2.yml`
workflow. `AMENDMENT_v2.json` records the sparse-checkout correction and explicit
standalone lockfile pin. The first launch snapshot and its manifest remain
unchanged; scientific kernels and all development evidence are unchanged.

The v2 manifest also passes in a separate sparse checkout using read-only local
Git object alternates (PCC-20261116-f44cc0). This checks source-path coverage;
object transfer and paid execution are separate requirements. [pcc:unverified]

Broad library validation is separate: it compiled, but the already-failing full
run was interrupted before completion. Retrying its41 reported failures after
tracked input restoration leaves32 passing and nine failing (archived records,
unavailable time-namespace proc files and unchanged registry assertions).
`development/broad-check-status.json` and the retry transcript retain that
limit under PCC-20261116-2d5f3f. The full suite is not reported as passing.
[pcc:unverified]
