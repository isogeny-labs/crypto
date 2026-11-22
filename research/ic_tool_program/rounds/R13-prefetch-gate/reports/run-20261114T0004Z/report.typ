#set page(paper: "a4", margin: 20mm)
#set text(size: 10pt)
= R13 replacement cohort: rejected
Instance i-06dc588064a36bd57; Intel Xeon 6975P-C. Native physical-core isolation: logical siblings 2 and 18. Five rounds per set. Cold base/candidate ratios, 40 pairs per primary set.
#table(columns: (1fr, 2fr, 2fr), [Degree], [Suite mean (95% interval)], [Holdout mean (95% interval)], [37], [0.995486 (0.992043–0.998941)], [0.999403 (0.996952–1.001859)], [47], [0.997786 (0.995150–1.000429)], [0.998734 (0.993303–1.004195)], [57], [0.994377 (0.987760–1.001039)], [0.990503 (0.984583–0.996458)])

Acceptance requires every primary interval above 1.01. None passes. Native verdict: rejected. No extension: all six half-widths below 3%.

A/A: 220 clean groups / 292 attempts. Suite: 399 clean groups / 400 scheduled / 493 attempts. One n59 candidate group exhausted bounded retries under contention. Holdouts: 240 clean groups / 295 attempts. Every attempt is retained.

Validation: 1,170 records and 2,340 scalar certificates pass. Independent recomputation: 114 fields, zero disagreements. Four seeded output replays pass on another local build; exact-arm Linux replay remains open. No timing is drawn from replay. No leaderboard promotion or VALIDATION.json.

Sources: REPORT.md, analysis.json, certverify.json, recompute.json, replay.json and runs.tar.xz beside this PDF; frozen protocol two directories above. Earlier host cohorts are separate. Canonical graphs remain unchanged because the candidate is rejected.
#image("ratios.svg", width: 100%)
