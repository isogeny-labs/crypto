# Amendment A1: relation-target stream without cycles

Frozen 2026-11-26, after the frozen run (`data/runs.jsonl`) and before any
A1 data. Machine-readable form: [`protocol_a1.json`](protocol_a1.json).

## What the frozen run showed

The frozen relation-target stream was an 8-lane r-adding walk
`R ← R + J_{h(x(R))}` with 16 jumps. Like a rho walk, it enters a cycle after
about `√(πℓ/2)` steps per lane.

The arms consume different numbers of targets:

- M2 consumes about `n′`-squared targets (`N^{2/3}`), which is more than the
  cycle length. Every M2 case repeated targets, and 4 of 64 M2 solves failed:
  - in two cases the rows stalled (18/20 and 35/38 rows over columns);
  - in the other two, duplicate rows gave only `Σc·b = 0` dependencies across
    147 and 539 LA calls.
- F, G and M3 consume about `3·2^{n′}` targets, but the same defect still hit
  some of them: 3/12 cases at n = 18 and 1/12 at n = 21 had repeated targets.

The check is `data/dupcheck_frozen_stream.jsonl`, which regenerates the frozen
stream. Every frozen record, the failures included, is kept in
`data/runs.jsonl`, and REPORT.md reports them.

## Change

Every IC arm (F, F0, G, M3, M2) is rerun on all frozen cases, with the
relation-target stream replaced as follows:

- 8 lanes, each with an independent random start `[a_k]G + [b_k]Q`;
- one common random stride `S = [a_s]G + [b_s]Q`;
- `R ← R + S`, batched across lanes;
- stream key `icstream-a1|n|ci|ti`.

A lane does not repeat before ℓ steps. Lanes could overlap, but with random
starts the chance is negligible; `dupcheck` is run on the A1 stream as well.

Everything else is unchanged:

- arms, factor base, stopping rule, LA, verification;
- curves, targets `Q` and seeds;
- analysis and hypotheses.

The rho records of the frozen run are reused as the matched reference, since
they use the same `Q` and the same rho streams. The A1 records
(`data/runs_a1.jsonl`) are the primary IC data. The frozen IC records are
reported beside them as `*_frozen`, and the conclusions are checked against
both.
