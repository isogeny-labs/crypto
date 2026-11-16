---
id: LEAD-ICP-coppersmith-presentation-audit
type: lead
title: Certified-reach audit across Semaev presentations (Idea 1)
tags: [prime-field, ecdlp, coppersmith, lattice, semaev, small-roots, audit]
confidence: measured
status: open
source_refs: [LEAD-ICP-coppersmith-smallroot-semaev]
added: 2026-11-04
superseded_by: null
next_action: run cuso + flatter on the six presentations when the stack is available; until then use the measured x-shift baseline below
---

## Statement
For each presentation of the 2-decomposition system, compute the reach of the optimal shift set
and confirm it by planted recovery. Presentations: (i) S_3 in x-coordinates; (ii) Vieta
(e1,e2); (iii) Riemann-Roch norm system; (iv) chained S_3; (v) 2-torsion symmetrisation;
(vi) integer lift with carry.

## What this audit adds (2026-11-04)
The finite-m Jochemsz-May x-shift lattice for S_3 was built and measured locally
(`experiments/idea1_lattice.py`, `experiments/idea1_lattice.json`). Reach proxy: an LLL vector h
with `|h(x0,y0)| < p^m` (i.e. h = 0 over Z at the planted root).

* `j = 0` and generic `j` have the **same 9-monomial support** in x-coordinates, hence the same
  Newton polytope and the same lattice reach. Idea 6's predicted "1/6 -> 1/5 at m = 2 for j = 0"
  cannot hold in this presentation; the specialness is invisible to the x-shift lattice.
* Measured reach (m = 2, 6 trials/cell, root extracted, not just a vanishing polynomial): the
  diamond family i+j <= 2(m-k) reaches delta ~ 0.13-0.14 at p = 2^40 and 2^56 (6/6 at 0.10-0.12,
  4/6 at 0.14), the same band as Probe B/C; the sparse family i <= 2(m-k) stays at 0-1/6.
  The certified question (does any presentation exceed 1/6?) still requires cuso.
* Harness validated by a univariate control at the theoretical 1/2 bound; false positives
  (multiples of f) and mod-p resultants are filtered.

## Next action (executable)
1. Install `cuso` (Sage) and `flatter`; run the three bound calculators on the six presentations.
2. Plant at 64/96/128 bits, delta = certified +/- 0.01, >= 20 trials, recovery = planted root.
3. Report dimension, `n^3 L / 64` charged word-ops, and GAE-weighted totals.

## Kill
All certified bounds within 0.005 of the single-polynomial values (1/6, 1/24, 1/80): then the
"presentation-invariance" hypothesis is proved for the automated-shift class and the lead closes.
