# Resume: cubic-extension IC graph elimination

## Goal
Integrate graph relation elimination into the existing Rust IC code in isogeny-labs/crypto and measure it against the other IC paths, focusing on algebraic relation finders over cubic extension fields.

## Lead / hypothesis
Reuse pencil_symbolic_graph::SymbolicGraph with Gaudry's subfield-coordinate S3/S4 relation oracles. Compare the forest with protected-target pivot chaining. Existing graph and Gaudry sources plus the 2005 double-large-prime paper were reviewed; research-kb executable is unavailable. No novelty or performance result is claimed.

## Spec
research/extension_ic_graph_20261115/protocol.json freezes primes 7,13,19,31, seeds 3,7,11,23, three oracles, five interleaved A/A/candidate rounds, 10000 residuals, and separate p=43 holdouts. Budget USD 5, CPU 4, RAM 8 GiB, deadline 900 seconds, zero automatic retries. Legacy multiplication-equivalent counters are diagnostics, not a complete native-cost ranking.

## State
Branch feat/extension-ic-graph-elimination, base 9e95f83262997336c1785ced4170b1edf8bd0d21. Native adapter, protected comparison API, paired driver, protocol and implementation report are written. Local development compilation/test command is running; no benchmark has launched. Original pivot boundary treats the target column as auxiliary; the new comparison protects it for both arms, keeping that correction separate from graph replacement. Current origin/main after fetch is d18e077b3687a233706374b02aca7203e3be9631; its session-log and pcc ledger additions must be preserved before publication.

## Next steps
1. Inspect /tmp/extension-ic-tests.log from cargo test --release --lib --no-default-features gaudry_cubic:: -- --test-threads=2.
2. Fix any new failures, run touched example tests and required full library suite, update research README and report with actual status.
3. Review diff, reconcile main without dropping other log/ledger entries, commit, push and open a regular PR; enable repository-default auto-merge only under its gates.
4. Obtain existing authenticated S3 checkpoint transport, pass scripts/checkpoint.sh gate, then publish the exact source/image and admit the EKS comparison. No alternative unapproved benchmark route has been used.

## Runs in flight
Local development only: cargo PID 11996 / compiler PID 12004 as of 07:10 UTC. Output /tmp/extension-ic-tests.log. Query process state before resuming; no EKS job, CodeBuild run, paid experiment or benchmark record exists. Source dependency fixtures were restored at the pinned revision. Canceled overly broad git archive/sparse prefetches after exact native files were retrieved; use /usr/bin/git if the other installed git diff crashes.

## Stop and success conditions
All admitted baseline/candidate outputs and target prefixes must agree; all completed scalars must replay against their target. Preserve failed/capped rows. Do not run experiments before checkpoint/publication/admission gates; do not claim measured improvement from this implementation note. Missing checkpoint access or a tested EKS command image remains an execution dependency; incomplete native operation emission remains a ranking limitation. User has been asked which existing runner/profile to use, without requesting secret text.
