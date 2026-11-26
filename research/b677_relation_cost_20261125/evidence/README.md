# Evidence

- `panel-02/`: the reported panel. Ten interleaved replicates, each in the order
  baseline, exact, sample:13:<rep>, frozen rho, sample:26:<rep>, rho-batched <rep>;
  then rho-batched seeds 11..40. Each run: `<name>.json` (stdout), `.stderr`,
  `.meta.json` (argv, exit, process wall, peak RSS, binary SHA-256, SHA-256 of the
  original stdout). After the panel, `relation_rows` was replaced by
  `relation_rows_sha256`/`relation_rows_count` in the IC arms to save space, so
  `stdout_sha256` refers to the unreduced output. `tools/compare_exact.py` passed on
  all 10 unreduced baseline/exact pairs before reduction; `exact_identity.json`
  re-checks the stored hashes. `summary.json` comes from `tools/summarize.py`.
- `panel-01/summary.json`: an earlier six-replicate panel, built before the flat
  index replaced `HashMap<_, Vec<_>>`, without the equal-engineering rho. Same
  counted costs; wall was higher (s13 397 ms, exact 827 ms).
- `sources.sha256`: hashes of every source compiled into the measured binaries.

Driver used for panel-02:

```
B=native/target/release; D=evidence/panel-02
for rep in $(seq 1 10); do
  python3 tools/run.py $D/r$rep-baseline $B/baseline-mitm 40
  python3 tools/run.py $D/r$rep-exact $B/cost-v2 exact
  python3 tools/run.py $D/r$rep-s13 $B/cost-v2 sample:13:$rep
  python3 tools/run.py $D/r$rep-rho $B/baseline-rho
  python3 tools/run.py $D/r$rep-s26 $B/cost-v2 sample:26:$rep
  python3 tools/run.py $D/r$rep-rhob $B/rho-batched $rep
done
for sd in $(seq 11 40); do python3 tools/run.py $D/x$sd-rhob $B/rho-batched $sd; done
```
