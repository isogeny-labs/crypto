# Item 08 smoke test (2026-10-18, before the frozen cards; seed 1, not a measured run)

Instrument built with `rustc --edition 2021 -C opt-level=3 -C codegen-units=1 -C target-cpu=native`
(6 s), run under `prlimit --as=4294967296`. Nothing here enters the decision rule: the smoke
instances use seed 1 and the training size n = 23.

| file | what | result |
|---|---|---|
| `selftest.txt` | field identities; e-form of S_4 == resultant (300 points, n = 31); engine find-all == brute force == field evaluation on 42 small draws, 3 orders; first-solution verdicts; learners; model round trip | SELFTEST OK |
| `corpus.jsonl` | **positive control**: regenerate the 60 committed Trimoska corpus instances, run arm a, compare with the committed stock WDSat decision counts | decisions_equal 60/60, verdict_equal 60/60, certificates 30/30 (CORPUS OK, 96 s) |
| `samples-n23.txt`, `fit.txt`, `model.txt` | the whole learning pipeline at n = 23 on 1 P + 1 R sampling instance, 1 P + 1 R validation instance | runs; model file written |
| `cell-n23.jsonl` | 2 P + 2 R targets, all eight arms, `--truth` | all arms agree, planted targets SAT, exact count agrees (32 records) |
| `cell-n23-cms.jsonl` | CMS 5.11.21 on the two planted targets | SAT, model verified |
| `cell-l6-n31.jsonl` | l = 6 secondary shape, 1 P + 1 R, arms a b b2 c | all agree; 5.0 s per unsatisfiable tree |
| `summary.txt` | `summarize` on the 4-target smoke cell (not a result: 4 pairs) | tables and verdict lines print |
