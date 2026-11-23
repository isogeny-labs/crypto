# Complete solver-only budget against the normalized control [pcc:unverified]

Deterministic derivation, no benchmark or solver rerun. derive.rb replays44full16rawcounter checks and22fixture pairings. Every COMPARISONS.tsv normalized count equals the original COUNT vector REDC entry; every vector satisfies REDC=2reduce128+pre+mul_pre. The normalized control is fixed across every fixture/field, not a fieldwise favorable mixture. Exact values, source digests and rational thresholds are in BOUNDS.json/tsv.

Let A be complete base-field REDCs, B complete subgroup-field REDCs, r>=0 the relative price of a subgroup REDC. A replacement that preserves base work and makes ALL subgroup work free has speed ceiling1+rB/A. Tenfold requires r>=9A/B. This is more favorable than a real linear-algebra replacement: it also erases LP, field setup and other subgroup arithmetic. No such free algorithm is asserted.

|genus|zero subgroup cost ceiling at r=1|minimum r permitting10x, range across fixtures|
|--:|--:|--:|
|2|1.038821–1.384270|23.421027–231.831818|
|3|1.021576–1.127200|70.754732–417.128834|
|4|1.005463–1.015441|582.868592–1647.354098|

At r=1 and r=10 none of22fixtures canreach10even in this favorable model. These are exact rational derivations with decimal display, not measured CPU/field prices. r=1 is an explicit equal-REDC counting convention, not an asserted runtime exchange rate. At sufficiently high r the ceiling maypermit10; this result does not exclude that mathematical price regime. Host memory/integer costs are unpriced and could alter runtime proportions.

Concrete g2p4001source1 has A112190,B38838. Even zero subgroup work gives151028/112190=1.346180587... at equal prices; tenfold requires subgroup REDCprice>=168285/6473=25.997991658... . Source17 is separately retained, A116288,B44686; it is not pooled with source1. The previous operator's2048step scalar17trace is not substituted for either of these original public workloads.

Decision: do not build a prefix/Krylov solver as a standalone10xstrategy while retaining the current geometry/base work. The exact prefix/transpose circuits remain reusable; lower operator cost is real at its component scope. A future redesigned collector could change the field proportions, matrix structure and this admission; it needs its own complete-cost argument. This result is conditional on unchanged base work, not a universal solver/isogeny lowerbound or a rejection of all black-box linear algebra.

## Joint redesign requirement

If base work is reduced by factor u and subgroup work by factor v, the declared price-model ratio is (A+rB)/(A/u+rB/v). Tenfold requires A/u+rB/v<=(A+rB)/10. Even taking v=infinity requires u>=10A/(A+rB). Thus a full strategy needs substantial earlier base-field work elimination, plus sound public input validation and final original-target verification. Do not multiply component speedups from different streams or give provided-prefix credit.

The next constructive admission must provide a relation-generation mechanism with explicit signed constraints on original mixed-term models, an exact target quotient/certificate, and a complete budget including setup/failed attempts/row production/verification. Earlier known-source Kummer information, late one-dimensional finishing, and collector-only substitutions are not by themselves that mechanism. No new mechanism is asserted in this accounting screen. Full10xgenus2+ objective remains active.
