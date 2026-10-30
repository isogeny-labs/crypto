# N19 signed-label Kummer fallback: evidence packet

The [frozen protocol](PROTOCOL.md) specifies the requested 1,260-target
same-process collector/rho comparison and 36 price cells. The new arm
retains exact signed `(a,b)` labels. When a selected differential-addition
output is zero, it reconstructs the signed successor from public `G,Q`,
validates both reduced divisors, and projects them by the classical
degree-0/1/2 Kummer chart followed by the published C1 transformation.
The collector source is [main_labelled.rs](../src/main_labelled.rs); the
paired runner is [paired_labelled.rs](../src/bin/paired_labelled.rs).

The label invariant follows by induction. The initial signed divisor is
`Q=0G+1Q`. A branch maps `D` to `2D+choice·G`, hence labels to
`(2a+choice,2b) mod r`. A restart samples `(a,b)` and constructs that exact
signed divisor. On a nonzero xADD output, the N18 geometric identity gives
the projected successor; on an all-zero output, the new path computes the
signed successor directly from those public labels. This covers the
zero-output branch without recovering the target scalar. Its cost is
charged when executed; the finite gate checks the implementation.

The finite gate passed on all 21 frozen models. It forced a zero xADD
output, checked signed-label reconstruction against the separate group
expression `2Q` or `2Q+G`, and compared degree-two output to an
independently coded C2 polynomial-remainder oracle. The valid divisor
`u=x(x-1),v=0` on each model has `a0=b0=0`: the older generic projection
returns zero, while the C1 chart gives a nonzero vector agreeing with C2.
Three ordinary signed transitions per model also passed projective
reconstruction. This forced-zero injection exercises the fallback code;
it is not a witness that a valid mixed-zero xADD occurs naturally.

The separate one-target paired diagnostic ran with transition validation
enabled. It solved and verified the planted scalar with 278 quotient steps,
140 signed split relations, 556 charged transition checks, and no natural
fallback step. Its raw [collector](results/gate/validated-target-16.jsonl)
and [rho](results/gate/rho-target-16.jsonl) rows are preserved. The
full cohort runs with this diagnostic option disabled, as frozen, and the
independent analyzer replays the final source scalar for every target.

The managed `n19-labelled-paired.service` run finished with exit code zero:
1,260/1,260 candidate and 1,260/1,260 same-process negation-rho solves
were verified. Its [receipt](results/cohort-receipt.json),
[input hashes](results/inputs.sha256), and [log](results/cohort-run.log)
pin the 25-minute run. The candidate processed 230,430,039 quotient
states, admitted 109,480,302 signed splits, recorded 739 intermediate
Recover failures and 1,080 restarts. It invoked the signed fallback zero
times in this finite cohort. This is an observation about these targets,
not an all-state exclusion of the mixed-zero locus.

The independent [analyzer](analyze.py) and separate [validator](validate.py)
passed exact joins and original-source scalar replay for all 1,260 target
pairs. All 1,260 fresh rho arithmetic counts match the archived source
control. The standardized [per-target prices](results/analysis/target-price.csv)
contain 45,360 target/cell rows; the [price grid](results/analysis/price-grid.csv)
contains all 288 slice/cell rows, including all 36 whole-cohort price cells.
There are zero failed or null candidate ratios. In each of the 36
whole-cohort cells, the candidate has zero target wins and 1,260 target
losses against rho; across all target/cell comparisons the counts are
zero wins and 45,360 losses. Whole-cohort full-cold N19/rho ratios span
8.95969168677384–55.935443295511135. The separate
[cost graph](cost-graph.svg) and [PDF](REPORT.pdf) render the validated grid
without replacing the CSV.

At `S/M=0.5`, `I/M=10`, and native charge `0.1 GAE/logical`, total cold
cost over all 1,260 targets is 15,631,573,588.27619 GAE for N19,
15,631,567,864.990482 GAE for the frozen uncached collector, and
1,088,802,370.3333325 GAE for matched rho. N19/rho is
14.356667485478237; N19/baseline is 1.0000003661363823. The
seven slice ratios at that same cell are:

| Bits | N19/rho | N19/baseline | Target wins/losses to rho |
| ---: | ---: | ---: | ---: |
| 16 | 5.4277295291 | 1.0001546999 | 0 / 180 |
| 20 | 8.2509078505 | 1.0000511213 | 0 / 180 |
| 24 | 12.7180099828 | 1.0000198982 | 0 / 180 |
| 28 | 14.0925998942 | 1.0000035374 | 0 / 180 |
| 32 | 14.3911075251 | 1.0000012042 | 0 / 180 |
| 36 | 13.5432809421 | 1.0000002663 | 0 / 180 |
| 40 | 14.5875700270 | 1.0000000703 | 0 / 180 |

The ratios above divide sums of cold costs in each slice. The 36-cell
range and every target-level win/loss remain in the CSV. Zero natural
fallback calls mean this cohort measures the cost of the total projection
at restart and its existing xADD path; it does not measure the frequency
or cost distribution of naturally reached all-zero xADD outputs.
