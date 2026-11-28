# Frozen native guarded K2048/K4096 continuation

Frozen before input reconstruction, model construction or search in this rung.
User instruction: restore Docker, resume the N83 factor-base sweep and use
`setec-astronomy-artifacts` for S3 storage. The packaged systemd Docker service
was inactive; it was started through authorized sudo. A guarded preflight
confirmed one CPU, memory.max=4294967296, memory.swap.max=0, no network and
successful loading of the existing native x86-64 worker. Docker access uses a
task-local sudo wrapper; global socket permissions and groups are unchanged.

## Inputs and provenance

Use primary a=0 GF(2^83), polynomial low terms [0,1,2,45], the 81-bit subgroup,
public-x-hash policy and seed 2026100801. K2048 has 339968 signed points;
K4096 has 679936. Preserve the exact historical compressed bytes:

| K | Compressed BLAKE3 | SHA-256 |
| --- | --- | --- |
| 2048 | 82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851 | aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a |
| 4096 | a84572e6ca52efa7867082261bf0543723d52c0d136061a45c970970716aa779 | ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad |

Reconstruct K4096 deterministically with the unchanged exporter under a separate
240-second native outer guard, then generic replay under another 240-second
guard. These input-recovery phases use single-CPU affinity and a 4-GiB
address-space bound; they are not guarded IC search timings. Fail closed if
any retained compressed/plain/point-set digest or exact count differs.
Upload K4096 with conditional, content-addressed writes and AES256 to the
user-selected bucket, then freshly GET both K2048 and K4096 into frozen input
panels and verify SHA-256/BLAKE3. Stop on any input or storage failure.

The worker's unchanged loader requires historical original-bucket sidecars.
Keep those manifests, generic replay and original upload receipts byte-identical
from the retained K2048/K4096 evidence. Those are historical provenance, not a
claim that the new GET uses the old bucket. Record current setec HEAD/GET
receipts separately. No write to the old bucket is authorized or planned.
Use the existing fixture-zero public corpus and keep the known-answer sidecar
outside relation collection; the driver reads it only after candidate recovery.

## Construction and search

Pin the local image ID and source snapshot; the worker's compiled source blobs
must equal the frozen source commit. Reuse the tested native worker only after
that attestation. For each K in order 2048,4096:

1. Construct the all-affine m=5 orbit-phase chained-S3 model with identity mask
   zero, maximum 150000 variables, maximum 2000000 domain clauses and a
   180-second wall cap. Require PASS_model_construction_only, original-group
   input validation, matching source counts and verified cleanup before search.
2. Run the original public query with m=5, orbit-phase domain, maximum one
   trial, 150000 variables, 2000000 domain clauses, 32 models and 100000
   conflicts. Each arm has a 600-second outer/worker wall cap. Use no first-pair
   ordering and no large-prime adapter in these cells. Retain every failed or
   censored outcome, source/config/input hashes, live Docker inspection,
   worker events, independent group replay and container cleanup.

Both stages use one CPU, a 4-GiB hard cgroup, zero swap, no network, dropped
capabilities and read-only inputs. Run arms sequentially. The enlarged domain
cap admits K4096's projected 1773110 domain clauses; it is identical for both
arms. The previous K2048 100000-conflict run on an emulated ARM host is
historical context, not a native throughput reference. Verify the actual query
and coefficient identities agree before calling these matched query results.

The maximum declared worker envelopes are 480 seconds for input recovery,
360 seconds for two construction calls and 1200 seconds for two searches:
2040 seconds total. Docker/image preparation, storage IO, audits, validation
and previous work remain separate charges. This is a fresh bounded rung under
the user's resume instruction, not a claim of unspent historical allowance.
Do not extend a search cap or add expensive cells without a new frozen protocol.

## Audit and interpretation

Retain source/config/binary hashes, input identities, live cgroup/inspect proof,
worker/outer consistency, complete terminal statuses and independent original-
curve query/relation replay. Use negative controls against forged worker
success. Archive raw public research evidence in the user-selected bucket and
retain compact review receipts in Git. Verify stored bytes by fresh download.

This rung measures bounded natural-query behavior and capacity on two exact
bases. A single query, a censored result or stage timing cannot estimate a
complete cold pipeline or establish the fastest tool. No external full-pipeline
comparison, A/A spread, useful rank campaign, verified column logs, individual
log or same-point rho result is supplied by these cells. All earlier finite-grid
requirements remain open; selected_best_total_runtime stays null unless those
separate complete-measurement obligations are actually met.
