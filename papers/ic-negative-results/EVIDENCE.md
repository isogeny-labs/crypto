# Evidence ledger for papers/ic-negative-results

Every number in the paper cites one of these files as [E#]. The SHA-256 is
of the file as read when this ledger was generated (first 16 hex digits;
the full digest is in `evidence/ledger.full`). Repository heads at that time:
- `crypto` at `64b87d52adda`
- `crypto-autoresearcher` at `e4b1f0d1323c`

| id | repository:path | sha256 (16) | what |
|:--|:--|:--|:--|
| E1 | `crypto:research/lit2026_w3_ggm_audit_20261011/reports/run-20261011T2012Z/table.json` | `26d462fe6f6de45c` | W3 audit table (208 rows, compact-orbit ladder) |
| E2 | `crypto:research/lit2026_w3_ggm_audit_20261011/reports/run-20261011T2012Z/report.md` | `2062646518f95046` | W3 audit generated report (compact-orbit tables) |
| E3 | `crypto:research/lit2026_w3_ggm_audit_20261011/README.md` | `4a023d24168ddda6` | W3 audit write-up (bounds used, classification, record problems) |
| E4 | `crypto:RESEARCH_ISOGENY_FIBERS_ECDLP.md` | `23205b00c7cc4c25` | Isogeny fibers note, section 15.1 (Theorem 15.1 and proof) |
| E5 | `crypto-autoresearcher:knowledge/findings/KN-FIND-007.md` | `9dd306d2fad7387d` | Decomposition-yield conservation finding |
| E6 | `crypto:research/cyclotomic_falsifier_20261015/README.md` | `be5199e1a20534e0` | Cyclotomic base falsifier (yield-neutral) |
| E7 | `crypto:research/factor_base_designs_20261015/lean/FactorBaseDesigns/Yield.lean` | `e325cb42e2c560fc` | Kernel-checked list form of yield conservation |
| E8 | `crypto:research/factor_base_designs_20261015/proof_logs/axioms.txt` | `06148bb5c423bf9f` | Axiom log for E7 |
| E9 | `crypto-autoresearcher:knowledge/findings/KN-FIND-a8990a.md` | `aaefb551e0fbee1c` | Semaev cover monodromy finding (Theorems A, B, C) |
| E10 | `crypto-autoresearcher:papers/semaev-conservation-specialization/paper.tex` | `6ce58081151c6e06` | Source manuscript of the monodromy proofs |
| E11 | `crypto-autoresearcher:experiments/EXP-SMON-e5cbe6/runs/RUN-SMON-e5cbe6-001/raw-result.json` | `a7f3cb1265cf1102` | Toy-scale verification of E9 |
| E12 | `crypto-autoresearcher:ledger/corrections/CORR-20260807-652652.yaml` | `67da07f933b70565` | Correction supplying the missing step of the monodromy proof |
| E13 | `crypto-autoresearcher:knowledge/findings/KN-FIND-6edecd.md` | `935ede09e62a5abf` | Independent corroboration of the monodromy group |
| E14 | `crypto-autoresearcher:knowledge/findings/KN-FIND-936151.md` | `a17623ab3189cba0` | Lemma 4 counterexample finding |
| E15 | `crypto-autoresearcher:papers/nagao-lemma4-note/note.md` | `300372bab1ebd7fc` | Lemma 4 technical note (hand proof of witness A) |
| E16 | `crypto-autoresearcher:ledger/corrections/CORR-20260921-942a62.yaml` | `f9def986c5ec51cc` | Correction: inside form of Lemma 4 is false |
| E17 | `crypto-autoresearcher:tools/lemma4_inside_form.py` | `779e865265a768a7` | Earlier independent implementation (Python, sibling repository) |
| E18 | `crypto:papers/ic-negative-results/evidence/checks.txt` | `ddd155bb68e11e12` | Native exact checks, this paper (output) |
| E19 | `crypto:examples/blindspots_19_checks.rs` | `751277a8c075483c` | Native exact checks, this paper (source) |
| E20 | `crypto-autoresearcher:knowledge/findings/KN-FIND-9cba50.md` | `812688a01a31d18a` | Assumption 1 on the correct subspaces |
| E21 | `crypto-autoresearcher:ledger/goals/GOAL-SEMBIN-5078bc/goal.yaml` | `107a5cc6507e58ff` | Goal record quoting Assumption 1 and Semaev section 4.5.1 |
| E22 | `crypto:research/blindspots_20261018/07-blocked-f4-degree-cells/README.md` | `131ad266f6cb604b` | Campaign item 07 (degree cells) |
| E23 | `crypto:research/blindspots_20261018/10-kernel-program-search/README.md` | `d954435afea4783e` | Campaign item 10 (conservation check not run) |
| E24 | `crypto:research/blindspots_20261018/01-fp2-gaudry-k2/README.md` | `34d06d7bdec87acc` | Campaign item 01 (Gaudry k=2 against rho) |
| E25 | `crypto:research/blindspots_20261018/14-prime-m4-cells/README.md` | `5a333ca46a54939f` | Campaign item 14 (prime m=4 cells) |
| E26 | `crypto:research/blindspots_20261018/02-deployed-binary-pricing/README.md` | `529c38f03889ab2b` | Campaign item 02 (derived pricing, gate open) |
| E27 | `crypto:research/blindspots_20261018/EXPERIMENTS.md` | `fe91cf9a1abd32fb` | Frozen campaign design (item 19) |
| E28 | `crypto:research/factor_base_designs_20261015/README.md` | `7aec9d11222cf315` | Factor-base designs register (Lean proof map) |
| E29 | `crypto-autoresearcher:coordination/review/sembin-20260916-propquant/report.md` | `a0c7642720eef546` | Blind reader report on Nagao 2015/984 definitions |
| E30 | `crypto:research/blindspots_20261018/11-learned-prefilter/README.md` | `943422dff35ed15f` | Campaign item 11 (learned prefilter, MATCHES) |
| E31 | `crypto:papers/ic-negative-results/lean/IcNegative/AffineCore.lean` | `178ea2dd12090590` | Lean: counting core of Theorem 15.1 |
| E32 | `crypto:papers/ic-negative-results/lean/IcNegative/YieldConservation.lean` | `ee973d4cd005356f` | Lean: yield identity and coverage bound |
| E33 | `crypto:research/pair46_factor_bases_20261015/13-xy-windows/RESULTS.md` | `f2f6a266dc807687` | xy-windows base geometry (mentions conservation) |
| E34 | `crypto-autoresearcher:papers/semaev-conservation-specialization/README.md` | `99888c7d58c128e6` | Status and blockers of the monodromy manuscript |
