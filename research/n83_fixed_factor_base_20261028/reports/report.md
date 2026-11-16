# Fixed n83 factor-base points

Observations from complete enumeration; no performance or degree claim.

All 134,217,728 candidate x-values per curve were enumerated with ell=27. Every m=2..9 uses the same per-curve manifest. Both signs of each affine lift are stored; infinity and subgroup filtering are excluded.

| Curve | Liftable x | Stored points | Compressed bytes |
|---|---:|---:|---:|
| koblitz | 67110393 | 134220785 | 1329389614 |
| binary | 67113567 | 134227133 | 1329453079 |
| prime | 67106828 | 134213656 | 1849491803 |

384/384 shards completed. All candidates checked, including omitted x-values; slow-arithmetic cross-checks and mutation controls pass. Stored compressed bytes match SHA-256.

R1 was cancelled before scheduling after MaxSpotInstanceCountExceeded. R2 admission rejected an odd CPU count. R3 failed before enumeration due to storage permissions. R4 used6threads on existing workspace Spot,8Gi virtual-memory limit and3300s producer timeout. Process PID: 1039476. Run: local-r4.

Protocol: "3e373a7c5550d747c4584cac7ec51eb86d7d046e57d044c327e5009645c6c674". Source: "a3d2c86829dcb70723fee72a5f2b4566d63cf8ad". Image (null for native host run): null.

Raw data: s3://setec-astronomy-artifacts/runs/n83-fixed-factor-base-e27-20261028-r4/local/

The standard first-fall sweep used varying ell and is not rerun here. No subgroup membership, end-to-end IC, independent research review, or timing claim follows.

Sources: [SEC1 v2, section2.3.4](https://www.secg.org/sec1-v2.pdf); [Taverne et al., binary arithmetic section2](https://cacr.uwaterloo.ca/techreports/2011/cacr2011-07.pdf).
