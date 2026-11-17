# Per-recovery interval context

See [DERIVATION.md](DERIVATION.md) for correctness, explicit cap/storage accounting, controls and replay. [report.pdf](report.pdf) includes the editable [flow.svg](flow.svg). Source is generated from the pinned predecessor by generate.py; context-checks.rs supplies independent ownership and equality controls. Evidence is unsigned producer evidence. [pcc:unverified]

Claim: PCC-20261107-5c8553 (unsigned); validation receipt pins 52 transitive source inputs.
