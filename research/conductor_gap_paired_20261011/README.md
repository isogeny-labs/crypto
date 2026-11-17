> **PROVISIONAL, legacy Python, NOT FOR MERGE: violates the repo's no-Python rule; kept as labelled historical evidence; matched native re-measurement required before any claim.**

# Paired conductor-gap smoke test over F_{p^2} (Lead 2), legacy Python

Question (autoresearcher PR #2041): do endomorphism-ring conductors predict
reproducible differences in algebraic relation cost on matched curve pairs
(same `q`, `t`, `N`, `r`, distinct certified `f_E`)? This directory is a
feasibility probe on toy fields `q = p^2`, `p <= 43`. No claim of any kind
rests on it. Read `RESULTS_PROVISIONAL.md` first, then `PROTOCOL.md`
(frozen before the runs; amendments 1 to 4 are dated).

Adjacent native work already on main: `research/isogeny_conductor_gap_ic_20261010/`
(Rust, `ecbench`); this probe covers only the cell that round left open
(extension field, intermediate conductors).

| file | what it is |
|:--|:--|
| `PROTOCOL.md` | frozen protocol and amendments |
| `RESULTS_PROVISIONAL.md` | measured results, gate status, gaps |
| `cgp_field.py`, `cgp_curve.py`, `cgp_certify.py` | `F_{p^2}`, curves, division polynomials, Velu, volcano walk, `f_E` certificate (legacy Python) |
| `cgp_engine.py`, `cgp_stats.py` | Semaev `S_3` engines, Buchberger, relation checks, statistics (legacy Python) |
| `enumerate_pairs.py` | selection run |
| `paired_trials.py`, `exploratory_yield.py` | trials and replay; post hoc exact-yield analysis |
| `tests/test_cgp.py` | 22 unit tests |
| `curves.jsonl`, `isogeny_edges.jsonl`, `selection_receipt.json`, `selection.log` | selection outputs |
| `runs.jsonl`, `paired_results.csv`, `analysis.json`, `environment.json`, `trials_main.log` | trial outputs (`runs.jsonl` is per attempt) |
| `runs_replay.jsonl`, `replay_report.json`, `environment_replay.json`, `trials_replay.log` | G1 replay |
| `exploratory_yield.{json,csv,log}` | exploratory analysis (outside every gate) |

Commands that produced the committed outputs (nothing else was run), from this
directory, one process at a time:

```
python3 -m unittest discover -s tests
timeout 1500 python3 enumerate_pairs.py | tee selection.log
timeout 900  python3 paired_trials.py --mode main   | tee trials_main.log
timeout 900  python3 paired_trials.py --mode replay | tee trials_replay.log
timeout 1200 python3 exploratory_yield.py | tee exploratory_yield.log
```

Outputs are never overwritten by the scripts; do not re-run them here. The
native re-measurement that would replace this is scoped in the report to the
coordinator, not implemented.
