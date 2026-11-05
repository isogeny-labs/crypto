#set page(paper: "a4", margin: 18mm)
#set text(size: 11pt)
= Nonbranch replay continuation
Prospective source packet — 2026-11-05. [pcc:unverified]

The prior pre phase stopped at its work deadline after 280.005 seconds, with zero compiler invocations. Its reported CPU time was 6.65 seconds; input files reside on NFS. These observations motivate a longer bounded hashing phase; they do not establish a measured completion time. Prior evidence is preserved and charged conservatively as 300 worker-seconds. [pcc:unverified]

#image("flow.svg", width: 100%)

The new maximum is 2,400 worker-seconds, one attempt, no automatic retry. The campaign ceiling stays 46,800 seconds and its deadline stays November 6 at 04:00 UTC. Previous debits remain separate. [pcc:unverified]

Every original realpath, metadata, size, SHA256, runtime and pre/post agreement check remains. The four compiler calls, 30 names, parser controls and intended negative mathematical failure remain unchanged. The new freeze pins the successor sources; each phase checks its externally supplied freeze digest before and after work. [pcc:unverified]

Durable bounded progress distinguishes checked files/bytes from declared totals, and retains current path/bytes read plus the last checked path. Filesystem blocking can delay progress updates. Partial hashes never become a pass or a resumed check. A new persistent workspace rejects repeats and preserves all failed outputs. [pcc:unverified]

Execution requires exact GitHub publication, independent prospective review, prior-unit reconciliation and remaining campaign headroom. See CONTINUATION.md, COMMAND.txt, replay.patch and freeze.json. This packet was prepared without compiling or executing the replay. [pcc:unverified]
