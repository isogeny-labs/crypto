# Bounded production contract

This extends PRIME_QUADRATIC_DESIGN.md without changing any of the42 retained
prime input identities. All seven m3–9 templates are attempted independently.
The same full three-family m2–9, fixed-base and elite-base objective remains.

The exact mathematical map is unchanged. Before elimination, every column
must be constructed. A partial coefficient matrix is never treated as the
original map, even if its current apparent rank is large. Full column rank
of the complete map plus source validation and pair-covering gives the
conditional lowerFFD7 certificate. Deficiency or a cap gives no new bound.

## Deterministic caps and retained state

Production limits per template:100,000 columns,5,000,000 coefficient entries,
50,000,000 scalar reduction updates,2,000,000 stored basis entries. Before an
operation would exceed a cap, stop at its boundary. Matrix-entry accounting
counts exact nonzero coefficients admitted in completed columns. A computed
column that does not fit is retained as pending, not admitted. For elimination,
retain the current reduced row and next reducer or insertion when capped.

An append-only JSONL journal records identity and limits, every completed
coefficient column, matrix completion, each completed elimination row with
its pivot if any, pending work, and terminal status. Flush after every event.
Completed pivot rows and all coefficient columns suffice to reconstruct the
basis; partial candidates are retained separately in events. Replay recomputes
the deterministic operation stream and compares every event and the result.
Truncated or extra events fail full replay. A hard interruption leaves an
unverified prefix, never a successful replay or mathematical certificate.

Each template has a separate native process with8GiB address-space limit,
300seconds plus5seconds termination grace. A producer failure prevents its
fresh replay but does not skip later m values. Successful complete or soft-capped
results receive a fresh replay under the same limits. The controller records
exit codes, timeout flags, result/journal hashes and non-attempted replay125.
Source assembly then checks all42 inputs; only templates with both successful
processes and full-rank certificates can provide the new lower bound. All
other case rows retain null new bounds and their template outcomes.

One process runs at a time. Two120second source-assembly passes and uploads
fit within a4800second outer limit. Periodic S3 upload preserves partial files;
the final manifest and content-addressed archive seal the complete output.
No automatic retry, performance claim or assumption that a cap is a degree.
Existing lower6/upper17 evidence is immutable and survives an inconclusive
new case. New combined intervals must match source/base hashes first.

## Controls and retained limitations

Native controls cover all four cap reasons, deterministic capped replay,
equality with the prior unbounded certificate for a deficient and full-rank
small-prime map, and rejection of truncated/extra journal records. Retain all
previous algebra/source controls. Runner controls inject a nonzero exit and
an actual timed-out process and verify later templates still run, with both
partial journals retained; missing executables are recorded explicitly.

Primary definitions remain Caminata–Gorla https://arxiv.org/html/2112.05579;
the written covering/regular-sequence proof is in PRIME_QUADRATIC_DESIGN.md.
Independent scientific review and full formalization are still pending.
