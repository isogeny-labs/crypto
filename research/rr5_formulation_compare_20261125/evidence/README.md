# Evidence

- `ladder-01/`: scales ×1, ×4, ×16, ×64 and ×256 of the frozen #4974 caps, 300 s deadline,
  12 GiB address cap. Targets planted4, planted2, query1, query2 and h0. Arms coupled,
  sectors, sector4, graeffe and deflated (planted2: coupled, sectors, sector2). Driver
  `tools/ladder.sh`.
- `ladder-02/`: ×1024 and ×4096 on planted4 and query1, 900 s deadline (`tools/ladder2.sh`).
  Also ×16384 on planted4 for sector4, coupled and graeffe, 1200 s deadline, 13 GiB cap:
  `python3 tools/run.py evidence/ladder-02/s16384-planted4-<arm> $X --arm <arm> --target planted4 --scale 16384 --seconds 1200`.
- Each run has `<name>.json` (stdout), `.stderr` and `.meta.json` (argv, exit status,
  process wall time, peak RSS, binary SHA-256, stdout SHA-256).
- `summary.json` in each ladder comes from `tools/summarize.py`.
- Every run exited 0 with an incomplete status (ResourceLimit or Deadline). No relation
  was extracted.
