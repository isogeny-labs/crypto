# V6 metadata checker correction review

Reviewed only external corrected checker `/home/ec2-user/ml-dataset-resume-20261029/wide-v5-checker-diagnostic-corrected.sh` versus original `/home/ec2-user/crypto-ml-wide-prime-f4-20261029/research/ml_dataset_wide_prime_f4_20261029/cli-fixtures-v5/check-wide-cell.sh` and retained `/var/tmp/wide-prime-cli-fe7ccef-v5`. No checker/fixture execution, generation, native replay, build, cloud action or repository edit was performed. Read-only jq inspection of saved JSON is not fresh verification.

Corrected checker SHA256: `1639b34c0a0821dd2f973d5157c25c3ff59bdeca4f6ebf6fb40df19babbaa251`.

## Finding

No blocking leakage or acceptance regression identified in the correction. The sole diff replaces key-name-only rejection with path-aware rejection, adding exactly one exception: path `["inputs","field","basis"]` with value exactly the string `canonical-residue`. Saved first-cell public features contain precisely that field, a public field-representation label. It is distinct from native polynomial basis data, which remains prohibited by the same forbidden key at every other path.

The new jq generator enumerates every nested path whose last component is a string, equivalent to examining object keys at every depth; numeric array indices are skipped but object keys inside array elements are still visited. Thus nested `basis` in a result object or array remains rejected. The public exception cannot carry an object, list, alternate string or embedded native data because its entire value must equal the fixed string. All other forbidden names remain rejected, including events, work/byte reservations, native_statistics, derivation flags, outcomes and private scalars. This preserves the existing blacklisting scope; it does not substitute for the producer's structural allowlist or certify arbitrary unknown keys safe.

Every other checker line is unchanged: native invocation and false derivation flag; exact manifest path/role cardinality; restricted retention; result strategy and false execution/operation verification; encoded-byte cap and numeric reservation envelope; role encodings; exactly one nonempty public static feature; feature/target/index associations; expected axes/modulus/target; graph exclusion; and actual source-manifest digest. Zero feature rows still fail the later exact-count check. Arithmetic remains in the native Rust verifier; no independent wide-negative/extraction consumer claim is added.

## Saved v5 status and v6 freeze requirements

Saved receipt records exit1 after7commands, source `fe7ccef6eb7f640fc8e95212471b80700d3cf96b`, binary SHA256 `a4ad63da4d7c1f89835c1aa51af37968ae7582cf60dea2a9acf9e2b51bde914d`. Retained first public field is `{basis:canonical-residue, characteristic:140737488355213, degree:1, modulus:140737488355213}` with numbers represented as existing canonical strings. Original protocol forbids key `basis` globally, so the original checker necessarily rejects this legitimate public field. This supports a metadata-checker correction, not an algorithm/input/limit change. V5 remains a failed software fixture and must not be relabeled as a completed six-cell run.

Before affected v6 data, freeze the exact checker hash and path/value exception in a new protocol/body, recompute canonical body digest and complete protocol binding, retain v5 original outputs and failure receipt, and preserve binary/seeds/shards/resource limits/no-resampling acceptance. The new run must use a distinct output path. This review authorizes no generation and reports no v6 result. Full-source S3 preservation and readback remain parent-owned requirements. No exporter modification is needed for this narrow public field label correction.
