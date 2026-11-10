| input | binary | n | base median s (min–max) | patched median s (min–max) | speedup (median) | speedup (min/min) | output |
|:--|:--|--:|--:|--:|--:|--:|:--|
| `g31_n12` | `n20_gf31` | 3 | 0.539 (0.503–0.576) | 0.138 (0.136–0.151) | 3.91× | 3.70× | same basis |
| `g31_n14` | `n20_gf31` | 3 | 1.122 (1.106–1.185) | 0.294 (0.275–0.300) | 3.82× | 4.02× | same basis |
| `g31_n15` | `n20_gf31` | 3 | 1.609 (1.504–1.658) | 0.390 (0.375–0.406) | 4.13× | 4.01× | same basis |
| `g31_n16` | `n20_gf31` | 3 | 7.201 (6.944–7.537) | 2.116 (2.046–2.201) | 3.40× | 3.39× | same basis |
| `overdet_n12_m24_p65521` | `n12_gf65521` | 3 | 0.140 (0.109–0.148) | 0.086 (0.067–0.092) | 1.63× | 1.63× | same verdict: unit ideal (no solution) |
| `overdet_n10_m15_p65521` | `n12_gf65521` | 3 | 0.091 (0.079–0.105) | 0.048 (0.046–0.051) | 1.90× | 1.72× | same verdict: unit ideal (no solution) |
| `quad_n9_p65521` | `n12_gf65521` | 3 | 7.445 (6.922–7.471) | 3.749 (3.578–3.974) | 1.99× | 1.93× | same basis |
| `root1` | `n35_gf2` | 3 | 8.291 (8.176–8.477) | 8.236 (7.759–8.624) | 1.01× | 1.05× | same verdict: unit ideal (no solution) |
| `root2` | `n35_gf2` | 3 | 2.025 (1.870–2.035) | 1.922 (1.892–1.952) | 1.05× | 0.99× | same verdict: unit ideal (no solution) |
| `root3` | `n35_gf2` | 3 | 4.538 (4.482–4.833) | 4.976 (4.875–5.187) | 0.91× | 0.92× | same basis |
| `root4` | `n35_gf2` | 3 | 7.685 (7.622–7.709) | 7.542 (7.329–7.753) | 1.02× | 1.04× | same verdict: unit ideal (no solution) |
| `root5` | `n35_gf2` | 3 | 8.802 (8.746–9.116) | 8.822 (8.729–8.941) | 1.00× | 1.00× | same verdict: unit ideal (no solution) |
| `root6` | `n35_gf2` | 3 | 2.064 (1.952–2.066) | 2.091 (2.042–2.110) | 0.99× | 0.96× | same verdict: unit ideal (no solution) |
| `root7` | `n35_gf2` | 3 | 1.821 (1.767–2.446) | 1.977 (1.751–2.373) | 0.92× | 1.01× | same verdict: unit ideal (no solution) |
| `root8` | `n35_gf2` | 3 | 8.242 (7.774–8.466) | 8.330 (8.024–8.380) | 0.99× | 0.97× | same verdict: unit ideal (no solution) |
| `root1` (re-run, 14 GB cap) | `n128_gf2` | 3 | 17.661 (16.554–26.796) | 19.540 (14.658–22.678) | n/a | n/a | censored: a run died (`std::bad_alloc` under the 6 GB virtual-memory cap, or the cgroup OOM killer at about 7 GB resident in the re-run); times are time-to-failure |
| `root3` (re-run, 14 GB cap) | `n128_gf2` | 3 | 21.132 (14.474–24.378) | 23.984 (17.751–24.683) | n/a | n/a | censored: a run died (`std::bad_alloc` under the 6 GB virtual-memory cap, or the cgroup OOM killer at about 7 GB resident in the re-run); times are time-to-failure |
| `wd_n16` | `n64_gf2` | 3 | 1.183 (1.021–1.183) | 0.941 (0.892–1.079) | 1.26× | 1.14× | same basis |
| `g2_n30` | `n30_gf2` | 2 | 128.517 (126.699–130.335) | 123.691 (117.392–129.990) | 1.04× | 1.08× | same basis |
| `g2_n30` (re-run, 14 GB cap) | `n64_gf2` | 1 | 37.403 (37.403–37.403) | 28.617 (28.617–28.617) | n/a | n/a | censored: a run died (`std::bad_alloc` under the 6 GB virtual-memory cap, or the cgroup OOM killer at about 7 GB resident in the re-run); times are time-to-failure |
