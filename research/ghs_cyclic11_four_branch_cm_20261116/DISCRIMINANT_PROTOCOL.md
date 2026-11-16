# Exact polynomial discriminant replay

16 November 2026. Check the discriminant identity
`disc(X^10-tX^5+3125)=5^30(t²-12500)^5` at `t=2,24`, and compute the
residual factor modulo the largest source prime divisor (`71,47`).
Run `gp -q discriminant_replay.gp` and retain stdout and stderr. This
checks polynomial and integer arithmetic only. The integral Tate
subgroup-existence argument is separately stated in `MAPS.md` with
its primary references.

The first multiline GP command failed to parse; its stderr is retained
losslessly as `DISCRIMINANT_REPLAY.failed.stderr.gz`. The corrected one-line script
produced `DISCRIMINANT_REPLAY.stdout`; no mathematical inputs changed.
