# Continuation after the attempt-01 runner error

The frozen `N83-FACTOR-BASE-PDP-FFD-LFD-20261031-V2` panel contains 144 systems for spike35–spike37. Attempt-01 produced and S3-readback-verified `spike35/koblitz-m2-planted`, and a fresh replay later passed. Its runner then stopped on a Python argument error before the next system. That attempt and its failure record remain in `../attempt-01/`.

This continuation covers only the other 143 systems. The prior certificate is explicitly excluded and is never recomputed. Scientific inputs, method, binary, per-case limits and claim scope remain bound to frozen V2. The runner and analysis hashes are frozen in `script-binding.json`; this directory and every output use a new S3 namespace under `.../v2/attempt-02/`, so attempt-01 objects are retained unchanged.

`preflight.json` lists all 143 source rows and binds the already replayed case excluded from this continuation. `source-index.json` maps each system to its original byte length and SHA-256. `run_remaining.py` validates the bindings, checkpoints certificates and attempt/replay records, and `analyze_run.py` independently checks the continuation results. See `FILE_LAYOUT.json` for every output's structure. The claim remains first-fall certificates only; no LFD, affine degree of regularity, solving degree or factor-base ranking follows from these records.
