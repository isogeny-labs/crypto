# Item 01 replay plan (validation gate, replay certificate)

Written 2026-10-22 by the experiment-director lane. Cards: `card-r01.yaml` to
`card-r05.yaml` in this directory. Nothing here has been run. Receipts, outputs
and stderr land under `replay/` (`receipt-card-rNN.json`, `raw/`); the recorded
originals under `../raw/` are not touched.

## What is replayed

The instrument is `research/blindspots_20261018/instruments/blindspots_01_fp2_gaudry.rs`
(sha256 `0629a73c7ff443e70c6ce16c0872b98a5aebc5a71d25f7bff60156c8693ff559`, the
digest every original card bound under the old path `examples/blindspots_01_fp2_gaudry.rs`).
Each card builds it with the original build argv (`prlimit --as=4294967296 rustc
--edition 2021 -C opt-level=3 -C codegen-units=1`), into `replay/bin-rNN/b01`,
and removes the binary and its directory at the end. Every `run` argv repeats the
original one (same `--family --bits --idx --arms --inv-cost --crosscheck
--control-trials`) with only `--out` changed to `replay/raw/<name>.jsonl`. The
instrument seeds everything from `(family, bits, idx, target)` (`Rng::new`,
`inst.set_target`, rho seed `0x120 + idx*1000 + target`), so a replayed row has
the recorded row's instance and target.

Cards pin no `commit`: the cards will be committed after they are written, and
`tools/task_card.py run` refuses a card whose pinned commit differs from HEAD.
The receipt records the HEAD it ran at (HEAD when the cards were written:
`f543b5523357eb11490a30fcac84d17359ce98ad`). The bind on the instrument digest
carries the identity of the code.

## Sample

Arms covered: `ic-a` (torsion and prime families), `ic-b`, `ic-c1`, `ic-c2`,
`ic-gls-neg`, `ic-gls-psi`, `rho-neg` (all three families), `rho-psi` (GLS), and
both negative controls (`control-shuffled-a` at 24 bits, `control-shuffled-rate`
at 40 bits). Sizes: nominal 24 bits (smallest) and 40 bits (largest).

| card | commands | recorded run it reproduces (rows in `../raw/`) | rows |
|:--|:--|:--|--:|
| r01 | `list-instances` | `instances.txt` (card-01 `list-instances`, all 60 instances) | file |
| r01 | `torsion-b24` | `torsion-b24.jsonl` (card-01), idx 0-5, `ic-a/b/c1/c2` targets 0-1, `rho-neg` targets 0-7 | 96 |
| r01 | `gls-b24` | `gls-b24.jsonl` (card-01), idx 0-2, `ic-gls-neg/psi` 0-1, `rho-neg`, `rho-psi` 0-7 | 60 |
| r01 | `prime-b24` | `prime-b24.jsonl` (card-01), idx 0-2, `ic-a` 0-1, `rho-neg` 0-7 | 30 |
| r01 | `control-b24` | `control-b24.jsonl` (card-01), idx 0-2, `control-shuffled-a` target 0 | 3 |
| r02 | `torsion-b40-i0` | `torsion-b40-i0.jsonl` (card-05), idx 0 | 16 |
| r02 | `prime-b40-i0` | `prime-b40-i0.jsonl` (card-10), idx 0 | 10 |
| r03 | `gls-b40-i0` | `gls-b40-i0.jsonl` (card-08), idx 0 | 20 |
| r03 | `rate-b40-i0` | `rate-b40-i0.jsonl` (card-04), `control-shuffled-rate`, 200 trials | 1 |
| r04 | `torsion-b40-i2` | `torsion-b40-i2.jsonl` (card-06), idx 2 | 16 |
| r04 | `prime-b40-i2` | `prime-b40-i2.jsonl` (card-10), idx 2 | 10 |
| r05 | `gls-b40-i2` | `gls-b40-i2.jsonl` (card-09), idx 2 | 20 |

282 of the 945 recorded rows (all 189 rows at 24 bits, 93 rows at 40 bits,
the 1 `rate` row at 40 bits). Recorded wall time of the kept commands, from the
receipts in `../raw/`: about 10 minutes plus five builds of about 9 s each. Card
timeouts are at least 2.9 times the recorded command time.

At 40 bits the two curves per family are idx 0 and idx 2: idx 0 is the first,
idx 2 has the longest recorded `ic-a` time in the torsion family (104 s).

## Dropped, and why

Dropped for runtime only (the full set would take about 19 minutes of recorded
command time at 40 bits plus the rest, too close to the 30-minute budget on a
shared 4-vCPU host with unknown speed):

