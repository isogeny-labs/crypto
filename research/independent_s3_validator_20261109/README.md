# Independent S3 arithmetic and its Lean foundation

The separate Rust implementation matches the recorded C++ S3 census on all
six prime fields: 942 nonsingular short models, 4,188,162 ordered coordinate
triples, 244,236 zeros, and zero false or missing zeros. Each base-field
element obtains a square root in the actual quadratic field; inverse and
point-sum membership checks pass. C++ reference aggregates were seen before
development. This is an exact known-answer check, not a blind discovery.
[pcc:unverified]

The Lean project proves square-root lifting for finite quadratic field
extensions, irreducibility/cardinality for the actual AdjoinRoot construction,
the actual Mathlib point-sum/S3 oracle equivalence, primality of the chosen
moduli, and the exact F3 control count 54. The repository axiom gate passes
11 declarations, with zero failing or trusted-native exceptions. It uses no
custom axiom, sorry/admit or native_decide. The full local check for
PCC-20261109-e789dd passes; the claim remains unsigned. These are mathematical
proofs, not proofs of Rust compilation or execution. [pcc:unverified]

The planned separate-host run did not execute. The single manual workflow
dispatch returned HTTP404; workflow lookup and main-branch file lookup also
returned HTTP404. The new file exists on the feature branch. This supports
a registration/default-branch gap, but permissions can also mask an endpoint
as404; the response does not identify an exclusive cause. No remote
compiler/binary/host exists in this record, and item13 remains partial.
No trusted workflow was repurposed to bypass the rejection. [pcc:unverified]

| Arm | Actual outcome | Native population |
|---|---|---|
| Local distinct Rust | Completed, fixed comparator passes | All six fields, 4,188,162 triples |
| GitHub-hosted | Not completed, dispatch HTTP404 | Missing; never inferred from local output |

Source commit275f566e preceded the frozen protocol commit770ee2e5. Protocol
hash44eea2cae6e05da1f23450dab8373984e369929ce9abf8a2d2227786db67e820
was frozen at05:43:59Z, before full local work05:44:11Z. Published source
7b0da356 and protocol9952aad4 preceded dispatch05:44:31Z. Per-host binary
identity is measured before native work; the local compiler is rustc1.99.1,
binary digest9cee15ca13d1a8160ecfdb8d5d5219b0d52e5a7efa3a23428a8b2d0d464d6afe.
Full metadata and exact tree mappings accompany the evidence. [pcc:unverified]

The complete flow is2planned/attempted,1completed,1not-completed,
0interrupted/resumed/excluded. The canonical export and all file hashes pass.
The first export mistakenly used the checkout commit for the pinned source
instrument; it was rejected, preserved under evidence, and replaced by a
new export with the unchanged source identity. The actual checkout stays in
host.json. No frozen code, protocol or raw output was changed. [pcc:unverified]

Preflight p3/p23 is retained and excluded from the six-field full population.
One local shared host, one thread, no clock metric, approved local
checkpoints, zero paid spend. The manual-only workflow uses a standard
public ubuntu24.04 runner, contents-read, no secrets/EKS/GPU, no cache or
artifact upload. Public standard-runner billing documentation and repository
visibility are retained. Its rejected dispatch incurred no native run.
[pcc:unverified]

Reproduce from the repository root, choosing a fresh output directory:

```sh
tools/lean/proof-gate.sh research/independent_s3_validator_20261109/lean
S3_VALIDATOR_ARM=local bash research/independent_s3_validator_20261109/run-driver.sh evidence/reproduction
python3 /workspace/crypto-autoresearcher/tools/experiment_protocol.py check-export \
  research/independent_s3_validator_20261109/export.json \
  --protocol research/independent_s3_validator_20261109/protocol.json --verify-files
```

See [Lean source](lean/ValidatorFoundation.lean), [gate](evidence/proof-gate.log),
[protocol](protocol.json), [export](export.json), [local output](evidence/local/actual.jsonl),
[comparison](evidence/local/comparison.json), [dispatch response](evidence/remote/dispatch-response.txt),
and [visual report](reports/run-20261109-independent/report.pdf).
The general binary Semaev formula, resultant recursion and degree proof are
not discharged by this finite check. The original fourteen-item acceptance
map remains [here](../theorem_census_20261109/README.md).
[pcc:unverified]
