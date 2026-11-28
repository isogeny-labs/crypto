#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Pivot operational attempt: terminal receipts
Evidence report · 2026-11-11

The native pre-check passed after verifying 16,736 dependency files totaling 3,111,892,318 bytes. It recorded no errors and zero compiler invocations. The outer launcher then looked for the native receipt at a different path and failed with FileNotFoundError. The pre-phase process returned 0 and its terminal owner was confirmed; the overall caller returned 1.

#image("timing.svg",width:100%)

== Execution and accounting
Only the pre-check phase ran. No Lean compiler or subsequent phase was invoked. The full 2,400-worker-second reservation is charged, giving 8,580 charged and 121,020 remaining from the 129,600-second campaign ceiling. The deadline remains November 11 at 12:00 UTC. These campaign charges differ from observed elapsed intervals. Dollar cost and scientific ratio remain null.

== Exact evidence
The canonical raw archive contains 21 files. Every member was rehashed against raw-manifest.json and compared byte-for-byte with its original native, outer or caller path. The archive SHA256 is
`5ea9ea216b6bd3a298bd7b65ae0c930acc76ec50a93d1cec0becae2cf620349c`.

== Review map
summary.json and metrics.csv retain exact values. graph-data.csv drives the plot. original-path-readback.json records all checked paths and digests. campaign-ledger.json preserves accounting at this attempt's terminal boundary. The later successor is separate and was not touched by this reporting task.
