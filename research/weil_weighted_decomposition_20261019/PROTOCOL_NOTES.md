# Control arrangement and counting

The frozen protocol's A/A requirement is implemented in two places.
Every run includes two c=1 PDP arms with identical targets and RNG state;
the runner asserts identical decisions and counters. Within a primary
case, full DLP recoveries are stored for c=1/2/4, not a fourth recovery.
The separate complete primary replay supplies the eight c=1 DLP A/A
repetitions, as well as repeating all c=2/4 recoveries. Its entire raw
record file compares byte for byte, including every recovery counter and
scalar. The A/A DLP control therefore occurs in a separate process rather
than as a fourth item in each primary case's recovery array.

The result headline counts 36 distinct curve/coefficient recoveries:
24 in the primary run and 12 on the four larger follow-up fixtures.
The replay's 24 successful duplicate recoveries are additional verification
and are not counted as 24 new curve/coefficient cases. The follow-up uses
the same larger curves and PDP panels as the primary, so its stage samples
are overlapping evidence, not an independent holdout. Seed 936451 was
the construction holdout fixed before the primary measurements.

No uncertainty intervals, publication-novelty finding, complete S or rho
speed ratio are inferred from these repetitions. Frozen protocol files and
raw records are preserved.
