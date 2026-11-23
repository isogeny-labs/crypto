#set page(paper: "a4", margin: 18mm)
#set text(size: 11pt)
= Independent materializer V2 delta review
Published producer:64afb6b895. Prior findings preserved in ed2db1be72. Preread plan preceded V2 implementation inspection. [pcc:unverified]

#image("review.svg", width: 100%)

M1 now hashes and copies one buffer, checks the destination against the approved digest, pins its manifest row and rechecks bound inputs before final manifest creation. [pcc:unverified]

M2 now cleans the owned process group regardless of leader completion, uses a two-second wait timeout, checks group absence and retains incomplete-cleanup errors. Outer unit/cgroup confirmation remains required. [pcc:unverified]

Six mocked controls passed under a ten-second external timeout: single-read candidate, candidate drift, destination corruption, exited leader, wait timeout and surviving group. Exit0;6 tests OK. Tests use fake process/kill functions and in-memory byte objects. No materializer, compiler, library test or target payload was invoked. [pcc:unverified]

Exact producer hashes were checked after controls. Source diff preserves the original pinned revision, include/vendor/cache rules and resource limits. Concrete admission and outer supervision still require publication/review and live qualification. [pcc:unverified]

Full source locations, command and raw output are in REVIEW.md, result.json, findings.csv and mock.stderr. [pcc:unverified]
