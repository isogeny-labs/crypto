| curve | log2 r | arm | verified/runs | S(w=0) | S(w=0.1) | S(w=1) | uncounted |
|:--|--:|:--|--:|--:|--:|--:|:--|
| icv1-fp32-tm31847-96b4d8d1 | 31.07 | ic-a2-1024 | 8/8 | 34.252 | 36.685 | 58.585 | — |
| icv1-fp32-tm31847-96b4d8d1 | 31.07 | ic-a6-256 | 8/8 | 17.830 | 18.902 | 28.550 | — |
| icv1-fp32-tm31847-96b4d8d1 | 31.07 | rho-cubic | 8/8 | 0.419 | 0.460 | 0.830 | — |
| icv1-fp32-tm31847-96b4d8d1 | 31.07 | rho-neg | 8/8 | 1.014 | 1.115 | 2.020 | — |
| icv1-fp32-t22549-777f0160 | 31.33 | ic-a2-1024 | 8/8 | 31.308 | 33.526 | 53.492 | — |
| icv1-fp32-t22549-777f0160 | 31.33 | ic-a6-256 | 8/8 | 17.422 | 18.624 | 29.444 | — |
| icv1-fp32-t22549-777f0160 | 31.33 | rho-cubic | 8/8 | 0.525 | 0.577 | 1.043 | — |
| icv1-fp32-t22549-777f0160 | 31.33 | rho-neg | 8/8 | 1.002 | 1.101 | 1.996 | — |
| icv1-fp32-tm86219-053c5532 | 31.45 | ic-a2-1024 | 8/8 | 32.034 | 34.559 | 57.282 | — |
| icv1-fp32-tm86219-053c5532 | 31.45 | ic-a6-256 | 8/8 | 17.509 | 18.819 | 30.613 | — |
| icv1-fp32-tm86219-053c5532 | 31.45 | rho-cubic | 8/8 | 0.607 | 0.667 | 1.206 | — |
| icv1-fp32-tm86219-053c5532 | 31.45 | rho-neg | 8/8 | 0.744 | 0.818 | 1.481 | — |
| icv1-fp32-tm92567-62490ae7 | 31.62 | ic-a2-1024 | 8/8 | 31.906 | 34.628 | 59.132 | — |
| icv1-fp32-tm92567-62490ae7 | 31.62 | ic-a6-256 | 8/8 | 17.694 | 19.168 | 32.435 | — |
| icv1-fp32-tm92567-62490ae7 | 31.62 | rho-cubic | 8/8 | 0.456 | 0.500 | 0.904 | — |
| icv1-fp32-tm92567-62490ae7 | 31.62 | rho-neg | 8/8 | 0.941 | 1.035 | 1.876 | — |
| icv1-fp32-t95947-3d71dd90 | 31.65 | ic-a2-1024 | 8/8 | 33.369 | 36.429 | 63.968 | — |
| icv1-fp32-t95947-3d71dd90 | 31.65 | ic-a6-256 | 8/8 | 17.180 | 18.575 | 31.131 | — |
| icv1-fp32-t95947-3d71dd90 | 31.65 | rho-cubic | 8/8 | 0.459 | 0.504 | 0.910 | — |
| icv1-fp32-t95947-3d71dd90 | 31.65 | rho-neg | 8/8 | 0.764 | 0.840 | 1.522 | — |
| icv1-fp32-t107491-16575bac | 31.86 | ic-a2-1024 | 8/8 | 32.933 | 36.159 | 65.198 | — |
| icv1-fp32-t107491-16575bac | 31.86 | ic-a6-256 | 8/8 | 17.676 | 19.314 | 34.053 | — |
| icv1-fp32-t107491-16575bac | 31.86 | rho-cubic | 8/8 | 0.584 | 0.641 | 1.161 | — |
| icv1-fp32-t107491-16575bac | 31.86 | rho-neg | 8/8 | 1.368 | 1.504 | 2.729 | — |

