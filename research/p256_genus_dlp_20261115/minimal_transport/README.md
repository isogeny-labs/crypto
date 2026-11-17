# P-256 transport: the Eisenstein coefficient lattice

This round adds kernel-checked integer bounds for the existing genus-3
finite eigenline and a source implementation of norm-minimal transport.
The new map candidate is not numerically qualified; no net speedup is claimed.
PCC-20261115-9ff011 remains unsigned. [pcc:unverified]

- [Report](REPORT.md), [PDF](REPORT.pdf), [diagram](transport.svg).
- [Proof map and inheritance](PROOF_MAP.md), [Lean instance](lean/P256MinimalTransport.lean).
- [Merged shared generic laws](https://github.com/aburan28/crypto-lean/pull/12).
- [Native candidate](native/src/lib.rs), [compile-only evidence](checks/build-001.txt).
- [Shared proof gate](checks/shared-proof-audit.txt), [P-256 proof gate](checks/instance-proof-summary.txt).
- [Supplemental identities](checks/identities.json), [protocol](PROTOCOL.md),
  [saved resume](RESUME.md), [execution instructions](RUNPOD.md).

Shorter coefficient descriptions preserve the same finite subgroup. They do
not establish cheaper map execution or a lower DLP cost. [pcc:unverified]
