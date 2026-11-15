# Amendment R2-B, frozen 2026-11-10 as PREREGISTRATION Amendment 13: the shape-matched `m = 4` null at `t = 4, 5`

**Status: FROZEN 2026-11-10 as Amendment 13 of research/prime_fall_degree_20261027/PREREGISTRATION.md, before any `pfd-m4-*` or
`pfd-m4n-*` job is submitted and when no `m = 4` row at `t >= 4` and no `m = 4` null row exists in the relaunch.** Drafted
2026-11-08 (round 2, slug `r2pkm`) for decision 6 of `research/open_leads_execution_20261108/DECISIONS.md`. Since the draft, only
the status lines and the resource sentence of section 4 changed (Amendments 9 and 12); sections 2, 3, 5, 6, 7 and 8 are as drafted.
Every number is `[pcc:unverified]`. Classes: *cited* (a file says so), *measured* (read from source or a command), *derived*, *plan*.

## 1. Timing and independence

Freeze **before any `M4N-*` cell is submitted**. The text below uses no recorded row of this study and no outcome of any cell: the
cells, flags, seeds, limits and readings are fixed from Amendment 8 (cited) and the pilot's source (measured). The Kummer `m = 4` cells
(`pfd-m4-*`) do not need this amendment; they run under Amendment 8. The null cells are paired with them by target.

## 2. What Amendment 8 already says (cited)

"The shape-matched null runs at `t = 4, 5`" (Amendment 8, Cells) and, in the draft section 2, "same `D` as Kummer on every recorded
cell". Amendment 8 does not give the command. At `m >= 3` the pilot runs the null only with `--null-chain` (the `B-m3-null` cell of
`run.sh hky-B` already uses it at `m = 3`).

## 3. The flags exist (measured, source of `examples/pkm_tower_pilot.rs` at f13a690df0)

- `--null-chain` is parsed at line 2008 into `Args.null_chain` (doc, lines 47-49: "runs the null control at `m >= 3` as well, with every `S3`
  link of the chain given random coefficients on its own support"). Without it, the cell filter at line 2651 is
  `Control::Null => m == 2 || a.null_chain`, so `--controls null --m 4` alone produces no cell.
- `--controls`, `--targets`, `--t-min`, `--max-t-m4` (m >= 4 ladders use `max_t_m4`, lines 2639-2645), `--hky-cmax`, `--hky-max-entries`,
  `--hky-max-cols`, `--budget` are the flags the Kummer `m4:` stage already passes.
- `prime_fall_degree_tools` is only a driver: it forwards `--null-chain` to the pilot (line 520); it has no flag of its own.
- Null construction (lines 995-1012): every `S3` link of the chain keeps its Kummer support with uniform random coefficients in `1..p-1`;
  with `--planted 0` there is no planted zero to subtract. The random-target draws (`--random 2`, `--targets k`) are those of the Kummer
  cell with the same `t`, `k` (same seed `0x504B4D54`), so the pair is matched by target index; the actual match is recorded by `x_r` in the rows.
  Measured in the dev check (section 8): at `t = 2`, draw 0, the Kummer and the null row carry the same `x_r` (1102336260). That is one
  draw at `t = 2`; at `t = 4, 5` the rows' `x_r` settle it afterwards, and the pairing is by `(t, k)` whatever they show.
- Dev check (seconds, not a result): see section 8.

## 4. Cells (plan)

Kummer, `m = 4`, chain presentation, `P1 = 2013265921`, seed `0x504B4D54`, `--planted 0 --random 2`, `--controls null --null-chain`,
`--engine hky`, one closure per job:

| job | `t` | `N` | draw `k` | `--hky-cmax` | counterpart |
|:--|--:|--:|--:|--:|:--|
| `pfd-m4n-t4-k0-c7` | 4 | 16 | 0 | 7 | `pfd-m4-t4-k0-c7` |
| `pfd-m4n-t4-k1-c7` | 4 | 16 | 1 | 7 | `pfd-m4-t4-k1-c7` |
| `pfd-m4n-t5-k0-c6` | 5 | 20 | 0 | 6 | `pfd-m4-t5-k0-c6` |
| `pfd-m4n-t5-k1-c6` | 5 | 20 | 1 | 6 | `pfd-m4-t5-k1-c6` |

Limits equal the Kummer counterparts': `--hky-max-entries 13958643712` (112Gi), `--hky-max-cols 4000000`, `--budget 172800`; request
12 cpu / 112Gi / 176,400 s, no instance pin, price guard of Amendment 12 (the frozen requests are `requests_r2g/pfd-m4n-*.json`; the older copies in `requests_pending_amendment/` and `requests_r2c_pending_amendment/` are superseded and must not be launched). `t = 5`, `c = 7` and `t = 6` have no null cell (they need the capacity of `CAPACITY_R2.md`, and Amendment 8 asks the null at
`t = 4, 5` only). Exact command, as the `m4n:` stage of `image/entry_r2.sh` runs it:

```
/usr/local/bin/pkm_tower_pilot --p 2013265921 --ladder-t none --planted 0 --random 2 --engine hky --kinds kummer --m 4 \
  --controls null --null-chain --hky-cmax C --hky-max-entries 13958643712 --hky-max-cols 4000000 --budget 172800 \
  --t-min T --max-t-m4 T --max-t T --targets K --out runs/M4N-tT-kK-cC.jsonl
```

The Kummer counterpart differs only in `--controls tower` and no `--null-chain`.

## 5. Order

After G3 has passed in `pfd-a` (as for all `m4` cells), after the Kummer counterpart of the same `(t, k)` has been submitted (so a
resource failure of the pair is known to be the same), and serially. The frozen requests are in `requests_r2g/`.

## 6. Readings (fixed before data)

No new hypothesis. H-null (PREREGISTRATION §6) is read as written, on the `m = 4` ladder, at the common completed `N` (16 and 20).
A cell that is censored at its `--hky-cmax` gives the one-sided value `d_F > cmax`; when both arms of a pair are censored at the same cap the
pair reads "mixed" for H-null, never "shape-driven". The null does not enter the Amendment 8 rise/flat/fall readings; those are Kummer-only.
Amendment 10 (R2-A, high-lambda) does not apply: `lambda(m = 4, t = 4, 5) = 1.1e-5, 1.7e-4` (derived).

## 7. Consequences stated before data

- At `t = 5`, `c = 6` both arms may be censored (Amendment 8 prediction: `c = 7` does not fit). The null then adds little at `N = 20`; it is
  still run because it is cheap relative to its counterpart and the pair at `N = 16` can read.
- Memory use of a null closure may exceed the Kummer's (random coefficients on the same support, same number of monomials, different
  sparsity of the reduced rows); a limit stop is recorded as censored.

## 8. Development check (seconds, dev build; not a result, not stage data)

Static musl binary built from f13a690df0 (`image/binaries.sha256`), `RAYON_NUM_THREADS=2`, one draw, `t = 2` (`N = 8`), `--hky-cmax 5`,
`--budget 600`, otherwise the flags of section 4. Files: `devcheck_r2/dev-m4-null.jsonl`, `dev-m4-tower.jsonl` (+ `.out`). Both exit 0 and write
one row. Null row: `control: "null"`, `first_fall_degree 4`, `falls [4, 5]`, closure `c = 5` has `cols 1023`, `entries 5535`, `muladds 44957`; Kummer row:
`control: "tower"`, same `cols`, `entries 8169`, `muladds 91054`. So the flags run and the null differs from the Kummer in the closure's content. `--t-min 4` was
**not** exercised (as in the first study's dev check) and no `t >= 4` null row exists.
