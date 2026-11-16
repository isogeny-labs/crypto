# Recovery fixture dependency clarification

This immutable addendum narrows interpretation of the84-check packet at /var/tmp/pencil-kummer-degree1-recovery-20261031. The fixture first computes expected with signed Cantor arithmetic and then constructs external_plus=project(expected+G). It passes this separately constructed non-torsion neighbour to signed recovery. The checks therefore validate recovery given that valid supplied neighbour; they do not demonstrate end-to-end signed recovery directly from the selected torsion-difference pair.

The helper does not receive expected as an explicit argument, but the supplied neighbour depends on expected. Consequently the fixture is not evidence of oracle-independent collector execution. The cost of constructing this neighbour was not separately priced or included in any end-to-end comparison. No such comparison was reported. Actual collector state retention or a charged construction of the required neighbour remains a separate implementation obligation.

The84 repaired arithmetic comparisons and degree1 helper checks remain recorded at their component scope. Earlier packets are unchanged. This addendum supersedes any broader reading of the wording about helper arguments or signed recovery composition.
