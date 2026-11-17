#set page(paper: "us-letter", margin: 0.72in)
#set text(font: "Helvetica", size: 9pt)
#set par(leading: 0.55em)
= K13 source-plan and counter audit

*Read-only scope.* This audit launched no scientific trial and makes no new
performance ranking. Reproducer: `audit.py`; machine result: `PLAN-AUDIT.json`.

== Exact plan identity

The local Rust 1.99.1 release binary (SHA-256
`7b062c58a5a1bfa0309d68732f57cf36f8d2394780bc3e111ee7b64b2e3092ed`)
expanded both committed specs. All eight K13 workloads and all six admission
arm objects exactly match the frozen panel. All 288 admission execution cells,
including their algorithm seeds, occur among the panel's 560 cells. The
admission planner warns that five measured rounds under `alternate` have
unequal order parity; the probe is an admission screen.

== Archived accounting boundary

The SHA-256 bound JSONL object
`dc16c97ae21a250cc1d47237231d9642960c100dc3fdb68f4dd210af171bff44`
contains 48 archived L0 records. All report internally complete `bitop/v1`
blocks with phase sums equal to totals; 24 native F4/F5/F6 rows carry their
solver completeness flag. This checks the producer's recorded counts. The
required instruction audit is absent, so the model cannot yet rank the arms.
The two-target L0 session does not qualify a timing advantage. Its scalar
answers were separately audited in `research/rootfind_k13_l0_independent_audit_20261116`.

#figure(image("flow.svg", width: 100%), caption: [Completed read-only checks and remaining K13 admission gates.])

== Next checks

Run the native launcher validator against v3 without submission. When cost and
checkpoint gates admit execution, run L2 admission and independent replay;
then the frozen ten-arm panel. Run an instruction audit on matching records
before modeled-work ranking. Source and details: `REPORT.md`.
