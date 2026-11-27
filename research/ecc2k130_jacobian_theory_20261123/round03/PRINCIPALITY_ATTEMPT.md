# Bounded fixed-input certificate attempt

Input: field.gp (exact H), ideal(263,x+1), source principality_attempt.gp.
Execution: existing local executor, one GP process; at most30 seconds,
256MiB initial/512MiB maximum PARI stack,1GiB process address-space cap.
No timing comparison or performance claim; no new cloud resource or paid job.
This is a bounded mathematical certificate attempt on the already active
executor, following the finite exact checks; a larger experiment remains
subject to the IsoLab budget/capacity gates.

No hypothesis is accepted from a timeout. A returned generator needs exact
ideal and sign checks. A provisional nonzero class vector requires
bnfcertify and narrow-class conditions before it can support nonprincipality.
The raw output and exit status are retained. The first requested step is
bnfinit(H,1); it may consume the whole bound before reaching the ideal test.
