# Cold certified S4 trace reuse: negative result

[Report](report.pdf), [full native summary](summary.json),
[flow/custody](local-evidence.json), [frozen plan](runtime-plan.json).
Source f8c6f8b89, Amendment13. All 864 batches complete in 131.978s:
144 warmups, 720 measured, 6912 coordinate calls. Complete outputs match
dense qualification, global protocol problems empty, zero retries.
[pcc:unverified]

All 33 controlled cases lose, with lower intervals above 1 and cold trace/fresh
ratios 2.981–16.086. The three timing rejections remain. All 12 largest-field
cases pass controls but lose at 11.823–14.956; their pools accept 6 or 7 replays
in the first measured batch. The frozen viability condition fails: zero
largest-field gains, no full-IC follow-up. [pcc:unverified]

Each batch contains eight distinct residual systems. Every trace batch starts
with a new pool; construction/destruction, learning, checks, rejection,
fallback and extraction are inside its timer. Shared curve/symbolic/residual
preparation and dense qualification are reported separately outside the phase.
This is a coordinate-phase result; no pipeline/rho extrapolation. [pcc:unverified]

Full 39910-byte summary and raw/source/binary/pre/post hashes retained. Raw and
copied binaries/checkpoint/host provenance remain under
/workspace/ic-local-runs/local-certified-s4-trace-20261116-18. No signed
independent verdict, strongest-external comparison, canonical encrypted-schema
validation, standard lab admission or admitted leaderboard/index row.
[pcc:unverified]

All four final report pages visually inspected. Exact PDF/SVG copies and 150dpi
PNG renders are under benchmarks/20261116-cubic-certified-s4-trace, with
presentation hashes checked and original PNG copies in the local run directory.
[pcc:unverified]
