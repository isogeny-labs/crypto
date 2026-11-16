# Native N83 boundary and free-selector witness continuation

The original known-witness control now passes independent native
model/domain/group replay at **787 conflicts, 35,307 decisions and
167.341433194 s worker wall**, within its unchanged 5,000-conflict /180 s,
one-CPU/4 GiB envelope. Its separate post-solve exporter adds no selector
assumptions. Initial propagation finishes at 192.850022 ms; CDCL enters at
193.136020 ms. Returned timers record 167102070169 ns XOR propagation and
70147211 ns clause propagation, localizing this observed control's cost to
XOR work during search. This resolves this exact witness gate. Natural
yield/rank and full-cold base selection remain open.

- [Result and complete observations](../../verification/native-selector-export-20261019/RESULT.md)
- [Boundary protocol](../../verification/native-sat-boundary-20261019/PROTOCOL.md),
  source 5e9ee5d4b1aa6ca22368213e5bc05b343e00bfb4
- [Export protocol](../../verification/native-selector-export-20261019/PROTOCOL.md),
  source cbe8e44da5ac0a792922d746368882a3b8f40170
- [Records](../../verification/native-selector-export-20261019/records/),
  [exact relays/assessments](../../verification/native-selector-export-20261019/transport/),
  [validation](../../verification/native-selector-export-20261019/VALIDATION.md),
  [forgery rejection](../../verification/native-selector-export-20261019/forgery-controls.json),
  [complete raw archive/replay](../../verification/native-selector-export-20261019/RAW.md)
- [Three-page inspected PDF](report.pdf), [plot data](figure-data.json),
  [render source](render.py), [Typst source](report.typ)

Three runs were requested/completed once each: fully pinned SAT/verified
audit; original free SAT/stopped unsupported-positive audit; separate free
export SAT/verified audit. All workers exit0, uncensored, cleanup verified,
no OOM. The original free card remains stopped because its frozen auditor
requires selectors that its worker did not export. Its audit is not
backfilled. The pinned relay initially hits a native float-roundtrip checker
defect; exact byte equality, a corrected native verifier and regression
test resolve transport verification, preserving the original failure.
A coherently rehashed exported phase forgery is rejected.

Each observer version has n=1, L0 on the shared Intel Xeon6975P-C x86_64
host; no uncertainty interval, A/A performance result or speedup is claimed.
The original and exported free solves share dimensions, assumptions and
search counters, while the fully pinned control has extra selector units.
Earlier180 s censoring remains historical; its different outcome is not a
speedup baseline. Five summands, exact1182/866 representative domains, eight
fixed x-coordinate blocks, original first-pair order, identity mask0;
fixture SHA256 fc203db28eed0164e47b0c755c3d9a09f80df1acd5e4c7e09dfe172fa5f0a538.
Primary curve icv1-f2m83-tm6151469093347-debefd74, a=0,
polynomial x^83+x^45+x^2+x+1, subgroup order2417851639230796216685689.

The three [phase checkpoint plots](phases.svg) use elapsed milliseconds
from each solve entry on log axes; they are not exclusive durations.
The [outer-wall plot](outer-wall.svg) uses supervised milliseconds, a
different interval. [Execution diagram](execution.svg) separates approved
publication from these later controls. Added allocation is540 s; summed
observed outer wall 337051.216021 ms is derived from the three receipts.
Build, test, audit, storage and rendering overhead remain separate.

The [approved49MB publication](../../verification/native-phase-publication-20261019/RESULT.md)
passed fresh S3 download, archive/manifest SHA256 and all 308 payload checks.
That archive predates these new controls; their 1,796,576-byte raw successor
is separately committed. Earlier rejected/pending plans remain immutable.

The [peer paired plot](peer-paired.svg) copies only the separate
[upstream ABBA receipt](../../verification/native-paired-k2048-k4096-20261019/RESULT.md)
and [audit fields](../../verification/native-paired-k2048-k4096-20261019/completed-run-audit.json).
It is not a new run or native-harness recertification: c7i.xlarge,
source 2b6bbb94998e562dd9bc0b623e5a9c803892cf71, native Rust worker,
original Bash/Python orchestration/audits/tests. Four capped queries report
1000 conflicts and zero models/relations/rank. Original reported container
means61.103 s /132.084 s and ratio 2.162 apply only to that one query/cap.
Do not pool its host/source/workload with these controls. Its raw S3 payload
remains at its documented original location; this publication did not move it.

Full validation: identical-library 4397 pass /118 ignored, exporter 19,
pinned-selector 1, new decoder1, final native runner 15. Research execution,
audits and tests are native; Python only renders and inspects visuals/PDF.
External comparison is not run because this is an observability/control
diagnostic, not an admitted performance ranking. Full-cold leaderboard,
scoreboard and progress timeline were checked and left unchanged. No
unknown scalar, natural useful rank, full-cold winner, matched-rho speedup,
fastest-tool or nonexistence claim is established. Proposed next work is a
frozen XOR-engine profile/candidate comparison and full natural positive
relation replay; neither is running or scheduled here.
