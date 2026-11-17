# One-lookup coefficient relocation diagnostic

Four frozen local calls compare common-logged baseline and corrected
matrix lookup, ordinary/saturation, at exactLA2. The source variants
have identical logging and build flags; one known-pivot lookup differs.
[pcc:unverified]

| Variant | Ordinary | Saturation |
|:--|:--|:--|
| Logged basis lookup | exit134; ASAN READ fault | timeout124 at5s |
| Corrected matrix lookup | exit0; exact expected basis passes | timeout124 at5s |

Both ordinary logs record old[0,2,0] copied to new[2,3,4], with live basis3.
The old lookup reads basis3 after source2 was copied into matrix3.
The corrected lookup reads matrix3 and completes the fixed fixture.
Both saturation failures remain. This supports the concrete backport
on the tested ordinary input, not generic solver correctness.
[pcc:unverified]

Pinned0.6.5's16bit reducer and current upstream32bit reducer already
use the matrix table. This is a backport/source audit, not a new upstream
bug claim. Exact patches, all changed-file checks, current source hash
and read scope are in provenance.json and evidence/. Earlier signedshift
remains unrepaired; address-only diagnostics do not establish C-defined
execution. [pcc:unverified]

The [Lean project](../../msolve_reindex_lean_audit_20261110/README.md)
defines the three operational copy updates and proves correct matrix
lookup for every coefficient type/content, failed live basis lookup at3,
an in-range wrong-coefficient example at2 and a direct universal negation.
Native trace authenticity, pointer/allocation semantics and whole C/F4Run
refinement are explicitly outside that theorem. [pcc:unverified]

Protocolversion3/hash99beb128, committed atbf2ce5da before all4calls.
Flow:4planned,4attempted,2completed,2timeouts,0excluded. Prior relevant
F65537lineage14calls remain retained. Export passes all schema/protocol/
file checks. Packet PCC-20261110-4867bb is not_covered and unsigned.
No paidcompute, timing, statistics or independent-host claim.
[pcc:unverified]

All28raw files are archived in evidence/runs.tar.gz with hash inventory
and byte-equal roundtrip. In a fresh checkout run
`python3 ../materialize-raw.py .` before validating export files; existing
unequal files are rejected. See the joint native/Lean
[report](reports/proof-20261110/report.pdf), with editable proof.dot.
[pcc:unverified]
