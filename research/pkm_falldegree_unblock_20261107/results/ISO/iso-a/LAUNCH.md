# iso-a launch receipt (Amendments 16–18), 2026-11-15

Measured. Every number is `[pcc:unverified]`.

- **Pod.** `w3ao0j8mfsiasz`, reserved, 16 vCPU, 128 GB, EU-RO-1 (secure cloud), US$1.04/h. Image
  `python:3.12-slim@sha256:5e2bdb1b…` (accepted by RunPod). Created at about 04:20Z. Create record: `pod-create.json`.
- **Uploads**, with the SHA-256 the runner reported back:
  - `pkm_tower_pilot_r4` `705b151b6f35…`
  - `prime_fall_degree_tools` `7e8a67030e8d…`
  - `hky_fp_tower_check` `c36d324e8a60…`
  - `repo.tar` `dc6a105bb86b…` = `git archive 10a0f1d7c253a5b13bc463d570afe9e72969e7cd $(cat iso/BUNDLE_PATHS)`
  
  At that commit `iso/BUNDLE.sha256` = `557ceba4d3cf…` and `iso/PINS.json` = `f02c03dd95a8…`, the hashes frozen in
  Amendment 16.
- **Start**, 04:21:38Z.
  - Stages: `g1 gaux A A-big DA g3 cert cap-A`.
  - Limit 120,000 s (Amendment 18), cost_per_hr 1.04, max_usd 40, memory floor 56 GiB.
  - Runner: `memory_bytes` 128,000,000,000, `cpus` 16, `runner_sha256` `85e8784de26a…` (the committed `runpod/server.py`).
  
  Every R3 guard passed. This is the first job the guarded runner has run.
- **Storage.** Git, per Amendment 17. Results are fetched at each check-in, checked against the pod's manifest, and committed
  in this directory.
- **Checkpoint hook.** Opted out through its recorded `CHECKPOINT_SKIP` (owner decision 12).
