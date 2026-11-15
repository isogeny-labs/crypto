# Out-of-sample test of the solving-degree rule (W6-TH-2), frozen before data (2026-11-14)

Question. Does the fitted rule SD = e_max + (m - 1) + ceil((m - 1)(s + m - 2)/4), with s the sum of
(e_j - 1) over the prime factors of d other than the largest (with multiplicity), predict the F4 solving
degree of the symmetrised subgroup-base decomposition system on cells that were not in its fit set?
The rule and its competitor (wave 5: SD = e_max + c with c in [2, 3] at m = 2) are stated in
`lanes/theory.md` (W6-TH-2) and `../curve25519_leads_wave5_20261112/lanes/synthesis.md` (W5-SYN-3).
In-sample both are checked in `checks/sd_rule_check.out.txt` (66 of 66 cells for the W6 rule).

Cells. k = 22 (p = 4194301, A smallest valid, the lab's family rule), m = 2, form sym, gidx 0, the
frozen binary `/home/user/bin-archive/curve25519_family-dc2c30a4` (sha256 dc2c30a4...3faf, the exponent
studies' binary), `f4sub 22 sub D 0 mont 2 sym N SEED CAP` with N = 2 targets, SEED = 20261108 + 2000 + D
(the parent's seed rule), CAP = 1800 s per target, `ulimit -v 12000000`, `RAYON_NUM_THREADS=1`, one process
at a time, order D = 341, 825, 1271.

| D | factors | e_max | s | W6 rule SD | W5 band SD |
|--:|:--|--:|--:|--:|--:|
| 341 | 11 x 31 | 31 | 10 | 35 | 33 to 34 |
| 825 | 3 x 5 x 5 x 11 | 11 | 10 | 15 | 13 to 14 |
| 1271 | 31 x 41 | 41 | 30 | 50 | 43 to 44 |

Predictions. P1: every completed cell's recorded `solving_degree_max` equals the W6 rule's value (both
targets). P2: at least one completed cell falls outside the W5 band. P3: all three cells complete under the
caps (the theory lane's estimate; the main session expects D = 1271 may censor on time or memory, which
would leave P1 decided on the cells that complete).

Decision. A completed cell with SD different from the W6 rule's value refutes the rule as stated; a cell
inside the W5 band and outside the W6 rule refutes W6 in favour of W5. A censored cell decides nothing.
Every number is [pcc:unverified]; no claim is registered. SD is the only claimed quantity; field_ops and
wall time are recorded, not claimed.
