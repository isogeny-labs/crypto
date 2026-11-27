# Rootfind production panel (2026-11-26)

This study separates two scopes:

- `p31-production-panel.json` is the admitted full production `ic.pipeline`
  panel for EKS L2. It contains the legacy and incumbent scalar identities,
  the versioned eight-target `rootfind-floor-batch-v1` identity, and a
  byte-identical legacy A/A control.
- `examples/rootfind_production_panel.rs` is a p31/p61 oracle diagnostic with
  planted hits and subtract-reference-negative targets. Its p61 rows are a
  full-width scalar control, not an end-to-end IC speed claim.

`protocol.json` is deliberately **draft**. Before freezing it, replace every
instrument commit with the exact published algorithm/spec commit, build and
plan the exact spec, and pin `analyze.py`. Do not silently make historical
`rootfind-floor` use batching.

## Local p31 diagnostics (not admitted evidence)

The public real-prime control is

```text
p = 2147483647
E: y^2 = x^3 + 2147483644*x + 21
#E(F_p) = r = 2147487637
G = (1391443431, 177979044)
```

All local rows below are `[pcc:unverified]`: one shared KVM, no isolation
admission, and source provenance unavailable because the binary was built from
a dirty tree. They establish the engineering baseline and reject candidates;
they are not the accepted result named by `protocol.json`.

The five-arm, one-round production smoke used A=640 and the frozen p31 spec.
All five rows recovered and independently verified the target scalar.

| Arm | Full method (ms) | Relations (ms) | Legacy/full | Legacy/relations |
|---|---:|---:|---:|---:|
| legacy MITM | 360.573 | 326.549 | 1.000x | 1.000x |
| subtract scalar | 209.151 | 171.205 | 1.724x | 1.907x |
| floor scalar | 98.088 | 62.761 | 3.676x | 5.203x |
| floor batch v1 | 73.510 | 41.112 | 4.905x | 7.943x |
| legacy MITM A/A | 385.803 | 349.084 | 0.935x | 0.935x |

The separate 64-target oracle panel used 32 planted hits and 32 targets that
the subtract reference classified negative, with five paired repeats pinned to
CPU 0. Its warm equal-mix paired median is 6.922x for preferred floor lanes
over legacy MITM; negative-only is 10.757x, hit-only is 2.683x, and setup plus
the first 64-target pass is 1.254x. IFMA and scalar all-negative ledgers both
charge 40,960 additions. The IFMA hit row charges the complete post-hit sweep,
40,992 additions versus 9,766 for scalar early exit.

Two additional CPU candidates were measured and removed:

- A batch-only one-multiply/vector-gather Bloom moved the absolute warm batch
  median from 9,444.5 to 8,927.7 ns/target, but the paired legacy ratio moved
  from 6.922x to 6.713x. The 4-6% candidate movement is inside the observed
  A/A range (up to 1.206x), so it did not justify a second hash contract and
  AVX-512DQ path.
- The witness-order-preserving packed blocked PairTable builder took 21.917 ms
  to prepare the batch arm versus 21.790 ms for the incumbent builder in the
  scalar floor control in the same binary. The historical large-table setup
  gain did not reproduce at A=640.

The local full-method baseline also supplies a hard scope bound. A 50x result
would require at most 7.211 ms, while factor-base construction, linear algebra,
verification, and unphased runner overhead already total about 7.737 ms even
if oracle setup and relation collection were free. The unchanged CPU pipeline's
oracle-only ceiling is therefore 46.60x. Existing production four-thread data
show only 1.46-1.90x scaling, and hit retirement can remove at most about 13%
of candidate additions on the natural stream. The remaining prospective route
is a separate, versioned CUDA batch-1024 identity plus faster linear algebra;
historical A100 results used different sizes and comparators and are not a p31
speed claim. That experiment remains blocked on current budget admission.

The Lean declarations prove the finite-domain additive-group equivalence of
decomposition satisfiability and direct subtraction/lookup. They do not verify
the Rust implementation, elliptic-curve formulas, PairTable/Bloom/IFMA code,
accounting, compiler, or an executable SAT solver.

## Validate and freeze

