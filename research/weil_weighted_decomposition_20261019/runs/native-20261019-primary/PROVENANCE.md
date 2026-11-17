# Primary and replay provenance

- Source revision: fc13c52e82202b44a13f9e32dca2fac2401e320e.
- Protocol revision: 6095144c (committed before measurements).
- Source snapshot: ../../sources/primary.rs.
- Exact argv and budgets: config.json. Replay uses the same argv except --out.
- Both native processes completed with exit status 0. Primary stdout was
  displayed in the interactive execution log; records.jsonl is the durable
  per-case evidence. Replay stdout is archived separately.
- The output records compare byte-identical with ../native-20261019-replay.
- Host observations were captured after the run on the same container.
- Isolation: L0, concurrent verification/build activity; no wall-clock claim.
- The initial measured binary hash was not captured before rebuilding. The
  committed source snapshot, Cargo.lock and repository revision remain the
  rebuild provenance. Do not substitute a later binary hash for that binary.
- Analysis/report generation uses later source revision 040b8e1a; it changes
  metadata export, documentation, tests and an invalid-scalar failure guard,
  without changing any successful measured record. Raw primary records are
  retained. Independent PARI 2.17.2 checks are in pari_checks.jsonl and pari.log.
