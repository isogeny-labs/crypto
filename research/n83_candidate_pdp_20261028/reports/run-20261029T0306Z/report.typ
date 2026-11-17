#set page(paper:"a4",margin:(x:15mm,y:14mm),footer:context align(right,text(size:8pt,fill:rgb("64748b"))[N83 / m3 LFD · #counter(page).display()]))
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:.5em)
#text(size:21pt,weight:"bold")[Retained n83 m3 degree-three closure]

#text(fill:rgb("087e8b"))[12/12 verified · 12 lower-bound 3 certificates · 0 exact LFD values]

Run `n83-degree3-m3-20261029-r1`. Producer observations; independent scientific review pending. 0 queues
exhausted. Capped subspaces are not complete V3. Unknown upper bounds remain
unknown; no resource limit or plateau gives a global degree certificate.

#table(columns:(2.9fr,.9fr,.6fr,.8fr,1.1fr),inset:4pt,stroke:.4pt+rgb("d3dae4"),
[Retained case], [V3 rank], [Lower], [Upper], [Stop],
[original-koblitz-planted], [32768], [3], [unknown], [row_cap],
[original-koblitz-natural], [32768], [3], [unknown], [row_cap],
[original-binary-planted], [32768], [3], [unknown], [row_cap],
[original-binary-natural], [32768], [3], [unknown], [row_cap],
[baseline-koblitz-planted], [32768], [3], [unknown], [row_cap],
[baseline-koblitz-natural], [32768], [3], [unknown], [row_cap],
[baseline-binary-planted], [32768], [3], [unknown], [row_cap],
[baseline-binary-natural], [32768], [3], [unknown], [row_cap],
[spike40-koblitz-planted], [32768], [3], [unknown], [row_cap],
[spike40-koblitz-natural], [32768], [3], [unknown], [row_cap],
[spike40-binary-planted], [32768], [3], [unknown], [row_cap],
[spike40-binary-natural], [32768], [3], [unknown], [row_cap],
)

#image("closure-ranks.svg",width:100%)

*What the new witness proves.* A quadratic row reached within degree 3 lies
outside the complete, replayed degree 2 space. Its source/reducer trace and
nonzero V2 remainder prove LFD>=3 for the saved presentation. A global upper
certificate is a separate requirement. These records do not establish a lower
regularity, faster solver, elite base or universal FFD/LFD assertion.

#pagebreak()
#text(size:18pt,weight:"bold")[Source identity, replay and preservation]

*Twelve retained identities.* Original, paired baseline and spike40; Koblitz
and general binary; planted and natural targets. Each m3 system has 164 variables.
All use the fixed 2^27-candidate per-family base manifests reused across m2..9.
No point, target or equation was regenerated. Original planted targets differ
from paired targets. Repeated natural sources are not independent draws.
The candidate x-coordinate presentation still requires lift/sign checks for
any proposed point decomposition.

*Ordinary-degree computation.* Complete original V2 traces are replayed first.
V3 admits original generators up to degree3 and all variable multiples of new
rows of degree below 3. Degree3 rows are never multiplied within this budget.
All original Boolean field equations are required. Direct graded-colex indexing
covers 735295 columns, 11489 u64 words and 91912 bytes per saved row. The format
map explains every offset, column and source/reducer reference.

*Termination and controls.* Stop at queue exhaustion, 32768 rows or 131072
operations. Remaining queue count and next source are replayed exactly. All 12 requested cases were attempted: 12 row caps, zero process errors and
zero timeouts. A cap proves only admitted memberships. Seven native tests check index bijection,
products crossing64/128, field and ID validation, multiplication bounds,
positive/negative LFD controls, both caps and tamper rejection. Runtime controls
also replay. Frozen m2 code and prior datasets are preserved unchanged.

*Verification.* 12 fresh replay checks and 12 S3-restored checks.
All 230 raw manifest entries match before and after replay.
Every raw row/trace, V2 input, executable, control, status and all 12 immutable
cumulative summaries are preserved. Source/protocol were committed and uploaded
before production. Data were committed and pushed before this report.

*Execution.* One native process, 8 GiB, 600 seconds per production and replay plus
5 seconds termination grace, 18000 seconds batch cap, no retries. Existing-host
fallback follows retained EKS shared-bucket denial; no IAM/capacity changes.
Run `2026-10-29T03:05:54Z` to `2026-10-29T03:21:53Z`; timestamps are diagnostic,
not a timing comparison. CPU timing was not measured.

#text(size:8pt)[Source: `6a2c028999634a4ddf9791dc3082180086b6c2ff`]#linebreak()
#text(size:8pt)[Protocol: `e4d9a88ecf15df5dd837e713d1cfaf936c02fae9b9b9359d6803d20214fbb438`]#linebreak()
#text(size:8pt)[Archive SHA256: `37d67c62bc7a551c54485693a613a22384144ace1fdeb713802b19438b5e32d4`]#linebreak()
#text(size:8pt)[Data commit: `cc063967c673e8bca317bfddf4cea1c35f7f76a1`]#linebreak()

Read `DEGREE3_M3_FORMAT.json`, `DEGREE3_M3_DESIGN.md`,
`experiment-degree3-m3.export.json` and `evidence/degree3-m3-storage.json`.
Restore the content-addressed S3 archive, check `manifest.sha256`, replay each
case with its saved input/V2 paths, and recheck the manifest.

*Remaining work.* Exact m3 and larger-m LFD, prime LFD, exact larger-m
regularity and held-out elite-base studies. Earlier regularity certificates
remain m=2 Koblitz 28/binary 28/prime 42 and the recorded binary chain intervals.
This batch makes no new regularity or base-quality claim.
