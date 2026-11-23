# Full-order twist seed: original candidate failed

The frozen native producer exits **1** with `no full-order seed in fixed search`.
It produces no complete 120-case census or GP script. The planned independent
GP execution is **unattempted**. [Failure record](FAILURE.json),
[descriptive run report and inspected PDF](../reports/generators-20261027/full-order/RESULTS.md).

The separately frozen [maximal-order successor](../execution-v7/README.md)
independently identifies the p827 twist group as C7 x C119, of order833 and
exponent119. A whole-group-order point does not exist there. The original
candidate is retained as failed, rather than silently redefined or reported
as a completed panel.

Source `9643e917922cca9c50ae91709ba12f4ea9d6bfa1`, frozen
[protocol](protocol.json) and [binary/input hashes](FROZEN.json) precede this
attempt. Native limit600s,4GiB,64MiB/file. Development p7 controls pass; they
do not imply that every panel group is cyclic.

The actual-execution [export](experiment.export.json) passes standalone
schema/file validation. Full frozen-protocol binding retains **four findings**
for the absent GP execution. An initial planned-row export is preserved: the
canonical schema rejects attempts=0, so it cannot represent that unattempted
row directly. The final actual export does not invent an attempt to satisfy
balanced binding. Its flow contains the one conclusive failed process;
the original protocol still plans two. No scientific count or performance
result is admitted from this failed run.

Raw `runs.tar.xz` is **976 bytes**, SHA256
`998844edb84d13ff70db937d6983215ff22f91b9e5b4bc740489056bc93ef839`.
The maintained Setec manifest and [fresh-download verification](RAW_VERIFICATION.json)
check all **3** file members and identical repacking, including the empty
partial-output directory. Hydrate with the artifact tool, check
`sha256sum -c runs.tar.xz.sha256`, and extract before reading the actual
producer argv/env/start/finish/exit receipt under runs/.

Replaying this original source/input should preserve its failure. The
successor's maximum-order rule and separate source/protocol are not substituted
into this record. Full IC costs, scalar transport and NIST benefit remain open.
