# Frozen pair-line / large-prime correctness gate

Same Q1/F127 source fixture, D1 and B=2*(3,21), kernel/alpha and lambda10.
No timing measurement or source change outside this study.

Fast rational points: for each127affine x, restrict F(x,Y,1), gcd withY^127-Y,
then split its linear part with native degree<=4oracle. Add infinity roots.
Require exact ordered equality to the original exhaustive129point list.
Use with_points constructor so candidate setup does not redo the quadratic scan.

Factor base is exactly first48rational points (all affine); anchor index0log0.
Reference canonical hyperplane indices0..3, checked x=0. Enumerate all1128pairs
from this base, normalize/deduplicate their lines. Divide the two known roots
out of the restriction, solve residualdegree<=2, account infinity once, reject
repeated/nonrational intersections. No scanning allpoints to find line roots.
Every admitted line has4distinct rational points and at most2large primes.

Rows sum(line)-sum(reference), anchored. Eliminate LP columns using pinned
Diem minimum-weight sparse routine, instrumented only to retain normalized
pivot rows. Require output rows, ops and entry counts identical to unmodified
routine. Factor-base matrix has47columns; require rank46/nullity1. Reverse
pivot rows reconstruct only LP logs whose dependencies are known. Unresolved
logs remain None and are forbidden in descent; never assign them zero.

The exhaustive full-base collector is control-only. Its rank127kernel is not
used to generate candidates or solve reduced system. Compare every recovered
coordinate to its single scalar multiple and check admitted-row residuals.
Native old principal-divisor controls and corrupted-row rejection also run.

Recompute both public maps, descend using known coordinates at translates0..64,
recover scalar by ratio and verify on source/destination. Any rank/descent/root/
LP mismatch is terminal and retained, no base-size tuning or rerun. Cap600s.
Record counts, exclusions, reducedrank, all known/unknown logs and scalar.
Source+lock+protocol committed and S3checkpoint before one release execution.
Build allowed beforehand. No performance/10x/asymptotic claim from this gate.

Literature: Diem, On the DLP for plane curves, JTNB24(2012), §1.2 and3.2,
https://www.numdam.org/item/10.5802/jtnb.815.pdf : lines through two FBpoints,
large-prime trees and sparseLA. This fixed-base gate does not implement its full
staged enlargement/depth proof. Pinned repo diem_quartic.rs eliminate_lp reused;
extract.sh records exact source/reconstruction-only instrumentation. Prior
mappedic13 exact solve and currentKBrevision9141e386254340a9bdba3036f2bb706a
queried 'Diem plane quartic two large primes factor base square root graph relations'.
No novelty assertion; currentKBnotes predate recent transport work.
