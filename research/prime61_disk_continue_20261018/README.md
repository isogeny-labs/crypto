# Prime61 checkpoint continuation, 2026-10-18

This study continues the public synthetic `p=2^61-1` experiment from the
verified Drive checkpoint, following merged PR #1977. It preserves the
requested exhaustive horizontal-class goal. The bounded batches below do
not establish full-class coverage or a class-wide factor-base ranking.

## Recorded continuation

| Expanded sources | Discovered curves | Verified maps | Measured curves | Committed streams |
| ---: | ---: | ---: | ---: | ---: |
| 32 (inherited) | 250 | 480 | 64 | 768 |
| 64 | 452 | 960 | 128 | 1,536 |
| 128 | 688 | 1,920 | 256 | 3,072 |
| 256 | 1,344 | 3,840 | 512 | 6,144 |

The final population completed **614,400,000 target trials, with zero
relations and zero solves**. Of these, 5,376 streams and 537,600,000 trials
are new since the restored checkpoint. The 192 replay-control streams are
not added to either population. All native phase commands completed within
their frozen limits; zero kernel failures were recorded.

The [final verifier](results/verification-256.jsonl) rechecks all 3,840 maps,
all 256 expanded-source rosters and 30,720 scalar transports. The separate
[prefix audit](results/immutable-prefix-256.json) matches all 1,500 inherited
immutable logical rows; the final dump has 11,330 rows. The component is
still open, with **1,088 discovered sources pending**. Among the discovered
curves, 832 remain unmeasured. No complete-class ranking is established.

Maximum worker peak RSS over all retained native phases is **18,396 KiB**,
about 18 MiB. Final live database size is 3,387,392 bytes; its compact snapshot
is 3,371,008 bytes, compressed to 1,078,580 bytes. These are observed capacity
values on this prefix, excluding compiler/test/audit/renderer processes;
they are not a full-class upper bound or a wall-speed comparison.

All three new batch archives were uploaded, metadata-read back, downloaded,
manifest-checked and compared byte for byte. The final archive is 1,965,257
bytes, SHA-256
`f4aaf0cba58693ac835d1d87ff0ecf9b6e0053a9bc72d3a0f89709086314f7f3`.
The downloaded final database also produces the exact original native
logical dump. Sanitized receipts:
[64](results/drive-64-receipt.json),
[128](results/drive-128-receipt.json),
[256](results/drive-256-receipt.json).

| Requested requirement | Status in this continuation |
| --- | --- |
| Continue from saved progress | Verified fresh-workspace restore, collector replay and immutable-prefix controls |
| Retain all horizontal-class workloads | Partial: 1,344 discovered, 256 expanded, 512 measured; full class and selected-degree generation remain uncertified |
| Several vertical maps if they exist | None over this base field for this fixed class, by the preceding fundamental-discriminant audit |
| Recoverable Drive storage | Verified for all three bounded batch archives; full-class quota remains unverified |
| Publish code, results and report by PR | Native scripts/helpers, raw archive, SVG, inspected two-page PDF and receipts included |

Pinned-source validation passed **4,168 library tests, zero failed,
117 ignored**, plus all four disk-worker tests and the existing evaluator
test. The first full library invocation retained six missing-fixture
failures; both tracked runtime fixtures were materialized, and the entire
suite was rerun successfully. See
[initial log](results/prime61-frozen-library-tests.log),
[final log](results/prime61-frozen-library-tests-final.log), and
[example tests](results/prime61-frozen-example-tests.log).
The separate [current-main recheck](results/prime61-current-main-recheck.log)
fails compilation; pinned-source success is not a claim that main passes.

## Frozen execution

[PROTOCOL.json](PROTOCOL.json) and the unchanged [spec.json](spec.json)
were committed as `72c21ed` before execution. Cumulative targets are
64/128/256 expanded sources and 128/256/512 measured curves. Each measured
curve keeps twelve seeded streams, three inverse pairs, twenty requested
relations and a 100,000-target cap per stream. Each native command has a
300-second outer limit; failures and unfinished work stay explicit.

Current-main source `e063d98475589d9aba7ae265a8cba830927e3156` failed
compilation in unrelated ecbench code. The initial sparse build also lacked
tracked include fixtures; those were materialized before the definitive
release-library recheck, which still exits 101 with 26 compiler errors,
including duplicate `Fp4` variants and unresolved `ecbench::fbm`.
No unrelated library source was patched.

[AMENDMENT.json](AMENDMENT.json), committed as `bfbef28` before new collection,
pins the exact previously validated worker source
`b68341cd72b51acf9c5ce6c4f755351cc83053db`. It is built in an isolated
detached worktree with Rust 1.99.1 and system `libsqlite3.so.0`. All continued
maps and runs use that one source revision. Current-main test failures and
pinned-source validation are reported separately. The evidence PR is based
on current main and changes no library code.

Exact parameters are in the spec: `a=p-3,b=111`, prime order
`r=2305843010818082053`, `G=(1,1796364306280342617)` and
`Q=(992726311114892094,558260110083040833)`. The scalar 539365399 is public
synthetic checking data, not an input to relation collection. Degrees are
`[3,7,11,13,19,23,53,61]`; seeds are `2026101600..2026101611` on each curve.
One first-parent transported workload is measured per discovered class.
Full measured EC1/UIDs and transported coordinates remain in the logical
records; registry/candidate promotion is pending.

