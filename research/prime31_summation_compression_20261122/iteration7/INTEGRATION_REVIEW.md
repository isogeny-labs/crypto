# Integration review of field and factor-base availability

Date: 22 November 2026 America/Los_Angeles / 23 November 2026 UTC. Written review only; all new assertions remain [pcc:unverified].

## Fixed source curve

Root proposed the fixed-curve averaging argument and independently checked the completed note against its two worker reviews. The endpoint indicator gives exactly (p+3+S_f-S_fg+epsilon_++epsilon_-)/4 liftable norm-one trace coordinates. Odd rational curve order excludes rational roots of f; nonsingularity makes f squarefree and the product f(X)(X^2-4) squarefree of degree five. Genus-one and genus-two Hasse-Weil bounds therefore apply without a missing point-at-infinity correction. [pcc:unverified]

The q/2+1 and q/2 exceptional trace counts are disjoint, leaving M=(p+1)/(2q)-1 regular fibres of size q. The average-error numerator is 3sqrt(p)/2+q/2+1. For the stated cap and p>=4096 it gives a dense-fibre fraction at least1/6. Each regular fibre has2q torus seeds; the small exceptional-seed fraction gives fewer than7 expected trials. Root checked both the exact formulas and conservative constants. This is an expected O(q log p) field-operation setup specification plus priced inversions, not an implemented timing result or relation query bound. [pcc:unverified]

The theorem varies the fibre parameter c on the same supplied curve. For a supplied whole-group prime-order curve it directly supplies a base in that group. It does not show density for a frozen c, for a proper subgroup, or for a newly asserted prime-order family. The separate free-order comb proof retains its own hypotheses.

## Selectable curve

Root read the complete independent derivation in SELECTABLE_CURVE_AVAILABILITY.md. For distinct coordinates the two affine evaluation maps have nonzero determinant, so the exact first two moments follow from pairwise independence before conditioning. The irreducible depressed-cubic count is (p^2-1)/3, obtained from the nonzero trace-zero elements of F_(p^3) modulo Frobenius. Intersecting the balanced-character event with irreducibility uses subtraction of probabilities, not independence after conditioning. [pcc:unverified]

For p=3 mod4 and F=-F, the involution b -> -b preserves irreducibility and balance, reverses trace, and has no fixed irreducible model. The [p]P anomaly test is justified for nonzero P by 0<#E(Fp)<2p when p>=7. These arguments establish the selectable-curve density and charged construction at the stated scope. They do not transfer a DLP from another curve. [pcc:unverified]

On composite odd-order curves the no-zero-prefix proof needs each permitted leaf to have order greater than m-1. Removing small-order leaves costs extra group checks and extra exact membership restrictions; the original iteration6 equation counts do not include those guards. Root checked the backward-peeling extension and the conservative O(m^3) deletion bound. [pcc:unverified]

## Growing field and source checks

The Linnik inequality applies to a least prime p=-1 mod4Q with Q a power of two. Capping q at the largest power of two below min(Q,sqrt(p)/16) retains a polynomial lower bound after taking the absolute exponent L>=2. This does not guarantee a particular desired base exponent. The original family note is retained at its source hash; its incorrect book title is explicitly corrected in INDEPENDENT_REVIEW.md. [pcc:unverified]

Root opened Koukoulopoulos, The Distribution of Prime Numbers, Chapter27 Theorem27.1; Waterhouse, Abelian varieties over finite fields, Theorem4.1; Galbraith's author-maintained Chapter10 and its genus/count conventions; and the explicitly labelled transcription of Weil's original theorem. Primary links and precise scopes are in the two notes. No source provides a new exact empty-target solver here.

## Integration and execution boundary

The source at a4ddc035c0c7519d6bab7a438d8a168f74ae40db restores inherited ecbench merge damage and retains unchanged arithmetic examples and original manifest/lock. At the latest observed check, original-lock workflow45097885135/job137441679059 was compiling/running the inherited spec regression step; the new rational-chain tests, deterministic receipt and identity issuance had not completed. The preceding failed run remains retained. No passing new native test or native identity is inferred from these mathematical proofs. [pcc:unverified]

The next algorithmic obligation remains a fully specified exact target solver. In the conventional full-rank model, the m=4 target with setup exponent at most2 and quadratic linear algebra requires an empty-inclusive query exponent below1 to meet the previously stated sub-square-root budget. Neither the quadratic verification circuit nor the base-availability theorem supplies that exponent. [pcc:unverified]
