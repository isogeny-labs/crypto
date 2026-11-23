# Carried cofactor and indexed bucket results

All five arms solved all 44 fixtures with original-curve verification. The four rolling arms agreed on every stopping anchor and accepted-row count. Every carried divisor and cofactor passed native add_place and exact division checks during audit.

| Genus | Cases | Original / combined median wall range | Faster cases | Equal-price REDC ratio |
|---|---:|---:|---:|---:|
| 2 | 24 | 0.976–1.528 | 22 | 1.008–1.286 |
| 3 | 12 | 1.605–2.301 | 12 | 1.513–2.242 |
| 4 | 8 | 2.112–6.313 | 8 | 2.113–3.572 |

Relative to the prior rolling solver, the cofactor update alone improved equal-price REDC counts in every cell, by factors 1.009–1.034 (g2), 1.047–1.087 (g3), and 1.176–1.365 (g4). Indexing the buckets alone preserved REDC counts in every cell and improved median wall time in all 20 genus-three/four cells. Both changes together improved wall medians in 40/44 relative to rolling and 42/44 relative to original.

Timing dispersion was substantial on some cells. For the largest observed ratio (g4, p=31, case 0) reference times ranged from 1.019 to 3.264 ms and combined times from 0.238 to 0.657 ms; its exact equal-price arithmetic ratio was 3.509. The 6.313 median ratio requires independent timing replication before being used as a size-specific performance estimate. This run uses the existing eleven curves and 44 independent-source fixtures, with no tenfold median result. All per-arm samples and ablations are retained.
