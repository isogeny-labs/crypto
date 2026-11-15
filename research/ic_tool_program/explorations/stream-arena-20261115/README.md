# One huge-page arena for the build's partition streams (exploration, dropped)

- **Status:** an exploration on this session's container, 2026-11-15, three
  isolated rounds an arm. This is a record, not rounds, and supports no
  claim. The change is reverted from code.
- **Why:** in the folded pair-table build, the stream push costs 13–16
  cycles a stored pair. A temporary pre-fault of the 256 partition
  vectors moved about 3.5–4.5 cycles a pair out of that step, so first
  page touches are about a quarter of it.
- **The change tried:** `PartStreams` put a run's partition streams in one
  zeroed allocation advised for huge pages, with a spill vector a
  partition past its room. `KIC_STREAM_ARENA=0` kept a vector a
  partition.
- **Result** ([`explore/`](explore/)), cycles a stored pair, median of
  three:

  | row | step | off | on |
  |:--|:--|:--|:--|
  | k0n53 | stream push | 14.1 | 12.7 |
  | k0n53 | whole build | 47.8 | 46.6 |
  | k0n61 | stream push | 15.6 | 14.7 |
  | k0n61 | whole build | 51.7 | 48.8 |

  - Build wall times overlapped between the arms.
  - Certificates and counts were identical in all 12 processes.
- **Reading:**
  - Huge pages took back only about 1–1.5 of the roughly 4 fault cycles.
    Most of a first touch is zeroing the page, which a huge page still
    pays.
  - About 2–3% of the build is below what a build A/B resolves on this
    host, whose A/A interval is about ±4%. So no A/B was run.
