# Evidence bundle: E-222 / M-221 analog ladder, defaults and H1 (draft for the validation gate)

**Status:** prepared by the producer session. **Not validated.** Nothing here is independent: the recompute and the replay were run by the session that produced the data, and the `independent` flags in the files written by `recompute.py` and `replay.py` are `false` on purpose. The gate (`docs/VALIDATION_GATE.md`) reads `independent: true` as written by the producer, and `docs/SABOTAGE_RESISTANCE.md` says a producer cannot supply the independence that word means. This bundle gives an independent party everything needed to do the replay and the recompute with code and a binary the producer did not control, and lists what is still missing. No leaderboard or `benchmarks/README.md` row is added by it.

## Claims (`claims.json`)

| id | claim | raw data | replayable here |
|---|---|---|---|
| C1 | New `solve-prime` defaults take 0.0015 of the old shipped defaults' total time, paired, 33 fresh instances, 12 to 23 bits; 128 runs, all recovered | `results/default_flip/paired/rows.jsonl` (git) | yes, trial counts deterministic |
| C2 | With the new defaults 720 runs on 240 fresh instances all recovered | `results/default_flip/robust/rows.jsonl` (artifact store, sha256 pinned) | yes |
| C3 | H1 with the new defaults: paired trials on/off 0.277 (cofactor 4), 0.153 (cofactor 8), exactly 1.000 (cofactor 1); 288 runs, all recovered | `results/h1_defaults/rows.jsonl` (git) | yes |
| C4 | Real E-222 and M-221: joint relation point costs 0.529 and 0.581 of the former path | `native/native_timing.txt` | no: wall-time re-measurement (see `claims.json`) |

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
