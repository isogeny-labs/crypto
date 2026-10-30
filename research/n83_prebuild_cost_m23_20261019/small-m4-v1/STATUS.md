# Small-base m=4 card: execution handoff

**Observed 2026-10-19 19:05:02 UTC: running.** The committed shell card was
still active as PID 707046 on the authorized SSH runner, with container
`n83-small-m4-k1` executing the first public arm. This is a time-stamped
observation, not a guarantee of later liveness or a completed experiment.

- Completed: two exact-base preflights, two constructed controls and two
  separate Linux general-field replays, all normal exits without OOM.
  Both controls also passed the local arm64 verifier. See
  [sealed control evidence](controls/README.md).
- K1: the retained [progress record](observation-20261019T1905Z/k1-progress.json)
  at 610,764.032616 ms records 89,205 completed decisions, 31,600 conflicts,
  116 restarts and 596,913,370,498 completed XOR nanoseconds. These counters
  are observational snapshots and do not establish an exact CPU-time
  fraction. The [host observation](observation-20261019T1905Z/host-status.txt)
  confirms the running card/container at that time.
- D1,D2,K2: scheduled serially by the already launched card after K1.
  None is reported complete by this snapshot. Natural relation/rank and
  full IC cost remain unresolved.

Run root: `/home/ec2-user/n83-m23-20261019/small-m4-v1`.
Native source: `611aad60e4d9952d6239a3382a77184da3640eeb`.
Launch policy: [protocol](../SMALL_M4_PROTOCOL.md),
[host card](../SMALL_M4_HOST_CARD.md), and
[v2 progress checks](../SMALL_M4_HOST_CARD_V2.md).

On resumption, check `card-status.txt`, the recorded PID, the named Docker
container and each arm's host-exit, phase/progress, worker or cap receipts.
Do not launch the card again into this output directory. Preserve a Spot
interruption and freeze a separately named continuation before any retry.
When all arms stop, export a new deterministic archive of their raw
receipts, verify its manifest, replay every returned natural row with
`n83_small_m4_replay`, and add a separate dated result report. Keep the
completed control archive and this historical observation unchanged.

The SSH script continues independently of the chat process. No automatic
chat check-in or PR-activity subscription is configured by this record.
