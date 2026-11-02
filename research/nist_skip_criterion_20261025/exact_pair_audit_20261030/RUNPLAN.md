# Exact allocation audit of existing split scores

30 October 2026. This is a retrospective arithmetic audit of our independently
verified69-position results, not a rerun of another repository's experiment.
The external report and its disclosed deviations are source evidence only.
Do not execute heldout.py, its Monte Carlo/null code or its 48-bit sampler.
Do not repair its original checkpoint/protocol history by relabelling this audit.

## Fixed sources and comparison

Our prior source/result is c6d8897d7582fed39e8eb7fdd4e312a70d692532, merged
in PR2666. Copy its exact82branch endpoint file; retain all69selected positions.
The source file SHA256 is
a4b53b2d22fc567658f315c6400cae3703f55acbbd463c156090d87f75e80510.
External sources are pinned to crypto-autoresearcher revision
3ff8c32c15f91b355d18e207d6044a1d2363499f. Its heldout.json reports0.5735294117647058
over102pairs and a200000-draw permutation estimate0.29276853615731924.
Those reported values are already visible; this audit is not a new holdout.

Use its written primary population definition unchanged: the42additional
order-width-only/other-root-only positions in192/224/239/256fields, versus
the eight publications in those fields. Exclude the eleven named omissions
from this primary comparison. Retain P384/P521in the input and identify their
exclusion from this pre-existing comparison explicitly. Do not pick a new cutoff,
statistic or favourable subset. Cutoff691, strictly+1characters, ties one half.

Copy only integer score/label fields from external JSON to a TSV. Verify all
60seed-identified source observations exactly, and compare the eight source
publication scores as within-field multisets because those source rows have
no seed identifiers. Do not claim an individual identity replay for these eight.
No model percentile or Monte Carlo probability is silently treated as a real-
field random-curve measurement. No unknown field is used numerically.

## Exact statistic and null enumeration

For each within-field(additional,published)pair count2for additional>published,
1for equality and0otherwise. This doubled score avoids floating arithmetic.
There are102pairs and204possible doubled units. Preserve exact wins/ties/losses.
Enumerate every assignment retaining each field's published count. Field
allocation inventories are C(24,3),C(9,1),C(12,3),C(5,1), giving20037600joint
assignments. Record the entire joint histogram, each field histogram and the
inclusive upper tail at the observed score. Report numerator/denominator
exactly; decimal approximations are presentation only.

Producer recursively enumerates each field's subsets and computes pair scores.
Independent verifier uses doubled midranks and a chosen-rank-sum dynamic program:
score2=2KH+K(K+1)-sum(doubled published midranks). Both convolve field histograms
with integer arithmetic. Shared TSV I/O contains no statistic or null arithmetic.
The finite allocation fraction becomes a p-value only under the stated
within-field label-exchangeability model. It is not a manipulation probability,
historical explanation, correction for every searched trait or security score.

## Execution and controls

AGENTS9a explicitly permits local analysis of existing results as development
work. Use CHECKPOINT_SKIP=exact_reanalysis_of_existing_verified_results and keep
that interpretation explicit. No new curve, point count, character, factor,
random draw, DLP, timing or paid job. No external experiment/checkpoint gate
is waived and no S3receipt is claimed. Preserve the denied old paper payload.

Commit sources, input identities and commands on the new feature branch before
actual allocation arithmetic. Build with C++17/warnings-as-errors. One native
process at a time,256MiBaddress ceiling,30seconds per phase, fresh directories,
no automatic retry. Toy tied/non-tied controls precede production. Producer
and verifier must agree byte for byte on all five output files. Then three
fresh copies alter a field count, joint frequency and summary tail; each
verifier must refuse with native exit1and no receipt. Seven exit0phases plus
three expected refusals constitute the ten-phase flow. Preserve every failure.

## Other sources and limits

RFC5639AppendixA.2was retrieved directly with normal proxy/TLS; its steps12–13
choose a smallest-x point and multiply by a public derived scalar. That standard
motivates a separate algebraic note on generator base changes; it is not evidence
that the1997generator used the same procedure. All prior G/b measurements and
papers remain frozen. An algebraic base-change equivalence is not a security proof.
The requested60000accepted real-curve baseline remains unrun; no independence-
across-small-primes model substitutes for it. Original decision logs unavailable.