## Recovery and correctness controls

The execution workspace had been reset: no earlier checkout, database,
binary or toolchain remained. The archived checkpoint was downloaded through
the connected Drive and checked before extraction. Its SHA-256 is
`eb5ee1c252943ddeae9d62f49e3065e6b2e5f06e20a2c8598d804f8fc35dec32`;
all eight internal manifest entries passed. The original archive remains
unchanged. Its logical dump SHA-256 is
`6e4c6158c9eee81d583a42eee822573fc1e6a29709a00bb3875f33f3da21c0fd`.

The native verifier checks all saved maps, subgroup/point identities,
expanded-source root rosters and run records. Before continuation, it also
replays the first sixteen curves' 192 streams and requires the logical dump
to match the archived dump byte for byte. This is a fresh-workspace recovery
control using the same implementation, not an independent mathematical
verifier or an independently certified host replay.

The separate native [audit_prefix.rs](audit_prefix.rs) compares every
inherited immutable logical row with the final dump. Only curve expanded
and measured flags may advance; payloads and old run IDs cannot change.
Every batch is fully verified before the next batch. Optional collector
replay trials are controls and are not added to the committed-run population.

## Reproduction and raw evidence

Build the pinned source on Linux; do not substitute an unvalidated main
binary. Run [continue.sh](continue.sh) from the evidence checkout with a
fresh output directory and the validated database and archived dump:

```sh
WORKER=/absolute/path/to/pinned/target/release/examples/isogeny_disk_walk \
  bash research/prime61_disk_continue_20261018/continue.sh \
  /new/output /restored/checkpoint.db /restored/records.jsonl
```

For public reproduction without access to the private Drive, the preceding
study's `results/pilot/checkpoint-32.db.gz` and `dump-32.stdout` contain the
same input database and logical records. The gzip SHA-256 is
`84481ed7b9b260fd59685d10a46c49842708aec6069c6a4752adbe965e7b95aa`.

All limits are cumulative. The worker's transactions and unique run keys
preserve completed work. On an interrupted phase, retain its stdout, stderr,
command and exit receipt, then resume the same cumulative target with a
new log name. Do not overwrite an earlier receipt or reduce stream budgets.
The script's output directory must be new; use the recorded native commands
for resuming an existing directory.

Raw commands, outputs, exits, logical dumps and snapshots are packed in
`results/runs.tar.xz`. Verify and extract from this study directory:

```sh
sha256sum -c results/runs.tar.xz.sha256
tar -xJf results/runs.tar.xz -C results
```

The archive is deterministic: sorted GNU tar, UTC mtime 2026-10-18,
owner/group zero, normalized modes and single-thread `xz -6`. Repacking
the extracted `runs/` from `results/` gives the same bytes. Build/test logs
and compact receipts stay loose because the report references their paths.
Private Drive URLs/IDs are intentionally omitted from this public study.
Archive bytes, hashes and actual round-trip verification are recorded in
the sanitized storage receipt; a small upload does not certify full quota.

[replay_helpers.sh](replay_helpers.sh) rebuilds the two native helpers using
the pinned worker's compiled serde_json dependency, reruns the prefix audit
against the public preceding dump, and regenerates the visual and summary
into a new scratch directory. It leaves frozen evidence unchanged:

```sh
bash research/prime61_disk_continue_20261018/replay_helpers.sh \
  /pinned/target research/prime61_disk_continue_20261018/results/runs \
  /new/scratch
```

Render the copied `reports/report.typ` there with Typst. The committed SVG
and summary come solely from the native phase records; the renderer only
lays out those recorded values.

## Scope and next step

The old exact fundamental-discriminant audit still excludes vertical
separable base-field edges in this fixed ordinary trace class. This is not
a statement about extension fields or other classes. The selected-prime
frontier remains a subset until complete generation and class coverage are
certified. A source expansion records all rational kernels of the selected
degrees; it does not enumerate every composite-degree isogeny separately.

The unresolved obligations remain in the
[class problem](../../problems/prime61-horizontal-class-memory.md) and
[backlog card](../../backlog/XP-20261016-prime61-horizontal-class/card.json):
certified full class structure, generation by the chosen primes, every
class workload, and a sized full-run disk/compute envelope. The SQLite
index and data still grow. No paid remote compute is provisioned, and
there is no scheduled continuation after this bounded run.

No admitted wall-speed, fastest-tool, full-cost, S, rho-reference or
end-to-end speedup result is produced. Canonical scoreboards, the leaderboard,
progress timeline and curve registry were checked for this fixture and left
unchanged: this study adds prefix coverage and capped relation-stage
observations, without a promoted performance result or new identity theorem.
The historical pilot visual remains frozen; this continuation gets its own
native [visual.rs](visual.rs), [summary](reports/summary.json),
[SVG](reports/progress.svg) and [PDF](reports/report.pdf).

Host and toolchain receipts are under `results/`. Isolation is unqualified:
this is capacity/relation evidence, with no wall-speed admission. Reported
RSS is the native worker's own process peak, excluding compiler, test,
audit and renderer processes. Python is used only as the Typst document
renderer binding. Conductor was attempted before edits and is unavailable.
The host exposes five Intel Xeon Platinum 8573C CPUs, a four-CPU cgroup
quota and 16 GiB memory. The previous pilot used a different host; no timing
comparison or independent-host certificate is inferred from this rebuild.
