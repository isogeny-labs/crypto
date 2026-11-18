#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
#set heading(numbering: "1.")
#set par(justify: true)

= Does the dense form also win at finding decompositions?

EXP-PKM-DENSE-DEC-001 · 2026-11-15 · protocol `PROTOCOL.md` (frozen in `76e3613594` before any row, no amendments)
· labelled extension to larger `S` on RunPod under Amendments 1–3 (§ Larger `S`) · instrument
`examples/pkm_algebraic_oracle.rs` (`--membership both|dense`, `--vkind coset|random`) · rows `runs/` ·
analysis `analyze.py` → `results.json` · figure `make_figures.py`. Counted F#sub[p] multiplications. Unisolated local
host, 4 threads. Nothing here concerns cryptographic sizes or Pollard rho [pcc:unverified].

EXP-PKM-IDEAS-001 found that writing membership in `V` as one polynomial `∏(x − v)` makes F4 *refute* systems far more
cheaply than the tower of degree-2 maps. That left the case relation collection needs open: targets that do
decompose, where the oracle has to *find* every relation. Here the full oracle runs (F4, the counted linear stage that
reads every solution off the basis, then lifting and sign checks) on the same `V` and the same targets, with
membership written both ways in one row.

== Result

*Yes.* On targets with a decomposition, the dense oracle finds exactly the relations the tower oracle and enumeration
find, at a small fraction of the tower's cost, and the gap widens with `|V|`. Enumeration over `V` is still far
cheaper than either.

#table(columns: 4, align: left,
  [], [prediction (frozen)], [verdict], [measured (planted targets, median)],
  [Validity], [V1 all agree; V2 planted have a relation; V3 tower = EXP-PKM-ODD-001 part B], [*valid*],
    [180 decided arm-rows agree with enumeration; every planted target has a relation; the tower's counts are bit-identical to the committed part B rows on all 6 shared cells (24 targets)],
  [P1], [dense ÷ tower `< 1` at every `S`], [*held*],
    [`m = 2`: 0.050, 0.018, 0.0054, 0.0025, 0.00099 (`S = 16 … 256`). `m = 3`: 0.13, 0.082, 0.041 (`S = 4, 8, 16`)],
  [P2], [the same, `< 0.5`], [*held*], [as P1; replicated at `P2`: 0.0054 (`m = 2`, `S = 64`), 0.081 (`m = 3`, `S = 8`)],
  [P3], [random ÷ coset `V`, dense, in `(0.5, 2)`], [*falsified* at one cell],
    [1.00–1.04 everywhere except `m = 3`, `S = 4`: 4.07],
  [P4], [dense ÷ enumeration `> 1` at every `S`], [*held*],
    [`m = 2`: 11, 21, 40, 87, 181. `m = 3`: 43, 3.5·10³, 3.1·10⁴, 2.5·10⁵ (`S = 4 … 32`)],
)

#figure(image("figures/find_cost.svg", width: 100%),
  caption: [Cost of finding all relations of a planted target. Tower and dense-coset run on identical targets.])

== Reading

- *Finding costs the same as refuting.* The linear stage is at most 0.3% of the dense oracle's work. Its staircase
  holds 2 points at `m = 2` and 3–6 at `m = 3`: the planted relation and its symmetric copies. So the dense form's F4
  advantage from EXP-PKM-IDEAS-001 carries over to relation finding unchanged.
- *Growth.* The dense oracle grows like `S^2.05` at `m = 2` (`S = 16 → 256`) and about `S^5.2` at `m = 3`
  (`S = 8 → 32`). The tower grows like `S^3.5` at `m = 2` and `S^6.1` at `m = 3` (`S = 8 → 16`). Enumeration grows like `S^1.03` and `S^2.0`.
- *What a dense win means.* It is a win over the slides' tower formulation, not over enumeration. Against enumeration
  the gap still grows: by about `S` at `m = 2` and about `S³` at `m = 3`. At these sizes, then, an algebraic relation
  oracle is not the cheap way to collect relations in either formulation.
- *Smooth or random `V`.* Under the dense form, a random `V` costs the same as the Kummer coset (P3 within 4% at 8 of
  9 cells). At `m = 3`, `S = 4` the coset's two-term membership `x^4 − g^4` makes it 4× cheaper. That is the one cell
  that falsifies P3, and the edge is gone by `S = 8`. So the smooth structure of `V` gives no advantage once
  membership is written densely. The isogeny question from EXP-PKM-IDEAS-001 gets the same answer for finding as for
  refuting.
- *Beyond the tower's reach.* The `m = 3`, `S = 32` cell finishes densely in 2.5·10¹⁰ multiplications per target (about 190 s for the cell's six targets).
  The tower formulation of the same size needed ≈ 5.1·10¹² and 18 GiB in EXP-PKM-ODD-001 (a different study, on
  refutation targets).

== Limits

One prime family (`P1`, replicated at `P2` on two cells), degree-2 Kummer towers only, 4 planted and 2 random targets
per cell, `m ≤ 3`, `|V| ≤ 256`. The tower and dense arms share F4 and the linear stage, so their difference is the
system alone. The dense expansion of `∏(x − v)`, `|V|(|V|+1)/2` multiplications, is counted in the dense arm.

