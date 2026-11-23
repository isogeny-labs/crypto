# Multiroot bucket extraction: cheap kernel, sparse coverage

[pcc:unverified] Freeze5c41bff6150 pushed before once-only invocation; input hashes pass, exit0. All334 eligible last-root recoveries match exact division and oracle output and pass full principal/native group identities. Two repeats have identical16counter vectors and REDC identities. Every enumerated pair reconstructs the same full pencil polynomial.

|genus|p|pair buckets|pair choices|eligible g-known buckets|insufficient known roots|last root repeats known|reference REDCs|Vieta REDCs|
|---|---|---|---|---|---|---|---|---|
|2|101|233|655|233|0|17|13938|466|
|3|113|775|1114|75|700|6|7462|150|
|4|109|672|1304|26|646|5|3842|52|

All signed rational places were available to each anchor. This is an optimistic full-point-table algebra panel, not a small-factor-base collector or original-target solve. Degree-deficient anchors skipped1/3/4 respectively; u-zero places excluded as declared. Setup, scanning, anchor construction, diagnostic exact/native checks and relation-rank work are excluded from the component counters.

The one-multiplication last-root kernel is correct across genera2/3/4 under its hypotheses. Its reference is full quotient construction plus sequential division by all g known roots and the linear oracle; this is not the previous optimized truncated quotient or fastest extraction baseline. Therefore large extraction ratios are implementation/component comparisons, not a mathematical10x result.

Coverage is the limiting observation: only75/775 g3 pair buckets and26/672 g4 pair buckets have enough distinct known roots, despite the full rational-point table. The previous truncated-quotient panel had28 split g4 cases; the difference of2 is retained, not erased: fully split repeated-root polynomials need not have g distinct known roots. A production dispatcher must retain a multiplicity-aware fallback and charge all scans, not select these eligible cases alone.

The theory rules out rank gain from all-pair enumeration within one fixed anchor/t. Pair-choice counts655/1114/1304 are not independent principal relations. This does not rule out improving scheduling, different pencils, LP processing or multi-anchor relations.

Decision: retain general Vieta kernel as an optional g-known-root fast path; reject all-pairs within one bucket as a new useful-rank source. No fullcollector speed claim. Next cost case must change how independent pencils/supports are generated or avoid scanning many nonproductive supports, with exact LP elimination and all unsuccessful work retained. Independent validation and full genus3/4 original-target coverage remain open.
