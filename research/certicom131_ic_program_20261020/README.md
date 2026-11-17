# Certicom 131-bit index-calculus program (2026-10-20)

Program text: [`docs/ic/CERTICOM131_IC_PROGRAM.md`](../../docs/ic/CERTICOM131_IC_PROGRAM.md).
Report: [`REPORT.pdf`](REPORT.pdf) (source `REPORT.typ`). **Class: measurement /
accounting. No sub-rho claim, no key-recovery claim.**

| path | content |
|:--|:--|
| `gates.md`, `gates.json` | `examples/certicom131_ic_gates.rs` output: cost model, floors, gates `D*`/`δ`, semi-regular reference |
| `runs/c*.jsonl`, `runs/c*.txt` | Falsifier A (`examples/certicom131_fppr_dreg.rs`, seed 20261020): one JSON line per measured draw |
| `runs/fb_*.jsonl`, `runs/fb_*.txt` | Falsifier B (`examples/certicom131_prime_combined_count.rs`, seed 20261020) |
| `tables/` | `examples/certicom131_report.rs` aggregation (`dreg_summary.md`, `fb_pooled.md`, `fb_cells.md`) |
| `figures/` | SVG (source of record) and 150 dpi PNG of the three graphs |
| `pkm_ext/` | prime-kummer ecbench row at 30 bits: curve, spec, sealed session `p30`, audit, tables |

Regenerate tables and figures from the records:
`cargo run --release --example certicom131_report -- research/certicom131_ic_program_20261020`.

## Not run (recorded, not hidden)

* Falsifier A cells `(13,3)`, `(13,4)`, `(17,5)`, `(19,6)`: launched, no draw
  finished in about 3 h of wall time at host load 90–170; no data, no files kept.
* 32-bit prime-kummer row: the ladder's exhaustive point counter had not
  finished a 32-bit curve after about 2 h; no spec exists.
* Curve registry: `icv1-fp30-t49927-d70f5de2` (the 30-bit curve) and ECCp-131
  (`icv1-fp131-t29529377007934231835-9681b699`, computed with
  `scripts/curve_id.py prime`) are **not yet in `docs/curves/registry.json`**
  (`ecbench plan` reports `registered=false`). Adding them changes the
  registry digest that `docs/curves/covers.json`, `cover-links.yaml` and the
  lab browser pin, and the cover regeneration (`curve_cover_check`, about 20
  minutes) did not produce an output in this session (the disk filled up
  during it), so the registry was left unchanged rather than half-updated.
  Follow-up: register both, regenerate covers, lab browser and leaderboard in
  one change.
* No `benchmarks/` entry: the ecbench rows are operation counts (L0), not a
  speed measurement (AGENTS.md "Performance results also land in `benchmarks/`").

## Host

Shared macOS host (Darwin 25.6, 14 cores), load average 90–170 from other
sessions during the runs; no isolation (`ecbench run --cpus none --allow-busy`,
level L0), so only operation counts are reported. rustc 1.93.1.