* sizes 28, 32, 36 bits (cards 01 to 04: `torsion/gls/prime-b28/b32/b36`, `control-b28`, `rate-b32`, `rate-b36`);
* 40-bit curves torsion idx 1, 3, 4, 5, gls idx 1, prime idx 1, and `rate-b40-i1`, `rate-b40-i2`;
* `smoke-b24` (card 01; its instances and arms are the 24-bit idx 0 rows already replayed; its stdout carries wall times);
* `summarize` (card 11): it reads every file in `../raw/`, so a partial replay directory gives a different summary. The headline recompute belongs to the validator report.

## Comparison rule (applied after the cards ran, against the recorded rows)

Join key per row: `(family, bits, idx, target, arm)`. For each replay file, the
row set must equal the recorded row set for that key (same count, no extra, no
missing); the recorded rows are those of the file named in the table, selected
by the key. Files compare as parsed JSON (`blindspot01.v1`), not as bytes.

**Must match exactly** (equality after JSON parse; deterministic given the seeds):

* `instances.txt`: byte-identical, sha256 `21d288f71ffdd49e17dc30221205fad4a17202520c4cd73d4411d85483b0438a` (the digest in `../raw/receipt-card-01.json`).
* Instance identity: `instance`, `p`, `n`, `r`, `cofactor`, `log2_n`, `log2_r`.
* Solve result: `verified`; `detail.planted_matches`; `detail.recovered`; `detail.exhausted`.
* Priced and total cost: `gae_priced`, `native_total`, `gae_total_by_weight` (`w0`, `w0.1`, `w1`), `s_by_weight`.
* Every counter in `natives` (name and value): index calculus `abscissa_lookups`, `base_columns_hashed`, `elimination_merge_modr_ops`, `filter_nnz_touches_lower_bound`, `la_modr_ops`, `target_dedup_probes`, `walk_step_hashes`; rho `canonicalisations_uncharged`, `dp_table_ops_uncharged`, `negations_examined_uncharged`, `partition_hashes_uncharged`, `psi_maps_uncharged`; control `enumeration_key_probes`.
* Index-calculus `detail` counts: `columns`, `small_columns`, `unknowns`, `trials`, `decompositions_found`, `hit_rate`, `relations_used`, `full_relations`, `relations_with_0_1_2_large_primes`, `discarded_by_large_prime_limit`, `duplicate_targets_skipped`, `la_attempts`, `la_attempts_skipped_same_core`, `la_attempt_log_relations_core_ok`, `core_unknowns`, `wiedemann_failures`, `oracle_calls`, `oracle_degenerate`, `oracle_unliftable`, `oracle_lift_failures`, `oracle_symmetric`, `oracle_symmetric_fallbacks`, `crosscheck_trials`, `crosscheck_disagreements`, `group_adds`, `group_doubles`, `phases` (every `mult_eq` and `gae`), `c_add_mult_eq`, `c_add_muls`, `c_add_invs`, `c_add_by_inv_cost`, `base` (`points`, `columns`, `points_per_column`), `max_large_primes`, `lp_exponent`, `inv_cost`.
* Rho `detail`: `steps`, `walks`, `distinguished_points`, `short_cycles_escaped`, `fold`, `expected_steps`, `steps_over_expected`, `group_adds`, `group_doubles`, `canonicalisation_fp_mult_eq`, `canonicalisation_gae_if_priced_by_count`, `c_add_mult_eq`, `inv_cost`.
* Control `detail`: `columns`, `trials`, `decompositions_found`, `hit_rate`, `full_relations`, `la_attempts`, `enumeration_adds`, `group_adds`, `group_doubles`. The `control-shuffled-rate` row is recorded with `verified: false`, `recovered: null`, `exhausted: true`; the replay must reproduce those values, not a solve.
* Floating-point fields are functions of the integer counts above and are compared for equality. A difference confined to the last unit of precision is reported with its relative size and is not explained away.

**Descriptive only** (host speed; reported side by side, never a pass/fail):
`detail.wall_seconds`, `detail.wall_seconds_filter`, `detail.wall_seconds_wiedemann`,
and the per-command `seconds` in the receipts.

Any mismatch in an exact field is a replay failure for that row and is reported
as found. A command that stops (`status` not `completed`) leaves its rows
unreplayed; the certificate lists only rows that were compared. The stderr files
(`replay/raw/*.stderr`) hold the instance header `[name] n = …, r = …, cofactor …`
and are provenance, not compared.

## Run order

`card-r01` to `card-r05`, each dispatched separately through
`python3 tools/task_card.py run <card>`; no card depends on another. Receipts
are write-once: rerunning a card needs its outputs and receipt removed by the
owner, not by the executor.
