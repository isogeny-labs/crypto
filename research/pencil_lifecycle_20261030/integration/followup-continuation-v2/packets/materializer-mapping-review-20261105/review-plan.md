# Preread review plan: materializer mapping correction

Scope: independent source-only delta review of the forthcoming published materializer correction. Candidate source has not been read at plan creation. Parent disclosed the native git ls-tree exit 128, a 4,680,295,288-byte pack and 4,294,967,296-byte RLIMIT_AS. Proposed direction is removing RLIMIT_AS while preserving hard two-CPU/four-GiB/no-swap cgroup controls. This is disclosed nonblind motivation, not an independently rediscovered failure cause.

Prior: virtual-address-space limits can prevent file mapping even when resident memory fits. Removing that limit may permit mapping but does not establish that materialization will fit resident-memory limits or succeed. No runtime conclusion follows from this source review.

Reviewer owns all delta joints:
1. Address-space versus resident-memory semantics. Attack: a large mapped pack consumes address space without proportional resident pages; inspect what is removed and avoid equating mapping size with RSS. Consult Python resource documentation and Git pack mapping configuration.
2. Hard envelope and fail-closed admission. Attack: direct child entry without cgroup could remove the only memory bound. Trace verified cgroup constraints and launcher bindings, two CPUs/four GiB/swap zero, timeout and ownership.
3. Regression and custody. Attack: a new directory/source revision silently reuses old artifact receipts or loosens hash, path, destination or cleanup checks. Compare exact delta with published predecessor and retain all safeguards.
4. Attempt accounting. Attack: correction document claims the spent attempt is reusable or self-authorizes retry. Require source-only state, used attempt retained, no fresh execution authorization, original deadline preserved.

Countermodels: no cgroup; mismatched memory limit; stale publication pin; child retaining a stricter inherited hard address-space limit; a mapping larger than the virtual limit but low resident pages; resident allocation exceeding cgroup limit. These are source-trace cases only, not executed tests.

Independent derivation: 4,680,295,288 minus 4,294,967,296 = 385,327,992 bytes of pack size beyond the stated address-space bound, before other mappings. This is a comparison of disclosed values, not a measurement or proof of native root cause.

No tests, compiler, materializer, or mock execution. Wait for exact GitHub-published source commit before inspection. Record line-linked observations, unresolved runtime checks, and separate machine/not-verified and source-fidelity dispositions. Do not promote to validated execution. Preserve original failures. KB unavailable per coordinator; use exact primary source/documentation and disclose that limitation. Shared branch/index integration remains coordinator-owned.
