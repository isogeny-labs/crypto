# Residual chord label transport — frozen development protocol

Date: 2026-11-25 (America/Los_Angeles). Study: `cayley-residual-transport-m31-v1`.
Policy: `isogeny-labs/experiment-seeds/v1`.

## Question and scope

For a fixed affine residual point K on an ordinary M31 elliptic curve, do
two trace-zero chord-evaluation labels define the graph of one invertible
projective transformation? Can that transformation enumerate exactly the
compatible pairs from supplied label sets and recover the signed residual
points? Can the supplied prefix factors be evaluated from public points so
the labels are evaluations of a four-point principal-divisor function?

This is a local, untimed Rust development correctness check. It does not
sample natural factor-base yield, compare runtime, search for unknown target
logs, run a complete solver, or consume paid infrastructure. The arithmetic,
geometric identities and tested component behavior determine acceptance.
The original x-coordinate Cayley-product extractor and its 2,818-case evidence
are preserved; that previous corpus is not rerun.

## Inputs and controls

Use exactly the two certified ordinary M31 fixtures in the retained source
package, separately: p=2147483647; (a,b,r,G) equal respectively to
(2147483644,21,2147487637,(1391443431,177979044)) and
(847514846,2087444724,2147446831,(625037715,2100627251)).
Copy the existing native arithmetic excerpt byte for byte and pin its hash.
For each fixture, choose the first two increasing s>=0 for which
f(s)=s^3+a*s+b is nonzero and -f(s) is a square; use the smaller positive
square root delta, so S=(s,i*delta) and Frobenius(S)=-S.

For each of four replicates and each fixture, generate 16 affine nonzero
residual targets in evaluator custody (128 total). Candidate interfaces
receive only the curve, point coordinates, certified anchors and supplied
label parameters, never a target-generation scalar.

For each target exercise four frames: distinct anchors with identity
prefix factors; distinct anchors with both factors -1; reversed anchors
with independent supplied factors; the same anchor twice with one random supplied factor
and one identity factor. For each frame check 24 slope cases: 16 deterministic
pseudorandom finite slopes, infinity, zero, p-1, tangent at -K, a slope
mapping to raw left identity, a slope mapping to raw right identity,
a withheld distinct-point decomposition, and the unique doubling
decomposition. Keep repeated cases if special values coincide.
Thus there are exactly 12,288 registered transport/extraction case rows.

Also evaluate one genuine four-point prefix per target, plus a negated-prefix
identity case, using separately authored direct Fp^2 function evaluation.
Exercise three bounded supplied-set intersections per frame: overlapping
known-compatible slopes, partly overlapping slopes, and empty/duplicate
inputs. Compare complete output pairs with the independent double-loop
reference; deliberate positives are correctness controls only.
Include malformed coefficient/point/anchor/label/projective-input cases.

## Randomness

Master seed: `0x5452414e53503331`. PRNG: Rust SplitMix64 with wrapping
64-bit arithmetic, Weyl increment `0x9e3779b97f4a7c15`, and finalizer
constants `0xbf58476d1ce4e5b9`, `0x94d049bb133111eb`; shifts 30,27,31.
Replicate keys: `0x5245504c00000001`, `0x5245504c00000002`,
`0x5245504c00000003`, `0x5245504c00000004`.
Fixture tags: `0x4355525600000001`, `0x4355525600000002`.
Stream tags: targets `0x5441524745540001`, factors `0x464143544f520001`,
slopes `0x534c4f5045530001`, positive controls `0x504f534954560001`,
prefixes `0x5052454649580001`.

Derive `key=mix64(mix64(master XOR fixture_tag) XOR replicate_key)` and
`child=mix64(key XOR stream_tag)`. These are logical stream identities;
there is no worker-dependent seed. Record all 40 child seeds and assert
pairwise distinctness before fixtures. Modulo sampling is used only for
deterministic development coverage; no unbiased distribution claim is made.
No algorithm-side randomness. Candidate/reference see byte-identical public
input records. Algorithm-side set order is deterministic (sorted BTreeSet).

## Acceptance

1. Candidate labels equal independent direct Fp^2 evaluations in every case.
2. Forward/reverse projective transport and independent label-to-slope
   inversion agree, including every registered infinity/pole case.
3. Complete residual output sets match the retained symmetric-Semaev
   reference after filtering by independently evaluated chord labels.
   Identity summands are explicit and tested separately. Every point and
   signed addition is checked with independent arithmetic.
4. Every withheld distinct or double pair is recovered. Full prefix labels
   match the direct principal-divisor evaluation and the four signed points
   add to the supplied public target.
5. Supplied-set intersection outputs equal a complete reference double loop.
   Report enumerated and membership counts as structural counts, not timings.
6. Invalid inputs fail closed; no output silently normalizes noncanonical
   input residues. The implementation admits singular curves/invalid anchors
   only as rejection tests, never as valid fixtures.
7. Two executions of the identical final binary produce identical case,
   seed and summary bytes. Freeze source/protocol before each build attempt;
   preserve any compiler or correctness failures and any amendments.

## Resource and provenance limits

One compile or execution at a time. At most 120 seconds and 1 GiB virtual
memory for a compile; 60 seconds and 512 MiB per test execution; one CPU.
Use the recovered native rustc 1.100.0, edition 2021, optimization, denied
warnings, one codegen unit and one linker thread. Compile/run an immutable
source copy. Retain prebuild, precall and postcall manifests separately,
compiler/executable identity, all raw case rows, stdout/stderr and replay
comparison receipts. The protocol may be amended only before execution or
with an explicit retained failure and new version; never erase an outcome.
No timing conclusions, statistical intervals, PCC ledger identifier, Lean
proof or external signed verification follow from this component gate.

## Prior work and ownership

Original source: crypto@eb55e93110676c91213d91d6c8d00489311a4bd8;
retained chord component in `upstream/`. PR #5232 is actively packaging that
component in another workspace and will not be changed by this local work.
PR #5229 owns five-term MITM engineering. Non-draft PR #5220 has a
materialized pair-support polynomial-norm/intersection prototype; its
compressed intersection query remains open. This work does not solve it.
Knowledge-base MCP/CLI service unavailable; read the exact prior package,
current repository instructions and relevant PR sources instead.
Rubin–Silverberg, Torus-Based Cryptography (CRYPTO 2003), section 5.2,
gives the standard rational T2 parameter and compressed multiplication.
The retrieved primary-source section is used; direct PDF opens failed.
Semaev's 2004 ePrint and the previously used mirror could not be reopened;
the retained derivation and separate symmetric reference are used and checked.
No novelty is asserted for rational torus coordinates, Mobius maps or
standard chord addition. The contribution is this explicitly scoped
residual compatibility implementation and its reproducible qualification.

## Pre-execution review amendment 1.1

Before any compilation or fixture execution, independent review identified
that certified frame fields must be private to protect cached maps, and that
a zero-ordinate residual must be explicitly rejected under this interface.
Both are corrected and a valid M31 two-torsion input is added as a rejection
control. Random/control aggregate outputs are separated; all full-prefix
controls are explicitly marked constructed-positive. The fixed seed panel,
12,288 primary cases and acceptance criteria are unchanged. A superseded
source registration is retained alongside the final prebuild registration.
