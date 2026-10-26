#set page(paper: "a4", margin: 20mm)
#set text(font: "Liberation Sans", size: 9pt)
= GLS symmetry v1 run record
UTC start: `20261021T1530Z` · initial audit-heavy implementation. Host: Intel Xeon 6975P-C, 32 logical CPUs; L0 shared host.

The native producer compiled with `rustc -D warnings -O` and completed one execution. Source SHA-256: `1a1f39d56acf1d66fa70f61563900f7d6cabd4c1c15d0186c0b9cf400b597164`. Raw NDJSON SHA-256: `27f9f04ba4cd4a500ef70ee79c8f243750ef070dfe948cc7105c97bde7ec2800`. Raw records: `gls_symmetry_results_v1.ndjson`; native summary: `gls_symmetry_summary_v1.csv`. All are in the parent study directory.

There are 102/102 expected records: 3 curves, 9 exact relation censuses, 9 negative controls, 54 cold index-calculus solves, and 27 matched rho solves. Zero recorded failures or timeouts. Independent order, group-action, witness-orbit, rank and recovered-scalar audit passed.

At $p=211$, $r=44221$, $B=12$: flat/fold collection queries `829/254`; flat/fold/rho recorded $F_(p^2)$ multiplications `242562/201581/25635`. L0 median nanoseconds for three targets: `9716891/7534945/823465`. The flat/fold multiplication-count ratio is `1.203298`. All cell values and min/max times are in the native summary CSV. These are descriptive counts and shared-host times.

#image("gls_fold_cost_v1.png", width: 100%)

The graph displays the nine recorded flat/fold multiplication-count ratios, with three target scalars per cell and one deterministic setup trajectory. Field additions, base-field arithmetic inside field operations, hashing and allocations are not exhaustively priced. This run report makes no speed claim. See `README.md` for links, commands and limitations.
