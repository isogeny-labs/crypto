# LEAD-ICP-icperf-binary-pdp-solver

## 1. Lead as recorded
- disposition: open_next_step (also open_next_step on 2026-11-01); family: ic-prime; last_reviewed 2026-10-24; source atlas_survey_20261107 --pursuit.
- title: Solver choice for the Weil-descended binary PDP (WDSat/CDCL+XOR vs Groebner/F4/BRiAl, side-constrained cores) on the ICPERF boundary.
- next-step excerpt (truncated): "Run H-ICPERF-c9380d: a Boolean-native Groebner column (BRiAl) with parseable d_solv and an RSS w..."
- Survey note (atlas_survey.md line 34): binary-field only; ANF too large to write below n~300 (+40.8 bits at ECC2K-130, -4.9 at n=409).
- next_step_cost / why_it_might_beat_sqrt_p: not read beyond leads.json fields shown (absent from the entry).
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry); /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 32-46, 236-250). Other hits (atlas_tally.json, atlas-inputs JSON files) are index data and were not read. No dedicated H-ICPERF-c9380d note found in docs or research/notes.

## 2. Hypothesis
On the ICPERF boundary (binary curves, n between the writability limit ~300 and n=409), a Boolean-native Groebner solver (BRiAl) on the Weil-descended binary PDP reaches a parseable solving degree d_solv within a bounded RSS, and its cost per decomposition is lower than WDSat (CDCL+XOR) at matched side constraints, with the winner not changing across the 0/0.1/1 GAE pricing band.

## 3. Preregistered prediction and falsification criterion
- Statistic: ratio R = (priced cost of BRiAl column) / (priced cost of WDSat) per cell, with d_solv and peak RSS recorded.
- Prediction: R < 1 in a majority of cells, and d_solv parseable in all cells that fit in RSS.
- Falsified if R >= 1 in a majority of cells, or d_solv is unparseable/OOM in most cells (then the Groebner column is dominated).
- Inconclusive if the order flips inside the 0/0.1/1 GAE band (AGENTS.md 5a).
- Null/negative control: random (non-descended) Boolean systems of equal size; positive control: planted-solution instance where both solvers must recover the key; A/A arm: same solver, same instance, repeated runs on two hosts.
- Even a win says nothing about beating rho: report the gap to matched rho as distance remaining.

## 4. Method
Not executable here. Intended: build BRiAl and WDSat on the Iso Lab EKS route (isolab-eks), generate Weil-descended Semaev-S3 PDP ANF for n in a grid across the boundary, seeds fixed, interleaved arm order, masked labels, S3 hashes, experiment.protocol/v1 frozen before data.
Local cost: none (no compute performed). A stdlib toy would not be faithful: a pure-Python CDCL or F4 at toy size cannot represent the solver-engineering comparison at the boundary.

## 5. Classification
blocked. Reason: the next step needs BRiAl (a Groebner engine, C++/build) and WDSat builds plus a large-memory host to reach the writability boundary (ANF exceeds 2^31 monomials); none is available under the stdlib-only, 200 MB, no-build limits. No run.py is written.

## 6. What a run cannot establish
Nothing was run. Any later result is labelled [pcc:unverified] until a replay certificate, validator report and matched rho reference exist. Toy sizes cannot transfer to n~409. Binary-only; no implication for prime fields. Solver ranking does not establish an improvement over the index-calculus baseline unless priced end to end.
