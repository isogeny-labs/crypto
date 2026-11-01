# PR2980 upstream lifecycle compile repair

Merged origin/main acf27a37aafe through merge commit
1ecfd5a335a547e18fb1c64c80880aa5be4cbf0d, preserving prior evidence ancestry.
The existing upstream repair fd3334c7368fbd3661d7e5e31c550c1c8b8e2632 adds
`diagnostic_lifecycle: None` to the volcano replay ChildInput initializer,
matching its existing no-diagnostics behavior. No local variant of that repair
was introduced.

At preceding head c9be327b5da227c5744ed629e298783dc4120686, four hosted checks
failed with E0063 at research/full_volcano_graphwide_ic_20261017/execution/driver.rs:253
because diagnostic_lifecycle was missing:

- Root Clippy: https://github.com/isogeny-labs/crypto/actions/runs/41135535425/job/124337676722
- Workspace check: https://github.com/isogeny-labs/crypto/actions/runs/41135535776/job/124337448446
- Factor-base sweep CLI: https://github.com/isogeny-labs/crypto/actions/runs/41135535772/job/124337722534
- Harness: https://github.com/isogeny-labs/crypto/actions/runs/41135535772/job/124337722420

Replay, artifacts, software, parity and recomputation passed on that preceding
head. Those successes do not qualify the new merged head, which needs its own
hosted checks. No failed workflow was blindly retried.

## Affected CPU validation

Main also extends the shared module's supported slot bound and adds an optional
query-budget API. Ordinary solve/profile calls retain multiplier20. The CPU
oracle's own cap remains unchanged. After merging, 45 standalone release
library tests pass, including all three archived complete-certificate fixtures.
Strict all-target release Clippy, formatting and the complete-example release
build pass. The upstream driver also passes rustfmt checking from committed
source. Root crypto/CUDA was not rebuilt locally under the user's disk
constraint; hosted CI must verify the root compilation repair.

Commands use crates/hashed-ecdl-cpu/Cargo.toml, --offline --locked --release,
and CARGO_TARGET_DIR=/tmp/cpu-projective-relations-target: cargo test --lib,
cargo clippy --all-targets -- -D warnings, cargo fmt --check and cargo build
--example complete. Logs and source hashes are retained beside this note.

## Frozen evidence and dashboard scope

`git diff --exit-code c9be327b5da227c5744ed629e298783dc4120686 --
research/hashed_ecdl_cpu_20261030/round20` passes with empty output retained in
frozen-round20-diff.log. No frozen protocol, source identity, binary, result or
certificate was replaced. Round20 still measures its named pre-merge snapshots,
not this new merged head. It remains unsubmitted pending the execution decision.

No new curve, priced IC/rho result, phase cost or performance comparison is
introduced by this CI repair. The existing round18 observations are explicitly
L0 fixed-fixture diagnostics and excluded from priced leaderboard/scoreboard
rows; round19 has no measurements and round20 has no execution. Therefore this
repair adds no priced dashboard row or numeric progress-timeline point. General
upstream dashboard changes arrive through main's own commits, not a regenerated
local index. Full dashboard/Pages qualification remains with its applicable
hosted gates and publication; none is claimed here.

T-1033 owns this synchronization record. No new cloud benchmark was launched.
