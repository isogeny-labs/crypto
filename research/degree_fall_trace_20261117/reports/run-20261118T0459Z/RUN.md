# Regularity from two survivor fibers

Run 20261118T0459Z uses the [matched toy bases](../run-20261118T0159Z/RUN.md)
and follows the [conditional survivor protocol](../run-20261118T0409Z/RUN.md).
The candidate rule is that the observed degree is the largest surviving
(y)-fiber size plus one when the two nonempty fibers have the same size.
A fiber is the set of points with a given value of (y). The target and
membership equations are fixed as in the previous run.

Check the rule on every reachable mixed target and every shared pin of
the common target. The independent reference computes ranks of evaluations
of low-degree monomials. Compare all unpinned target degrees with the prior
Singular table. The separate point-set controls pair every subset of the
five (x)-values ({0,1,2,17,43}) on (y=1) with every subset on (y=2).
Skip the empty pair of subsets. Include two three-fiber controls with the
same occupancy to test where the two-fiber rule stops applying.
