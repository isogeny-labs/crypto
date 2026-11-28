#set page(paper: "a4", margin: 1.8cm)
#set text(size: 10pt)
#set par(justify: true)
= NIST seed skip rule: holdout and mechanism checks
_2026-10-26 · research/nist_skip_mechanism_20261026 · protocol frozen in 6a14c3d before native computation_

*Status.* Exploratory. Every frozen test was negative or uninformative. This makes no claim about the historical skip rule and has no bearing on ECDLP hardness.

*Question.* Do the post-hoc separators of the 2026-10-25 criterion search (PR 2378) hold out of sample, and does any mechanism make independent predictions? Population: the 21 valid 1997–98 prime seed candidates (10 published, 11 skipped), plus the NSA binary curves.

*Definitions.* $t = p+1-\#E$, $D = t^2-4p$. A prime $l$ is Elkies if $(D/l) != -1$. $L_E$ is the smallest $l$ at which the product of the odd Elkies primes up to $l$ exceeds $4 sqrt(p)$. Within-stream null: all 4,480 labellings that keep each run's published count. Gekeler null: Frobenius mod $l$ is uniform in $"GL"_2(FF_l)$ with determinant $q$, independent over $l$, conditioned on $l divides.not \#E$.

#figure(image("mechanism.svg", width: 100%), caption: [Recorded values (`out/curves.tsv`, `out/binary.tsv`). (a) The $L_E$ percentile of each prime candidate under the Gekeler null of its field (20,000 draws, seed 20261026). Blue: published; orange: skipped. (b) NSA binary curves (B1).])

*Results.*
- SEC 2 holdout: both rules reject all 4 primary curves. The curves were excluded as SECG/Certicom ("MinghuaQu" seeds), so they say nothing about the NSA generator.
- Schoof special branches: the two curves that the $t mod 13 != 6$ clause removes hit no branch for $l <= 23$.
- H2 (skipped curves have a larger $L_E$): $P(>= "obs") = 0.9998$, falsified. Reverse tail $P(<= "obs") = 0.00112$.
- H4 ($t > 0$): $P = 0.50$.
- B0: the GP port matches every Python value.
- B1: mean percentile 35.9 for B-233/283/409/571, Monte Carlo $P = 0.83$ (20,000 replicates; L_E is discrete, so Irwin–Hall does not apply).
- B2 (Elkies-failure covering set): $S_max = emptyset$, and 0 of 4,480 labellings admit a cover, so the test had no power.
- Controls (amendment 2, after the data): PC1 planted smallest-$L_E$ labels are found at the smallest attainable $P = 0.00022$. PC2: at the published-prime effect size (mean 74.6) B1 would give $P = 0.045$, so its power was marginal.
- Post hoc: mean null percentile 74.6 for published, 39.9 for skipped. Primes dividing the twist order: $P = 0.62\/0.41$. Other Elkies primes below 200: $P = 0.032$.

*Records.* `protocol.json` (`experiment.protocol/v1`) validates as a draft written after the data. The tool refuses to freeze it, since its analysis plan was not frozen before the data, and refuses to export against an unfrozen protocol, so the test rows are in `out/rows.jsonl`. The only pre-data freeze is addendum B (commit 6a14c3d). Under the protocol rules H2 and B2 are in-sample calibration; B1 is the one decisive out-of-sample prediction.

*Limits.* The 2026-10-16 study (H1) had already recorded the same lean ($p = 0.008$, not significant after Bonferroni). The present result is a different null on the same data, not a replication. The Gekeler null was not validated against exact point counts: the check was blocked by the checkpoint gate, which lacks an S3 credential here. The figure and B1 depend on that model; the permutation results do not. No public NSA-generated labelled candidates exist outside the 21: SEC 2 and the X9.62 binary seeds are Certicom's, the NSA binary runs have no skips, and the B-163 seed is unexplained.
