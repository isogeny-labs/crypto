# Frozen primary-N83 Linux XOR natural-query screening panel

This is one bounded solver-access experiment on the exact primary `a=0`
Koblitz GF(2^83) public first-pattern target. It asks whether hash versus
Gray enumeration and 5+1 versus 4+2 splitting change the cost of reaching a
complete natural SAT model on the four existing XCNFs. It does not treat a
fixed-conflict wall comparison as relation yield or as total IC runtime.
The [machine protocol](protocol.json) and this runner must be committed before
the hosted process starts; its GitHub Actions trigger is a branch push of the
workflow, protocol or runner.

The protocol pins preflight run `40177842203` attempt 2, exact artifact
`12593635829` and ZIP SHA-256, both executable SHA-256 hashes, pinned worker
source commit, all 11 S3 input identities, Ubuntu 24.04 ARM runner class,
package version and complete CryptoMiniSat flags. A fresh hosted job downloads
the exact binaries from the provider ZIP, repeats the two-variable wide-XOR
SAT smoke, fetches and independently replays all 11 inputs from S3, then
checks the binary bytes and every input again immediately before timing.
The job is one ephemeral hosted VM with sequential one-thread processes;
host and cgroup observations are retained. It is a hosted screening tier,
not a dedicated bare-metal cost reference.

Each arm has one million conflicts, a 300-second solver cap, 330-second
external watchdog, and 8 GiB process address-space limit. The six-arm order
is hash 5+1 twice adjacently (A/A repeat), Gray 5+1, hash 4+2, Gray 4+2,
and hash 5+1 once more (drift control). All arms use the same solver flags
and first-pattern public target. The total watchdog ceiling is 33 minutes;
the hosted job timeout is 45 minutes including setup and artifact upload.
No arm is silently rerun after a failure. A rerun of the GitHub workflow is a
new attempt with its own artifact and host identity.

The runner records each raw stdout/stderr stream, SHA-256, exit status,
conflicts, process wall, user and system CPU, peak RSS, watchdog and OOM
observation. `INDETERMINATE`, time/conflict cap, OOM, watchdog and malformed
streams are censored. A reported SAT model stops the panel for exact worker
verification against the downloaded archives, export and original group
equation. A group-rejected model requires a later blocking-clause search;
it is not a useful partial. A single pattern-level UNSAT result applies only
to that formula, not the factor base. The provider artifact retains all
partial outputs on failure; independent replay is required before any
measurement is used.

The primary objective still includes factor-base construction and import,
relation failures and closure, rank and linear algebra, unseen-target descent,
artifact I/O, and matched same-target rho with one-target cold accounting.
`selected_best_total_runtime` therefore remains `null` after this panel
unless those separate gates are completed. The input objects stay under
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/linux-xor-panel-20261025/inputs/`.
