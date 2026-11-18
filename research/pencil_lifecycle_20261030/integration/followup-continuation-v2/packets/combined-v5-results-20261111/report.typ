#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Combined signed/Kummer v5 result
Receipt audit · 2026-11-11

The materializer failed while opening the first declared overlay. At subsequent readback all eight declared overlay paths were absent from the v5 packet. The corresponding v3 files matched every declared byte count and SHA256. The sole command exited 1 and was reaped. No compiler or tests ran. Cleanup completed and the terminal output seal passed.

#image("timing.svg",width:100%)

== Resources and accounting
The native envelope was 2 CPU and 16 GiB. Peak observed target bytes were 2,547,773,710. The full 2,400-worker-second reservation is charged: 13,380 charged and 116,220 remaining at this terminal boundary. The hard deadline remains November 11 at 12:00 UTC. Dollar cost and scientific ratio are null.

== Evidence and review
All native terminal-manifest entries and the completion-bound manifest digest were rehashed. Exact native/caller files are retained in the raw archive. summary.json and metrics.csv retain exact values; graph-data.csv supplies the plot. overlay-readback.json records eight absent paths and the matching prior copies. Source and cache contents were not changed by this reporting task; root reclamation is separate.
