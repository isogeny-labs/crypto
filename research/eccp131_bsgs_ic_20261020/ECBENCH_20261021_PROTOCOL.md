# ECCp-131 exact-probe stage benchmark and ecbench control

**Frozen before the 2026-10-21 measurements.** This measures the code added for the exact ECCp-131 two-sum relation probe. It also runs a separate native `ecbench` IC/rho control on a registered 20-bit prime curve. The control uses `ecbench`'s `u64` prime-field implementation, while the exact probe uses the 131-bit `MontField<3>` implementation. Their costs must not be divided into a cross-curve ratio.

## Question and boundaries

- Exact curve and published target: the Certicom ECCp-131 parameters in `CurveParams::eccp131()` and `ic`'s named target profile; subgroup order `n`, cofactor 1.
- Exact-stage question: how do pair-table construction and a fixed 128 setup plus 128 target candidate scan change with small-x base size `B`? Report cold process time, peak RSS, internal base, pair-table, controls and probe intervals, natural relation count, independent rank, target status, and every failure. These are bounded-stage measurements, not complete ECDLP costs.
- Native ecbench control: the registered `prime_search(bits=20, seed=59297)` fixture, two planted targets, `rho.negation`, `ic.pipeline` with `prime-abscissa:size=32` and `mitm:negation_folded=1`, plus an identical rho A/A arm. Report native operations and correctness on that small curve only. Its matched reference is `rho.negation`; its generic floor is the harness's `sqrt(pi/(2A))` with `A=2`.
- No speedup, scaling, ECCp-131 rho, or full attack-cost claim is admissible from this session. macOS has no CPU affinity; wall measurements are descriptive L0 resource observations. Every derived ratio must retain its exact arm, curve, accounting window and sample count.

## Frozen exact-stage matrix

- Source: this PR head after the exact probe implementation; record the commit, binary SHA-256, `Cargo.lock` SHA-256, host, tool versions and pre-run load.
- Arguments: `ic prime --curve eccp-131 --size B --trials 0 --controls 8 --setup-candidates 128 --target-candidates 128 --seed 131 --json --out FILE` with `RAYON_NUM_THREADS=1`.
- Arms: `B256`, `B512a`, `B512b` (the same arguments as `B512a`, for an A/A drift control), and `B1024`. Five rounds, in this fixed order: `256,512a,1024,512b`; `512b,1024,512a,256`; `256,512b,1024,512a`; `512a,1024,512b,256`; `256,512a,1024,512b`. Each invocation is a new process and output file. No process is warmed in advance; the `ic` command itself constructs the base and pair table twice, so its whole-process cost includes that duplication.
- Stop: a 600-second wall watchdog per invocation, one Rayon thread, no adaptive extra candidates or rounds, and stop the sequence after any invalid curve/point, failed planted control, or nonzero exit. Keep all attempted raw files. The B1024 pair table is capped by the command's finite `B²` enumeration; monitor resident memory and stop the sequence if available host space or memory becomes unsafe.
- Exact run artifacts: `bench_20261021/exact/raw/<sequence>-<arm>.json`, `.stdout`, `.stderr`, `.exit`, plus a manifest of revisions, hashes, host and command. The driver is `bench_20261021/run_exact.sh` and refuses to overwrite a raw result.

## Frozen ecbench control

- Spec: [`bench_20261021/ecbench-prime20-spec.json`](bench_20261021/ecbench-prime20-spec.json), which fixes two one-target workloads, three measured rounds, one warmup, alternate order and a 120-second child timeout. Operation counts are the primary data; any L0 wall time is descriptive.
- Before run: build this branch's release `ecbench`, run `ecbench plan --spec ... --json` and retain its output. Refuse the run if the curve is unregistered or the plan changes method identities unexpectedly.
- Run `ecbench run --cpus none --allow-busy --spec SPEC --out bench_20261021/ecbench-session`; output must not exist. Then `ecbench verify --dir SESSION --replay-all --out bench_20261021/ecbench-audit.json`, `compare --a rho-neg --b ic-mitm --save`, A/A `compare --a rho-neg --b rho-neg-aa --save`, and `table --dir SESSION`; keep raw streams and exit codes. An invalid or failed session is retained, not excluded.

## Reporting

The write-once run report lives under `reports/run-<UTC start>/` with a source-linked graph or diagram and inspected PDF. The study result report names the exact-stage and ecbench control separately. The existing ECCp-131 problem record remains open until natural independent rank, target descent and `[x]G=Q` are demonstrated with complete cold cost. A `benchmarks/README.md` performance row would require the repository's independent validation artifacts and matched reference; this protocol does not preclaim that gate.
