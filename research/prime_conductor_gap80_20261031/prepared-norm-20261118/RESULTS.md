# Prepared native context: correctness outcome

The optional `--prepared-norm` backend matches the existing quotient and
point images in **21 pipeline replays, 483 point-input evaluations and four
fields**. Native primitive controls pass **768 direct-root/reuse evaluations,
384 collision controls and six invalid-modulus rejections**, using 96 prepared
moduli across three fields including the actual 96/202-bit fields. Default
output bytes and existing stage counters remain unchanged; 18 CLI rejection
controls pass. These are producer development observations, not elapsed-time
or memory measurements. [pcc:unverified]

PCC-20261118-2521f3 and summary-final.json carry the final corrected
interpretation. PCC-20261118-ef4b3c and summary.json retain the earlier process
wording; native correctness results are unchanged. Producer standing remains
unsigned, unverified/not_covered. [pcc:unverified]

## Implementation and costs

PreparedNorm borrows a fixed monic baby polynomial h. It prepares its reversed
inverse and trace functional once per baby block, caches the field inverses
2/3, and reuses scratch polynomials across jet evaluations. The same established
log coefficients u, v-u²/2 and w-uv+u³/3 are traced. gcd collision detection
and all source, degree, coverage and destination checks remain intact.

The typed path uses FLINT objects directly and avoids the per-evaluation text
encoding/parsing, field retest, context setup and h-only preparation performed
inside the existing bridge. The old textual path remains the default; no
existing method is silently replaced. The preparation/evaluation diagnostics
are local counters, not total field-operation accounting. [pcc:unverified]

The two degree-dependent cache polynomials and reusable scratch remain live
through subsequent tile building. Peak RSS and elapsed time remain unmeasured. The complete target's window size, rectangle count, kernel
walk and arithmetic obligations stay unchanged. This does not establish a
feasible complete 80-bit construction schedule.

## Actual validation and failures

| Check | Outcome |
|---|---|
| Native CLI and primitive checker | Builds with warnings denied |
| Original fixture | Bytes equal to the retained default, including tiny known-answer recovery controls |
| Full map schedules | Seven schedules, each affine and caps 7/64; old/new default bytes equal; prepared mathematical output equal |
| Independent degree-10009 reference | Completed attempt-a PARI baseline reused for the identical source/kernel/point panel |
| Other full map references | Fresh PARI for each small-degree case, including the 96-bit degree-5 field |
| Option/input gates | Default and prepared versions of eight old rejections; duplicate/unknown option reject |
| Whole qualifier A | Exit 124 at its 300-second cap; primitive/default and two baseline/reference outputs completed |
| Qualifier B | Exit 0 under the same cap; references reused explicitly, no mathematical source change |

The native output leaves pari_direct_reference false wherever no fresh
reference call ran. The degree-10009 results match the complete input,
source/kernel, quotient, coverage and point-image projection of the pinned
PARI record. This is not a fresh independent expansion for every schedule.
Source/input/executable hashes match before and after. [pcc:unverified]

The frozen POSIX driver shares a label variable across functions, so later
filenames accumulate suffixes. Actual native walk metadata confirms each
of the seven groups ran modes 0/7/64. Raw names remain unchanged. The current
driver isolates run_mode in a subshell to correct labels; it is syntax-checked
but was not used to produce these reported results. No native arithmetic test
was repeated for that presentation-only driver change. [pcc:unverified]

After the cap, a process listing without state was initially interpreted as
an active orphan. The explicit state check showed Z (defunct), before an
unnecessary kill; it remained Z. summary-final.json and the additive correction
preserve that distinction. No active computation beyond the cap is established.
No certificate, target or benchmark job is now active. [pcc:unverified]

## Reproduce and storage

Use the pinned old projective evaluator for the baseline, the retained source
copy for historical manifests, and the source/header/helper-object hashes in
pre-call.sha256. Compile native/complete_pipeline.cpp and check_prepared.cpp
with C++17, FLINT 3.1.3, PARI 2.17.2 and GMP, as in the prior evaluator build.
Keep projective_walk.hpp and prepared_norm.hpp beside the main source.

Both frozen qualifiers accept repo, candidate, baseline, checker and a fresh
output directory. The reuse driver requires attempt-a's completed reference
records. qualify-current.sh only corrects future output labels; its outputs
are not claimed here. No performance session is launched by these drivers.
The worker Dockerfile copies/hashes both headers, but no new image has been
built or published. The earlier image cannot attest to this source.

The complete raw tree is in runs.tar.xz; check runs.tar.xz.sha256, then
extract with tar -xJf runs.tar.xz from this directory. Named PCC/replay/manifest
subjects remain loose under the path-resolution exception. All other raw
outputs retain their bytes in the deterministic archive. The earlier large
wide-harness archive still requires inaccessible S3 publication. [pcc:unverified]

## Original goal status

| Requirement | Status |
|---|---|
| Minimum 80-bit certified prime endomorphism-conductor crossing | Open; source/kernel unchanged, target quotient and images absent |
| Faster complete construction | Optional repeated setup removed; actual time/RSS and full work feasibility unmeasured |
| Several fields and dozens of scalars | Development controls only; counts include repeated and exceptional records |
| Endpoint scalar/ECDLP/IC measurements | Open; no new performance session; wide IC/reference admission missing |
| Publication | Existing draft PR updated; prior S3 blocker retained |

The next construction obligation remains an affordable complete schedule with
actual quotient/non-kernel images. The separately certified 4774-bit Frobenius
prime stays a distinct arithmetic result. No conductor-gap, performance or
leaderboard row is promoted by these controls.

Sources: immutable scope README.md and CONTINUATION.md, native optimized.c,
existing direct-root oracle and supplied cases, FLINT public reference and
installed headers. KB unavailable (wrapper 127). Earlier graphs stay pinned
because their source versions/counts did not change.


The immutable pre-call README retains one keyword-only prose-lint finding on
its statement that no elapsed advantage is assumed. It predates execution and
makes no positive performance claim; its pinned bytes are preserved. Final
result prose and PR text are checked separately. [pcc:unverified]
