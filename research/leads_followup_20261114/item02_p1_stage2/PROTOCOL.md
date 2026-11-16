# Item 2: P1 stage 2, direct D = 6 closure at k = 15 (protocol, written before any data; no data taken)

Status of every statement: [pcc:unverified]. Written 2026-11-15. No pod was created (see RESULTS.md).

Cell: one closure, instance `ml-cryptanalysis/experiments/ic-bounds-k14/instances/unit-p65537-a1-s10-k15.json`
(p = 65537, y^2 = x^3 + x, seed 10, unit instance, points = 0, columns 768,212; sha256
0629858740bc2d5a2880b4efa64f8d65597999294b294bd75ab584047b3f8f29), D = 6, `--gens all`, full sweep (no `--no-repair`
unless the repair path is the thing under test), `--stop-on-one` NOT set (the full rank is wanted).

Prediction (derived, research/leads_chase_20261112/RESULTS.md P1): q = 167,665 = 768,212 - dim V_6(15),
so dim V_6(15) = 600,547 and 1 not in V_6(15) (contains_one false). Out-of-sample band: q/2^k in [2.5, 6.0]
at j <= 3 only; at D = 6 the only anchor is 3.067 (k = 11), so q/2^15 = 5.117 is an extrapolation.
Per-part split to be reported: two parts of 384,394 and 383,818 columns.

Falsifiers: (F1) rank differs from 600,547 by any amount: the quotient-law extrapolation is wrong at D = 6, k = 15
(report the distance, not pass/fail). (F2) contains_one true: the unit-instance non-membership claim fails at k = 15.
(F3) `closed` false or sweep not clean: no bound, the rank is only a lower bound. Pass requires status done,
closed true, clean sweep, rank = 600,547, contains_one false, and the independent checker (ic_bounds_cert_check.py)
on the kept basis agreeing.

Instance rules: fixed above; no other seed, prime or target is drawn. Control: none on this host (the k = 14 pair
of RESULTS.md in ic-bounds-k14 is the committed positive control for the engine and checker).

Resources: GPU pod, lifetime cap 9 h, launch card in LAUNCH_CARD.md. Checkpoints every 30 min on a volume.
