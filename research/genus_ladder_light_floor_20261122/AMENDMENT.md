# Amendment 1 (2026-11-22, after a launch error, before the complete data)
The command in `PROTOCOL.md` omits `--curves 6`. The driver caps the admitted curves at `--curves` (default 2) even with
`--curve-seeds`, so the first launch measured only curves 4 and 5 of each prime (files kept in `partial2/`). I ran
`analyze.py` on those partial rows (after fixing a key-name bug in it, `mean_weight_*`) before noticing: the thresholds, settings and analysis
are unchanged by what I saw (L2 0.860/0.849/0.828, best E = 0.3 on 2 curves), but it is a peek and is declared. The full run
adds `--curves 6` and nothing else; `run_all.sh` carries it. Curves 4 and 5 are deterministic, so their rows reproduce the partial ones (checked in RESULTS.md).
