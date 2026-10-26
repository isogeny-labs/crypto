# Addendum A (written after seeing the P2 generator family, before any new data)

Observed in run-20261019-n45: the generators family (20 generators x 2 vertices,
10 solves each) is "more extreme than the family": Kruskal-Wallis p = 1.2e-4,
max |z| = 4.76 (Bonferroni 3.23), flagged arms v158g12 (1.64x) and v158g17 (0.61x).
This is not a preregistered confirmation. It is a deviation from the null that
has to be checked, not explained.

Replication (new data, new seed, otherwise the identical command):

    isoclass-rho measure --walk P2/walk.json --mode generators --generators 20 \
      --max-vertices 2 --trials 10 --controls 0 --seed 20261019 --out P2-gen-rep1

Rule fixed now: the deviation "replicates" if Kruskal-Wallis p < 0.05 again;
otherwise it is reported as not replicated. Which arms flag is not compared
(generators are redrawn). The result is reported either way, next to the first.
