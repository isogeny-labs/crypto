# Inherited merge-gate failures

Main 833a35d21ec2368ab081b2083d7762ed0bfd2bbc was checked independently.
PCC ledger verification reports 151 claims without the required claim_type,
across a ledger of 1361 entries. Claim, ledger and keyring bytes remain unchanged.
The benchmark gate encountered object-valued validation metadata in closed
record 20261117T0334Z-hec-multi-production-cold. The schema requires a retained
validation-file path or null. [pcc:unverified]

The benchmark tool now rejects that malformed field normally instead of raising
TypeError. All 60 tool controls pass, including malformed-type refusal and the
existing missing-file requirement. The closed record remains unchanged; the
tool does not reinterpret its metadata as a validation certificate.
[pcc:unverified]

The artifact wiring generator updated 36 live workflows using the committed
artifact manifests and admitted validation metadata. Its recheck passes;
57 artifact-tool controls and 40 frozen-view controls pass. YAML comparison
confirmed that the edits change only generated pull steps and artifact-tool
bootstrap checkout patterns. The 39 workflow pin/index/manifest objects remain
unchanged. These checks make no artifact-store request. [pcc:unverified]

The fresh metadata checkout is sparse, so it is not authority for missing local
research paths. The malformed benchmark field and the PCC failures are explicit
base-revision failures independent of those missing paths. Neither is waived.
Artifact-dependent CI and merging remain blocked during the reported S3 outage.
No additional record exclusion, claim rewrite, keyring change or owner approval
is inferred from the earlier F3 disposition.
