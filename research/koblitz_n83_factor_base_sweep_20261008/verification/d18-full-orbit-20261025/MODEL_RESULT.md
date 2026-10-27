# Complete D18 m4 model-construction result

The source-pinned [`MODEL_PROTOCOL.md`](MODEL_PROTOCOL.md) gate passed on the
primary Koblitz `a=0` GF(2^83) public fixture zero. The standalone worker
was built from clean commit `968553675b9addbca1269e545098c510a3d86b41`,
binary SHA-256
`7aefc2aa5f81fa47555ecea57271b7b9c46a88a0ab742f1a9df561ee40f4b7d2`.
It re-imported the exact S3-custodied 32,643-column, 5,418,738-point archive
and reconstructed its full signed-Frobenius orbit maps before making one
four-summand chained-S3 identity-mask-zero model. The result and a durable
input receipt are [`model-only-20261025.json`](model-only-20261025.json) and
[`model-only-20261025.json.preflight.json`](model-only-20261025.json.preflight.json).

| Measurement | Observed |
| --- | ---: |
| Analytic admission variables | 329,726 |
| Analytic domain clauses | 11,238,656 |
| Actual SAT variables | 329,726 |
| Actual **total** CNF clauses | 11,424,742 |
| Actual XOR rows | 3,901 |
| AND gates / S3 nodes | 62,001 / 3 |
| Full archive import | 65.908 s |
| Model construction, after import | 0.510 s |
| Whole process wall | 68.19 s |
| Maximum resident set size | 3,556,982,784 bytes |
| Peak memory footprint | 5,311,369,392 bytes |

The raw [`/usr/bin/time -l` record](model-only-process-20261025.txt)
supports the process figures; the [host receipt](model-only-host-20261025.json)
records a 48 GiB arm64 macOS machine and a 600-second outer cap. This is
shared-host L0 evidence without a hard memory cgroup. The committed source
and binary identities appear in the worker and host receipts. The model
completed normally before the cap; neither SAT search nor a group relation,
rank update, linear algebra, target descent or same-target rho ran. The
11,424,742 total clauses include the 11,238,656 domain clauses plus the
field-arithmetic circuit; using either count as a search-time estimate would
be unsupported. `selected_best_total_runtime` remains `null`.

The measured peak leaves room under the [frozen Linux 32 GiB gate](NATURAL_PROTOCOL.md),
but this local result does not establish the memory footprint or success of
a long SAT search. The next result must run the positive control first, then
the same-source public natural arm under the specified hard cgroup and
independent replay before any useful-rank or runtime inference.

A subsequent [L0 planted-control run](l0-positive-20261025/RESULT.md) used
the complete pinned circuit and reached its 300-second whole-process cap
without a solver answer. It leaves the Linux positive-control gate open.
The [shared-root opt-in model and planted control](shared-root-20261025/RESULT.md)
remove 8,617,488 clauses exactly, but the latter is also censored at 300
seconds. The Linux positive-control gate remains open.
An [additional selector-and-phase pinned control](root-pinned-20261025/RESULT.md)
returned a checked planted row with independent group replay; the original
free-selector Linux positive-control and natural gates remain open.
Two [single-dimension choice controls](choice-pins-20261025/RESULT.md) also
returned the same checked row: phase pinned with selectors free took 0.239 s
of SAT search; selectors pinned with phases free took 16.589 s. Both are
constructed L0 runs, with full import and process costs retained separately.
The [phase-first free-choice constructed control](phase-first-20261025/RESULT.md)
subsequently returned the same independently replayed row with all selectors
and phases admissible. Its resource-accounted repeat took 1.584 s of SAT
search and 79.903 s worker wall; the earlier unrestricted shared-root policy
was censored at 300 s. Natural yield and end-to-end IC cost remain open.
