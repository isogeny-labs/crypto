# PKM Kummer tower, m = 3, N = 21: the msolve instrument with a long wall

Registered 2026-10-18 before the run, under the 50-dollar budget of
[`research/dreg_closure_m4l6_20261018/PROTOCOL.md`](../dreg_closure_m4l6_20261018/PROTOCOL.md) §9.

## Question

Does msolve's F4 take a productive step at degree 8 or more on the committed
`m = 3`, `t = 7` (`N = 21`) Kummer systems at `p₁ = 2013265921`, or finish
with `D = 7`? Item 3 of the next-10 campaign
([`03-pkm-degree/README.md`](../ic_next10_20261014/03-pkm-degree/README.md))
could evaluate neither way: both runs were stopped by the 45-minute wall
inside the second degree-7 step at 2.6 GB peak (*recorded*: `progress.txt`
lines 27–49 and `runs/*_m3_t7_*.mem` there).

## Instrument (item 3's, with the wall raised)

- systems: `research/ic_next10_20261014/03-pkm-degree/systems/pkm_kummer_p2013265921_m3_t7_x221026520.ms`
  and `..._m3_t7_x704078485.ms` (the cell's two random targets, committed).
- msolve `-t 1 -v 2`, `OMP_NUM_THREADS=1`, `prlimit --as` at `CAP` = 0.85 × the
  host's cgroup memory limit, `timeout WALL` with `WALL` = half the host's
  remaining lifetime minus 20 minutes per target, both targets one after the
  other; then `m3_t8` (`N = 24`) with whatever remains, if any.
- msolve version: 0.6.5 from the distribution package where available,
  otherwise built from the `v0.6.5` tag; the version and its sha256 are
  recorded in `runs/host.txt`. A different version voids the identity with
  item 3 unless the control below agrees.
- control: `m3_t5` target 0 (`N = 15`, committed `D = 7`, finished by msolve in
  item 3) runs first with a 45-minute wall and must give the same answer.
- `run.sh` is item 3's script with `CAP`, `WALL` and the systems path as
  parameters; `.mem` files record peak VmHWM/VmPeak, wall seconds and exit.

## Decision rule (item 3's, fixed)

| outcome at N = 21 | reading |
|---|---|
| finished, `D = 7`, both targets | the plateau holds one more level (`L = 6`); H1a still needs `N ≥ 27` |
| a productive step at degree ≥ 8 on either target | H0 (growth resumes) from a lower bound |
| wall or memory stop with no degree-8 step | inconclusive; nothing new |

The `f4_fp_tower` cell of `research/pkm_lead1_m3n21_20261016/PROTOCOL.md` is
not replaced by this; it stays blocked on a 192 GB host and budget.

## Host

A RunPod CPU pod (general-purpose flavour, 8 vCPU, 32 GB) at about 0.64
dollars per hour, lifetime 10 h, launched by
`research/dreg_closure_m4l6_20261018/pod/launch.py` with `--job pkm-msolve`.
