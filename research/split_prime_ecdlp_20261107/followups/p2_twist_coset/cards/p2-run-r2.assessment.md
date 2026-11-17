# Assessment: CARD-p2-twist-coset-20261108-r2

```yaml
card_id: CARD-p2-twist-coset-20261108-r2
relay_verbatim: not_machine_checked   # executor text was not saved to a file; see reasons. Every value quoted in the dispatch matches the receipt and disk.
outcome: executed_as_specified
stopped_at_step: null
recommendation: proceed_to_analysis
reasons:
  - card sha256 6facc223... and receipt sha256 d504306c... equal the dispatch values; receipt status=completed, commands_total=4, commit 97c70ec24e81432c387c882ea58ba6424c248c3e.
  - receipt command order build, sha256-p2coset, sha256-msolve, driver-m3 equals the card's argv order; all exit 0, 1 attempt each, no retry, no timeout (driver-m3 668.6 s of 180000 s).
  - all 9 bound files re-hashed on disk equal the card's binds (PROTOCOL-R2, PROTOCOL, README, p2coset README/main.rs/Cargo.toml/Cargo.lock, driver.sh, /usr/bin/msolve c2722288...f759fa).
  - built binary sha256 1d038411...59f4 (sha256-p2coset.stdout and disk agree); build.stderr shows Finished release, stdout empty.
  - m3.jsonl sha256 12773291...1d7d31 equals the receipt output digest and the driver-m3 stdout line; driver-m3 stdout: calls=288 nonzero_exit_calls=0 cells=144 cells_not_emitted=0 msolve_nonzero=0 msolve_timeout=0; driver-m3.stderr is empty.
  - m3.jsonl has 432 lines = 144 emit + 144 parse (288 calls) + 144 msolve; every exit is 0; 144 distinct emit ids; every emit argv has --target unplanted (144/144), --cseed 0 (144/144), --tseed in {101,102,103,104} 36 each, and the id suffix -tsNNN equals the argv --tseed in every row.
  - per-cell seeds equal card coset_seed for all 8 (kind,p,t) combinations; 4 primes x (m2 t4, m2 t6, m3 t4) x arms (idx:0, idx:1, max) x 4 tseeds = 144, 4 cells each.
  - card text says "C = 10000 in every cell" but 12 cells (dickson-1049599-m2-t6, all arms, ts101..104) ran with --C 8199, hard-coded in the bound driver (driver.sh lines 135-146) and consistent with PROTOCOL.md line 25 (C = min(10,000, number of cosets)); 132 cells ran C=10000. The executor ran the driver as bound; this is a card-text inaccuracy, not an executor deviation. Reported, not interpreted.
  - write scope: the p2 card's new paths are the run dir (1017 files in receipt dirty_after) and the receipt itself. 130 other paths new in receipt dirty_after lie outside write_scope (p1_rr_dickson cards/runs/calibration run-r2a files, benchmarks/records/20261108T0538Z-split-prime-p1-r2a-armf-caps.json, p3_cross_trace/p3census files, problems/p3census-rho-walk-bimodal.md). They belong to concurrent p1-run-r2a/p3 work in the same checkout (p1-run-r2a run dir timestamp 053528Z, after the p2 start 05:29); the p2 driver writes only under its run dir (all redirects in driver.sh target $RUN/$STAGE or $J). Not attributed to the p2 executor; the receipt cannot prove the attribution.
  - dirty_before already contained benchmarks/records/20261108T0530Z-split-prime-p2-r2-unplanted.json (the BENCHREC harness line file); the executor did not run benchrec steps (none in argv).
  - post-run analysis dirs analysis/derived_r2 and analysis/p2r2analyze have mtimes 05:42-05:43, after the run; they are main-session work, not part of the card.
interpretation: not_assessed
```

## Caveat on relay_verbatim
`python3 tools/task_card.py check-relay` needs the executor's returned text saved to a file; none was provided. The summary in the dispatch was compared by hand with the receipt (exit 0, status completed, commit, build exit 0, p2coset digest, msolve digest, driver-m3 counts line, m3.jsonl digest) and every item matches. Run check-relay on the saved text to close this.
