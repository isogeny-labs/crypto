# Prime31 norm-one-torus m=5: fair-baseline ablation and x-only query

Browsing copy for the follow-up to PR #5229. The source of truth is
[`research/norm_one_torus_ablation_20261127`](../../research/norm_one_torus_ablation_20261127/).

Against a fair control (native 64-bit inversion and an Fx hash, 15.410 ms),
the PR #5229 batch + signed-quotient lane takes 6.195 ms (2.487x [2.476–2.497])
and the new affine x-only query takes 3.358 ms (4.589x [4.580–4.604]). The PR's
own pair reproduces at 4.538x. Medians of 41 rounds; 2-vCPU Xeon @ 2.30 GHz,
one pinned core, not isolated. There is no matched rho reference and no keyring
signature, so this is not an admitted benchmark row. [pcc:unverified]

- [Report PDF](report.pdf)
- [Headline figure (SVG)](headline.svg) [pcc:unverified]
- [Headline figure (150 dpi PNG)](headline.png) [pcc:unverified]
- [Source report](../../research/norm_one_torus_ablation_20261127/REPORT.md)
