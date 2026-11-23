# First authorized local pilot

The complete published source revision and native binary hash are pinned in
[runtime-plan.json](runtime-plan.json); raw and checkpoint hashes are retained
in the pre/post-call hash files. [pcc:unverified]

[Report](report.pdf) · [Native summary](summary.json) · [Figure](ratios.svg)

All 150 planned executions completed: 25 retained warmups and 125 measured
executions, with five paired rounds per oracle on p=67, seed=3, subgroup
order 300193. All scalar, input-prefix and predeclared local A/A checks pass.
The best graph variant is symmetric hybrid triple at wall/rho 25.61,
round-bootstrap 95% interval 24.16–27.62. These are producer-run local
observations on one curve; no large-subgroup parity is asserted.
[pcc:unverified]

The raw 344208-byte JSONL stays at
`/workspace/ic-local-runs/local-p67-all5-20261115-02/raw.jsonl`, outside Git
under the artifact-size policy and the user's local-custody authorization.
No S3 upload or standard EKS/L2 admission is asserted. Setup directory 01
failed before measurement; directory 02 contains the only measurement attempt.
Canonical experiment-export validation is unavailable because the accessible
canonical skill/schema files are encrypted VLT1 blobs. Local protocol, flow,
hashes, raw data and native analysis are retained without inventing a
canonical receipt. [pcc:unverified]

The report's SVG was regenerated to label every stratum. Numerical strata
are byte-identical after canonical JSON formatting to the original analyzer
output; the first run's copied analyzer and original SVG remain preserved
in its local checkpoint. [pcc:unverified]
