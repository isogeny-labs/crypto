# Resume checkpoint — operational non-split transfer

Inspect actual heads and uncommitted diffs first. Read QUEUE.md and the latest
session-log entries. Reuse immutable passing receipts when inputs match;
do not repeat completed baseline work. All scientific standings remain
unsigned/unverified. [pcc:unverified]

## Current durable sources

- Crypto branch research/chained-jacobian-transfer-20261110, existing PR3877;
  last published checkpointcdaa61178c4135846a85f57b9844ed38d2997e3d before
  the pending operational-map proof/field replay checkpoint. Read actual HEAD.
- Native runtime pin75a737b8366ec6c78ed8672886423fcbe69853a2; its seven inputs
  in small-prime-inputs.sha256 remain unchanged.
- Operational producer cryptanalysis9cd20ac5f80cdd140be2d16d49701c1b950ef13c,
  branch research/chained-transfer-nonsplit-map-20261111, PR1478. Its base is
  merged PR1477,c321a8f5af96185aaa5e8de0d27f0c3f4080772a. Observe current
  GitHub status before a new publishing decision; do not assume a merge.
- NONSPLIT_MAP_CHECKPOINT.md binds source digests, report/PDF, scope and
  independent field witness. Earlier cubic/balanced/kernel checkpoint files
  and every source pin/raw receipt remain historical evidence.
- Crypto-autoresearcher8edec422a60d87b2cbc656fb02f99f42eba959c7 is vault
  locked. No plaintext written there; use INTEGRATION_HANDOFF.md on unlock.

## Completed checks: reuse unchanged inputs

- Consumer full guard: native25/reference43 tests,1 existing benchmark ignored,
  formatting, strict Clippy and exact runtime guards pass; receipt
  nonsplit-map-pinned-01.txt/PCCbd303e. All285 rational classes in each
  direction satisfy both dual norms[2], paired additivity and bijection;
  every19 elliptic scalar/source norm[4]/inverse5 check passes.
- Independent pre-existing flat Fp6 engine replays128 tower-field products,
  inverses/Frobenius cases with exact seed checks, Rabin irreducibility and
  primitive-element bridge; PCC167595, independent-field6-02.txt and frozen
  TSV. Only a unit test/fixture added; engine arithmetic unchanged. This
  checks the field layer, not independently the whole Jacobian map.
- Four generic RichelotAlgebra.lean certificates pass7-declaration gate and
  local PCCf7eb4f check. Total44 written lemmas, gates22+8+7+12+6+5+7.
  Finite-field/Picard/global dual geometry remains written primary input.
- Current producer map diagram/PDF rebuilt and every2 pages inspected.
  Earlier130-witness cubic replay, balanced original/translated10404-class
  comparisons and kernel/PGL2 checks remain passed snapshots, not new runs.
- Root library attempt04 is finished:6184 pass/13 fail/133 ignored. Restoring
  tracked files fixes6 failing tests;5 need missing archives and2 unavailable
  kernel clock metadata. Full-suite status remains failed. Do not rerun the
  two-hour suite without resolving its actual causes. [pcc:unverified]

For changed inputs, use installed Rust/Lean on PATH:

```sh
export PATH="$HOME/.cargo/bin:$HOME/.elan/bin:$PATH"
# From cryptanalysis:
experiments/chained-transfer-reference/check.sh
# From crypto:
cargo test --release --bin icprog chained_nonsplit_compositum_witnesses -- --nocapture
tools/lean/proof-gate.sh research/chained_jacobian_transfers_20261110/lean/RichelotAlgebra.lean
```

## Next: CT15 shifted E109; CT16 chain obligation remains open

CT13b operational F17 map is complete; reuse its receipts. CT15 now assesses
frozen proposal8774df: shift2 of the existing E109 source. In U=X−2 the
source cubic coefficients are[56,13,6,1], cover[56,0,13,0,6,0,1], generator
(U=0,Y=37). Root norm45 is square, but no shifted kernel/map was computed.
Parameterize checked cubic quotient construction/square roots and the source
quadratic cover, retaining all F17 controls. First inventory determinants,
leading coefficients, twists and field descent before a map implementation.
If target leading is nonsquare, retain the unsupported boundary and derive
an explicit split-infinity coordinate change; never silently retwist. Check
source-coordinate shift back to original E and recovery inverse4=82 mod109.
This is a proposed input/acceptance plan, not an accepted shifted transfer.
[pcc:unverified]

CT16 assesses the accepted F17 codomain's next nonbacktracking kernels and
Galois2-torsion. Its PGL2 curve distinction is not yet an unpolarized-Jacobian
distinction. Preserve any forced backtrack or equivalent model. No continuation
is accepted yet. CT14 production/catalog/large-prime/extension/normalization
coverage and all solver-cost obligations remain open. [pcc:unverified]

## Continuing constraints

- No benchmark, collector experiment or paid allocation before successful S3
  checkpoint admission and live IsoLab budget/capacity visibility.
- Crypto's frozen PCC ledger-prefix fork remains unresolved. Extend PR3877;
  do not choose a history, rewrite entries, rebase away evidence or bypass CI.
  No crypto auto-merge while ledger/archive/review gates remain blocked.
- Local Lean pass and unsigned statement receipts do not create signed
  scientific standing. KB/ecdb unavailable, atlas locked; no novelty claim.
- New arithmetic Rust/Lean only, no new Python research arithmetic.
- Use guarded immutable GitHub object publication if ordinary Git auth still
  fails; verify remote heads, never force. Commit exact resume state frequently.
- Poll PRs on repository cadence(about50min absent new activity), or when
  actual merge state is necessary for the next publishing decision.
