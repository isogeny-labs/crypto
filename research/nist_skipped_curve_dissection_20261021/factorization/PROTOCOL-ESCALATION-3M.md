# Further bounded ECM continuation

Recorded before this search stage on 2026-10-21. Six of the nine target
integers have complete native-certified factorizations. Three extension-
twist orders retain composite cofactors after 100 curves at B1=100000 and
200 further curves at B1=1000000.

For each of these same three residuals, attempt up to **100 further curves
at B1=3000000**, with a **600 second process limit per cofactor attempt**,
at most two simultaneous workers, and at most 1024 MB of ECM stage-2
memory per worker. Retain exact inputs, parameters, residues, logs, and
exit statuses. Continue after a split only with the remaining curve
allocation. Certify each proper factor and independently replay the final
partition. No new targets or acceptance criteria are introduced.

An exhausted allocation or timeout leaves a composite unresolved. It
provides no lower bound on its smallest prime factor. Further escalation
requires a new protocol record; no remote machines are launched here.
