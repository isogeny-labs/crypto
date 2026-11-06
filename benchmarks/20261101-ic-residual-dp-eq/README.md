# 20261101-ic-residual-dp-eq (evidence directory; not an admitted row)

Report copies, the independent replay certificate, the validator report and `VALIDATION.json` for `research/ic_residual_dp_eq_20261101`.
The row is **not** listed in `benchmarks/README.md`: the validator's `raw_data` are the two `records.jsonl` files, which the artifact policy keeps in the
artifact store (not in Git), and the validation gate (`scripts/validation_gate.py`, CI job `gate`) reads repository files only, so in CI it reports
"is not a file". The evidence is complete and valid when the records are hydrated (`tools/artifacts/artifacts.py pull research/ic_residual_dp_eq_20261101`,
`ARTIFACTS_REMOTE=s3://setec-astronomy-artifacts/store`; `scripts/validation_gate.py evidence benchmarks/20261101-ic-residual-dp-eq` then says `valid`).
Restore the row once the gate can read artifact-store raw data (headline: eq 0.64-0.68 and dp 0.82-0.94 of the baseline's counted S at w = 0; the w > 0
band as built prices a table-byte counter as operations, see the report; distance to rho at w = 0: best arm 1.54x on p36, 1.64x on p37).
