# Complete recovery on one RTX PRO 6000

The user requested the largest feasible complete recovery on one RTX PRO
6000. This ladder measures a bounded operational frontier, not a largest
possible field, a security claim, or a certified timing advantage.

The initial ladder uses frozen `protocol-v3.json`, hash
`36c8f17ea57cc552d84b076521f0b344fe2d2aa65cf41aa649228b0f110142b2`.
Version 1 failed before compilation because two wrapper commands were joined.
Version 2 passed the recovery/device preflight, then failed while compiling
the native chart analyzer, before any ladder data. Both archives and deletion
receipts remain under `failed-launch-v1/` and `failed-launch-v2/`.
Version 3 repairs the analyzer literal and compiles instruments with Rust
edition 2021 and warnings denied. The ladder code and inputs are unchanged.

The subsequent indexed-lookup panel is frozen separately in `protocol-v4.json`,
hash `29ff437233af158c060614d5e660a7b925d01a4edb6826bb0d945fe5cae333a6`.
It was designed after v3 completed through 52 bits and its first 56-bit cell
timed out; v3 continues unchanged. The index panel uses48/52/56/58/60-bit
fixtures, the same two seeds and limits. On48/52bits the retained binary
search (b) is compared with indexed search (u) and its identical repeat (x),
with reversed order by seed. Higher rungs use u/x. At most24executions run;
the first incomplete rung stops the ladder after all its cells are retained.
Every successful cross-arm certificate must be byte-identical.

The index stores lower-bound offsets for high x bits of the sorted full-point
table, at most65537u64 entries. Empty buckets, duplicate points, x=0,
last-field coordinates and infinity are explicitly tested. Infinity uses the
full-table lookup; finite buckets preserve the lowest original tuple id.
`ER_QUERY_BINARY_REFERENCE` disables the index; both measured services log
row counts and admission bytes with `ER_TABLE_DIAGNOSTICS`. Admission bytes
are a conservative allocation bound, not measured peak device usage.
[NVIDIA's lower-bound contract](https://github.com/NVIDIA/cccl/blob/main/thrust/thrust/binary_search.h)
and the existing full-key comparator informed this conventional indexing
change. No new algorithmic exponent or external-tool superiority is claimed.

## Implementation and mathematical checks

The standalone Rust adapter now accepts a nonzero point killed by the
declared prime group order. Deterministic u64 primality checks, the curve
equation, nonsingularity and the exact integer Hasse interval certify that
the full group order equals the declared order, provided it is the unique
multiple in that interval. Ambiguous tiny cases retain exhaustive counting;
unproved large cases are rejected. This follows the same argument used in
`crates/hashed-ecdl-tensor/src/curve.rs`, rather than trusting supplied orders.
Square roots use Tonelli–Shanks instead of an O(p) lookup, with exact parity
handling; intermediate products use u128 when needed.

The reduction still collects N+1 relations from independent slot-specific
hash bases and uses generic meet-in-the-middle decomposition on CUDA.
Elimination stores its factors and reconstructs the first dependency once
by triangular back-substitution, instead of updating N+1 provenance entries
at every pivot. An independent eager implementation checks exact dependency
coefficients in unit tests. Certificate replay accumulates the sparse
slot entries without constructing a dense matrix. The original tiny-curve
constructor and default 512-column limit are preserved. Larger column/work
limits are explicit CLI arguments, not silent overrides.

The supplied paper's §1.2/§3 separates factor-base construction, relation
collection, linear algebra and extraction. Its conditional asymptotic tensor
result does not give practical-curve speed evidence. This experiment does
not replace MITM with a tensor backend. Primary sources checked before work:
[Hasse and point-order background](https://www.hyperelliptic.org/tanja/teaching/isogeny-school21/csidh-sidh-week-3.pdf),
the repository's arbitrary-precision curve/arithmetic implementation, and
[PARI ellcard documentation](https://pari.math.u-bordeaux.fr/dochtml/html-stable/Elliptic_curves.html).

## Inputs, controls and stopping

Existing registry curves cover fields of 20,24,28,32,36,40 bits. A fixed PARI
rule generates 44,48,52,56-bit public fixtures before solve timing: at
`precprime(2^bits)`, choose the first nonsingular `a=-3,b=1..8192` curve with
prime full cardinality, then its first liftable x in 1..1000. The driver
independently certifies every order using the supplied nonzero generator.
Fixture-generation output and errors are retained. PARI receives no solver
results, and no curve is selected for observed recovery speed.

For each field, seeds 101 and 202 select public Q by the first even-y lift
from x=1009*seed. **No target scalar is generated or given to recovery.**
The same seed controls the solver's hash/trial RNG. Each seed runs GPU arms
u/x with identical binaries and inputs; their order reverses for seed 202.
CPU arm h runs on the 20/24-bit cases and must produce byte-identical
certificates. Every successful result also gets fresh-process CPU replay;
a copy with its scalar incremented modulo r must be rejected with exit 1.
Tiny known-answer and invalid-input tests run before the ladder.

All cases use eight slots, tau=2*ceil(r^(1/8))+2, max_rows=536870912,
chunk_rows=1048576, max_columns=16384, max_work=1073741824 and max_trials=16.
These are admission limits, not up-front allocation requests. GPU admission
still reserves sort workspace and leaves a quarter of free memory unused.
Independent inverse-hash replay has an explicit 2^24 candidate limit.

There are at most 44 executions. Each solve gets 300 seconds, each independent
replay 60 seconds, fixture generation 180 seconds, the worker 2400 seconds and
the controller 2700 seconds. Complete the prescribed cells of the current
rung, then stop higher rungs if any current cell times out or hits a limit.
Retain partial output and mark higher rungs unattempted. An incorrect answer,
accepted corrupt certificate, sanitizer error or unexpected process failure
invalidates the run rather than being reported as a scale frontier.

## Accounting and interpretation

`solver_seconds` starts before native public-target construction and ends
after the recovery driver exits: process startup, curve checks, GPU service
startup, all base trials, relation search, elimination, in-process replay,
serialization, flushing and child teardown are included. `wall_seconds`
continues through independent replay, the corruption check and certificate
comparison. Additional replay time is recorded separately. Raw driver logs
give base, target/check, oracle, elimination and in-process replay stages,
including discarded trials. Field-corpus construction and build/validation
costs are outside solve timing and retained in bootstrap logs.

Shared RunPod hardware earns L0 only. All wall_valid flags are false; no
statistical significance, fastest-tool or sub-rho claim is made. The native
frozen analyzer includes every attempted execution and draws incomplete
rows in red. Two fixed targets do not establish a general success rate.
The reported frontier means all prescribed cells at that rung completed
under these exact limits; a timeout is not a mathematical impossibility.
