# CPU complete recovery diagnostics

These copies come from the [CPU study](../../research/hashed_ecdl_cpu_20261030/).
Each panel uses 120 fixed executions: 12 warmups and 108 measurements. The
reports describe full certificate equality and shared-host L0 durations, not
certified speedups. Full raw records and checked exports remain in the study.

- [Initial cache report](initial-cache.pdf), [SVG](initial-cache.svg), [PNG](initial-cache.png).
- [Parameter cache report](parameter-cache.pdf), [SVG](parameter-cache.svg), [PNG](parameter-cache.png).

The native analyzer in the study generates the graphs from each panel's raw
rows. PNGs use `rsvg-convert -d 150 -p 150`; PDFs use the committed Typst sources.
The global benchmark index/IC leaderboard is not promoted: the repository's
validation gate additionally requires independent review and matched rho.
Neither a fastest-external comparison nor a matched rho measurement was run;
those gaps are unknown. The CPU changes are generic decomposition, not the
paper's conditional tensor contraction result.

The larger-field attempt reached its scheduling deadline without solver rows:
[report](scale-scheduling.pdf), [diagram](scale-scheduling.svg), [PNG](scale-scheduling.png).
Its incomplete export fails the full-panel completeness check; no larger-field
recovery result is recorded.
