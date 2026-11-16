# AMENDMENT-1 (written after the primary run, before any replication data)

Primary outcome under the frozen rule: SIGNAL, from exactly one cell:
n = 23, K1, arm rsub seed 5 (Z = 23.957, matched null max 23.601, null median 15.469).
The frozen rule is not changed; this amendment only fixes the replication the
protocol requires ("fresh-seed replication and n = 29").

## R1: fresh nulls for the flagged cell
Same curve, same FB (rsub seed 5 is deterministic), 200 fresh null seeds (salt 0xF8E5).
Replicated iff Z_flagged > the 99th percentile of the 200 fresh null Z (i.e. > the 198th order statistic).

## R2: fresh structured factor bases
On n = 23 K1: rsub seeds 6..25 (20 fresh random subspaces), each with 20 fresh null seeds
(salt 0xF8E5), scored by the frozen rule. Replicated iff the non-flat rate among these 20
exceeds the null-vs-null base rate (derived R3 below) by a one-sided binomial p < 0.01.

## R3 (derived calibration, from existing data only)
Base rate of the frozen rule on nulls: each null seed scored against the other 19 nulls of its
group, over every (curve, arm) group of the primary run.

## n = 29
Blocked here: the n = 29 cell needs about 2^28-point FFTs (Bluestein, > 16 GiB) and a
2 GiB table; this container has 15 GiB and no route to isolab-eks (no AWS identity).
Recorded as not attempted.

## Overall
SIGNAL is upheld only if R1 and R2 both replicate. Otherwise the primary SIGNAL is recorded
as a single-cell false positive of the frozen rule and the reading is FLAT at this scope.
