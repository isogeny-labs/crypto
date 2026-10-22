# PKM Kummer tower at P-224's 2-adic shape: follow-up (2026-10-22)

Copies of the visual record of the addendum (section 8) to
[`research/pkm_p224_kummer_shape_20261022/`](../../research/pkm_p224_kummer_shape_20261022/README.md),
which is the source of truth: `report.pdf` (the whole report with the addendum), and each graph
as SVG with a 150 dpi PNG render. The graphs are produced from the recorded rows by
`pkm_p224_shape_report` (`f4`, `price`, `mem`) and rendered by
`research/pkm_p224_kummer_shape_20261022/render.sh`:

- `d_vs_n`, `madds_vs_n`, `s_ratio_by_size`: the report's three graphs refreshed with the
  `N = 24` cell (the frozen versions stay in
  [`../20261022-pkm-p224-kummer-shape/`](../20261022-pkm-p224-kummer-shape/));
- `memory_by_step`: resident memory per F4 step of the `N = 24` system, before and after the
  memory fix, with `B'` at 4 bytes an entry.

Headline (as the addendum states it): the memory fix changes no count (E1-E3: every checked
row and step identical to the frozen ones) and does not lower the peak resident set (E2:
-2.4% at `N = 22`, +12% at `N = 20`); the `N = 24` cell finished with `D = 6`, 4.22 x 10^13
multiply-adds, peak resident 5,991 MB, 2.0 h on a shared 4-vCPU host; the derived F4 arm at the
24-bit curve is 7.7 x 10^10 times matched rho (lower bound). No speedup and no improvement of the
index-calculus baseline is claimed.

**No row in `benchmarks/README.md`.** The validation gate
([`docs/VALIDATION_GATE.md`](../../docs/VALIDATION_GATE.md)) admits a row only with an
independent-host replay certificate and a validator report; this follow-up ran on one shared host
and the F4 arm is derived, not an ecbench arm. As for the frozen round, the result stays in its
research directory and these are browsing copies.
