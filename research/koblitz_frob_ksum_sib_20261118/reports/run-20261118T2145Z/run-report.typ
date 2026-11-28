#set page(paper: "us-letter", margin: (x: 0.72in, y: 0.68in))
#set text(font: "DejaVu Sans", size: 9pt)
#set par(justify: true, leading: 0.55em)

#align(center)[
  #text(size: 13pt, weight: "bold")[
    Descriptive Run Report: `run-20261118T2145Z` (`koblitz_frob_ksum_sib_20261118`)
  ]
  #v(3pt)
  #text(size: 8.5pt, fill: rgb("#374151"))[
    Commit `bac5b145b51f93346cfa796ff13c27d9a734c64c` (`git_dirty: false`) · Binary `0dd4f18943744e11546be87399393397648f8f7c5ecd3e92109d5a1f2c89107f`
  ]
]

#v(5pt)

== 1. Execution Metadata and Audit Status
- *Host & Isolation:* `ECBENV3h348f48ffd10a` / `ECBHW1hf5f6005c788f`, sequential single-process execution (`--cpus none`, `isolation_required = "L0"`), `order = "alternate"`, `warmup = 1`, `rounds = 2`.
- *Specs & Seeds:* `research/koblitz_frob_ksum_sib_20261118/specs/{K1-n29,K0-n23,K1-n23,K0-n37,K0-n41,K0-n53,B21,B24,B27}.json`, target seeds `2026111801..2026111809`, measurement seeds `8101..8109`, comparison bootstrap seed `20261118` (4000 resamples).
- *Verification & Replays:* All 9 sessions completed with 0 failures, 0 timeouts, 100% verified discrete logarithms (`[k]G == Q`), 100% identical deterministic replays under `ecbench verify --replay-all`, and `bitop/v1` `complete = true` (`unpriced = []`, `known_uncounted = []`). Summary receipts are recorded in `research/koblitz_frob_ksum_sib_20261118/summary.json`.

#table(
  columns: (auto, auto, auto, auto, auto, auto),
  align: (left, left, left, right, right, right),
  inset: 4pt,
  [*Spec*], [*Session ID*], [*Curve Slug*], [*Planned*], [*Measured Verified*], [*Replays Identical*],
  [`K1-n29`], [`ECBS1h9130ca664e3d`], [`icv1-f2m29-tm40309-30c52b96`], [672], [448 / 448], [448 / 448],
  [`K0-n23`], [`ECBS1hb7bd5c42674b`], [`icv1-f2m23-t5197-69e76b73`], [576], [384 / 384], [384 / 384],
  [`K1-n23`], [`ECBS1hb366f3156d09`], [`icv1-f2m23-tm5197-1f85e9e1`], [576], [384 / 384], [384 / 384],
  [`K0-n37`], [`ECBS1h3f13fef4265b`], [`icv1-f2m37-tm534059-32aad96b`], [672], [448 / 448], [448 / 448],
  [`K0-n41`], [`ECBS1h764e82418bf0`], [`icv1-f2m41-tm2308219-7f48b14a`], [360], [240 / 240], [240 / 240],
  [`K0-n53`], [`ECBS1h0a25307fd6a4`], [`icv1-f2m53-tm56619371-dac20a85`], [192], [128 / 128], [128 / 128],
  [`B21`], [`ECBS1h3ca155cc5657`], [`icv1-f2m21-t2591-de54d72e`], [480], [320 / 320], [320 / 320],
  [`B24`], [`ECBS1hc4213df6893f`], [`icv1-f2m24-tm3047-2d0c0e8c`], [480], [320 / 320], [320 / 320],
  [`B27`], [`ECBS1h022bf6240f56`], [`icv1-f2m27-t12543-569dca8b`], [480], [320 / 320], [320 / 320],
)

== 2. Recorded Quantitative Figures
#figure(
  image("fig-band-speedup.svg", width: 96%),
  caption: [Cohort K recorded paired ratios and cold single-target $S(w=0)$ values (`ECBS1h9130ca664e3d` through `ECBS1h0a25307fd6a4`).],
)

#figure(
  image("fig-bitop-and-binary.svg", width: 96%),
  caption: [Cohort K recorded `bitop/v1` gate ratios to floor and Cohort B paired ratios (`ECBS1h3ca155cc5657` through `ECBS1h022bf6240f56`).],
)

*A/A Controls (measured):* Counted `S(w)` ratios between `rho-strong` and `rho-strong-aa` (and `rho-neg` vs `rho-neg-aa`) are identically `1.000` on all 9 sessions; paired L0 solve-wall A/A ratios lie in `[0.990, 1.008]` across all 9 sessions.
