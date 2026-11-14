# Address-only fault-site diagnostic

Four new frozen local cells retain ordinary/saturation x exactLA1/2
under address-only instrumentation. The original0.6.5source is unchanged.
Earlier signed1<<31 remains uninstrumented and unrepaired; this is
not a proof of C-defined execution. [pcc:unverified]

| Exact linear algebra | Ordinary | Saturation |
|:--|:--|:--|
| 1 sparse/dense | exit0; exact expected basis passes | timeout124 at5s |
| 2 sparse | exit134; ASANSEGV la_ff_32.c543 | timeout124 at5s |

The fault is a READ in reduce_dense_row_by_known_pivots_sparse_17_bit.
Its trace reaches exact_sparse_reduced_echelon_form_ff_32 and core_f4.
This identifies a concrete fault site. It does not establish a complete
root cause, a source fix, generic ideal regularity or Crefinement.
The originalLA1basis passes the same frozen C++ known-answer checks.
[pcc:unverified]

Protocolversion2/hashdc99c976, sourcecommita549657b, precedes all4cells.
Flow4planned/4attempted/2completed/2not_completed(timeouts)/0excluded.
Prior combined-sanitizer8cells and earlier2failures remain retained.
All-file export validation passes. Packet PCC-20261110-abac7a is
not_covered, signedstandingunverified. Zero paidcompute; no timing,
statistical, independenthost or execution-proof claim. [pcc:unverified]

Raw28files are in evidence/runs.tar.gz with hash inventory and byte-equal
roundtrip checks. In a fresh checkout use
`python3 ../materialize-raw.py .` before verifying export files.
Existing unequal files are rejected. [pcc:unverified]

See [report](reports/proof-20261110/report.pdf), protocol.json,
provenance.json, export.json and evidence/pcc-verify.json.
The existing frozen Lean F4Run/saturation proofs are unchanged.
[pcc:unverified]
