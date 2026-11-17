# Index-calculus S3 component instruction-work report

Source of truth: [study](../../research/trace_zero_rowmap_20261117/README.md), [PDF](report.pdf), [SVG](instructions.svg), [150dpi PNG](instructions.png). Copies are byte-identical to study artifacts. The fixed ECC2K-130 uniform control uses22.87% fewer complete-oracle instructions and11.21% fewer complete-query instructions against the merged optimized kernel, under the recorded focused-build qualification [pcc:unverified].

This is simulated instruction work on AMD EPYC9V74, CPU0, with no native-time/isolation or full-cost admission. No row is promoted in benchmarks/README.md or the IC leaderboard because their independent replay/full-cost contract remains unmet. Existing canonical runtime/scoreboard/timeline values stay historical.
