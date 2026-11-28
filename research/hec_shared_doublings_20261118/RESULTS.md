# Shared generator doublings and signed verification

All four arms solved and verified all 44 public inputs in each pass. Fused arms retained the same relation stream. Every CELL and COUNT line was identical between the registered run and the separately frozen replay. Reported ratios divide the original normalized median wall time by the signed shared-table median on each input.

| Genus | Fixtures | First pass | Same-host replication | Fixtures reaching tenfold in both |
|---|---:|---:|---:|---:|
| 2 | 24 | 2.079–3.146× | 2.013–3.111× | 0 |
| 3 | 12 | 2.407–3.726× | 2.409–3.913× | 0 |
| 4 | 8 | 3.674–10.518× | 3.537–10.863× | 1 |

Genus four, p=31 case 1 reached 10.518× (1.514208 ms to 0.143958 ms) and 10.863× (1.515605 ms to 0.139521 ms). Its original-to-signed field reduction ratio was 9.205×. Against the fused collector with ordinary independent multiplications the signed table contributed 1.404× and 1.506× on this target. Both the collector and the group-check changes contribute to the complete ratio. Case 0 stayed below tenfold at 9.832× and 9.976×. Each timing ratio uses twelve rotated observations per arm per input, reported separately for the two passes.

The signed table was faster than the fused collector with ordinary multiplications on all 44 targets in both passes. Binary-to-signed timing changes were mixed, especially in genus two. The complete range over the three genera does not meet the full-panel tenfold objective. Both passes reused identical targets and host; they measure timing repeatability on that panel. The comparative speedup is against the original normalized IC baseline.

RUN.txt and replay/RUN.txt identify the prospective commits. The two CONTROL.txt files contain individual observations. FIRST.sha256 pins the first pass before replay and OUTPUT.sha256 covers the final record.
