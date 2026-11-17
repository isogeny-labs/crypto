# Equal-advice preprocessing rho vs compact-orbit IC, wide rungs (2026-10-18)

**Status: instrument built and checked; the n = 73 / n = 83 session is frozen and NOT run.**
Owner request 2026-10-18 after `research/ic_leads_review_20261018/README.md` (section 2.1 item 1, section 3.2).

- [PROTOCOL.md](PROTOCOL.md): frozen question, rungs, advice budgets and their sources, ABBA session in one process group and envelope, controls, decision rule, memory and time requirement, the exact command.
- [run.sh](run.sh): thin orchestration (`run.sh n73 n83`); `run.sh n41smoke` is a harness smoke test.
- Code: `examples/ic_equal_advice_wide.rs` (wide BL arm, `targets`/`bl`/`selftest`), `examples/ic_equal_advice_claims.rs` (replay certificates, `vs_rho` claims, native claim check, statistics, decision).
- Instrument check at n = 61 (narrow vs wide, frozen item-9 fixture, same seeds: identical counters on 24/24 rows): [reports/run-20261018T0620Z/report.pdf](reports/run-20261018T0620Z/report.pdf), data in `instrument_check_n61/`; harness smoke in `smoke_n41/`.
- Related and already on main: `research/20261016-n83-precomputed-rho-control/` (PR #1987, time- and RSS-matched BL at N83); this protocol adds equal bytes, n = 73, one process group, and claim-shaped output.

What this does not say: nothing about n = 73 or n = 83 outcomes; no wide step cost on an idle host; claim-check will report `FAIL` on independent validation, isolation level and (until replayed on a second host) `independent_replay_pointer`. Classed accounting/audit, not an improvement to the IC baseline.
