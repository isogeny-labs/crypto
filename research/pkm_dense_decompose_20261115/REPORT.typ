#set page(paper: "a4", margin: 1.8cm, numbering: "1")
#set text(size: 10pt)
#set heading(numbering: "1.")
#set par(justify: true)

= Does the dense form also win at finding decompositions?

EXP-PKM-DENSE-DEC-001 · 2026-11-15 · protocol `PROTOCOL.md` (frozen in `76e3613594` before any row, no amendments)
· instrument `examples/pkm_algebraic_oracle.rs` (`--membership both|dense`, `--vkind coset|random`) · rows `runs/` ·
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
