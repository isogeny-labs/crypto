# Cohort index V3: public feature size/split binding

V3 fixes the confirmed V2 split gap. Source exporter `src/ml_dataset/export.rs:1313` computes the core feature split from `inputs.n`. V2 checked only the declared component and partition. `v2-regression-reproduction.json` records that V2 accepted a synthetic inputs.n change from 23 to 24 after updating artifact and manifest hashes while retaining field-size-n23/train.

V3 requires every public feature to have a non-null split and component. Core features must contain a positive integer inputs.n and their component must equal field-size-n{inputs.n}; the existing deterministic partition calculation then applies. Auxiliary public features with a direct n field receive the same consistency check. Historical nonempty auxiliary ECDLP inputs contain a field descriptor, not a direct n field; this correction does not invent an n extraction rule for those shapes. Targets with unavailable features retain their legitimate null split behavior.

Twenty-four tests pass, including rehashed core-size substitution, missing core size, null public split, direct-n auxiliary inconsistency, and null unavailable-target compatibility. The final historical verification also passes for seven cohorts, 78 manifests and 926 artifacts. Output is `verified-cohort-index-v3.json`, whose bytes equal the V2 index: the new admission checks change validation, not the valid historical metadata.

A tentative additional equality check between all auxiliary records and their source target split failed on retained records that do not carry a matching split. That speculative rule was removed; its failure logs are retained as `extra-channel-join-attempt-*`. It is not part of V3's claims. V2 remains unchanged in its original directory; inherited evidence is segregated under inherited-v2-evidence/.

All V2 typed plan binding, mandatory source bindings, integrity, JSON, safe path, join and non-pooling checks remain. This remains a provenance-only index, not a semantic replay or privacy proof. No source dataset was altered, no new scientific data generated, and no native solver executed. Final command logs and hashes are in validation-receipt.json.

Usage with a fresh output:

```sh
python3 cohort_index.py --inventory inventory.json --output new-index.json
```
