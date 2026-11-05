# spike29 PDP source-binding audit

This audit binds the saved `spike29` PDP inputs to the frozen V2 first-fall arms and to the saved fresh-process top-form replay. It verifies the complete fixed factor-base manifests, the 48-case family × m × target cross, every input's factor-base membership references, the shared input-index hash, and the listed SHA-256 manifests.

Run from the repository root:

```sh
python3 research/n83_candidate_pdp_20261028/evidence/n83-factorbase-variants/r29-attempt-02/source-binding-audit-20261103/audit.py --check
```

The checker returns `PASS` only when the deterministic [`audit.json`](audit.json) still matches its sources and all bound files verify. Running without `--check` refreshes that JSON from the present source files. The checker reads source and manifest files; it does not invoke `pdp`, `fall`, a Gröbner engine, or a cloud job.

## Bound source layout

- `protocol-pdp-factorbase-v2.json` freezes three `spike29` family arms at source commit `52665c7a6e8308f86201e10c9305afa346fd134b`; each binds the `pdp` binary hash and coordinate map.
- `r29-attempt-02/factor-base-output/` contains the fixed Koblitz, binary, and prime point bases, their family manifests, and the full file hash manifest. Each family manifest identifies one complete base reused for every m from 2 through 9.
- `r29-attempt-02/pdp-inputs/attempt-02/candidate-structure/` contains `inputs.json`, the family manifest index, and 48 case input/system pairs. `attempt.json` binds the input anchor, factor-base index, and producer binary.
- `r29-attempt-02/top-form/attempt-01/` contains the replay summary and top-form run binding for those same 48 source systems.
- `audit.json` records the checked hashes and per-case source-to-replay bindings. `FILE_LAYOUT.json` describes each file in this audit directory.

## Scope

The successful audit proves that these saved systems, factor-base manifests, protocol arms, and top-form replay records are source-bound and hash-consistent. The top-form artifacts are not first-fall certificates. This audit produces no first-fall degree, exact LFD, affine PDP degree of regularity, solving degree, factor-base ranking, or new index-calculus measurement.
