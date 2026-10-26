# Calibration runs (non-frozen targets)

Raw receipts of the runs that set the budget in `../PROTOCOL.md` ("Calibration
disclosure"). They use `calib_32_manifest.json` (six targets, target seed 32,
SHA-256 `8d0d9f23e5913e540ad5077186f78adec25ea6c3dbf32e78c31c48e2f6116e5a`), none of which is in
the frozen manifest, and earlier builds of the producer (the header `producer_sha256` of each file
names the build; the form-sharing variable cache is in `cal_cse.jsonl` only). Not part of any result
table. Several were stopped early on purpose (`calib_run1`, `calib_run2`), so they have no `end` line.

| file | what it ran |
|---|---|
| `calib_run1.jsonl` | `subgroup_xor`, branch priority on all unknowns, 60 s wall backstop; stopped after 4 attempts |
| `calib_run2.jsonl` | the same with priority on the core unknowns; stopped after 7 attempts |
| `cal_none.jsonl` | arm A on `a0`, no branch priority, 40 s backstop |
| `cal_long.jsonl` | arm A on `a0`, core priority, 280 s backstop: UNSAT at 873,695 conflicts |
| `cal_rest.jsonl` | targets `a1 b1 c0 c1`, both arms, core priority, 400 s backstop |
| `cal_sym.jsonl` | `subgroup_xor_sym`, targets `a0 a1`, both arms, 60 s backstop |
| `cal_cse.jsonl` | the final encoder (shared linear-form variables), targets `a0 b0`, 100 s backstop |
