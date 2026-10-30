# Exact order-witness and residual arithmetic screen

This complete diagnostic panel compares candidate `19748cd3248d` against the
unchanged optimized baseline `2bd4588fba60`. All 120 executions completed:
108 measured and 12 warmups, with byte-identical certificates across arms,
known-scalar checks, independent replay and corrupted-scalar rejection.

| Fixture | Baseline median ms | Candidate median ms | Baseline A/A median ms | Derived baseline/candidate |
|---|---:|---:|---:|---:|
| 8-bit, 3 slots | 0.471432 | 0.225746 | 0.467688 | 2.088 |
| 16-bit, 3 slots | 6.777632 | 2.067202 | 6.804898 | 3.279 |
| 16-bit, 4 slots | 7.142432 | 2.484374 | 7.134884 | 2.875 |

Host: Intel Xeon Platinum 8488C, one software thread, 2 vCPUs and 4 GiB
reserved. These are L0 shared-host diagnostic medians, not certified speedups.
The further 10x goal is not met. Different hosts served different panels;
compare only matched arms within a panel, never absolute times across panels.

The [frozen protocol](protocol.json) has hash
`0e878acaf6d00b2ca54b3db91e2921fd9d93f69cd8342ef4b5cdedec5d0843e2`.
The [native summaries](results/summary.csv), [raw rows](results/rows.csv),
[checked export](results/export.json), [build identities](results/build.json),
[terminal receipt](status.json), [archive receipt](results-receipt.json) and
[inspected report](../reports/round4/report.pdf) preserve the complete run.
The analyzer remains the frozen native Rust implementation. The Python adapter
only packages recorded rows and hashes; it performs no numerical analysis.

The complete timer includes constructor/known-target setup, all table setup,
discarded trials, relation collection, elimination, internal and second replay,
and serialization. It excludes process launch, archive I/O and the negative
control. No check or phase was removed to obtain the recorded ratios.

Constructor time is now about 0.009 ms on the 16-bit fixtures. On-demand roots
increase base enumeration and replay relative to lookup-table construction.
Candidate bases/replay/oracle medians are respectively 0.220313/0.333475/0.586966
ms for three slots and 0.244822/0.385913/0.823394 ms for four slots. These
observations motivate the next candidate; they do not measure that candidate.

## Next candidate: implemented, unmeasured

The next changes retain field-dependent Tonelli-Shanks parameters once per
curve and share the input exponent across root/residual/Legendre calculations.
They also batch the additions constructing each MITM sum-table row. Setup
remains charged and tuple order is unchanged. The source uses standard
square-root methods from [HAC Chapter3, Algorithms3.34/3.36](https://cacr.uwaterloo.ca/hac/about/chap3.pdf),
read before implementation, and standard Montgomery batch inversion.

All 25 release tests and strict Clippy pass. An additional control compares
cached roots against the retained uncached algorithm exhaustively through
p=65521 and on sampled 40/61-bit residues. Existing all-target tuple tests and
archived full-certificate checks pass with batched construction.

No new timing result exists for these changes. Freeze a fresh panel against
baseline2bd4588fba60; do not ratchet the further10x denominator to a later
candidate. Strict isolation/review, matched rho/external comparisons and the
20–40-bit public-target scale ladder remain unresolved.
