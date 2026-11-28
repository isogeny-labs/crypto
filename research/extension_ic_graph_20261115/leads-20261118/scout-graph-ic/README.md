# scout-graph-ic: new leads in the graph-IC lane (2026-11-18)

All content is [pcc:unverified]. This lane reads files and writes notes
only. It did not build or run anything, used no paid compute, registered no
pcc claim and had no remote side effects.

## Question

Which leads in the graph-based extension-field IC lane has nobody queued?
For each, this note says what was already tried and gives the cheapest local
test that could falsify it. It ranks them by expected effect on the measured
bottleneck (descent) and on the distance to rho.

## Commands run

All commands were read-only.

- `ls`/`find` over the lane, and greps of the lane's `NEXT.md`,
  `AMENDMENT_*` and `*_REVIEW.md` files.
- `python3` one-liners that print fields of the recorded JSON:
  - Run50 `qualified-original.json`;
  - `deferred-graph50/recorded49-graph-replay.json` (anchor and vertex
    trajectory);
  - `complete-accounting/runs/recount-seed389-n3415.json`;
  - `descent-finder/runs/*summary.json` and the gzipped rows.
- `grep` of `src/cryptanalysis/zero_dim_la.rs` and `f4_fp.rs`.
- `scripts/research-kb status`, which reports no executable.
- `df -h /`: 14 GB free.

## Results

[`LEADS.md`](LEADS.md) ranks six leads and lists eight rejected ideas.

1. **L1, the strongest lead.** Treat the deferred graph's exact-log quadratic
   classes as a descent factor base. It counts three split patterns:
   `(1^4,2)` (15x), `(1^2,2^2)` (45x) and `(2^3)` (15x) relative to a full
   split. Estimated inputs from recorded data:
   - an exact-log fraction f of about 0.079 at Run50's stop, and 0.125 at
     108k sources;
   - a class universe of about 2.6e5, from a vertex-saturation fit.

   The model then predicts about 2.5x fewer descent residuals (3.6x at
   108k sources). The two-quadratic and three-quadratic terms are new.
   C2 and M2 cover only `(1^4,2)`.
2. **L2.** Collect past rank readiness to enlarge the exact-log set.
   Marginal anchors per source rise from 0.10 to 0.72 in the recorded
   trajectory, and the model is about break-even at Run50's stop.
3. **L3.** f scales roughly like 1/p, so L1 is strongest at small p, where
   IC is still behind rho. In the fresh-ladder model it moves the crossover
   down.
4. **L4.** C1 integration, already queued as M1.
5. **L5.** Triage of C4. The counted linear stage is 94% `mac`s, so replaying
   the normal-form pattern can save at most about 5% of the linear stage.
   Measure the per-kernel split first.
6. **L6.** Collection becomes the bottleneck after L1, L2 and L4 land.
   Pointer only.

The Run48 discriminant-character screen **was done in Run49**: it saved
3.18% of the unscreened pair subtotal, and Run50 runs on top of it.

## Failures and limits

- research-kb unavailable: no MCP tools are loaded and the CLI wrapper has
  no executable. Primary papers were not re-read; the note relies on the
  lane's recorded reads.
- Every multiplier is a heuristic model:
  - the pattern densities are cycle-type densities of random polynomials;
  - U comes from a uniform-draw saturation fit;
  - "anchors" is taken to equal the number of exact-log classes.

  None was measured here. Run50's LOCAL witnesses are absent, so
  |D_exact| at kernel time is not known exactly.
- The deferred-graph trajectory is a replay of Run49 sources, not Run50's
  own collection.

## Next steps (cheapest first, all local)

1. L1 step 1: add a factorization-pattern tally to
   `descent-finder/traced_descent_check.rs`, in the mini-build (about 30 s).
2. L1 step 2: an exact count of the class universe for `p = 1009`, in Rust
   (seconds).
3. L5: per-kernel `mac` tallies in the complete-accounting patch (13 s).
4. L1 step 3 / M2: a local deferred collection that exports the exact-log
   classes and the match count. It needs a scoped build of the deferred
   pipeline.
5. L3: the same on a dry-run p=127 target, as a control only.
