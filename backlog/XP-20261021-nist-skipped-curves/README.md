# Run DiSSECT on the skipped NIST candidates

Status: runnable candidate-branch jobs, not queued. The card remains
`draft` until its implementation is merged and pinned, a resumable runner
is available, and an execution environment is selected. The manual workflow
has not been dispatched. The linked [factorization continuation](../../research/nist_skipped_curve_dissection_20261021/factorization/README.md)
imports the earlier external run as unchanged evidence and resolves eleven
of its fourteen timeout-valued fields: six of nine distinct integers are
completely factored, while three retain composite cofactors. This does not
complete a fresh 23-job run or establish a historical skip reason.

The panel is the P-224 seed stream's candidates 777, 1485 and 5736, the
P-256 stream's candidate 876, and published P-224/P-256 controls. Full
`p,a,b,n,h` and control generators are in [curves.json](curves.json);
seeds, stream coordinates and root conventions are in
[provenance.json](provenance.json). The skipped historical generators
remain unknown. Original imported fixture keys are preserved.

## Why these jobs

The existing [seed census](../../research/nist_seed_provenance_20261016/README.md)
and [skip-filter study](../../research/nist_seed_skip_filter_20261017/README.md)
already reproduce the streams and test many selection hypotheses. This
card adds the specifically requested upstream DiSSECT suite and an
independent algebraic audit on the four selected candidates. It does not
repeat the 21-candidate statistical feature search.

The separate [census/registration PR #2333](https://github.com/aburan28/crypto/pull/2333)
also covers these candidates. It was unmerged when this card was written.
Its models use the opposite sign of `b` for P-224 stream blocks 777 and
5736. Those models are isomorphic over the base field but have different
model identities. Preserve the selected inputs here; register their exact
models before citing result identities, and reuse an existing registration
only when the coefficients match.

| Jobs | Work | Expected coverage |
|---|---|---|
| shards 0–21 | Actual unmodified upstream DiSSECT, one trait per shard | 22 traits, six curves, 438 default parameter cases |
| shard 22, `algebra` | Native PARI/GP: independent counts, twists, CM, embedding, small neighbors, explicit 3-isogenies and same-field comparisons | six curves, seven pairs; prime isogeny degrees 2–31 |

[jobs.json](jobs.json) gives the exact trait order and parameter grids.
The [protocol](PROTOCOL.md) states budgets, acceptance, known upstream
defects and the mathematical meaning of each comparison.

The [Skipped NIST curve dissection workflow](../../.github/workflows/nist-skipped-curves.yml)
provides the launch path: after merging, select **Actions → Skipped NIST
curve dissection → Run workflow**. It runs the same 23 jobs from
`jobs.json`, with at most four jobs at once on separate runners, preserves
failed artifacts, and retains downloads for 90 days. It has no push, PR or
schedule trigger. Download and land the artifacts before retention expires.
The workflow is separate from the draft card's distributed lab promotion.

## Prepare the environment

The host needs Bash, git, jq, ripgrep, GNU coreutils (including `timeout`),
PARI/GP with `pari-seadata`, and SageMath **9.8**. No MongoDB server is
needed. The Sage image used in the earlier external execution is pinned at
`sagemath/sagemath@sha256:1dca662086a58ea620c54af578ee1d2ee5d0dbad01efed442a58ebf99c09fea9`.
A host installation or container wrapper may be selected using the
single-executable paths `SAGE_BIN` and `GP_BIN`; do not pass shell command
strings in those variables.

On a 32 GB host using Docker's `vfs` storage driver, use one persistent
container and execute commands inside it sequentially. Creating fresh
containers for concurrent wrapper calls exhausted disk during authoring.
The earlier external batch remains separate evidence; the new runner's
container command checks were incomplete after that resource failure.

From the repository root, choose an external scratch directory and install
the unchanged upstream tool's import dependencies:

```bash
mkdir -p /tmp/nist-skips-runtime
git clone https://github.com/crocs-muni/DiSSECT.git /tmp/nist-skips-runtime/DiSSECT
git -C /tmp/nist-skips-runtime/DiSSECT checkout --detach 8edc3030fb91411a55aa6c07d050507d840c5e55
sage -pip install --target /tmp/nist-skips-runtime/deps pymongo==4.6.3 dnspython==2.6.1
export DISSECT_ROOT=/tmp/nist-skips-runtime/DiSSECT
export DISSECT_DEPS=/tmp/nist-skips-runtime/deps
```

The pinned source, version and lack of tracked source edits are checked
before each trait run. `curves.json` contains no cached properties. The
external DiSSECT command is used directly; no new Python harness is added.

## Run and verify

Each output path must be new or empty. These commands run one requested shard:

```bash
bash backlog/XP-20261021-nist-skipped-curves/run.sh run isogeny_neighbors /tmp/nist-skips-neighbors-01
bash backlog/XP-20261021-nist-skipped-curves/run.sh verify /tmp/nist-skips-neighbors-01
bash backlog/XP-20261021-nist-skipped-curves/run.sh run algebra /tmp/nist-skips-algebra-01
```

To run every job sequentially, with a fresh directory for each:

```bash
while IFS= read -r job; do
  bash backlog/XP-20261021-nist-skipped-curves/run.sh run "$job" "/tmp/nist-skips-batch-01/$job" || exit "$?"
done < <(jq -r '.[].job' backlog/XP-20261021-nist-skipped-curves/jobs.json)
```

The loop stops on a failed shard, whose artifacts remain intact. Retain
that attempt, investigate it and run remaining shards explicitly. The
backlog card resolves each shard separately; follow
[backlog/README.md](../README.md) for lab leases and publication. A card is
not a submitted job. No lab task, cloud instance or funding is created here.

Every successful directory has a receipt, input hashes, runtime, exact
command, raw result and `SHA256SUMS`. Integrity and coverage verification
does not certify every DiSSECT value. The known torsion/trace bugs require
the independent audit described in the protocol. Keep factorization
timeouts and missing-generator nulls visible.

## Resource envelope and completion

Each shard has one CPU, 4 GB RAM and 3,600 seconds plus a 10 second
termination grace. PARI factorizations also have a 120 second limit. The
23-job ceiling is 23 core-hours (23 wall-hours if sequential, excluding
setup); this is a configured upper envelope, not a runtime estimate.
Reserve 32 GB disk for the runtime and artifacts. No GPU is required.

Done means all job artifacts are accounted for, the 438 upstream records
are checked for coverage, and the native algebra checks and per-field
audit are published with all discrepancies and incomplete computations.
The historical reason for skipping remains open unless independent
historical evidence supports it. Preserve unsuccessful runs. Result PRs
must supply the report, graph, PDF and curve registrations required by the
protocol.

Funding is proposed as zero until this executable candidate branch is
reviewed and the lab is selected; it does not preempt the existing funded
cards. Promotion to `ready` is a separate reviewed change. The local
Conductor binary was unavailable during authoring; the documented
[fallback](../../docs/CONDUCTOR.md) was used, and no shared reservation is
claimed.

## Sources

- [Anthropic: ECDSA curve origins](https://www.anthropic.com/research/ecdsa-curve-origins)
- [Valsorda: seeds solved](https://words.filippo.io/seeds-solved/)
- [Pinned stream reproduction](https://github.com/FiloSottile/mostly-harmless/blob/5fea0f56c717b8499710fb0237cda6f6ed75653d/seeds/seeds.go)
- [Pinned upstream DiSSECT](https://github.com/crocs-muni/DiSSECT/tree/8edc3030fb91411a55aa6c07d050507d840c5e55)
