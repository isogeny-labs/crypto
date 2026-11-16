---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Exact native calibration against Lean controls

Both frozen v2 cells complete and match their pre-data Lean counts.
Rust and a separately implemented C++ checker agree on every actual
polynomial pair, Cantor addition, complete group table and distinct cyclic
subgroup. This is same-host, different-language checking. Scientific PCC
standing remains unsigned/unverified. [pcc:unverified]

| Recorded value | F3 positive | F5 all-smooth control |
|:--|--:|--:|
| Complete reduced pairs | 8 | 16 |
| Split-u pairs | 7 | 16 |
| Ordered Cantor additions checked | 64 | 256 |
| Associativity triples checked | 512 | 4096 |
| Distinct cyclic subgroups checked | 8 | 16 |
| Proper nontrivial unequal subgroups | 7 | 0 |

The fixed equation is $y^2=x^5-x$ in both fields. Identity is included.
The complete count and splitting controls are Lean-backed; full geometric
Picard identification is separate. No typical-subgroup, asymptotic,
independent-machine or security conclusion follows. [pcc:unverified]

![Recorded counts from export-v2.json, one execution per field. These are complete exact populations, so no sampling interval is claimed.](counts.pdf){width=87%}

The raw records, C++ summaries, exact TSV, native analysis, launch logs,
host facts and file hashes are in `../../evidence/calibration/`.
Canonical v2 check-export with file verification passes with zero findings.
No separate-runner review has been filed. [pcc:unverified]

\newpage

# Protocol, flow and failures

Native source `11100a8512bac3840bf1f85b1e7f0f523cd78464` was committed
before affected calls. Rust binary SHA begins `851c5594fc67`; the C++
binary SHA begins `8030184af28b`. Exact source, compiler, dependency and
binary pins are in `../../native-census-v1/sources.json`. Each cell runs
Rust census, C++ verification, then source-frozen Rust analysis; JSON must
regenerate the checked TSV byte-for-byte. [pcc:unverified]

The first frozen protocol (v1, `f2572b4d`) failed in the launcher because
`/usr/bin/time` was absent: exit 127, zero native curve calls. Its second
cell was not attempted under the frozen failure rule. Empty stdout, error,
exit status and the failed version remain retained. This was an
infrastructure failure, not a mathematical negative. [pcc:unverified]

Version 2 replaces only that timing dependency with Bash builtin `time -p`
and fresh output directories. It was frozen at 22:17:41Z, hash
`400a9c96380841436a15a857ead2893f8c66c79c8d99422a6792bf9b8419369c`,
committed locally as `7c1dac39` and published as `9ee9f91b` before calls.
Arithmetic, fixtures, seeds, thresholds, stopping rules and 120-second
per-cell caps were unchanged. [pcc:unverified]

| Flow | v1 failed launcher | v2 calibration |
|:--|--:|--:|
| Planned cells | 2 | 2 |
| Launcher attempts | 1 | 2 |
| Native curve calls | 0 | 2 |
| Completed cells | 0 | 2 |
| Unattempted cells | 1 | 0 |

The v1 export records attempts=0 for its cancelled cell. The canonical
schema requires attempts>=1 even for that case, so the retained v1 export
fails that single schema constraint. The accurate custom launcher-failure
receipt is retained; no attempt is invented to make validation pass.
The v2 export passes all schema/protocol/file checks. [pcc:unverified]

Metadata-only seeds are 202611090301 (F3) and 202611090501 (F5), replicate
zero. Both used one thread on host `762f5fc120c9`, local capacity with
zero paid compute, isolation L0. Diagnostic elapsed times were 0.07 and
0.08 seconds respectively; both clocks are invalid as scientific timing.
No inference or performance comparison uses them. [pcc:unverified]

The next separately frozen protocol exhausts all monic quintics in the
two fields with two deterministic repeats. This calibration does not
replace that population census or the original fourteen-item audit.
[pcc:unverified]