| curve | pair | verdict across the band |
|:--|:--|:--|
| icv1-fp32-tm31847-96b4d8d1 | ic-a2-1024 vs ic-a6-256 | ic-a6-256 < ic-a2-1024 |
| icv1-fp32-tm31847-96b4d8d1 | ic-a2-1024 vs rho-cubic | rho-cubic < ic-a2-1024 |
| icv1-fp32-tm31847-96b4d8d1 | ic-a2-1024 vs rho-neg | rho-neg < ic-a2-1024 |
| icv1-fp32-tm31847-96b4d8d1 | ic-a6-256 vs rho-cubic | rho-cubic < ic-a6-256 |
| icv1-fp32-tm31847-96b4d8d1 | ic-a6-256 vs rho-neg | rho-neg < ic-a6-256 |
| icv1-fp32-tm31847-96b4d8d1 | rho-cubic vs rho-neg | rho-cubic < rho-neg |
| icv1-fp32-t22549-777f0160 | ic-a2-1024 vs ic-a6-256 | ic-a6-256 < ic-a2-1024 |
| icv1-fp32-t22549-777f0160 | ic-a2-1024 vs rho-cubic | rho-cubic < ic-a2-1024 |
| icv1-fp32-t22549-777f0160 | ic-a2-1024 vs rho-neg | rho-neg < ic-a2-1024 |
| icv1-fp32-t22549-777f0160 | ic-a6-256 vs rho-cubic | rho-cubic < ic-a6-256 |
| icv1-fp32-t22549-777f0160 | ic-a6-256 vs rho-neg | rho-neg < ic-a6-256 |
| icv1-fp32-t22549-777f0160 | rho-cubic vs rho-neg | rho-cubic < rho-neg |
| icv1-fp32-tm86219-053c5532 | ic-a2-1024 vs ic-a6-256 | ic-a6-256 < ic-a2-1024 |
| icv1-fp32-tm86219-053c5532 | ic-a2-1024 vs rho-cubic | rho-cubic < ic-a2-1024 |
| icv1-fp32-tm86219-053c5532 | ic-a2-1024 vs rho-neg | rho-neg < ic-a2-1024 |
| icv1-fp32-tm86219-053c5532 | ic-a6-256 vs rho-cubic | rho-cubic < ic-a6-256 |
| icv1-fp32-tm86219-053c5532 | ic-a6-256 vs rho-neg | rho-neg < ic-a6-256 |
| icv1-fp32-tm86219-053c5532 | rho-cubic vs rho-neg | rho-cubic < rho-neg |
| icv1-fp32-tm92567-62490ae7 | ic-a2-1024 vs ic-a6-256 | ic-a6-256 < ic-a2-1024 |
| icv1-fp32-tm92567-62490ae7 | ic-a2-1024 vs rho-cubic | rho-cubic < ic-a2-1024 |
| icv1-fp32-tm92567-62490ae7 | ic-a2-1024 vs rho-neg | rho-neg < ic-a2-1024 |
| icv1-fp32-tm92567-62490ae7 | ic-a6-256 vs rho-cubic | rho-cubic < ic-a6-256 |
| icv1-fp32-tm92567-62490ae7 | ic-a6-256 vs rho-neg | rho-neg < ic-a6-256 |
| icv1-fp32-tm92567-62490ae7 | rho-cubic vs rho-neg | rho-cubic < rho-neg |
| icv1-fp32-t95947-3d71dd90 | ic-a2-1024 vs ic-a6-256 | ic-a6-256 < ic-a2-1024 |
| icv1-fp32-t95947-3d71dd90 | ic-a2-1024 vs rho-cubic | rho-cubic < ic-a2-1024 |
| icv1-fp32-t95947-3d71dd90 | ic-a2-1024 vs rho-neg | rho-neg < ic-a2-1024 |
| icv1-fp32-t95947-3d71dd90 | ic-a6-256 vs rho-cubic | rho-cubic < ic-a6-256 |
| icv1-fp32-t95947-3d71dd90 | ic-a6-256 vs rho-neg | rho-neg < ic-a6-256 |
| icv1-fp32-t95947-3d71dd90 | rho-cubic vs rho-neg | rho-cubic < rho-neg |
| icv1-fp32-t107491-16575bac | ic-a2-1024 vs ic-a6-256 | ic-a6-256 < ic-a2-1024 |
| icv1-fp32-t107491-16575bac | ic-a2-1024 vs rho-cubic | rho-cubic < ic-a2-1024 |
| icv1-fp32-t107491-16575bac | ic-a2-1024 vs rho-neg | rho-neg < ic-a2-1024 |
| icv1-fp32-t107491-16575bac | ic-a6-256 vs rho-cubic | rho-cubic < ic-a6-256 |
| icv1-fp32-t107491-16575bac | ic-a6-256 vs rho-neg | rho-neg < ic-a6-256 |
| icv1-fp32-t107491-16575bac | rho-cubic vs rho-neg | rho-cubic < rho-neg |
