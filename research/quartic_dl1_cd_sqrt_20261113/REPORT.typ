#set page(paper: "a4", margin: 1.8cm)
#set text(size: 10pt)
#set par(justify: true)

= Smaller double-large-prime factor base and a fixed-exponent square root in the plane-quartic index calculus (GHS-weak family)

_research/quartic_dl1_cd_sqrt_20261113, 2026-11-13. Protocol frozen at 978de1aef2 before data. Every result is an unverified assertion (pcc:unverified); no leaderboard row moves before the validation gate._

== Question
The frozen N4 v2 grid (research/quartic_n4v2_20261020) ran the double-large-prime arms D, DL1 and DL with factor-base constant c#sub[D] = 2.0. Two changes, same instances: (1) c#sub[D] = 1.55 at k ≥ 18, near the large-prime-graph percolation threshold; (2) a fixed straight-line chain for the per-pair square root disc#super[(q+1)/4] (24 instead of 36 multiplications at q = 4194271), same values.

== Gates
H0: 129 BASE records reproduce the frozen grid's counted fields exactly. H1: in 174 record pairs the chain changes only the relation-stage multiplication count; the in-run chain check finds 0 mismatches. H2: 116 of 116 units verified, no retries.

== Result under the pre-registered rule: improvement for the headline arm DL
#table(columns: 7, align: (left, left, right, right, right, right, right),
  [change], [IC arm], [k = 18], [19], [20], [21], [22],
  [both], [*DL*], [*0.684*], [*0.677*], [*0.627*], [*0.630*], [*0.584*],
  [both], [DL1], [0.760], [0.757], [0.650], [0.657], [0.569],
  [both], [D], [1.713], [1.686], [1.606], [1.589], [1.568],
  [c rule], [DL], [0.706], [0.680], [0.660], [0.647], [0.624],
  [chain], [DL], [0.963], [0.994], [0.946], [0.972], [0.934],
)
Cold M-eq against the frozen configuration on the same instance (geometric mean over seeds; frozen grid constants; w = 0). DL and DL1 are below 1 at every seed and at w = 0, 0.1, 1; D is dearer. DL against matched rho at k = 22: 0.0036 → 0.0021 (plain), 0.0040 → 0.0023 (negation map).

#figure(image("ratio.png", width: 100%), caption: [Left: paired ratio by size. Right: DL against the frozen matched rho.])

== Caveats (post-hoc, not pre-registered)
*The c rule trades relation work for heavier linear algebra, and the M-eq model prices additions at zero.* At c#sub[D] = 1.55 the relations phase costs about 0.6×, but cycle rows are 3× heavier (weight 27.5 → 89.8 at k = 22). With additions priced at 0.5 M-eq the DL ratio is 0.85–0.91; at 1.0 it is about 1 except at k = 22. In CPU seconds (diagnostic, L0) DL is about even at k = 18–21 and 0.77–0.87 at k = 22; DL1 and D are slower. *The chain* lowers counted multiplications but showed no CPU gain in this interpreted implementation.

== Execution
Local run on the session container (owner decision after the monthly IsoLab budget was found exceeded), with the published image's binaries and the frozen pod runner. Counted operations are host-independent and H0 checks them against the earlier independent grid; time is diagnostic only.

== Next
A c#sub[D] between 1.55 and 2.0, or a lighter cycle basis, to keep the relation saving without the matrix growth; a compiled chain; then a validator report before any leaderboard change.

#pagebreak()
= Follow-up: c#sub[D] = 1.7 and 1.8 (cd_sweep/, protocol frozen before data)
60 units at k = 18–22, seeds 1–3, one unit at a time; 90 records reproduce the main study's c#sub[D] = 2.0 and 1.55 units exactly; all verified, no retries. Pre-registered robustness: additions priced at 0.5 and 1 M-eq.

#table(columns: 5, align: (left, right, right, right, left),
  [DL vs c#sub[D] = 2.0], [additions free], [at 0.5 M-eq], [at 1 M-eq], [CPU (diagnostic)],
  [c#sub[D] = 1.55], [0.62–0.71], [0.88–0.92], [1.00–1.04 (not robust)], [0.76–1.15],
  [*c#sub[D] = 1.7*], [0.73–0.75], [0.82–0.83], [*0.87–0.88*], [0.72–0.91],
  [c#sub[D] = 1.8], [0.81–0.82], [0.87], [0.89–0.90], [0.72–0.99],
)
Headline: c#sub[D] = 1.7 and 1.8 are robust improvements for DL at every k = 18–22 (below 1 at every seed with additions free and at 1 M-eq); 1.7 is the best once additions cost anything. Mean cycle-row weight: 24–28 (2.0), 84–90 (1.55), 41–48 (1.7), 32–38 (1.8). DL1 is robust at 1.7 and 1.8; D (plain Lanczos) is dearer at every c#sub[D] below 2.0. Recommendation: c#sub[D] = 1.7 for k ≥ 18.

#figure(image("cd_sweep/sweep.png", width: 100%), caption: [DL (solid), DL1 (dotted), D (dashed) against c#sub[D] = 2.0 by addition price.])

#pagebreak()
= Follow-up 2: c#sub[D] = 1.7 with the square-root chain (combo/, protocol frozen before data)
45 units at k = 18–22, seeds 1–3, one at a time. 90 records reproduce the sweep exactly; the chain changes only the relation-stage multiplication count in all 45 pairs; all verified, no retries.

#table(columns: 5, align: (left, right, right, right, left),
  [DL vs c#sub[D] = 2.0], [additions free], [at 0.5 M-eq], [at 1 M-eq], [CPU (diagnostic)],
  [c#sub[D] = 1.7 alone], [0.73–0.75], [0.82–0.83], [0.87–0.88], [—],
  [*c#sub[D] = 1.7 + chain*], [*0.68–0.74*], [0.79–0.83], [*0.85–0.88*], [0.80–0.91],
)
Pre-registered headline: robust improvement for DL at every size (prediction 0.68–0.74 with additions free). DL1 0.64–0.75 (free), 0.92–0.98 (1 M-eq); D stays dearer (1.07–1.12). The chain adds 0.6–6.5% in counted cost over c#sub[D] = 1.7 but no measured CPU gain in this interpreted implementation.

#figure(image("combo/combo.png", width: 100%), caption: [DL and DL1 against c#sub[D] = 2.0: c#sub[D] = 1.7 alone and with the chain.])
