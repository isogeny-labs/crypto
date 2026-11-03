#set page(paper: "a4", margin: 18mm)
#set text(font: "Liberation Sans", size: 9pt)
= Native S5 pilot · p=97
Run ID: `run-20261022T0620Z-quartic-s5-p97`. Intel Xeon 6975P-C, 32 logical CPUs; shared-host L0. The frozen seed-zero L0 trace supplies one hit and one miss on `[271]G`. Both solver outputs match the exact signed group oracle.

#table(columns: (1.2fr,.6fr,.8fr,1.6fr,.7fr), inset: 5pt,
  [query], [index], [decompositions], [solve $F_p$ multiplications], [agree],
  [first_success], [14], [1], [1,211,126,402,567], [True],
  [first_failure], [0], [0], [1,197,004,768,304], [True],
)

Symmetrisation: 1,226,109,078 $F_p$ multiplications. Each solve: degree 29, 41,780 Macaulay rows, 40,920 columns. Timings are shared-host L0; no whole-method speed claim. Source, command, raw data and audit details are in `README.md`.

#image("s5_cost.png", width: 100%)