== Reproduce

`run.sh`, then `python3 analyze.py` and `python3 make_figures.py`. The binary's SHA-256 is in `binary.sha256`.
`cargo test --release --lib pkm_smooth_tower` runs 11 tests, including one checking that the dense oracle returns
exactly the enumerated relations.

= Larger `S` on RunPod (Amendments 1–3)

A labelled extension, not part of the frozen verdicts above. Predictions L1–L4 were fixed in Amendment 1 before any
row; Amendment 2 fixed a stopping rule mid-run; Amendment 3 fixed the rerun after the first pod was lost.
Rows `runs/large/` (first pod) and `runs/rerun/` (rerun) · analysis `analyze_large.py` → `results_large.json` ·
figure `make_figures_large.py`. CPU pods, four lanes of 4 threads. Nothing here concerns cryptographic sizes or
Pollard rho [pcc:unverified].

#table(columns: 4, align: left,
  [], [prediction (Amendment 1)], [verdict], [measured (planted targets, median)],
  [Validity], [V1, V2 as above; rerun rows equal surviving first-pod rows], [*valid*],
    [84 rows in 14 cells; every decided arm agrees with enumeration; the 5 first-pod rows that survived the pod loss
     (`m2-t10-both` 3, `m3-t6-dense` 2) equal the rerun rows on every field but `ms`],
  [L1], [dense ÷ tower `< 0.5` at `m = 2`, `S = 512, 1024`], [*held*], [0.00046 and 0.00011 (tower 8.7·10¹⁰ and 1.4·10¹²)],
  [L2], [dense ÷ enumeration `> 1` in every finished cell], [*held*],
    [`m = 2`: 203 (`S = 256`) → 3.2·10³ (`S = 4096`). `m = 3`: 1.7·10⁶ (`S = 64`)],
  [L3], [growth exponent: `m = 2` in `[1.8, 2.3]`, `m = 3` in `[4.5, 6.0]`], [*held*],
    [`S^2.00` (`S = 256 … 4096`, `P3`); `S^4.87` (`S = 32 → 64`)],
  [L4], [random ÷ coset `V` in `(0.5, 2)` at `S ≥ 64`], [*held*], [1.0001–1.0010 in all 5 cells],
)

#figure(image("figures/find_cost_large.svg", width: 100%),
  caption: [Finding cost to larger `S`. Open triangles: dense cells censored at 1,800 s per target; the
  multiplications counted by the cutoff are a lower bound.])

== What did not finish

- *`m = 2`, `S = 8192`* (coset and random) and *`S = 16384`* (coset): every target hit the 1,800 s budget. Lower
  bounds: ≥ 8.1·10⁹ and ≥ 9.4·10⁹ multiplications at `S = 8192` (≥ 5,000× enumeration), ≥ 2.8·10⁹ at `S = 16384`.
  The bound *falls* with `S`: F4 completes fewer counted multiplications per second as its matrices grow (42 s per
  target at `2^11`, 320 s at `2^12`, a 7.6× step against 4× more multiplications), so wall time, not the counted
  metric, ends these cells.
- *`m = 3`, `S = 128`*: killed by the kernel (signal 9) at 108.5 GiB peak RSS, 11.3 h into its 24 h budget, with no
  row. Censored by memory; no lower bound on multiplications is recorded.
- *Not run* under Amendment 2's stopping rule (the `m = 3`, `S = 128` cell ended at 2026-11-16 05:37Z):
  `P3-m2-t14-random` and both `S = 32768` cells. The pod ran them anyway because the controller was down; their
  rows stay in `runs/rerun/` and are excluded from every result.
- Peak memory of the dense oracle at `m = 2`: 2.1 GiB (`S = 2048`), 8.7 GiB (`4096`), 32 GiB (`8192`, at cutoff).
  At `m = 3`: 24–33 GiB at `S = 64`.

== Infrastructure

RunPod removed the first pod (`y8fnixt44o3ziv`, on demand) mid-run; rows written after the one copy taken at 07:01Z
were lost. The rerun used `tools/runpod_resilient/`: rows on a network volume, a supervisor that restarts the job on
a replacement pod, the oracle's `--resume 1` (same target draws, finished indices skipped; tested equal to an
uninterrupted run), and a client-side controller that mirrors rows and replaces lost pods. A deliberate pod deletion
at 18:14Z was detected and replaced in 5 minutes, and the job resumed without duplicating a row. The controller died
with its own host container on 2026-11-15 ≈ 23:55Z; the job still finished on the volume at 2026-11-16 12:23Z, and
the pod then idled until RunPod removed it ≈ 2026-11-17 06:05Z. Billed: \$11.03 (first pod), \$0.44 (failover test),
\$37.26 (rerun, of which ≈ \$18 idle), plus the volume (\$0.05 so far).

== Reading

The four predictions held. The dense form's advantage over the tower keeps widening (≈ 10⁴× at `S = 1024`); a random
`V` costs the same as the Kummer coset at every size; and the dense oracle stays two to three orders above enumeration
at `m = 2` and six above at `m = 3`, with the gap growing like `S` and `S³`. Beyond `S ≈ 4096` at `m = 2` and `S = 64`
at `m = 3` the limit is wall time and memory, not counted work.
