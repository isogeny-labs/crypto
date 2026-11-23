# Primary N83 D18 cadence-32 attempt 7

**2026-11-14.** The source-bound full-D18 cadence-32 constructed control
completed in 50.414 seconds and passed separate original-group, relation-row,
and retained-assignment replay. The matched first public natural query reached
the frozen 100,000-conflict limit in 161.878 seconds of process wall, returning
`UNKNOWN_search_cap` with zero models and zero relations. The two outcomes and
the earlier failed wrapper are retained separately.

## Frozen identity and custody

The curve is primary N83 `icv1-f2m83-tm6151469093347-debefd74`, Koblitz
`a=0` over the stated GF(2^83) representation, subgroup order
`2417851639230796216685689`. The base is the full D18 direct-subgroup
generation: 5,418,738 points and 32,643 columns. Its immutable S3 object is
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/d18-full-direct-subgroup-v1/a0/objects/c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz`.

| Bound input or executable | SHA-256 or source identity |
| --- | --- |
| Producer manifest | `1d3650eabc4662d8f2597498d7ac92b7cd0a4134ac5f8155aaa49c7af3407dce` |
| S3 custody wrapper | `1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616` |
| Compressed D18 object, 160,747,529 bytes | `eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b` |
| Clean worker source commit | `45ce91d503ee0d0acc57e3d8d5cb49108c90ded6` |
| Darwin arm64 worker binary, Rust 1.93.1 | `5f302b8cfd86470e4d08da34c972308f0c8b5dbc80422dc8b578625f8e075fd2` |
| Patched verifier source blob | Git blob `1290dd0b70bc49ee6d5ab930917532517f396461` |
| Darwin arm64 verifier binary | `d597f1ba70dfe2d33bee9b19e6cbce403eb80349c2f5b3d903468804039e718b` |

A new checksum-enabled GET supplied both S3 objects. The provider metadata
reports `FULL_OBJECT` SHA-256 for each, and the locally recomputed hashes
match the frozen values. The worker was built offline from the clean source
commit; 20 focused worker tests passed. The terminal-exit verifier was built
from the exact patched blob against that library; its five focused tests
passed. The source checkout was restored clean before launch. The corrected
Darwin wrapper used `/usr/bin/time -p`, which returned zero on a preflight
`/usr/bin/true` check.

The positive preflight sampled one-minute loads 7.44, 7.15, and 6.65;
the natural preflight sampled 10.35, 9.55, and 8.56. All were below the
frozen 24 limit. Cumulative swapouts stayed 164,320 through both preflights
and arms; scratch free space exceeded 2 GiB. These are local shared-host
diagnostics under the frozen protocol, not a Tier 0 timing comparison.

## Execution and replay

| Arm | Terminal status | Process wall | Import | Model | Search | Solver | Peak RSS |
| --- | --- | ---: | ---: | ---: | ---: | --- | ---: |
| First constructed wrapper | `PRODUCER_FAILURE_MISSING_EXIT_RECEIPT` | 51.11 s from `time -p` | retained | retained | retained | SAT artifact, exit unrecorded | retained in worker receipt |
| Corrected constructed control | `PASS_CONSTRUCTED_CONTROL` and independent `PASS_NATIVE_GROUP_ROW_AND_ASSIGNMENT_REPLAY` | 50.414 s | 49.404 s | 0.145 s | 0.320 s | SAT, 1 conflict, row `[0,166,332,498]`, rank 1 | 5,625,561,088 B |
| First public natural query | `UNKNOWN_search_cap`; receipt audit `PASS_CENSORED_CONFLICT_CAP` | 161.878 s | 49.706 s | 0.134 s | 111.962 s | Unknown at 100,000 conflicts; zero models and relations | 5,593,858,048 B |

The first wrapper used zsh's read-only `status` variable for its post-process
exit assignment. It failed before writing `result.json.exit.txt`; its SAT
artifact therefore did not release the natural arm. Its raw result, model,
phase stream, wrapper and timing stderr remain under `evidence/positive/`.
The new `positive-2` directory records actual worker exit `0`. The patched
verifier passed on that unmodified result, including reconstruction and
replay of the complete original CNF/XOR assignment and generic group row.
Separate changed-target and changed-row-digest copies were rejected with
exit `1`, respectively at the subgroup and coefficient-row checks.

The natural target is fixture 0 of the frozen public corpus,
`[1834913121178197242856238, 8797964190732846213981377]`. Its terminal
worker exit is `0`; the phase stream records `started`, `import_checked`,
`model_constructed`, and `solver_returned`, with `Unknown` and
`conflict_cap` at 100,000 conflicts. The separate native relation verifier
only promotes terminal SAT or internal wall-cap records, so
[`audit_unknown.py`](audit_unknown.py) independently checks this terminal
conflict-cap receipt, public target, formula shape, exact source/input/provider
hashes, phase ordering, host gates, and absent model/relation fields. It
returns `PASS_CENSORED_CONFLICT_CAP`. This status retains the observed cap;
it is not a relation-yield estimate.

The two process-wall values include import and formula construction on this
host. They are stage diagnostics. The result's
`total_index_calculus_runtime_ms` and `selected_best_total_runtime` remain
`null`; the next discriminating experiment needs a dated solver/encoding
amendment and natural-query corpus before rank, full logs, unseen-target
descent, and same-point rho can enter a complete-cost selection.

## Retained evidence and reproduction

[`evidence-manifest.json`](evidence-manifest.json) hashes 63 compact raw files:
provider GET metadata, manifests, host samples, both constructed attempts,
the natural receipt, model sidecar, phase streams, exits, verifier output, and
tamper rejections. The 160,747,529-byte base object remains at its frozen S3
key. A full local copy is at
`/Volumes/SSD990-2/tmp/n83-d18-attempt7-20261114/input/objects/` and the
source checkout at `/Volumes/SSD990-2/tmp/n83-source-45ce-20261114`.

Run the audit against a fresh checksum-enabled GET of the same object and the
exact clean source/binary identities:

```sh
python3 audit_unknown.py \
  /Volumes/SSD990-2/tmp/n83-d18-attempt7-20261114 \
  /Volumes/SSD990-2/tmp/n83-source-45ce-20261114 \
  /Volumes/SSD990-2/tmp/n83-d18-target-20261114/release/examples/koblitz_n83_factor_base_export
```
