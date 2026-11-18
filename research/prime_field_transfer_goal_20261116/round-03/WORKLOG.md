# Round03 checkpoint

The original GOAL.md remains active. The implemented components are compact
CM fixtures, local projections, auxiliary Frobenius arithmetic and a known
target-only Smart solver. The arbitrary-Q common-field evaluator is open.

## Commands and actual outcomes

- `g++ -std=c++17 -O2 -Wall -Wextra -Werror round-03/wild_target.cpp
  -lcrypto -lgmpxx -lgmp`: build01 fails on five indentation warnings;
  build02 passes; build03 fixes formal-kernel output handling and public
  labels before first arithmetic. All logs retained.
- Frozen qualifier01 calls `timeout 60s .../wild-target m` for m1,12,24,
  48,96,112: six exits0. Four target-only public answers per fixture,
  all18 small-fixture auditor answers. No source-to-target answer.
  Native claim PCC-20261116-99cae8 remains unsigned. [pcc:unverified]
- Assembly01 fails because root-relative preparation ran in the Lean
  subdirectory. No result/source file was written by that preparation.
  Assembly02, concrete endpoint01 and axiom01 commands pass. [pcc:unverified]
- `USE_FRO_CACHE=1 tools/lean/proof-gate.sh --out .../proof-gate-01
  research/prime_field_transfer_goal_20261116/lean`:33 audited theorems,
  zero trusted native exceptions, nine named new endpoints. No limits
  raised; one Lake process at a time. Claim PCC-20261116-1856b2 remains
  unsigned. lean4checker is unavailable. [pcc:unverified]
- First IDC issuer include renamed old main and exposed its implicit-return
  warning. Retained failed source; reused its unchanged helper prefix with
  the new entrypoint, without suppressing compiler warnings. Build02 and
  frozen identity call01 pass; two IDC1 records,32 points each, two rejected
  perturbations. Rust replay unavailable. [pcc:unverified]
- Report: `python3 render-round03.py` (recorded-data plot only),
  `pandoc round-03.md --standalone --css round-03.css -o round-03.html`,
  `PLAYWRIGHT_MODULE=/opt/codex/cua_node/lib/node_modules/playwright
  node print-round03.cjs`. First PDF has two pages but diagram sizing needs
  correction; final render uses responsive SVG. Both final A4 pages are
  inspected at1300-pixel height before freezing report inputs. PNG previews
  remain outside Git.

## Publication and preservation

- Isolated worktree starts at71f59bcaf and fast-forwards to main730e036d5;
  no round02 source/proof/report pin is edited. No original GOAL.md change.
- PR4388's observed external merge, failed checks and ledger discrepancy
  are retained in round02-external-merge-audit.json. Episode
  E-d3e2e7e3a740 records the partial result; no green-at-merge assertion.
  Automation_f75bf7f0256c819195a395c6ba527bce is disabled.
- New publication uses the existing study lane with a fresh regular PR
  after4388. Preserve old unpublished ledger evidence if concurrent main
  appends require fresh registration; never rewrite a published history.
- Cargo/full workspace tests, Conductor and KB unavailable. Primary
  sources and existing native/GMP/OpenSSL code are used. No EKS job,
  spending, timing result or benchmark/leaderboard row.

## Exact next research obligation

Compute the target residue of Q's selected H-point at fixed common-field
places without discovering its source scalar or tabulating the subgroup.
Include source-relation and CT124 chimera controls and costs for common
field construction, local precision, branch selection, target solve and
source verification. The tested CM family does not supply arbitrary-curve
applicability. Dense degree r−1 is not a compressed-evaluation lower bound.
