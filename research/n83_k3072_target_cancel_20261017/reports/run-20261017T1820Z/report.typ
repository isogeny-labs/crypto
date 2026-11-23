#set page(paper: "a4", margin: 18mm)
#set text(font: "Arial", size: 10pt)
#set par(leading: 0.55em)
#set table(inset: 5pt, stroke: 0.4pt + rgb("ccd4dc"))
= N83 K3072 target-worker screen
*17 October 2026 historical execution · report completed 21 October*

The serial arm reached its 1,200-second cap during target-independent construction on a heavily shared macOS host. It produced no target rows, completed base dump, or online target interval. The cancellation arm was left unrun. This execution supports no policy ranking; the separate 24-point holdout remains sealed and unused.

#figure(image("outcomes.svg", width: 100%), caption: [A censored setup interval and an unrun arm. The arrow marks the timeout; the blank cancellation row is not a zero-time result.])

#table(columns: (1fr, 2fr),
 [*Recorded field*], [*Value or boundary*],
 [Policy], [Serial: one target thread, cancellation disabled],
 [Factor-base layout], [K3072, root11, indexed five-byte state table],
 [Outer wall], [1,200.396926 seconds; process cap reached],
 [Child maximum RSS], [3,290,841,088 bytes],
 [Completed target rows], [0 of the planned 128],
 [Completed setup / online time], [Unknown / unknown],
 [Cancellation arm], [Two target threads, cancellation enabled; not run],
 [Fresh holdout], [24 public subgroup points; unmeasured],
)

== Conditions and interpretation
The frozen screen uses the same 128 public points for both planned policies, rank seed 29 and four rank threads. The local host had reported load averages around 90–130 and repeated disk pressure. This is an ordinary shared-host diagnostic. It has no matched A/A control, completed A/B pair, calibrated throughput claim, or external-tool comparison. The observed child RSS is a process resource receipt; it is distinct from planned index allocation.

The cap neither selects nor rejects either policy. A matched run under the existing protocol would need completed construction and independently replayed target rows for both arms before a policy comparison. This report starts no new computation or cloud allocation.

== Source and custody
Frozen plan SHA-256:
#text(size: 8pt)[`d1a8489cdb646b9c2c9bd06d44847600ea7ef14a2d78ab14c62cf418f65d73b8`]

Executed Rust source SHA-256:
#text(size: 8pt)[`8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829`]

Release binary SHA-256:
#text(size: 8pt)[`df8340eb76ac90d2520e0202159505426b9d7879978f10c597731c4bfe73045a`]

The provenance manifest verifies every frozen input and the retained executable against the plan. It records byte-identical copies of the run receipt, command, empty outputs, candidates and holdout manifest in `raw/`. The retained receipt matches the committed timeout receipt. No completed arithmetic output exists to replay. Raw source: `/Volumes/SSD990/crypto/artifacts/n83_k3072_target_cancel_20261017/serial`. The original protocol, plan and timeout record are preserved.
