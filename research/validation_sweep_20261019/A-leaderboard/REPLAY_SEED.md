# Replay sampling seed (committed before any replay)

Slice A of the validation sweep, 2026-10-19. Audited tree: main at
`cb2ec6873f30b683f7885b53c410d69965f02c20`.

    seed = aef99bac5399d47e3a7242127ea75628

Rule (frozen here, before any replay or any candidate listing is read):

1. For each row, list its replayable runs (a run is one stored measured
   process or record that carries a recovered scalar and its public
   target) in the order the source file lists them.
2. Key each run `k = sha256(seed + "|" + row_id + "|" + run_id)` (hex).
3. Take the 3 runs with the smallest `k` (more where the row's card says so).
4. No run is dropped or replaced after the draw. A run that cannot be
   replayed is reported as a failed or not-covered replay, never swapped.
