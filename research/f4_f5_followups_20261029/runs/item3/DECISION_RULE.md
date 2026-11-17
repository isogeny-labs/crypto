# Item 3 confirmation rule (written 2026-10-29 before the confirmation run)

Exploratory run (5 rounds, this host) suggested `KIC_F5_UNPACK_DIRECT=1` helps the
SelectiveEchelon form (`KIC_F5_ECHELON=2`) on a one-thread pool (7/7 cases faster in
wall time, 1.01–1.15x) and is mixed elsewhere.

Confirmation: a fresh run, 7 rounds, arms alternating first, form 2, RAYON_NUM_THREADS=1,
`f4_f2_bench 3 24 f5 0`. Gate direct unpack on for that setting only if
(a) rows, row space, terms, rank and word counts agree in every arm and round, and
(b) median wall time off/on >= 1.00 on at least 6 of 7 cases, and
(c) the geometric mean of off/on over the cases is >= 1.02.
Otherwise nothing changes.

## Amendment 1 (after the form-2 confirmation, before any form-0 confirmation data)

Form 2 with degree < 4 or n < 20 runs the same code as form 0, so the form-2
result says the effect is a one-thread one. Same run, same criteria, for
`KIC_F5_ECHELON=0` at RAYON_NUM_THREADS=1: if it passes too, the gate is "one-thread
pool, any output form"; if not, the gate stays "form 2, one-thread pool".
