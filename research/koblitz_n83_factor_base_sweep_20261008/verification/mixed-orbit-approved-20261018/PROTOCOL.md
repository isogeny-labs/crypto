# Approved first retained-N83 mixed-orbit replication

Execute the already frozen [K1182/K2048 first-gate protocol](../mixed-orbit-search-12600-source-20261012/PROTOCOL.md)
once, against the public `a=0` fixture-zero point and the exact signed-Frobenius
S3 objects identified there. This is a fresh replication of a previously
retained bounded trial, not a new relation-yield estimate. The run-specific
[authorization](authorization.json) binds fresh build and run directories and
the user's additional 2,400-second local grant. Preserve both S3 GET hashes,
the offline Linux build, exact import, one 3+2 search with a 5,000-conflict
cap, independent group/event audit, all first attempts, and Docker cleanup.

The source supervisor's v2 authorization and budget fields require the
historical `prior_total_charged_seconds: "12600"` baseline. That field is
**the frozen gate's vintage accounting reference**, not the study's current
cumulative charge: subsequent independently retained runs already advanced
the ledger. Report this round's conservative charge separately, and do not
present a 15,000-second cumulative figure as current. A later cumulative
ledger reconciliation must include the intervening gates rather than silently
reusing the historical baseline.

The caps remain an offline four-CPU, 8-GiB, 450-second build (600-second
charge); a one-CPU, 4-GiB, zero-swap, networkless 360-second import; a
one-CPU, 4-GiB, zero-swap, networkless 1,200-second search with a 1,150-second
worker watchdog; and a 240-second cleanup/audit reserve. Charge all launched
stages at their conservative caps, at most 2,400 seconds for this round.
One search trial uses three K1182 small orbits and two of the 866 residual
K2048 orbits, ordered first-pair symmetry, 32 models at most, 150,000 SAT
variables and 12,000,000 domain clauses. Timeouts, resource caps and solver
caps remain censored. A complete cold-runtime winner requires natural useful
yield, graph/rank growth, linear algebra, individual-log extraction, complete
verification and a matched same-point rho comparison.
