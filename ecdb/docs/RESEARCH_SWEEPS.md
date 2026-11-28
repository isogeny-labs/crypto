# Research sweeps, factor-base objects and ML features

Migration 0016 turns repeatable curve cohorts and model contracts into database
objects without confusing a plan with a result. It extends the existing curve,
isogeny, CM, relation, FB1/FBM1 and measurement layers; it does not replace any
of their identities.

## Requirement map

| Requested capability | Database/API object |
|:--|:--|
| Saved Koblitz, non-Koblitz binary and prime-field size sweeps | `ecdb/sweeps/*.json`, `research_sweep`, `research_sweep_cell`, `GET /v1/sweeps`, `scripts/run-sweep-api.sh` |
| Isogeny, isomorphism and imaginary-quadratic strata | `cm-isogeny-strata.json` over the existing isogeny classes, typed relations, volcanoes, End(E), order class groups and CM fields |
| Model-ready curve features | `feature_definition`, `feature_observation`, `v_curve_ml_features`, `GET /v1/curves/{ref}/features` |
| FB1 and FBM1 learning rows | `v_factor_base_ml_features`, `v_factor_base_measurement_ml_features`, existing `/v1/factor-bases` data |
| S3/object-store references for factor bases | `factor_base_artifact -> artifact.storage_uri`, returned in factor-base documents; write with `POST /v1/factor-base-artifacts` after upload/verification |
| Reproducible datasets and leakage-aware splits | `ml_dataset`, `ml_dataset_feature`, `ml_dataset_member`, `ecdb/datasets/model-ready.json`, `scripts/export-dataset-api.sh` |
| Everything known about one curve | `GET /v1/curves/{ref}` now includes the structural ML row, factor-base summaries/artifacts, observations and memberships in addition to the normalized catalogue |

“Everything” means every value the imported producers have established plus an
extensible, versioned observation contract. An absent or inapplicable value is
null (or an observation with `missing_reason` and status); it is never silently
zero. New invariants can be added as a feature version without widening the
normalized curve tables.

## Load the saved configuration

```sh
cd ecdb
ecdb migrate
ecdb import --repo-root .. research
```

`ecdb import research` content-binds the feature catalog, four sweep manifests
and three dataset recipes. Reusing a name/version with different canonical JSON
is reported as semantic drift. Importing twice is a no-op.

The Lambda and Docker packages carry these inputs, and `ecdb import all` runs
the research importer after factor-base/result import.

## Resolve a sweep

Each `ecdb.research_sweep/v1` cell stores:

- an exact `GET /v1/curves` filter and deterministic selection policy;
- a target sample count and seed;
- selectors for factor-base families already defined by the FB1 protocol;
- controls, grouping axes and the required feature keys.

```sh
export ECDB_URL=http://localhost:8080
scripts/run-sweep-api.sh sweeps/koblitz-ladder.json /tmp/koblitz.jsonl
scripts/run-sweep-api.sh sweeps/binary-ladder.json /tmp/binary.jsonl
scripts/run-sweep-api.sh sweeps/prime-ladder.json /tmp/prime.jsonl
scripts/run-sweep-api.sh sweeps/cm-isogeny-strata.json /tmp/cm.jsonl
```

The JSONL output is a candidate snapshot, not evidence. A producer freezes its
chosen rows in `research_sweep_member` with provenance before measuring them.
Sweep manifests do not claim that every stratum already has curves or results.

## Factor-base objects

FB1 names the point set and FBM1 names
`curve x FB1 x oracle x solver x linalg x instrument`, exactly as
[`FACTOR_BASE_PROTOCOL.md`](../../docs/ecbench/FACTOR_BASE_PROTOCOL.md)
requires. The existing `factor_base`, `factor_base_measurement`,
`factor_base_result` and `v_factor_base_result` tables remain authoritative.

Large dumps, matrices, relations and log tables belong in the immutable object
store. Upload and remotely verify the bytes first, then link their digest and
URI:

```json
{
  "factor_base": "FB1h0123456789ab",
  "role": "dump",
  "is_primary": true,
  "sha256": "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789",
  "size_bytes": 4096,
  "media_type": "application/json",
  "storage_uri": "s3://crypto-autoresearcher/store/sha256/ab/abcdef...",
  "repo_path": "store/index.json",
  "description": "ecbench.factor_base_dump/v1",
  "agent": {"name":"producer","method":"crypto store verify --remote"}
}
```

```sh
export ECDB_WRITE_TOKEN=...
scripts/attach-factor-base-artifact-api.sh /tmp/factor-base-artifact.json
curl -s "$ECDB_URL/v1/factor-bases/FB1h0123456789ab" | jq '.artifacts,.ml_features'
```

The endpoint stores the reference; it does not upload or verify remote bytes.
Keep the object-store verification receipt as evidence.

## Feature observations and datasets

The three views have different sample grains:

| View | One row per | Typical inputs/targets |
|:--|:--|:--|
| `v_curve_ml_features` | exact curve model | field, Frobenius, CM order/class group, automorphisms, isogeny/isomorphism graph, best linked FB result |
| `v_factor_base_ml_features` | FB1/FBS1/archive base | construction/size, curve stratum, results, rank/yield, object coverage |
| `v_factor_base_measurement_ml_features` | FBM1 configuration | oracle/solver/linalg/instrument, yield, solving degree, Macaulay shape, complete charged cost |

Computed or producer-specific values that do not belong in those stable views
are typed observations. Exactly one subject is named and the feature definition
controls its scope and JSON type:

```json
{
  "fbm_id": "FBM1h0123456789ab",
  "feature_key": "ic.decomposition_probability",
  "version": 1,
  "value": 0.03125,
  "context": {"fb_id":"FB1hfedcba987654","run":"research/.../records.jsonl"},
  "status": "proved",
  "agent": {"name":"producer","method":"verified relation count / attempts"}
}
```

```sh
scripts/record-feature-api.sh /tmp/observation.json
curl -s "$ECDB_URL/v1/features?scope=factor_base_measurement" | jq
```

Dataset recipes pin subject grain, exact feature versions/roles, selection SQL,
seed and split policy. The supplied policies keep all samples sharing an
isogeny class or an FB1/curve group in one split to reduce structural leakage.

```sh
curl -s "$ECDB_URL/v1/datasets/fbm1-solver-outcomes" | jq
scripts/export-dataset-api.sh fbm1-solver-outcomes /tmp/fbm1.json 5000
```

The export is an exact JSON envelope. A materializer may write Parquet/Arrow,
apply and freeze the stated split, upload the bytes, and then attach the
resulting `artifact_id` to the dataset. Training code should retain feature
versions, source commit, selection query, split assignment, status and missing
reason.

## Sources and evidence boundary

- [`ecdb/docs/SCHEMA.md`](SCHEMA.md) defines exact identity, status,
  provenance, CM/isogeny and relation semantics.
- [`docs/ecbench/FACTOR_BASE_PROTOCOL.md`](../../docs/ecbench/FACTOR_BASE_PROTOCOL.md)
  defines FB1/FBM1 and the Git/S3 store contract.
- [`docs/ic/SWEEPER.md`](../../docs/ic/SWEEPER.md) defines the existing search
  spaces and control boundaries used by the manifests.
- Semaev, *Summation polynomials and the discrete logarithm problem on elliptic
  curves* (ePrint 2004/031), and the repository's Koblitz implementation notes
  motivate relation yield, solving-degree and matrix features. Their presence
  in a dataset does not establish an advantage on any curve.

No sweep plan, feature definition or dataset recipe is a performance claim.
Measured rows retain their producer, FB1+FBM1 identity, unit, matched control
and validation status.
