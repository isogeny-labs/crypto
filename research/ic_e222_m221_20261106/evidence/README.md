# Evidence bundle: E-222 / M-221 analog ladder, defaults and H1 (draft for the validation gate)

**Status:** prepared by the producer session. **Not validated.** Nothing here is independent: the recompute and the replay were run by the session that produced the data, and the `independent` flags in the files written by `recompute.py` and `replay.py` are `false` on purpose. The gate (`docs/VALIDATION_GATE.md`) reads `independent: true` as written by the producer, and `docs/SABOTAGE_RESISTANCE.md` says a producer cannot supply the independence that word means. This bundle gives an independent party everything needed to do the replay and the recompute with code and a binary the producer did not control, and lists what is still missing. No leaderboard or `benchmarks/README.md` row is added by it.

## Claims (`claims.json`)

| id | claim | raw data | replayable here |
|---|---|---|---|
| C1 | New `solve-prime` defaults take 0.0015 of the old shipped defaults' total time, paired, 33 fresh instances, 12 to 23 bits; 128 runs, all recovered | `results/default_flip/paired/rows.jsonl` (git) | yes, trial counts deterministic |
| C2 | With the new defaults 720 runs on 240 fresh instances all recovered | `results/default_flip/robust/rows.jsonl` (artifact store, sha256 pinned) | yes |
| C3 | H1 with the new defaults: paired trials on/off 0.277 (cofactor 4), 0.153 (cofactor 8), exactly 1.000 (cofactor 1); 288 runs, all recovered | `results/h1_defaults/rows.jsonl` (git) | yes |
| C4 | Real E-222 and M-221: joint relation point costs 0.529 and 0.581 of the former path | `native/native_timing.txt` | no: wall-time re-measurement (see `claims.json`) |
| C5 | Section 5a band: on 72 instances the IC arm (default configuration, all counters emitted) costs more than the matched negation-map rho on the same instance at every weight w = 0, 0.1, 1 in all 72 (and in all 48 with H1); pooled median IC / rho 254 / 244 / 179; the distance grows with size | `band/` (120 per-instance outputs, hash manifest `band/MANIFEST.json`) | yes, counters and rho GAE are deterministic given the seed |

Scope of every claim: prime-order or small-cofactor toy instances (subgroup orders up to about 2^23), local unisolated host, wall times diagnostic, `trials_total` the operation count. The 660x in C1 is against the old untuned defaults of this tool. Cite as `[pcc:unverified]` until a pcc claim exists.

## Reproduce

```
# recompute the headline numbers from the raw data (stdlib only)
python3 -I research/ic_e222_m221_20261106/evidence/recompute.py \
    [--robust-rows <artifact-store copy of results/default_flip/robust/rows.jsonl>] --out recompute-output.json
# nonce-sampled replay with a binary built from the commit under review
cargo build --release --bin ic
python3 -I research/ic_e222_m221_20261106/evidence/replay.py --ic target/release/ic --k 30 [--nonce HEX] [--robust-rows ...]
```

`run_band_panel.py --ic target/release/ic --out DIR` regenerates the 120 band outputs; `band.py DIR/default/*.json` prices them at w = 0, 0.1, 1.

`replay.py` regenerates the three instance sets from their seeds and checks their manifest hashes (`claims.json`), draws rows with a nonce, reruns them with the recorded arguments and compares `trials_total`, relations collected and the recovered scalar. It samples the old-defaults arm from rungs up to 17 bits only (a 22-bit old-defaults solve takes about two minutes). It records the blake3 of the binary it ran and `git rev-parse HEAD`.

## Outputs of the producer's own runs

- `recompute-output.json`: all headline numbers recomputed within the stated tolerance. It found one display issue in `RESULTS_default_flip.md`: per-rung median trial counts are half-integers (rung 22: 2,354,250.5 and 75,224.5) that the table printed rounded; corrected there.
- `replay-log.json`: see the section at the end of this file.
- `gate-output.txt`: `python3 scripts/validation_gate.py evidence` on the draft manifest, which the gate refuses for the reasons listed below.

## What the gate still needs (`VALIDATION.draft.json` shows the shape)

