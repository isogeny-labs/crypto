# Actual public-target recovery through two-large-prime graph IC

[Report](report.pdf) · [Complete original Rust summary](summary.json) · [Flow](flow.jsonl)
· [Frozen protocol](../AMENDMENT_24_TWO_LP_TARGET_RECOVERY.md)
· [Original custody](local-evidence.json) · [Post-call coverage audit](coverage-audit.json)

Original IC and the new adapted two-LP graph variant both recover scalar
147253026768918773 from the same public weak-family challenge. Both pass
native and extra elliptic replay and agree with the held-out answer. The
new variant's protected-target system also yields that scalar under the
standalone qualifier's independent batch Gaussian elimination. All 1,122
affine certificates replay successfully. [pcc:unverified]

| Candidate observation | Recorded result |
|---|---:|
| Full base / small base |530 / 397|
| Unique B values / ordered lines |1018081 / 534807|
| Verified original source rows |1253|
| Zero / one / two LPs |132 / 467 / 654|
| Verified descents / attempted residuals |2 / 1019|
| Stream advances |1018|
| Complete rows / certificate replays |1122 / 1122|
| Independent matrix rank |398|
| Output nonzeros |21271|
| Graph pivots / merges / cycles |133 / 1646 / 990|
| Fill refusals / original-solve fallbacks |0 / 0|

The protected target has coefficient-b and RHSa in each verified descent
row. The LP hypergraph removes only omitted point identities. All original
sources, descent prefixes, projection/solve attempts and the final affine
proof are retained in the original raw. The candidate uses one projection/
solve attempt; the worker sees no planted scalar and rebuilds transfer with
a zero answer placeholder. [pcc:unverified]

Observed primitive subtotals are3080275208820bitops for original IC and
7186744533268for the candidate. They are incomplete. Observed relation
phase prices are1679089021154and6152303556284respectively; the candidate
enumerates the full fixed B space and returns surplus equations. No ratio,
cost ranking or speed gain is inferred. [pcc:unverified]

**Post-call coverage finding:** RelationSystem uses local modular helpers
and Shoup fixed-factor multiplication which bypass the modular hooks.
Its legacy counter records7,711,970products outside the primitive subtotal;
modular subtraction counts were not separately emitted. Inversions are
already hooked. Consequently the candidate's observed LA subtotal is not
the full solve cost. The raw, original copied qualifier and original summary
remain unchanged. The next accounting patch must add exact product/addition
hooks without double-counting inverses; collection oversupply is a separate
optimization question. [pcc:unverified]

Sourceaf48df0d30dccce461370cc8e787a45c2640134d was published before the sole
call. Exit0in125.248seconds, fixture plus two workers complete, one validation
round/zero warmups, CPU0/8GiB/one claimed thread,900seconds per worker/
1900whole/60fixture and no retries. Original worker process walls23.639and
82.756seconds are practicality observations without A/A or usable intervals.
All172source/pre-call/pre-worker/post hashes pass. [pcc:unverified]

The original14738-byte summary,7384-byte IC raw, exact flow, fixtures,
argv/status/progress/timing and hashes are in Git. The original1020695-byte
two-LP raw exceeds64KiB and remains exact with its hash under
/workspace/ic-local-runs/local-cover-two-lp-dlp-validation-20261116-29,
alongside immutable driver/qualifier/wrapper/source and host facts. The
post-call coverage note adds no retrospective numeric replacement.
[pcc:unverified]

Legacy report calibration counters are separately emitted outside the IC
primitive subtotal, while captured process wall includes that work. Native/
RNG/container coverage and complete strongest-rho pricing remain open.
Complete totals stay null and comparative/rho eligibility stays false.
No independent signed group verdict, canonical encrypted-schema admission,
fastest-external result or successful S3 checkpoint upload is asserted.
[pcc:unverified]

The single final PDF page was visually inspected. Exact PDF/SVG/150dpiPNG
benchmark copies are hash-checked, with originals local. The final report
adds the explicitly labelled post-call arithmetic-coverage finding to the
predeclared presentation; the original template is also retained locally.
No endpoint, qualifier, report or hash failure occurred. [pcc:unverified]