```sh
protocol_tool=research/primefield-companion-followup-20261102/tools/experiment_protocol.py
python3 "$protocol_tool" validate research/rootfind_production_panel_20261126/protocol.json

# The generic protocol validator does not resolve this repository-relative
# analyzer path for hash verification.  Keep this explicit binding gate.
test "$(sha256sum research/rootfind_production_panel_20261126/analyze.py | cut -d' ' -f1)" = \
  "$(python3 - <<'PY'
import json
from pathlib import Path
protocol = json.loads(Path(
    "research/rootfind_production_panel_20261126/protocol.json"
).read_text())
print(protocol["analysis_plan"]["script"]["sha256"])
PY
)"

# After the exact code/spec commit is pushed, and before affected data:
python3 "$protocol_tool" freeze \
  research/rootfind_production_panel_20261126/protocol.json \
  --commit auto --filed-before-affected-data
python3 "$protocol_tool" validate research/rootfind_production_panel_20261126/protocol.json
```

Build `ecbench` at that exact commit and require `ecbench plan` to accept the
spec before freezing:

```sh
cargo +1.99.1 build --locked --release --bin ecbench
target/release/ecbench plan \
  --spec research/rootfind_production_panel_20261126/p31-production-panel.json \
  --json > /tmp/rootfind-p31-plan.json
```

The workflow itself builds with Rust 1.99.1 and records the executable digest.

## Publication and checkpoint gates

After committing and pushing algorithm/spec commit `A`, retain a source receipt
that deliberately omits the not-yet-filed protocol:

```sh
python3 taskq/deploy/eks-karpenter/check-experiment-git \
  --repository isogeny-labs/crypto \
  --path research/rootfind_production_panel_20261126/p31-production-panel.json \
  --path research/rootfind_production_panel_20261126/README.md \
  --path examples/rootfind_production_panel.rs \
  --path tests/rootfind_batch_pipeline.rs \
  --path research/rootfind_production_panel_20261126/analyze.py \
  --path src/bin/ic/bench.rs \
  --path src/cryptanalysis/ecbench/methods.rs \
  --path src/cryptanalysis/ic_framework/mod.rs \
  --path src/cryptanalysis/ic_framework/rootfind_floor.rs \
  --path src/cryptanalysis/ic_framework/rootfind_kmitm.rs \
  --path src/cryptanalysis/ic_framework/stages.rs \
  --path src/cryptanalysis/ic_boundary.rs \
  > publication-receipt-A.json
```

Then, while `HEAD=A` and before affected data, pin every protocol instrument to
`A`, freeze it, commit and push filing commit `B`, and retain the second receipt:

```sh
python3 taskq/deploy/eks-karpenter/check-experiment-git \
  --repository isogeny-labs/crypto \
  --path research/rootfind_production_panel_20261126/protocol.json \
  --path research/rootfind_production_panel_20261126/p31-production-panel.json \
  --path research/rootfind_production_panel_20261126/README.md \
  --path research/rootfind_production_panel_20261126/analyze.py \
  > publication-receipt-B.json
```

Run the repository checkpoint gate without overwriting the tracked shared
`.resume/RESUME.md`; preserve all existing entries when adding this lane's
handoff. Run `python3 tools/benchrec.py gate` before launch/push and resolve any
pre-existing failure explicitly rather than bypassing it. In a sparse checkout,
historical missing-payload failures require the prescribed full checkout; do not
create new exclusions or alter closed records.

The 2026-11-26 read-only budget telemetry attempt failed before any paid AWS
work because GitHub OIDC could not assume the budget role. Do not dispatch this
experiment until a fresh admission proves current daily and monthly headroom.

## Accepted launch

Dispatch the workflow definition on `main`, while building the exact pushed
feature commit. A feature-branch workflow dispatch is not an EKS run.

```sh
source_revision=FULL_40_CHARACTER_PUSHED_ALGORITHM_SPEC_SHA
gh workflow run isolab-experiment.yml \
  --repo isogeny-labs/crypto \
  --ref main \
  -f source_revision="$source_revision" \
  -f spec_path=research/rootfind_production_panel_20261126/p31-production-panel.json \
  -f instance_type=c8a.2xlarge \
  -f seconds=4800
```

The source revision above is the algorithm/spec commit (`A`). The prospectively
frozen protocol is filed by a later commit (`B`) and pins every instrument to
`A`; retain publication receipts for both. The five arms, one warmup, six
measured rounds, and 120-second per-method caps permit 4,200 seconds of method
time, so the 4,800-second job deadline includes provisioning and harness
overhead without silently shortening an arm.

Record the dispatch time and run ID. Accept a result only when the immutable
source/image/binary/spec identities match and the native all-record L2, replay,
A/A and S3 payload-hash gates pass. There are zero automatic measurement
retries. Failed, timed-out and censored rows remain results.
