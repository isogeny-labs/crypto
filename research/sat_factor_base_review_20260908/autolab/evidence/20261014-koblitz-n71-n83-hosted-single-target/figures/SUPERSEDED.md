# Figures: superseded online points (2026-10-20)

The files in this directory plot the empty-table-rho online points of the
n71/n83 hosted single-target panel, with IC setup excluded. They are
superseded (validation sweep B 104.1) and are not improvements. The original
bytes are left as written (sha256 below); the marked copy is
`benchmarks/20261014-n71-n83-hosted-single-target/n71-n83-hosted-online-ms.{svg,png}`
(renderer `plot_marked.py` there), which draws the rho series hollow and
labelled SUPERSEDED and, for N83, overlays the Bernstein-Lange precomputed-rho
arms from `research/20261016-n83-precomputed-rho-control/` (K600). No
precomputed-rho control was measured for n71.

Do not rerun `plot.py` here: it writes over the original figure files.
No unit-pinned manifest covers these files.

| file | sha256 |
|---|---|
| data.json | `92390175f0847e662d9156d6bcf5a09430b44f7dc00048677c0d9b39d8c3ebd5` |
| n71-n83-hosted-online-ms.png | `a0367f05734a8c25fc5c000af237699176fddec5b339bd0e273fdac8c734a1a0` |
| n71-n83-hosted-online-ms.svg | `a7d54fcd759c602b5adb75e8a53c03fc2777442d3ce3ad9d958793db4d9ffcdf` |
| plot.py | `99482a34cff2474ce4a03c833f6027ac4314b3cb226c9f21725bf9b36e6205a4` |
