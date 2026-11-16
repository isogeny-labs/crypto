# Amendment 1 (2026-10-19): the §5a pricing band, and the P8 rerun

## 1. The pricing band was missing from the protocol

`AGENTS.md` §5a (owner rule, 2026-10-15) requires every cross-method
comparison to do three things:

- list every arm's unpriced counters;
- price them at 0, 0.1 and 1 GAE per operation, the same way on every arm;
- rank two arms only where the order holds at every weight.

The rule predates `PROTOCOL.md` (2026-10-18), which overlooked it. This
amendment adds it.

When it was added, P5–P7 had been analysed, P8 had not finished, and no
result had been written.

**What changes.** For every root-finder/F4 and root-finder/F5 pair, the
analysis now also reports the following.

- **`S_w`** = (total GAE + w × every unpriced operation) / √r, for
  w ∈ {0, 0.1, 1}. Every arm's unpriced counters are listed and counted
  from its own record:
  - the tower arms' solver multiply-adds;
  - lookups;
  - square roots;
  - batched inversions;
  - Legendre symbols;
  - row operations;
  - target-guard probes;
  - rho's canonicalisations.
- **The §5a verdict:**
  - **"root finder ranks below (cheaper) at every weight"**;
  - **"root finder ranks above (dearer) at every weight"**;
  - otherwise **"unresolved inside the band"**.
- **A derived figure, the crossover weight**
  `w* = (G_root − G_g) / (U_g − U_root)`. This is the price per unpriced
  operation at which the two arms cost the same; above it the root finder
  is cheaper. It is reported, and it ranks nothing.

**What stays.** The preregistered questions Q1–Q5, their statistics and
their rules are reported as written. The band is added beside them.

## 2. P8 was rerun

A worker restart at about 01:20 UTC on 2026-10-19 killed the P8 session
after 20 of its 36 runs. `ecbench` cannot resume a session, so P8 was run
again in full from the same commit (`ebed0cc51`) and binary, into
`sessions/P8`.

The interrupted session is kept unedited in
`interrupted/P8-worker-restart-20261019/`. It enters no analysis.
