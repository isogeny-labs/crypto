# E12a terminal output preservation — 2026-10-28

This observational packet records the end of the admitted E12a attempt. **Spot
reclaim interrupted the job with exit 143; zero of the two requested target
records completed.** The retained target file is empty. No exact last fall
degree, degree lower bound, first fall conclusion or official hypothesis decision
follows from this attempt. This packet supersedes earlier statements that this
job was still running; it leaves those historical records unchanged.

| Item | Retained observation |
|---|---|
| Job | `20261028-021829-fd-e12a-r6a-20261028-r3-0c1d7f` |
| Instance | `i-0f1e979a30bc4f52b`, `r6a.8xlarge`, Spot |
| Requested cell | `(m,n,ell)=(3,21,7)`, `N=42`, cap 5 |
| Frozen sampling | seed `20261012`, first two unsatisfiable draws |
| Start recorded by runner | `2026-10-28T02:19:14Z` |
| Target command started | `2026-10-28T02:34:06Z` |
| Spot notice and command exit | `2026-10-28T12:03:18Z`, exit 143 |
| Terminal status updated | `2026-10-28T12:04:22Z` |
| Completed target records | **0 of 2** |
| Target degree result | **Not available** |

The [terminal receipt](status.json) records `phase=interrupted` and a Spot-reclaim
reason. The [job log](job.log) records the interruption notice, command exit 143
and final snapshot. [instance.json](instance.json) records the instance as
terminated; its generic `User initiated` label is distinct from the runner's
Spot-interruption cause. The original deadline was `2026-10-30T02:18:31Z`.
No replacement, retry, deadline change or remote command was performed to prepare
this packet.

## Frozen execution and preserved gates

Source commit: `371035216942660c4615dfbb88d85a14afaf5c16`.
Protocol commit: `f1e7c2d25df19da22437950cdccd2f771d831bb4`.
The retained [protocol hash](out/protocol_sha256) is
`834d442955c6540c2f74c6fb7ec417bab48cf429521d32dc796ca0c04fdcd6dc`, checked
against the exact preregistration blob at that commit. The recorded
[hky_par binary digest](out/sha256_hky_par) is
`533f182b8e72cfb6a6c174aefc8f365854fed727086cb6a212fb993bbb30872a`.
The binary itself is not included or replayed by this packet.

The frozen wrapper invoked:

```text
hky_par newcells --cells 3:21:7 --seed 20261012 --keep 2 --cmax 5
```

The producer's [run log](out/run.log) reports gates passed before that command.
[gate0.log](out/gate0.log) records three passing native counter tests.
[gate1.log](out/gate1.log) ends `AGREE 24 DISAGREE 0`. The four saved gate JSONLs
each have four objects, and their saved diff files are empty. These are retained
producer gate outputs, not an independent gate or closure replay. The separate
[recovered gate packet](../recovered-a-gates-20261028/README.md) retains the seven
gate/input JSONLs that had been outside the uploader's output-directory coverage.
That recovery remains a separate, immutable evidence record.

[out/E_3-21-7.jsonl](out/E_3-21-7.jsonl) and its error file are both zero bytes.
[out/status](out/status) contains only `gate_passed`, and `out/cells_status` is
absent. The files establish that the wrapper started the requested command;
they do not identify how many internal candidate draws or closure steps were
attempted before interruption. Neither requested draw has a completed record.
The registered degree-5 reading therefore cannot be evaluated from this attempt.

## S3 durability and coverage

The canonical bundle is already persisted at:

```text
s3://setec-astronomy-artifacts/research/fall-degree/e12/terminal-a-20261028/3030d78c8e0a21bbd9c774294b338586cb58a136565e4fd35c5da4678bd09ba9/terminal-evidence.tar.gz
```

It is **14,706 bytes**, SHA-256
`3030d78c8e0a21bbd9c774294b338586cb58a136565e4fd35c5da4678bd09ba9`.
[canonical-durability.json](canonical-durability.json) records the conditional
PUT, fresh GET, byte comparison and contained-hash verification at
`2026-10-28T12:26:22Z`; the [PUT](canonical-put.json) and
[GET](canonical-get.json) responses are retained. This packet locally compared
the existing upload and readback copies; it did not issue another cloud request.

The original [output.tar.gz](output.tar.gz) is preserved byte-for-byte:
**6,432 bytes**, SHA-256
`7e614253f5abcffe3b14ba8c4c80a62ddbbbf55924f068f8b9eafb999ff9db6b`.
Its [snapshot manifest](latest.json) identifies the legacy read-only archive URI
and `2026-10-28T12:03:42Z` creation time. The archive has 28 members: one
directory and **27 regular files**, including 25 under `out/`, plus `host.txt`
and `rusage.txt`. All 25 output files are also copied here without modification.
The nine-member canonical terminal bundle preserves that original archive and
the status, log, resource-usage, instance, legacy GET and hash-list evidence.

These are durable **output snapshots, not solver-memory checkpoints**. No
in-flight degree-5 closure basis or complete process state is present. The
receipt's generic phrase `resume from this job's outputs` does not establish
exact solver resumption. Frozen Amendment 13 explicitly states that these
engines have no mid-closure checkpoint and that interruption ends the attempt.
This packet does not authorize a restart or convert missing intermediates into
persisted data. The separate E12c report's approximately 30 GB closure basis was
in-memory state; no such basis checkpoint file was recovered by this E12a packet.

The [resource-usage record](rusage.txt) says the wrapper terminated by signal 15.
Its trailing `Exit status: 0` does not supersede terminal exit 143. The small
wrapper CPU totals are not complete solver resource accounting. No timing,
performance, billing or cost-efficiency comparison is made.

## Preservation checks and limitations

[manifest.json](manifest.json) binds the copied evidence;
[checks.json](checks.json) records the scope of local checks.
[evidence.sha256](evidence.sha256) also binds this packet's derived notes.
From this directory:

```sh
sha256sum -c evidence.sha256
sha256sum -c source-SHA256SUMS
jq -e '.phase == "interrupted" and .exit_code == 143' status.json
jq -s -e 'length == 0' out/E_3-21-7.jsonl
```

Hashes, archive membership, JSON parsing, copied bytes and completion markers
were checked with native tools. **No solver run, gate replay, independent
algorithm validation, formal `review-run`, Lean verification or schema-validated
experiment export is supplied.** The historical run used frozen Markdown
amendments. This observational preservation does not promote it into a validated
computational claim. Existing degree data and graphs remain unchanged.

Sources checked before writing the packet:

- [Frozen Amendments 11, 13 and 14](https://github.com/isogeny-labs/crypto/blob/f1e7c2d25df19da22437950cdccd2f771d831bb4/research/fall_degree_bounds_20261008/PREREGISTRATION.md): two-draw target, cap 5, ordered capacity alternatives and terminal interruption policy.
- [Frozen run wrapper](https://github.com/isogeny-labs/crypto/blob/371035216942660c4615dfbb88d85a14afaf5c16/research/fall_degree_bounds_20261008/e12/run_par.sh): command, gates and completion markers.
- [Persistence contract](../continuation-20261028/S3_PERSISTENCE.md): saved files versus solver checkpoints.
- [Recovered E12a gate evidence](../recovered-a-gates-20261028/README.md) and [interrupted E12c evidence](../interrupted-c-20261028/README.md): distinct retained records and coverage limits.

Delivery status and remaining checks are in [HANDOFF.md](HANDOFF.md). The total
USD 200 campaign cap is unchanged; this preservation adds no compute allocation.
