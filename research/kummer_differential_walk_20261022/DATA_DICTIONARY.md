# Data dictionary

`results/transition-{bits}.jsonl`: one transition row, keyed by `bits`,
`p`, `target_index`, `step` (0–2), and source `workload_id`. `input_k`,
`input_k_plus_g`, `output_k`, and `output_k_plus_g` are homogeneous fast
Kummer coordinates. `expected_k` and `expected_k_plus_g` come from the
independent signed Jacobian computation. `input_label` and `output_label`
are coefficients on the frozen source generator and target modulo `r`.
`expected_signed` and `label_signed` are separate Jacobian computations.
`torsion_repair_roots` records each of three xADD branches, with null when
ordinary xADD succeeds. `status` is explicit for every row.

`results/legendre_diagnostic/transition-*.jsonl` preserves the first
partition's exact rows. `results/summary.json` is recomputed by
`validate.py`; its cost fields are explicit nulls.