| gate item | state | what is missing |
|---|---|---|
| replay certificate | draft `replay-log.json` with `independent: false` | an independent party reruns `replay.py` (or their own replay) with their own binary and writes the certificate; under the sabotage contract it carries a keyring signature (`pcc verdict`/`sign`) |
| validator report | draft `recompute-output.json` with `independent: false` | an independent recompute of the headline numbers from the raw data, with its own code |
| rho reference with the 0 / 0.1 / 1 GAE band | not complete | `solve-prime-file` does not emit its unpriced native counters (table probes, field operations outside group additions), so the IC arm cannot be priced at the band; the matched rho in `results/rho1/rho_matched.json` is for generated curves of the same sizes (negation walk, A = 2), not run in the same process on these instances |
| raw data in the repository | `results/default_flip/robust/rows.jsonl` is in the artifact store | fetch by sha256 (`.artifacts.json`) into the evidence directory before the gate reads it |
| pcc claim ids | none registered | `python3 tools/pcc/pcc.py claim new --kind ... --statement ... --proof ...` by the producer, verdict signed by a keyring holder; not run here because registering appends to the shared hash-chained ledger |
| process | not done | Conductor task, knowledge-base queries (`research-kb` has no binary here), `ecdb` lookup, resume checkpoint, benchmark record (`tools/benchrec.py`), EKS run (AWS credentials in this environment are rejected; no isolab image contains `ic`; `../validation_path/REPORT.md`) |

Pre-gate status: these results sit under `research/` only. If any is promoted to a row, it renders as "unvalidated (pre-gate)" at best and the builder refuses it as new. Do not add one before the items above exist.

## Replay result (producer-run, independent = false)

`replay.py --ic target/release/ic --k 30` with nonce `3f11131df8784337`, binary blake3 `1e8e07b434fc35810636e6486ec3241489b0ce66a91e2ae113b0a7e88c0f5f16` (the binary that produced the band panel, `band/binary.json`) at git head `b4506f65dc`: 120 rows drawn (30 each from the paired, robustness, H1-with-defaults and band files; the old-defaults arm only from rungs up to 17 bits), 120 reproduced exactly (`trials_total`, relations collected and the recovered scalar for the first three; every IC counter, the matched rho's mean GAE and its counters for the band), 0 mismatches; the three regenerated instance sets match their pinned manifests. Output: `replay-log.json`. This shows the recorded counts are reproducible by a rebuilt binary, not that anyone other than the producer ran it.

## What the gate says about the draft

`gate-output.txt` is the output of `python3 scripts/validation_gate.py evidence` on a copy of this directory (same directory depth, with `band/MANIFEST.json`) with `VALIDATION.draft.json` renamed `VALIDATION.json`. Exit status 1, for exactly these reasons: both pinned files say `independent` is not true (they are producer-run), and the robustness rows are in the artifact store, not the repository (`../results/default_flip/robust/rows.jsonl` is not a file; fetch by sha256 from its `.artifacts.json`). The rho reference (matched negation-map walk, same unit, same process, band at 0 / 0.1 / 1 GAE, order holding across the band) no longer appears among the refusals; its values are in `VALIDATION.draft.json` and the reading in `../RESULTS_band.md`.

## What the gate still needs

| gate item | state | what is missing |
|---|---|---|
| replay certificate | `replay-log.json`, `independent: false` | an independent party reruns `replay.py` (or their own replay) with their own binary and writes the certificate; under the sabotage contract it carries a keyring signature (`pcc verdict` / `sign`) |
| validator report | `recompute-output.json`, `independent: false` | an independent recompute of the headline numbers from the raw data, with its own code |
| rho reference with the 0 / 0.1 / 1 GAE band | done for the panel (C5) | the band is for toy sizes; limits in `../RESULTS_band.md` (rho's cost is a floor if it has unnamed unpriced work; IC group operations are derived from the scalars) |
| raw data in the repository | the robustness rows (`results/default_flip/robust/rows.jsonl`) are in the artifact store | fetch by sha256 (`.artifacts.json`) into the evidence directory before the gate reads them |
| pcc claim ids | none registered | `python3 tools/pcc/pcc.py claim new --kind ... --statement ... --proof ...` by the producer, verdict signed by a keyring holder; not run here because registering appends to the shared hash-chained ledger |
| process | not done | Conductor task, knowledge-base queries (`research-kb` has no binary here), `ecdb` lookup, resume checkpoint, benchmark record (`tools/benchrec.py`), EKS run (AWS credentials in this environment are rejected; no isolab image contains `ic`; `../validation_path/REPORT.md`) |

Pre-gate status: these results sit under `research/` only. If any is promoted to a row, it renders as "unvalidated (pre-gate)" at best and the builder refuses it as new. Do not add one before the items above exist.
