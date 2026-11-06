#set page(paper: "a4", margin: 20mm)
#set text(font: "Liberation Sans", size: 9pt)
= GLS symmetry v2_aa run record
UTC start: `20261021T1536Z` · repeated v2 A/A control. Host: Intel Xeon 6975P-C, 32 logical CPUs; L0 shared host.

The native producer compiled with `rustc -D warnings -O` and completed one execution. Source SHA-256: `3483134315e51797b956df8eb29410252e65e86418285855e94d7b463bc92626`. Raw NDJSON SHA-256: `d64ab66dc2a38adf4a13beded258acb76308e31db2343d3cc47b32a2b44c5a87`. Raw records: `gls_symmetry_results_v2_aa.ndjson`; native summary: `gls_symmetry_summary_v2_aa.csv`. All are in the parent study directory.

There are 102/102 expected records: 3 curves, 9 exact relation censuses, 9 negative controls, 54 cold index-calculus solves, and 27 matched rho solves. Zero recorded failures or timeouts. Independent order, group-action, witness-orbit, rank and recovered-scalar audit passed.

At $p=211$, $r=44221$, $B=12$: flat/fold collection queries `829/254`; flat/fold/rho recorded $F_(p^2)$ multiplications `110453/67414/25635`. L0 median nanoseconds for three targets: `6052836/3880655/814606`. The flat/fold multiplication-count ratio is `1.638428`. All cell values and min/max times are in the native summary CSV. These are descriptive counts and shared-host times.

#image("gls_fold_cost_v2_aa.png", width: 100%)

The graph displays the nine recorded flat/fold multiplication-count ratios, with three target scalars per cell and one deterministic setup trajectory. Field additions, base-field arithmetic inside field operations, hashing and allocations are not exhaustively priced. This run report makes no speed claim. See `README.md` for links, commands and limitations.
