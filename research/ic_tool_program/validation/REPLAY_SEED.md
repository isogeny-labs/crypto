# Replay seed for the validation gate (PR #1978)

Committed before any native replay ran, so the sample cannot be chosen after
seeing results.

- **Seed:** `77767620d255d08ba58c8cadde0823c4`
- **Draw:** in each evidence directory, every timed record whose isolation
  record is not contended is ranked by `sha256("<seed>|<evidence dir>|<record path>")`;
  the first **4** are replayed. No draw is dropped or swapped; a draw that
  cannot run is reported as a failure.
- **Replay:** the exact command in the record's `.isolation.jsonl`, run with
  the binary of the same name, its SHA-256 checked against the round's record,
  on this host, output to scratch. Compared: both arms' certificates (curve,
  generator, target, scalar), `counts`, `rho_counts`, and the run's
  verification flags. Timings are not compared.
- **Evidence directories:** rounds R06, R08, R10, R11, R12; rule v6, v7;
  explorations A14, A15, A16, A17, A18, A19, scan-stages-v8.
