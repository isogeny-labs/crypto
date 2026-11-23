# N83 merged-source full-fixture Linux validation on AWS Spot

Frozen before host launch on 2026-11-16 UTC. This is a release-library
**validation** for [PR #4295](https://github.com/isogeny-labs/crypto/pull/4295),
not a factor-base, relation, rank, descent, or runtime comparison. Preserve
the preceding [macOS merged-tree failure](../k600-budget-v3-contract-20261116/MERGE_VALIDATION.md)
as the reference; do not pool the two test counts.

## Question, identity, and acceptance

Run the exact merged `main` commit
`a91b8b66d0f1ffc175fdce984d1059144ce232f6` (whose second parent is
PR head `014dabd822849f9ad67aaa5d166855abe2cac575`) on a fully
materialized Linux ARM64 checkout. `git diff --name-only 014dabd82284
a91b8b66d0f1 -- src Cargo.toml Cargo.lock tests gpu tools` returned zero
paths, so these commits have the same library-validation source. Require the
public GitHub clone to resolve to that SHA, `core.sparseCheckout=false`, a
clean tracked worktree, tracked `gpu/fes/fes_solve.cpp` and
`research/ic_planner_validation_20261020/sessions/r24/records.jsonl`, and the
one archived targetwalk fixture below. The protocol and toolchain Dockerfile
are committed separately from that source; they make no library/Cargo/test
changes. If the PR's code paths advance, this run remains for the frozen SHA
and a further current-head validation is owed.

Archived ignored fixture:
`research/curve25519_f1_targetwalk_20261108/sessions/k20-s0/records.jsonl`,
2,293,097 bytes, SHA-256
`8970667e2f18587eeb08d033123dab029b94042ac5c07c870b08988f1ee5b738`.
Read it locally from the existing
`s3://setec-astronomy-artifacts/store/sha256/89/8970667e2f18587eeb08d033123dab029b94042ac5c07c870b08988f1ee5b738`
object through `ecc-challenges-new`; verify length/hash before transfer by
SCP and again on host and in the container. Do not put AWS credentials on
the instance or in the container. The full repository is publicly cloneable
from GitHub; no private source archive is transferred.

Success requires `cargo test --release --lib --locked -j 1` exit 0, zero
failed tests, exact source/fixture checks, Docker child and outer exit 0,
no OOM, and clean source after the run. A nonzero exit is retained as a
failed validation, not reclassified by a previous Linux run. Record all test
names and errors, including any `ChainedS3` assertion, deadline, or missing
file. The reference macOS run had 6,413 passed, 17 failed and 135 ignored at
this source. Its twelve Linux `/proc` failures are a host hypothesis, not a
guaranteed resolution of the whole gate.

## Bounded host and execution

Use the configured `ecc-challenges-new` IAM identity only to request a
one-time `c7g.2xlarge` **Spot** instance in `us-east-1f` on AMI
`ami-061bb79b9c35a0b94` (AL2023 ARM64), subnet
`subnet-0c343d40111cce085`, existing SSH key `meow34` and existing
source-IP-restricted security group `sg-0b3da730791ffdefc`. The exact
`RunInstances --dry-run` with Spot options must return `DryRunOperation`
before launch. Set Spot `MaxPrice=0.18` USD/hour, one-time/terminate behavior,
one instance, and a 150 GiB `gp3` root volume with
`DeleteOnTermination=true`. No instance role or additional ingress rule is
needed. Set instance-initiated shutdown behavior to `terminate` and pass the
committed [`user-data.sh`](user-data.sh), which schedules shutdown 225 minutes
after boot as a backstop. This is a bounded capacity decision, not an IC cost estimate. Record
the observed Spot price and actual instance lifecycle; terminate the instance
within four hours of launch, whether the validation passes, fails, times out,
or setup stops. Verify EC2 termination and root-volume deletion. Do not
start a second instance without a versioned amendment.

Install Docker on the fresh host and build this directory's `Dockerfile`,
whose base is the Linux ARM64 manifest digest for Rust 1.99.1. Record the
built image ID, `rustc --version`, Docker version, AMI, instance type, CPU,
memory and disk. Run one container with a 12 GiB hard memory limit, zero
swap, four CPUs, 512 PIDs, all capabilities dropped, no new privileges,
network access for locked Cargo dependencies, the full source mounted
read-only, and a fresh writeable Cargo target on the 150 GiB root volume.
Do not mount the Docker socket or cloud credentials. Check the cgroup values
and source/fixture inside the container before Cargo starts. Use GNU
`timeout --signal=TERM --kill-after=30s 7200` for the Docker child and a
four-hour instance ceiling. Require at least 20 GiB free on the instance
root before the child and retain disk samples. Do not run concurrent tests
or reuse a prior Cargo target. This is a cold validation build, not a
cold IC timing.

The attempt begins at `RunInstances` and ends after EC2 termination is
verified. Preserve launch and termination UTC times, instance/volume IDs,
dry-run result, exact commands, source and fixture hashes, full raw Cargo
log, child exit, Docker inspect including `OOMKilled`, outer receipt,
disk samples, and cleanup. Poll the named live process, retain partial
logs across SSH interruption when available, and never restart solely
because an observation timed out. If Spot capacity, dependency download,
host setup, source or fixture checks, Docker/cgroup admission, or disk
floor fails, record the stop and do not call it a test verdict. An
interruption is censored at its last retained output.

## Claim boundary and next action

This run can clear or refine the required library validation gate for its
exact source. It cannot select a factor base or establish an IC/rho ratio.
The K600 H2 host-build/staging packet, cumulative budget allowance, natural
relations, rank, individual logarithm, matched rho and finite-grid
selection remain separate. After this run, retain a write-once result,
source-linked diagram and inspected PDF beside the records; update the PR
with the exact outcome and keep required CI/review gates in force.
