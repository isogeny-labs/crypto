# Resume: RR Dickson degree-capped preprocessing
## Goal
Retain RR lower solving degree while reducing matrix growth enough to beat the strongest equal-output solver.
## Lead / hypothesis
November 7–8 P1 follow-ups show lower RR degree but wider matrices than F at m3. Constant-pivot elimination capped at input degree two may remove auxiliary variables without losing solutions. Solving degree and cost remain untested.
## Spec
research/rr_dickson_matrix_reduction_20261110/PROTOCOL.md (published; execution not admitted). Baseline RR/F at T24/T31 m3 t3–5, identical inputs and output normalization; first stage correctness and development only.
## State
Base source inspected at 2e8afd46dc898cdbfc7c469c698b8f928ed4982b. Opt-in native preprocessor, digest-checked arbitrary-solution lift, four native tests, upstream selftest, source-linked report/PDF and read-only review are complete as producer checks (not signed verification). No scientific runs or new solver measurements. See checks/ and the protocol. KB unavailable; explicit repository and primary-paper fallback used. No AWS identity configured; GitHub authenticated. Shared budget headroom unknown. Filled task resume gate failed for absent checkpoint upload transport; saved its contents here and restored the unrelated tracked main .resume file.
## Next steps
Publish/review the implementation PR. Restore existing checkpoint transport and current budget/pool readback before any experiment launch. Build the complete point-output normalizer and qualify against independent enumeration before freezing claim inputs and a source-built image. Read study protocol and README for commands. No expected performance outcome has been obtained.
## Runs in flight
None. No experiment submitted. Rust toolchain installed for local development checks only.
## Stop and success conditions
No launch without S3 checkpoint, known budget headroom, published source and EKS admission. Success requires equal-output validated solving cost and measured lower matrix width with retained solving degree against the strongest baseline. An implementation or plan does not establish success.

## November 12 continuation

PR https://github.com/isogeny-labs/crypto/pull/3835 remains open. A merge of current
main (55713d200) was attempted after Copilot did not supply a fix; the append-only
PCC ledger conflicted, so the merge was aborted without choosing either history.
The native `extract` command is now implemented and qualified on bounded m3
common-chart fixtures (nine native tests; claim PCC-20261112-699e09 unverified).
See EXTRACTION-20261112.md for interface and exact limits. Full rational solver
assignment enumeration and excluded-chart coverage remain open. No F4 comparison
or cloud job ran. Checkpoint transport/live budget and pool admission are still
absent; no identity or spending permission was added. Next: evidence-preserving
ledger reconciliation, complete solver-output collection, then the admitted
matched m3 panel. The scientific objective remains unresolved.
