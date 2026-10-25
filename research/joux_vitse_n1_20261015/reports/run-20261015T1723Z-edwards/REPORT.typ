#set page(paper: "a4", margin: 1.6cm)
#set text(size: 9.2pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)

#let S = json("summary_edwards.json")
#let r2(x, d: 2) = if x == none { "—" } else { str(calc.round(float(x), digits: d)) }
#let sci(x, d: 2) = {
  if x == none { return "—" }
  let v = float(x)
  if v == 0 { return "0" }
  let e = calc.floor(calc.log(calc.abs(v), base: 10))
  let m = v / calc.pow(10.0, e)
  [#calc.round(m, digits: d)·10#super[#e]]
}

= Run report: $k = 5$ Edwards route with F4 trace replay, 2026-10-15T17:23Z to 2026-10-20T03:58Z

Descriptive record of the runs; observations only. The interpretation is in `../../REPORT_EDWARDS.pdf` and `../../README.md`. Rendered from `summary_edwards.json` (a copy of the study's file, beside this report).

*Spec.* `PROTOCOL_EDWARDS.md`, frozen at commit `3a8599b3f`. Orchestration `run_edwards.sh`, then `run_edwards_resume.sh` after the resumable driver was added. Binary `examples/joux_vitse_edwards.rs`. The four cost runs, `p = 31` seed 1, `p = 61` seed 1 and `p = 31` seed 2 ran the build of `3a8599b3f` (no checkpointing). `p = 41` seed 2, and the counted runs of `p = 71` seed 1 and `p = 41` seed 1, ran the resumable build, saved as the frozen binary (SHA-256 `622a47d3…`, built 2026-10-16T03:39Z at commit `c2cb903ac`). The trial, trace, accounting and records are the same between the two builds; only checkpointing and the wall-time cap were added (`DEVIATION.md` items 4 and 7).

*Host.* Claude Code cloud container, 4 vCPU Intel Xeon \@ 2.10 GHz, 15 GB, rustc 1.97.0. Isolation: none. Arms overlapped on the host at times and the host was restarted at least three times.

*Timeline (UTC).* Cost runs 17:23 on 2026-10-15. DLP arms: `p = 31` seed 1 17:24 to 18:14 on the 15th; `p = 61` seed 1 20:16 on the 15th to 00:02 on the 16th; `p = 31` seed 2 02:14 to 04:07 on the 16th; `p = 41` seed 2 04:07 to 06:55 on the 16th; `p = 71` seed 1 from 06:55 on the 16th, resumed at 11:52 on the 16th and 16:12 on the 19th, solved 17:57 on the 19th; `p = 41` seed 1 counted run 17:57 on the 19th to its cap at 03:58 on the 20th. Two earlier attempts at `p = 41` seed 1 and `p = 71` seed 1 were killed by a restart at about 02:05 on the 16th and their partial records kept apart (`killed_20261016T0205Z/`, not used). Every deviation, with its reason, is in `data/edwards/DEVIATION.md`.

*Requested and completed.* Cost: 4 runs × 32 residuals, all complete. DLP: 6 arms requested; 5 solved with both answers verified; 1 (`p = 41` seed 1, a six-point base) stopped at its 10-hour wall cap with 3 of 7 relations. No arm was substituted.

*Measured* (unit: `F_p` multiplications; counts, not wall time). Checks failed: *#S.checks_failed*.

#table(columns: 7, align: (left,) + (right,) * 6,
  [record], [$|F|$], [state], [trials], [relations], [IC online], [rho $FF_p$ mult. (ops)],
  ..S.dlp.map(d => (
    d.file, str(int(d.base_points)),
    if d.complete { "solved, verified" } else { "stopped at the cap" },
    str(int(d.trials)),
    if d.complete { str(d.relations) } else { str(int(d.relations_recorded)) + " of " + str(int(d.unknowns)) },
    if d.complete { sci(d.online_muls) } else { sci(d.cost_to_cap_online_muls) },
    sci(d.rho_fp_muls) + " (" + str(d.rho_group_ops) + ")",
  )).flatten()
)

#table(columns: 6, align: (left,) + (right,) * 5,
  [cost record], [residuals], [committed solve, mean], [replay, mean], [factor, mean], [checks failed],
  ..S.cost.map(c => (c.file, str(c.residuals), sci(c.full_mean), sci(c.replay_mean), r2(c.factor_mean), str(c.checks_failed))).flatten()
)

#figure(image("edwards_replay.svg", width: 78%), caption: [Cost of one test per residual, plotted from the cost records by `../../figures/plot_edwards.py`.])
