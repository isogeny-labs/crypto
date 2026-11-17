# `batch_trials` development screen on `ic.koblitz_batched`

Local, diagnostic, counted-unit screen (isolation L0). Not a registered
tournament round, not promotable. Read [REPORT.pdf](REPORT.pdf) first.

| | |
|:--|:--|
| Question | How does the counted cost (GAE) of the tournament worker's pair-table pipeline move with `batch_trials` on round 0026's four development cells? |
| Protocol | [PROTOCOL.md](PROTOCOL.md), frozen in `d98cc0d107`, Amendment 1 before any data in `60a451606b` |
| Instrument | `ic.koblitz_batched` (PR #3134, source `729847dcea`) |
| Result | counted cost falls monotonically as the batch shrinks, to 1; `batch16` is 0.69–0.80× the incumbent `batch64`; 2 of 5 frozen predictions held (P2, P5), 3 did not (P1, P3, P4) |
| Execution | [cards/card-1.json](cards/card-1.json), receipt and [assessment](cards/card-1.assessment.md): `executed_as_specified` |
| Verification | `ecbench verify --replay-all` on all four sessions: 1600 records verified, 1280 replays reproduced, 0 problems ([audits/](audits/)); A/A control 1.000 |

## Layout

- `specs/`, `make_specs.sh`, `run.sh`: the frozen panel and its run script.
- `cards/`: the task card, its helper gates, the receipt, each command's output and the assessment.
- `audits/`, `tables/`, `comparisons/`: the replay audits, one-unit tables, and each arm's paired comparison against `ic-b64` (text and saved JSON).
- `figures/`: `make_summary.py` (records → `summary.json`, predictions scored by code), `plot_figures.py` (SVGs), `make_report.py` (→ `REPORT.typ`).
- `REPORT.typ`, `REPORT.pdf`: the report.

## Where the records are

`sessions/<cell>/records.jsonl` (about 17.7 MB each, 70.8 MB in all) are in the
artifact store, not in Git: each cell directory carries an `.artifacts.json`
manifest (sha256 and size) and the managed `.gitignore`. Hydrate them with

    python3 tools/artifacts/artifacts.py pull research/ecbench_batched_screen_20261101/sessions

`figures/make_summary.py` reads them; run on the stored records it reproduces
the committed `figures/summary.json` byte for byte.
