# Resume: crypto @ lane/prime31-matched-aux

## Goal
Execute the authorized Prime31 isogeny comparison: three rational order-1024 kernels on y^2=x^3-x, A+ control, quadratic/quartic/dense equations, and target degree-three native/transported bases. Continue PR4923.

## Lead / hypothesis
PKM section3.4 motivates auxiliary quotient fibers. H0,H1,H2 change the factor base; blocking changes representation; target-isogeny transport preserves exact relations up to signed columns. Compare complete same-target cold cost after admission; capped outcomes remain unknown.

## Spec
research/prime31_aux_matched_20261122/isogeny-choice contains continuation inputs and protocol under preparation. Source p2147483647,a847514846,b2087444724,n2147446831,G(625037715,2100627251),Q(984837385,1142774582). Degree1024; coset1; m3; query seed4188. Production F4 and counted negation/v6 plus prime_fast tuned1thread references. Current canonical freeze remains unsatisfied.

## State
Implementation commit 9da13e4e83cb15e544fff2c99ca236e045c3f494 is published in PR4923. The integrated current-source suite records 54 passing optimized contracts, zero failed or ignored, from copied/prehashed binaries. All 230 prepared cells pass structural validation. The post-publication source check matches 72 inventoried files present in that HEAD. Target transport, independent replay, matched plain controls and degree-four constraints are implemented. No campaign, counting sweep or performance comparison has run. Source and receipts are in this study. Claim PCC-20261123-2689b1 remains unverified. CLI push lacks credentials; the GitHub connector can publish commits.

## Next steps
Before experiments validate/freeze the canonical protocol, obtain live shared-budget/capacity receipts and authenticated IsoLab submission, and pass scripts/checkpoint.sh gate. Bind the worker/image and original absolute deadline, and register separate warmup/A-A admission. Then use isogeny-choice/run-choice.sh on the admitted backend and independently replay every result. Do not call the run subcommand directly as a bypass. The complete code, panel, inspected PDF and development evidence are already in PR4923; preserve the frozen evidence files.

## Runs in flight
None. Local development builds/tests are separately recorded; no experiment job submitted.

## Stop and success conditions
Prospective aggregate ceilingUSD5 includes build,idle,failedattempts,storage. Live remainingbudget unknown: no paidlaunch. No automaticretries. Proposed cells120s,4096MiB,concurrency1; absolute campaign deadline must bebound at admission. Preserve outcomes, relationinvariants and completecosts. Unit tests do not satisfy campaignexecution.

## Open questions
No IsoLab MCP, AWS CLI/identity, authenticated workflowdispatch, checkpoint uploader or current budgetreceipt. Canonical protocolaccess and full-library gate open. These are execution dependencies, not mathematical conclusions.
