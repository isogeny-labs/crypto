# Resume: cubic-extension IC graph elimination

## Goal
Integrate graph relation elimination into the existing Rust IC code in isogeny-labs/crypto and measure it against the other IC paths, focusing on algebraic relation finders over cubic extension fields.

## Lead / hypothesis
Reuse pencil_symbolic_graph::SymbolicGraph with Gaudry's subfield-coordinate S3/S4 relation oracles. Compare the forest with protected-target pivot chaining. Existing graph and Gaudry sources plus the 2005 double-large-prime paper were reviewed; research-kb executable is unavailable. No novelty or performance result is claimed.

## Spec
research/extension_ic_graph_20261115/protocol.json freezes primes 7,13,19,31, seeds 3,7,11,23, three oracles, five interleaved A/A/candidate rounds, 10000 residuals, and separate p=43 holdouts. Budget USD 5, CPU 4, RAM 8 GiB, deadline 900 seconds, zero automatic retries. Legacy multiplication-equivalent counters are diagnostics, not a complete native-cost ranking.

## State
Branch feat/extension-ic-graph-elimination. Implementation commit 0214fed7, merged-head development check e03fc7ab, initial base 9e95f83262997336c1785ced4170b1edf8bd0d21. Native adapter, protected comparison API, paired driver, protocol and two-page implementation report are written. Final component release tests report 24 passed; current-head example release type check passed. These are producer-run development checks [pcc:unverified], not admitted measurements. The pre-fingerprint full library suite was interrupted after unrelated failures and long PKM cases; it is not a passing full-suite result. Logs and component hashes are under research/extension_ic_graph_20261115/validation/. Original pivot boundary treats the target column as auxiliary; the new comparison protects it for both arms, keeping that correction separate from graph replacement. No benchmark has launched. Latest fetched main is f7b24157b441cce8410d2a3626415e6775ebd155; preserve other sessions' log/ledger entries during publication.

## Next steps
1. Commit final validation/report/log updates and reconcile main without dropping other sessions' entries. Publish a regular PR with the full-suite and measurement gaps stated. Global benchrec gate currently reports inherited open records and missing sparse historical evidence; no record/progress edits or bypass were used.
2. Obtain an existing authenticated S3 checkpoint transport and EKS runner location from the user. Run scripts/checkpoint.sh gate; do not launch unless it succeeds. Missing profile/token is recorded in validation/checkpoint-gate.log.
3. Pin the published source revision in the experiment request, build a supported command image, pass check-experiment-git and pool admission, and retain receipts. Apply the EKS warmup/L2/A/A contract; do not substitute the unrelated fresh-owner fixture image.
4. Execute the two frozen commands in research/extension_ic_graph_20261115/README.md under the USD 5/900-second/zero-retry limit, wrapping them with the required bench-record lifecycle. Retain all capped/failed rows and upload raw evidence. Pricing is incomplete, so no full native-cost ranking follows from legacy multiplication-equivalent counters.
5. Restore exact missing historical fixtures for parent validation or use the maintained full lab checkout; complete cargo test --release --lib there without disabling tests. Recheck changed components if their bytes differ from validation/component-source.sha256. Review applicable PR checks before merge.

## Runs in flight
None as of 07:56 UTC. Development checks finished; the earlier full-library process was stopped with SIGTERM. No EKS job, CodeBuild run, paid experiment or new benchmark record exists. Full interrupted output is /tmp/extension-ic-full-library-tests.log (255078 bytes; hash in validation/full-library-interrupted.txt); durable upload is pending transport access. Source dependency fixtures were restored at the pinned revision. Avoid broad sparse reapplication/archive hydration in this partial clone. Use /usr/bin/git; the other installed Git intermittently crashes on diff.

## Stop and success conditions
All admitted baseline/candidate outputs and target prefixes must agree; all completed scalars must replay against their target. Preserve failed/capped rows. Do not run experiments before checkpoint/publication/admission gates; do not claim measured improvement from this implementation note. Missing checkpoint access or a tested EKS command image remains an execution dependency; incomplete native operation emission remains a ranking limitation. User has been asked which existing runner/profile to use, without requesting secret text.
