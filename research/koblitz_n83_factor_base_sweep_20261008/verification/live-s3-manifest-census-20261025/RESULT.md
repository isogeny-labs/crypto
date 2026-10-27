# Live N83 S3 manifest and point-set identity census, 2026-10-25

The [offline catalog auditor](catalog.py) reconciles **all 77** current
content-addressed factor-base objects from the earlier [two-round byte
custody](../live-s3-full-custody-20261025/RESULT.md) to exact manifests.
Twenty-three fresh S3 `GetObject` calls retrieved 84,664 manifest bytes,
all with AES256. Those manifests name 76 distinct objects; their compressed
BLAKE3 names and sizes match the downloaded-object receipts. The remaining
direct-subgroup dimension-18 object has a manifest and complete generic
replay in the separate source commit
[`46c86f47c5530f309453d94ee7246957b03544af`](https://github.com/aburan28/crypto/commit/46c86f47c5530f309453d94ee7246957b03544af).
Exact copies of that manifest, replay and original S3 receipt are saved here,
SHA-256 pinned by the auditor, and bound to the currently downloaded object
by compressed BLAKE3, SHA-256 and byte length. Its prior generic multi-limb
replay checked all **2,760,082** point rows and 16,627 representatives;
this census did not repeat that group calculation.

| Curve arm | Content-addressed objects | Distinct manifest-declared point-set hashes | Redundant archive identities |
| --- | ---: | ---: | ---: |
| Primary `a=0`, 81-bit subgroup | 50 | 50 | 0 |
| Diagnostic `a=1`, different 53-bit subgroup | 27 | 15 | 12 |

The twelve diagnostic aliases are six three-way seed repetitions in its
sequential and Gray-prefix K=64/256/600 pilot bases. No primary objects
share a manifest-declared point-set hash. These hashes are metadata claims
bound to byte-verified archives, not a fresh semantic replay of every
archive by this census. The two curve arms cannot be pooled for a runtime
ranking.

The [saved catalog](evidence-01/catalog.json) maps every object key to its
manifest source, curve arm, compressed-byte identity, declared point count,
orbit columns and point-set hash. The 23 raw S3 manifests and their GET
responses are in the same directory. The panel manifests whose S3 directory
is content-addressed also matched the BLAKE3 digest in that directory. Four
tests pass, including rejection of a missing manifest, an altered S3
manifest claim even with its local receipt SHA updated, and a changed direct
replay receipt.

Recheck the frozen saved observation from this worktree with the pinned
`blake3==1.0.8` Python package:

```sh
python3 -m pip install --target /private/tmp/n83-s3-audit-pydeps -r research/koblitz_n83_factor_base_sweep_20261008/verification/live-s3-full-custody-20261025/requirements.txt
PYTHONPATH=/private/tmp/n83-s3-audit-pydeps python3 research/koblitz_n83_factor_base_sweep_20261008/verification/live-s3-manifest-census-20261025/catalog.py
PYTHONPATH=/private/tmp/n83-s3-audit-pydeps python3 -m unittest discover -s research/koblitz_n83_factor_base_sweep_20261008/verification/live-s3-manifest-census-20261025 -p 'test_*.py' -v
```

The direct object's standalone manifest was retained in Git by its prior
producer but is absent from this S3 prefix; its archive itself is present
and byte-verified. This catalog is a frozen 2026-10-25 storage/provenance
observation, not a new relation or timing experiment. The existing larger
U/D four- and five-summand panels in the direct-base branch reached bounded
natural-search caps without checked natural rows. The current smaller XOR
solver-setting panel is still running. Neither path supplies useful rank,
linear algebra, individual logarithm, fully charged one-target cold time or
matched same-target rho. `selected_best_total_runtime` remains null.
