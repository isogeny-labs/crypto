# Independent genus-4 audit, 2026-11-16

This is an audit of existing public synthetic evidence, not a new performance
experiment. No paid resources are authorized or launched. Local checks are
explicitly requested. Counts are the observable; shared-host wall time is not a
speed result. All new outputs remain unverified assertions until the repository's
independent signed claim gate accepts them.

## Frozen inputs and scope

- Original record and protocol at `1a3f5a3d464b60bb0d3ce69459d3b47e9118d06f`.
- Original source commit `2ca7ece552ad276908db4ec211c2ab452283dd58`.
- Original raw bytes recovered from historical commit
  `6df35c0a4bf537cce0674f25b3279a242a5ef498`, SHA-256
  `e0f7050d3401da7db046a91bf1b0fb872dcf86023bd2c5c1caddd6c7a2efa32f`.
- Later follow-up and validator evidence at
  `2193ace0fdfaebc24e7366a8b95c3408af04674f`, retained as a distinct study.

Recalculate every recorded row, distinguish 3 targets from 12 solver runs,
retain failed rows, examine additions and rho native work, compare c/v/w IC
arms, and assess external IC controls and transfer assumptions. The acceptance
criterion is a supported verdict at the measured scope; no ECDLP conclusion is
inferred from a Jacobian DLP result.

## Native checks

`native/stage.sh` assembles unchanged Rust source slices from the original
source commit to avoid compiling unrelated library modules. The complete
genus-ladder production module before its tests is included unchanged. The
shared relation graph, peeling, both Wiedemann solvers and rho walk are the
original routines. No cryptographic algorithm is ported to Python.

1. Rust recalc: all rows, ratio of sums, additions sensitivities, per-run losses,
   and an explicitly distinct 100,000-resample paired cluster bootstrap using
   StdRng seed 20261116. Original study clusters on curves; follow-up clusters
   on curve/target, with solver seeds sampled within each. Original producer
   intervals are retained separately: its code resamples numerator and
   denominator seeds independently, despite describing the comparison as paired.
2. Replay: original c/v/w/R rows, three local threads, 20-minute wall ceiling,
   50-million trial cap, same seeds, factor-base size, restarts and target.
   Every shared emitted counter must match; wall time excluded. Independently
   retain recovered scalar, G, T and f. Any mismatch or missing row blocks a
   full replay claim. No result from an unfinished check is silently excluded.
3. Separate relation-check replay for w: enable the original relation-check
   option. Compare deterministic work counters except the intentionally
   zeroed Cantor tallies, require zero bad relations. Charge this verifier
   separately; the frozen experiment ran with this check disabled.
4. Independently certify available dumped and replayed scalars using new Rust
   dynamic-polynomial Cantor arithmetic with general extended gcd, a
   right-to-left scalar ladder, validity and squarefreeness checks. This
   arithmetic shares no implementation with the producer. It does not by
   itself verify a full relation transcript or all matrix operations.

## Resource and evidence boundary

At most three replay threads on the existing local host; no additional cloud
capacity. Whole-process peak RSS is descriptive and not a per-stage memory
profile. Preserve sources, lockfile, hashes, commands, outputs, errors and scope
limits in this directory. Raw original output is referenced by its historical
Git URL, not copied over the frozen file. No leaderboard or benchmark promotion.

KB unavailable: no loaded KB tool and `scripts/research-kb status` exits 127
(executable unavailable). Conductor check exits 127; `docs/CONDUCTOR.md` permits
warning and continuation without its client. Existing checkout is unchanged
outside this audit directory. Direct Git transport lacks credentials; repository
evidence is read through the connected GitHub API.

Sources checked: original protocols, analysis code, raw rows, Rust source;
genus-4 red-team review; later genus-4 follow-up and validator B13--B18.
Primary literature: Diem, arXiv:math/0606607 (double-large-prime complexity and
storage assumptions); Tian, arXiv:2012.07173 (cubic extension-field transfers,
not a prime-field construction). The GTTD paper ePrint 2004/153 was located;
direct full-text retrieval from IACR/AMS was denied, so Diem's primary text
provides the accessible algorithmic comparison. No novelty claim.

## Follow-up check extension

After inspecting the existing B13 evidence, the audit also replays the
follow-up's 18 plain-rho cases with the original Rust plain walk's
`cycle_window=0`. One case has a 300-second bound; the other 17 have a
900-second shared bound with three workers. Same instances, targets, seeds,
jump setup and step caps. Accept only equal retained group/field/step counts,
equal recovered scalars, zero window compares and no failed scalar check.
This extension was recorded here after launch; it is reproduction of a
specified counterexample control, not a claim of prospective preregistration.
Any incomplete cases remain incomplete. Source counts, not wall time, decide
the comparison. The overlap/resource deviation is disclosed in out/CHECKS.md.

The 900-second attempt exited 124 with 16/17 rows complete and matching;
the separate first case makes 17/18 complete. Its last case is
p8209/cs393/target1000396/seed1, whose registered plain walk takes
198,717,560 steps. Retain the censored attempt and retry only this case with
one worker, no overlapping audit jobs and a new 480-second ceiling. Same
inputs, counters, scalar checks and acceptance criteria. This is an explicit
resource-envelope amendment after the observed timeout, not a rewritten run.
