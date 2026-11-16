# AWS resume: complete primary N83 D18 factor base

Resume from PR [#2557](https://github.com/aburan28/crypto/pull/2557), branch
`codex/n83-aws-handoff-20261025`, stacked on PR #2551. The published
construction and custody result is commit
`4e9faa76506cf6a19c48b9c0ce7a665121eb91d3`; the exporter and protocol
were frozen at `5bac5f1d9221b908f743520aa9039dd48d175f0e`. Preserve those
identities when auditing this archive. The curve is
`icv1-f2m83-tm6151469093347-debefd74`, Koblitz `a=0`, prime subgroup
order `2417851639230796216685689`. Do not combine it with the separate
`a=1` diagnostic subgroup or treat its performance as primary-curve evidence.

The complete direct-subgroup dimension-18 archive has policy
`direct_subgroup_x_d18_full`, **32,643 signed-Frobenius columns** and
**5,418,738 unique labeled point rows**. Its immutable S3 object is:

```text
s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/d18-full-direct-subgroup-v1/a0/objects/c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz
```

The hash-bound S3 manifest is the sibling `manifest.json`. The compressed
object has 160,747,529 bytes, SHA-256
`eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b`,
and BLAKE3 `c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f`.
The S3 manifest SHA-256 is
`1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616`.
The independently reconstructed point-set BLAKE3 is
`31aa8e1c1340f4db4898144af59f6f34c87fa63e0d88b175b854f58ce9008a7d`.
The [result](RESULT.md), [generic replay](replay.json),
[fresh-GET replay](fresh-replay.json), [provider receipts](s3-independent-audit.json),
and [`verify_s3.py`](verify_s3.py) give the complete existing audit. All raw
provider metadata and five rejection controls are retained here.

To independently fetch and replay on an AWS Linux host, first check out the
exact PR head and use a fresh output directory. The commands below assume
this directory is the current working directory and that AWS credentials
can read the user's bucket; choose any new path for `base_dir`:

```sh
base_dir="$(mktemp -d)"
mkdir -p "$base_dir/objects"
cp manifest.json "$base_dir/manifest.json"
prefix='factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/d18-full-direct-subgroup-v1/a0'
object='c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz'
aws s3api get-object --bucket crypto-autoresearcher --key "$prefix/manifest.json" \
  --checksum-mode ENABLED "$base_dir/s3-manifest.json" > "$base_dir/manifest-get-metadata.json"
aws s3api get-object --bucket crypto-autoresearcher --key "$prefix/objects/$object" \
  --checksum-mode ENABLED "$base_dir/objects/$object" > "$base_dir/object-get-metadata.json"
cmp s3-manifest.json "$base_dir/s3-manifest.json"
printf '%s  %s\n' \
  1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616 \
  "$base_dir/s3-manifest.json" | sha256sum -c -
printf '%s  %s\n' \
  eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b \
  "$base_dir/objects/$object" | sha256sum -c -
cargo run --release --manifest-path ../../../../Cargo.toml \
  --example n83_d18_full_orbit_export -- replay "$base_dir"
```

The generic replay requires the committed source CSV and census summary in
this checkout. It independently checks every point, subgroup label,
Frobenius phase, sign, orbit closure and global point-set hash. Check the
fresh GET response's `ChecksumSHA256`, `ServerSideEncryption`, `ContentLength`
and metadata against the saved receipts. Keep the GET metadata, fresh replay,
exit status and host/resource observations with the new run. If any digest or
identity differs, stop before measuring a search.

## Solver admission for this larger base

The opt-in `primary-full-d18-adapter-check` command imports this exact
archive into the generic signed-Frobenius base and independently reconstructs
its orbit maps and labels. It requires the committed producer manifest,
generic replay and S3 audit, exact compressed SHA-256/BLAKE3 and point-set
hash. Run it only after the fresh S3 fetch above:

```sh
cargo run --release --manifest-path ../../../../Cargo.toml \
  --example koblitz_n83_factor_base_export -- \
  primary-full-d18-adapter-check "$base_dir" "$base_dir/import.json"
```

Require a `PASS` receipt before the [frozen first access gate](NATURAL_PROTOCOL.md).
The [separate local positive-control diagnostic](l0-positive-20261025/RESULT.md)
re-imported the complete base and built the pinned circuit, then reached its
300-second shared-host L0 cap before a solver answer. Its audited censoring
does not replace the Linux control or justify a natural-yield estimate.
The [opt-in shared-root comparison](shared-root-20261025/RESULT.md) reduced
the full-base model by exactly 8,617,488 clauses, but its own planted
control also reached the same 300-second cap without a solver answer.
The [selector-and-phase pinned planted control](root-pinned-20261025/RESULT.md)
subsequently passed with independent original-field group and row replay.
It supplies a circuit correctness gate, not a free-selector or natural-yield
result. Keep the Linux control and natural-query gates separate.
The [two choice-isolation controls](choice-pins-20261025/RESULT.md) use the
same archive and checked planted row: phase zero pinned with selectors free
searched for 0.239 seconds, and selectors pinned with phases free searched
for 16.589 seconds. Their independent audit passed. Both are constructed L0
controls; the free-choice Linux and natural-query gates remain open. The
opt-in modes are `primary-full-d18-m4-l0-shared-phase-pinned` and
`primary-full-d18-m4-l0-shared-selector-pinned`. Their source commit is
`d775b18dc427188e0279f4189c3c63b570f780fc` and binary SHA-256 is
`0826acbe6a3e55fc747be480453cd1bca9c053adb584ce7e55548e13985644e4`.
The [phase-first free-choice constructed control](phase-first-20261025/RESULT.md)
then kept every selector and phase admissible while prioritizing phase wires
with initial zero polarity. Its corrected L0 supervisor run exited zero and
returned the independently replayed planted row in 1.584 s of SAT search,
after a 77.410 s full import. The first retained worker also passed but its
macOS time wrapper failed; both raw runs and the correction are committed.
This is an opt-in `primary-full-d18-m4-l0-shared-phase-first` mode from source
`57c20963924ab0097111b08fc6cb157400e87e53`. Freeze and validate a
Linux hard-cgroup natural extension before interpreting any yield.

The [same-binary L0 public natural screen](phase-first-natural-20261025/RESULT.md)
is now retained from clean source commit
`43a422ce66ebce82b48af036d15dcbef8a5ac785`. Its constructed gate
passed, including independent original-field row replay. The one public
natural query imported and modeled the full D18 base, then returned
`UNKNOWN_wall_cap` at the frozen 300-second whole-process cap. This is
censored, not a zero-yield observation. See the raw arm directories and
[`audit.json`](phase-first-natural-20261025/audit.json); the exact binary
SHA-256 is `9605c08a4aeeb61e6ce3196cb74d5a87ddea9d13f9f1d79a0a4e643d85ac5152`.
To reproduce, build this frozen source commit, freshly fetch and verify the
same object and manifest, then run the positive and natural modes serially
with fresh outputs under the [protocol](PHASE_FIRST_NATURAL_PROTOCOL.md).

The [extended local screen](local-extended-natural-20261025/RESULT.md) used
clean source commit `a79536d6605216d258771dffba9cee94072363a5` and
release binary SHA-256
`ee2fd534ed952f512577440ea6b2adb0c0fab8a35a25ea0da6d9991a83a6e645`.
Its newly fetched S3 object passed provider checksums and importer replay;
the same-binary constructed arm passed independent original-field relation,
rank-one and tamper replay. One public natural arm then reached its
1,200-second whole-process cap after a 79.071-second import and 0.294-second
model build. The solver did not return, so its conflicts and useful natural
yield remain unknown; the retained post-model window was 1,120.361 seconds.
The 1,210-second supervisor did not fire. The raw files, negative cap check,
compressed validation logs and SHA-256 inventory are committed there. This
L0 Mac run had no hard memory cgroup and does not replace the pending Linux
capacity gate or support a total-runtime selection.

The 17,757,116-byte minimal source archive for earlier D18 import source
`54cbef1a6ce834a77632c1867acc801a31268ca2` is now at
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/d18-full-direct-subgroup-v1/a0/runs/first-m4-20261025/source/n83-d18-source-minimal-54cbef1a6ce.tar.gz`.
Its SHA-256 is `e679260bd849fed12f3588a57643277547b969399ab576a4883668821dabb3e4`;
an immutable conditional put and fresh GET passed the provider and archive
integrity checks in the [custody receipt](phase-first-natural-20261025/source-archive-custody.json).
This archive predates the phase-first search code. Use the PR branch and the
frozen source commit above for that experiment.

The next [Linux phase-first protocol](linux-phase-first-20261025/PROTOCOL.md)
extends the same free-choice shared-root circuit to an exact 8 GiB,
zero-swap, one-CPU cgroup with a 1,200-s internal and 1,210-s independent
outer cap. The opt-in command is
`primary-full-d18-m4-linux-shared-phase-first positive|natural BASE_DIR FIXTURE_DIR NEW_OUTPUT_JSON`.
The branch-specific
[`n83-d18-phase-first-linux.yml`](../../../../.github/workflows/n83-d18-phase-first-linux.yml)
checks out the pushed source, freshly fetches and checks the full S3 object,
builds one Linux binary, audits the constructed gate independently, and runs
one public natural query only if that gate passes. It uploads source, image,
S3, cgroup, stage and audit receipts on every terminal path. Its run status
must be checked live; a queued dispatch or a completed constructed control
does not establish natural yield or a runtime winner.
[Run 40220890684](https://github.com/aburan28/crypto/actions/runs/40220890684)
started from exact source/protocol commit
`2b7b04e1d613debe9d71c6403afba09ac3d9b289` and [stopped at the S3
access gate](linux-phase-first-20261025/attempt-1/RESULT.md). GitHub's AWS
principal has no `GetObject` allow for the D18 manifest. The retained provider
artifact proves that no binary build, cgroup preflight or worker search ran.
An [exact two-object read policy](linux-phase-first-20261025/attempt-1/IAM_READ_PROPOSAL.md)
is prepared but **not applied**. Explicit authorization for that persistent
cross-service read is required before retrying; subsequent documentation
commits are not the attempted run's source.

That gate adds an opt-in `primary-full-d18-m4 positive|natural` worker with
hard resource checks, durable phase/shape records, SAT-model verification,
group lift, row replay and rank-one control. Its natural result requires a
separate replay before interpretation. The standard panel policy whitelist
and the previous 16,627-class D18 worker remain separate. PR #2490 has a
shared-root-bit encoding for the half-size base; test a reviewed integration
against this full-base identity before using its capacity or timing claims.

For the existing per-representative signed-Frobenius circuit at `n=83`,
`m=4`, `r=32,643`, the exact **preflight bound** from
`chain_s3_max_variables` plus `frobenius_orbit_domain_size` is 329,726 SAT
variables and 11,238,656 *domain* clauses. The earlier 300,000-variable /
8,000,000-domain-clause cap rejects it before construction. A 400,000-variable
and 12,000,000-domain-clause cap admits the preflight calculation; it does
not establish that a model fits memory or searches usefully. Its full CNF
also contains field-arithmetic clauses. Measure actual build wall, RSS,
total clauses, XOR rows, propagation, and search under a hard memory and wall
cap before allocating a natural-query panel. An optimized shared-root-bit
encoding needs its own exact size and equivalence controls.

The [natural protocol](NATURAL_PROTOCOL.md) freezes the first positive and
natural arms before execution. Use the same public primary-curve fixture as
the earlier U/D panel and retain distinct base identities. Run known-relation
positive and changed-target negative controls through the exact importer and
model. Then run same-target natural
queries with independent group and rank replay. Record import, construction,
all failed/capped searches, useful-rank growth, sparse linear algebra,
unseen-target descent and matched same-target rho as separate cold charges.
A timeout or memory cap is censored. The earlier half-size D18 m4/m5 natural
panels and this PR's XOR panels ended at their stated bounds without a
checked natural relation; none supplies a yield estimate for this larger
base. `selected_best_total_runtime` remains `null` until the full comparison
is measured and independently replayed.

## Ordered D18 and native XOR cadence continuation

The [ordered first-pair result](ordered-first-pair-20261025/RESULT.md) used
the complete D18 base at committed source `c2d39e4948a3`. Its constructed
four-point relation independently replayed, while the first public natural
query reached its 2,400-second internal wall cap. The raw initial supervisor
mislabel and corrected independent censor are retained. This arm supplies no
measured natural yield.

The opt-in [cadence-32 protocol](xor-cadence-20261025/PROTOCOL.md) changes
the native XOR elimination schedule while retaining the ordered formula and
base. The 120-case exhaustive mixed CNF/XOR differential, full release
library suite, touched example, and N83/boundary Python suites passed. A
source-bound attempt at `00b3d3929e49` fetched the exact D18 S3 object and
wrapper again. The 300-second constructed worker reached its internal cap
before `import_checked` on a saturated shared Mac: 303.192 seconds of
supervisor wall, 31.21 seconds of process CPU, one-minute host load 100.8
before and 173.8 after. Natural search was skipped. The
[raw receipt and independent audit](xor-cadence-20261025/RESULT.md) classify
`CAPACITY_CENSORED_BEFORE_MODEL` and reject an altered cap. Commit
`7544cd49b84e` adds the audit and future cap classification; build a fresh
release binary from the chosen clean PR commit before repeating. Do not
compare cadence wall time from that capacity-censored attempt.

On an authorized, resource-qualified host, fetch the immutable
`d18-full-direct-subgroup-v1/a0` object and wrapper using the exact keys and
full-object checksum checks above. The local input layout needs
`objects/c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz`,
`s3-manifest.json`, the bound producer `manifest.json`, and the two
`*-get-metadata.json` provider responses. The earlier Python supervisor and
auditor are historical evidence paths. New comparisons use the native
[frozen continuation](native-cadence-replay-20261026/PROTOCOL.md). From a
clean source checkout, build both
`cargo build --release --example koblitz_n83_factor_base_export --example n83_d18_native_relation_replay`.
The [retained assignment amendment](native-cadence-replay-20261026/ASSIGNMENT_CERTIFICATE_AMENDMENT.md)
requires `result.model.bin` beside a cadence-32 `result.json` SAT receipt and a separate
native CNF/XOR, field and group replay before promoting that result.
Record the source revision and SHA-256 of each release binary before timing.
The producer `manifest.json` has SHA-256
`1d3650eabc4662d8f2597498d7ac92b7cd0a4134ac5f8155aaa49c7af3407dce`;
the distinct S3 custody `s3-manifest.json` has SHA-256
`1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616`.

After the addendum's host preflight, invoke the worker subcommand
`primary-full-d18-m4-l0-ordered-xor-cadence32 positive BASE FIXTURE RESULT`.
Keep stdout, stderr, `/usr/bin/time -l`, the exit code in `RESULT.exit.txt`,
and all phase/result/cap files. Wrap the worker's 300-second internal cap
with `gtimeout --kill-after=5s 310s`. Run
`n83_d18_native_relation_replay BASE RESULT WORKER_BINARY FIXTURE RECEIPT`
against a terminal result, or pass `RESULT.cap.json` for an internal cap.
Only a passing constructed relation and native replay authorize one
`natural` call with the worker's 2,400-second internal and 2,410-second outer
caps. A natural relation still needs useful-rank, factor-base log,
unseen-target descent, fully charged cold runtime and same-target rho gates.

At the 2026-10-25 checkpoint, the hosted GitHub AWS principal stopped at
`AccessDenied` for the full-D18 manifest, and its [exact two-object read proposal](linux-phase-first-20261025/attempt-1/IAM_READ_PROPOSAL.md)
was unapplied. Read-only inspection then found the existing
`n83-online-screen-ssm-20261019` profile has only its separate artifact put,
while `n83-xor-panel-20261025` reads only the earlier XOR-panel input prefix.
Neither profile granted this D18 object read. The local SSH endpoints named
`awsspot` and `runpod` were unavailable at that inspection.

## 2026-10-26 native continuation status

The user subsequently authorized the exact two-object temporary policy
`N83D18ExactInputRead20261025` on IAM user
`rho-gh-snapshot-readonly-20261024`. The [live readback receipt](../../../../problems/evidence/n83-d18-exact-read-20261026.json)
matches the [reviewed proposal](linux-phase-first-20261025/attempt-1/exact-read-policy-proposal.json)
after canonical JSON sorting. Its sole statement permits `s3:GetObject` on
the manifest and compressed object named above. Confirm the live grant before
using that identity, and remove it with a verified absent-policy readback
after the hosted attempt ends or is cancelled. Other AWS identities need
their own read authorization; this grant does not make the spot or RunPod SSH
endpoints available.

The current source handoff is [PR #2649](https://github.com/isogeny-labs/crypto/pull/2649)
and the [frozen native protocol](native-cadence-replay-20261026/PROTOCOL.md),
including its [retained-assignment amendment](native-cadence-replay-20261026/ASSIGNMENT_CERTIFICATE_AMENDMENT.md).
The [second source-bound preflight](native-cadence-replay-20261026/attempt-2/REPORT.md)
used source `45ce91d503ee0d0acc57e3d8d5cb49108c90ded6`, worker SHA-256
`11220d8b838bf5cac2985dbcba4ea96e69a64b70dc54044d180cf6a3e8970e49`,
and separate verifier SHA-256
`e89578703888568d67f79e5b2645e47b911a30a9d7a8a3d57e795be997296b71`.
Fresh checksum-enabled S3 GETs matched both full-object hashes. Six selected
one-minute local loads were 26.44–66.14, exceeding the frozen maximum 24;
swapouts rose by 390,448. The constructed worker and natural arm were not
launched. Use a host that passes the frozen load, swap and scratch gates,
then run the positive arm and its independent assignment/group replay before
the one public natural arm. Preserve every cap, timeout or infrastructure
failure with its exact source and input identities.

GitHub Actions on PR #2649 has stopped before runner assignment with an
account billing or spending-limit annotation. The default AWS identity
returned `ParameterNotFound` for `/ephemeral/config` in `us-east-1` and
`us-west-2`. A configured second profile reached the deployed runner in
account `104186811093`, but a direct original-manifest HEAD returned `403`.
The [third preflight and exact two-sided access proposal](native-cadence-replay-20261026/attempt-3/REPORT.md)
preserve that observation; the proposed bucket and runner-role policies
remain unapplied. The documented `ephemeral` runner defaults to Spot capacity,
while the current solver has no application checkpoint. This PR adds an explicit
`ephemeral run --on-demand` path to both CLIs. Local market selection and Rust
CLI unit tests passed. A live no-source `t3.small` On-Demand smoke was denied
at `RunInstances` by the account's `SpotOnlyEC2Launch` v2 identity policy;
the [attempt-3 receipt](native-cadence-replay-20261026/attempt-3/REPORT.md)
records the empty tagged instance readback. The exact S3 read grants alone
cannot lift that launch denial. No hosted native full-D18 positive/natural
execution has been started from this handoff. The
[problem record](../../../../problems/n83-mixed-orbit-factor-base-selection.md)
tracks these gates and the downstream total-runtime selection obligations.

A subsequent [local attempt 4](native-cadence-replay-20261026/attempt-4/REPORT.md)
passed the exact fresh S3-input and local capacity gates, then exited in
1.05 seconds before result generation because the worker binary's compile-time
`CARGO_MANIFEST_DIR` still named the dirty original checkout. The correction
is a fresh release build in a clean checkout of frozen source `45ce91d5`,
with new worker/verifier hashes recorded before the next positive arm.

The resulting [attempt-5 constructed control](native-cadence-replay-20261026/attempt-5/REPORT.md)
used a new clean-source worker, fresh provider-checked D18 GET, and a launch
load of 21.65. It produced constructed SAT after 85.564 seconds of import,
0.153 seconds of model construction, and 0.352 seconds of SAT search; its
retained model passed separate native group/row/assignment replay. Its
`/usr/bin/time -l` wrapper exited 1 on a denied macOS `kern.clockrate`
query, independently reproduced with `/usr/bin/true`, so the natural arm
was held. The [exit-policy amendment](native-cadence-replay-20261026/EXIT_POLICY_AMENDMENT.md)
uses `time -p` with an explicit terminal exit-0 gate for a fresh rerun.
Attempt 6 has a newly verified full-object GET, but its current host load
remains above the frozen launch maximum; no attempt-6 worker was launched
at this handoff update. The verifier's terminal path now explicitly requires
exit 0; its five focused tests pass, and it rejects attempt 5's actual exit 1.

Two subsequent [no-source Spot capacity smokes](native-cadence-replay-20261026/spot-capacity-smoke-20261026/REPORT.md)
ran through the deployed `ephemeral` account: `t3.small` and `c7i.2xlarge`
each executed a marker command, uploaded the expected S3 output with exit 0,
and reached EC2 `terminated` state. The larger host reported 8 logical CPUs
and 15 GiB memory, above the constructed worker's 4,769,202,176-byte peak
RSS. This establishes a live Spot admission and cleanup route in that account,
not endurance under a 2,400-second solver query. The runner's original-D18
manifest read still returned `403` at the last exact check; the reviewed
two-sided bucket/runner-role grant is unapplied pending authorization.
On-Demand remains denied by `SpotOnlyEC2Launch`, and a Spot interruption would
censor the current no-checkpoint worker. The [Linux Spot protocol](native-cadence-replay-20261026/LINUX_SPOT_PROTOCOL.md)
is now frozen as a plan; no Linux cryptographic arm has run. Before a hosted
timing run, get/read back the exact S3 permission, verify both original S3
objects on the worker, seal the per-job source/binary/host launch lock, then
apply the positive exit-0 and separate replay gates before natural.

A subsequent [build-only `c7i.2xlarge` Spot check](native-cadence-replay-20261026/linux-build-smoke-20261026/REPORT.md)
allocated and self-terminated its host, but stopped before Cargo: the runner
could not fetch the private GitHub source over HTTPS without noninteractive
authentication. The raw child error, status and EC2 termination are retained;
no binary, D18 GET or cryptographic arm resulted. Do not retry the unchanged
`--ref 45ce91d5` path. The earlier approved 17,757,116-byte source archive
was freshly fetched with its full-object checksum and correct SHA-256, but
freezes `54cbef1a` and lacks portable Git metadata; it cannot supply the
frozen `45ce91d5` cadence source. A reviewed exact-source delivery route
must pass before the Linux build, independently of the still-unapplied
two-sided original-D18 object read proposal. The temporary user IAM grant
remains active pending completion or cancellation of the hosted attempt.

The [follow-up no-source Rust Spot smoke](native-cadence-replay-20261026/linux-build-smoke-20261026/REPORT.md)
executed `rustc` and AWS CLI, uploaded outputs with exit 0, and its EC2
instance terminated, without trying private GitHub. A 58,011,243-byte
[exact-source archive proposal](native-cadence-replay-20261026/linux-build-smoke-20261026/SOURCE_DELIVERY_PROPOSAL.md)
is prepared with SHA-256 `dcc242fc0c4e6b4af99a854b885b0dece5d18ef69716a82cc5b5cc98d86044cd`.
Fresh extraction resolves clean `45ce91d5`, the patched verifier blob and
no Git remote; offline release example tests passed 20 worker and 5 verifier
cases. The archive has not been uploaded. A proposed three-object temporary
bucket/runner-role policy would cover this exact source key plus the two
original D18 inputs and would supersede the unapplied two-object proposal
only after explicit authorization. No Linux cryptographic arm has run.
