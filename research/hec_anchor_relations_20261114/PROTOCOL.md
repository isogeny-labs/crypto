# Target-seeded anchor relations and signed-gain forest (2026-11-14)

The goal remains a mathematical >=10x complete-cost improvement for genus2+. No current result
meets it. Principal-row pilot: all330 solves correct but unrestricted rows lose at every size.
Table pipeline: all660 solves correct and220 exact stream/pivot gates pass; best-native total
gain1.15..1.97x. These observations motivate eliminating generic group additions and a large
linear-algebra core together. This protocol precedes the new implementation and timing.

## Construction and mathematical boundary

Retain the exact principal identity in hec_relation_math_20261114/DERIVATION.md. First descend
D1 and D2 into full-base place sums; every place appearing in either decomposition becomes an
anchor. Fill to H distinct anchors using the seed, where H is max(target support, g+1,
ceil(2 p^(1/3))) for g2, ceil(p^(2/3)) for g3, ceil(p^(3/4)) for g4, clamped to the catalogue.
Choose g+1 distinct anchors and interpolate h of degree<=g. Split Q=(f-h^2)/A of degree g with
the exact tabulated oracle. Keep only relations with at most2 distinct off-anchor places.

Each relation has <=2g+1 terms, including at most two off-anchor logarithms. Weighted union-find
represents L_u=alpha_u L_root+beta_u dot H. For an incoming cu Lu+cv Lv+h dot H=0, substitute
both endpoint expressions. Different roots are joined after dividing by a nonzero coefficient.
For a cycle with combined root coefficient zero, h is a pure anchor relation. A nonzero cycle
coefficient instead determines its root in terms of anchors; attach it to the distinguished
zero vertex. This is coefficient-aware elimination, not unsigned cycle counting.

All target rows are on anchors, so only the anchor logarithms and k remain in final elimination.
The number of possible interpolants is at most2^g binomial(H,g+1), after quotienting global
negation. Distinct anchor tuples can still yield the same interpolant; deduplicate normalized
principal rows too. Finite supply and rank failure are real failure modes, not hidden retries.

For g2, a heuristic random signed graph suggests a cycle threshold around half the off-anchor
catalogue size; that is a cost hypothesis only. The geometric outputs need not be independent
random edges. At g3/4 the requirement of <=2 off-anchor places can erase any relation gain.
The construction is related to geometric index calculus and large-prime forests, already cited
in the two preceding studies. No novelty is established by this combination.

## Correctness gates

Check every accepted principal row by exact Cantor group addition. Obtain an ell-torsion
projection certificate for the full catalogue by finding h prime to ell, within conservative
integer Hasse bounds, such that (h ell) kills every catalogue place. Check hF_j is killed by ell.
Then check every emitted pure-anchor row on projected places. This gate-only computation is
counted and logged separately, not charged selectively to one timed arm.

Check weighted-forest identities against generic elimination on planted modular systems,
including balanced/unbalanced cycles, signed coefficients and singleton partials. Descents,
normalization and final k D1=D2 verification remain mandatory. Certify ell^2 exceeds Hasse upper
bound before normalizing logs. No knowledge of planted k enters the candidate algorithm.

## Pilot and caps

Same eleven instances, five seeds20260916..20260920. Compare to strongest observed cold native
AND table configurations from hec_table_oracle_20261114/run/pipeline_summary.json, with matched
input and target. No precomputation is shared between solves. Include lookup construction,
catalogue, inverse cache, descents, row dedup, every forest step and solve retry in total time.
All arms must verify, and count rank/capped failures explicitly. Cap1million tuple draws,
16 solve attempts and10000 steps per descent. H growth is not allowed in this frozen pilot.
The pilot is exploratory; >=10x still requires frozen arms, held-out curves, scaling and replay.

Relevant sources already checked: GTTD three-author primary draft (2005), section3.1, signed
cycles; Magma handbook on principal-divisor/geometric IC; existing table-oracle Cardano/Ferrari
proof and gates. No new family or special-subfield assumption is used.
