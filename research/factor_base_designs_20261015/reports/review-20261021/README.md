# Amendment 7 review correction, 2026-10-21

The historical run directory and raw `runs/X1-*` / `runs/X2-*` records are unchanged. The original runner turned an unsuccessful `dmesg` read into the same sentinel it used for a successful read with no OOM lines. Consequently those sentinel files do not certify capture success. The revised analysis marks kernel capture UNVERIFIED and withholds strict R1/R2 attribution and prediction scores for these historical rows.

The four allocation failures and SIGABRTs remain observations, at the recorded times and RSS values. `ulimit -v` constrains virtual address space; an allocation failure under that cap does not establish a minimum resident working set above 64 GB. Both required address space and resident-memory requirements remain unmeasured. No completed cost or end-to-end performance comparison is supplied by these four rows.

The corrected native analysis is retained in `analysis-amendment7-corrected.txt`. It is derived from the same frozen inputs, not a new experiment. The prior analysis remains in `runs/analysis-amendment7.txt`. Corrected runner controls use mocks and no research engine or cloud host.

[Rendered correction report](report.pdf).
