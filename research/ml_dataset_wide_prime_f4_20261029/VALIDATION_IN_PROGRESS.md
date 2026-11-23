# Wide-prime F4 adapter validation — in progress

Validated runtime for the current compile and focused tests:
`fe7ccef6eb7f640fc8e95212471b80700d3cf96b`.

The bounded native engine reuses the existing generic F4/Fp64 implementation;
source and literature/provenance constraints are in DESIGN.md, IMPLEMENTATION.md
and ADAPTER_CONTEXT.md. This is bounded supervised prime m=2 software support,
not higher-m support, a fall-degree result, a performance claim or a full corpus.

## Executed checks

| Focused group | Passed | Failed | Ignored |
|---|---:|---:|---:|
| Generic F4 | 15 | 0 | 0 |
| Fp64-related | 5 | 0 | 0 |
| Dataset modules | 225 | 0 | 1 |
| Checked collector | 26 | 0 | 0 |
| Total | 271 | 0 | 1 |

The executed dataset controls include fixed n47/n53 positive native invocation
on both twists, n61 signed-domain no-lift, exact/one-less extraction work,
original deadline preservation, rehashed context overflow, sink/payload failure
retention and central semantic tampering controls. The no-lift expectation uses
the fixed domain P,-P and target3P in a subgroup of order greater than5; native
unit derivation alone does not certify absence.

The full library suite passed5388 tests, with0 failures,123 ignored and0 filtered,
in1648.41 seconds, exit0. The immutable executable hash remained unchanged.
The release CLI built from this runtime; its
SHA256 is a4ad63da4d7c1f89835c1aa51af37968ae7582cf60dea2a9acf9e2b51bde914d.
Strict release Clippy passes at59227cd2220 after rewriting the equivalent u64
range predicate as Range::contains; the only source delta is that expression.
The original two style diagnostics are retained. No CLI-fixture, current-head CI, merge, or scientific generation pass is claimed.
The first combined compile failed because sparse checkout removed42 include
references and a closure needed an explicit Result<(), String> annotation.
The failed logs are retained. The corrected driver restores126 tracked build
inputs and verifies their SHA256 digests after Git integration and before build.
The earlier native-only compile had two Option/u64 test assertion errors; these
were corrected and its later20 focused checks passed separately.

## Independent source review

The final reviewed donor is b1abc32f6e279949e2e1f6e15cbf39018c45a80b.
Integrated execution/mod/solvers/prime_f4_wide file hashes match that review.
The report SHA256 is
`2f2c9803dad1fdeb8cbf624502286b7147ac5474adb623c06d28823c7086eb0a`.
Review preserves the original findings and every correction boundary. It found
no remaining concrete source blocker within scope; it is not an executed test,
basis-derivation proof, whole-worker instruction count or historical timing proof.

## Dataset and export scope

Frozen CLI draws remain pending. The committed v5 contract selects seed42, n47/53/61, both twists,
m2 and factor-base size2 with bounded runtime and no retries or resampling.
Construction/pre-entry censorship fails the six-cell invocation gate honestly.
Current export is the existing static structural view. Canonical equations,
basis, events and extraction remain restricted source artifacts; public wide
polynomial features are not implemented. The retained consumer supports generic
hashes, phases and positive PDP certificates, not independent wide extraction
or negative-claim replay. No scalar recovery or full execution/cost claim.

## Durable evidence

Focused executable/logs/control receipt/source reference:
s3://setec-astronomy-artifacts/ml-dataset/20261029/validation/wide-adapter-focused-fe7ccef/b46f889e3f248fcd8f33ddc18dad368eed61611d95bc90afec4f1ad0f188bbe7/focused.tar.gz

SHA256 `b46f889e3f248fcd8f33ddc18dad368eed61611d95bc90afec4f1ad0f188bbe7`; 43795150 bytes.
Conditional PUT, HEAD, ETag-bound fresh GET, byte comparison and contained-file
hash checks passed. Full-suite and CLI results will be recorded separately when
terminal. Source checkpoints and failed attempts are retained, never overwritten.

No new cloud worker, IAM change, budget increase or deadline extension.
Original total ceiling remains USD200; scientific cloud cells remain0.
