# N83 cold factor-base comparison visualization

This sidecar plot summarizes the frozen 16-point K1103/K1107 fresh-process cold comparison recorded in [the study](../n83_cold_base_confirm_20261021/RESULT.md). K1107's mean cold-process time was 113.755 s versus 114.950 s for K1103; the paired difference was 1.195 s with a two-sided 95% Student-t interval [0.957, 1.433] s. All 16 per-point differences favored K1107.

![Paired cold-process differences by public point](paired-cold-difference.svg)

The observation is bounded to this factor-base pair, root11 dynamic layout, source build and one-target cold workload. It has independent replay and retained raw evidence. It is **not an admitted row in `benchmarks/README.md`**: the current validation gate requires a rho reference in the same cold unit and process, while this protocol recorded same-point rho online only. Do not infer a cold IC/rho ratio, online winner, or a general K1107 performance claim. The IC/Rho progress timeline and leaderboard therefore remain unchanged.

The graph is deterministically generated from the study's admitted `results/analysis.json`:

```sh
python3 research/n83_cold_base_confirm_visualization_20261021/render_paired_cold_difference.py --check
```
