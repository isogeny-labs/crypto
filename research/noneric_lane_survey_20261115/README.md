# Non-generic IC lever survey for Table A / Table B binary (2026-11-15) `[pcc:unverified]`

Desk survey only: no run, no code change, no leaderboard effect. Every number below is quoted from the
named file and is an unverified assertion unless that file says it was validated.

## Sources checked

- **research-kb: unavailable.** No `research-kb` MCP server or `kb_status`/`search_knowledge` tool in this
  session (ToolSearch "research-kb kb_status search_knowledge": no match). The knowledge-base pass is owed
  before any lever below becomes a task card. No novelty claim is made.
- **Lead atlas:** `/home/user/crypto-autoresearcher/knowledge/leads/ecdlp/` is not present in this container;
  `research/leads_20261112/README.md` reports the atlas bodies as vault-encrypted. Not read.
- Repository (origin/main unless noted): `docs/ic/LEADERBOARD.md` (K1 Koblitz GAE section, Table A, Table B,
  both 2026-11-15 k-sum sections, "Phases, as a share of S"); `docs/session-logs/2026-11-15.md` (18:30Z binary
  k-sum, rootfind rounds 3-5, ladder panel, wave-6, SAT-PDP plan); `research/ic_tableb_binary_ksum_20261115/README.md`
  (this branch); `research/leads_20261112/README.md` §1, §2e, §3, §5; `research/ecc2k130_open7_20261114/README.md`
  (L1-L7 verdicts); `research/ic_gpu_rootfind_{ic,pairs}_*/RESULTS.md`, `research/ic_native_sweep_20261115/RESULTS.md`;
  `src/cryptanalysis/ecbench/methods.rs` (oracle dispatch: `solve_ic_prime` ~L6716, `solve_ic_binary` ~L7511);
  `src/cryptanalysis/ic_framework/plugins.rs` (`RootfindSubtractOracle` implements `DecompositionOracle<PrimeCurve>` only).
- GitHub open PRs (isogeny-labs/crypto, 60 most recently updated, read 2026-11-15 ~18:50Z).

## Current standing (quoted)

| row | best recipe | S | gap to rho | phase that dominates |
|:--|:--|--:|--:|:--|
| Table A, Koblitz one target, n41-61 (clock unit) | aimed m = 3 collection, m = 2/3 descent | 4.15-5.9 | 4.08-11.1x | reusable set-up (FB + relations + LA) 98.5-99.5% |
| K1 Koblitz, ecbench GAE (n23, n29), validated | `koblitz-standard-subspace:dimension=9/10, pdp3-koblitz:m=3,engine=rootfind-negfold` | 43.9 / 3182 (w = 0) | 5.59 / 9.47 bits | not split in the row |
| Table B random binary n21/n24/n27 (this branch, PR #4317) | generic `ksum:m=5` on smallest-abscissa base | 7.01 / 6.99 / 6.41 (w = 0) | 5.56 / 5.77 / 6.00x | relations (PDP) 88-96% under the old recipe |

Two facts frame every proposal:

1. k-sum is a **generic decomposition** (bounded by sqrt(r); cannot beat rho or BSGS by construction).
   Table B binary is now 5.3-7.5x behind rho with a generic oracle, and Table B's own subspace recipe
   (`inc-tb`) is 1.4-3.4x *worse* than the generic tuned pair table.
2. Prior verdicts that close most non-generic routes at these sizes: two-summand schemes have exponent 2/3
   (leads_20261112 §2e); Koblitz curves proper are excluded from the m >= 4 Gaudry route by the group-order
   trap (same); every prime-degree Koblitz IC arm moves *away* from rho at +0.05-0.15 log2 per bit of r, and
   a win must cut cost per trial or trials per relation by a factor growing with r (ecc2k130_open7 L7);
   subspace-polynomial gcd at m = 3 costs 300-1500x the pair-table oracle (L4a); SAT reuse and degree <= 3
   pre-filters are NO-CHANGE (L2, L3); dissection and Frobenius/negation-closed non-linear bases already done (L1, L6).

## Lane map (who is where, what is open)

| lane | branch / PR | in flight | open threads they list | collision for binary/Koblitz IC |
|:--|:--|:--|:--|:--|
| Table B binary k-sum (this session) | `claude/f4-f5-algorithm-speed-bvp7qd`, PR #4317 | validated; leaderboard section landed on the branch | re-record Table B rows with the tuned incumbent; **ksum on Koblitz rows (A/C) "a separate question (other lanes active there)"** | owner of `ksum` binary dispatch (`methods.rs` one line) |
| Table B prime k-sum | merged (54040ed524) | done | none listed | none |
| GPU / native rootfind | `claude/index-calculus-gpu-optimization-otouqv` (#4276, #4310, #4314 merged) | round 5 (m = 3, incremental-gauss-echelon) blocked on RunPod funds; native sweep PR pending | round 5 rerun; validator on native sweep | **prime only** (`fp28-fp38`); edits `ic_framework/rootfind_gpu.rs`, `plugins.rs` |
| Ladder wwalk / ksum6,7 (prime) | merged 615b324625 | validation gate pending | 64-bit wwalk table for 44/48 bits | prime only |
| K1 Koblitz pair-table sizing | ml-cryptanalysis `fb-pt-lgrow-k1` (evidence copied, VAL-20261114-d9190c) | validated row | none in this repo | owns the K1 GAE row recipe (`pdp3-koblitz`, `rootfind-negfold`) |
| koblitz_fast perf | merged bff7c3c13f / 72f4a1f91a | done | — | `koblitz_fast` scalar code |
| Koblitz sparse LA | merged 61b3d71c84 etc. (11-12) | done | — | LA is <= 1.2% of S: not a lever |
| ECC2K-130 open-7 sweep | merged 2964ea4be7 | done | counted-GAE Koblitz row with a real §5a band; m = 83 counted row; GF(2^n) `w_cal`; L4 gcd arm into `ic_framework`; S4/Gaudry refutation tests | — |
| N83 / K600 six-summand (codex) | PRs #4295, #4281, #4290 | open | — | Koblitz N83 base objects; not Table A sizes |
| ECC2K trace-zero linearisation (codex) | PR #3744 | open | — | ECC2K-130 relations |
| SAT-solver speed for PDP | `research/sat_pdp_exotic_20261115`, PR #4240 (B0) | B0 baseline, amendments | crate SAT solvers, director card, EKS run | **binary PDP incl. `koblitz-orbit`**: any SAT oracle work collides |
| Fall-degree / closure (PKM, Kummer, D9) | #4098, #4291, #4278, #2902 | running | m = 3 LFD, flat-presentation exponent study | prime-chain closures; K2 (binary m = 3/4 flatness) not started by anyone |
| Single-target IC parity C = 1 | #4307 | open | k = 32 phase split | curve25519 family (prime) |

No open PR or log entry since 2026-11-11 touches: a Frobenius-folded k-sum on Koblitz curves, the existing
binary `descent-algebraic` oracle on Table B's random-binary rows against k-sum, or a native GF(2) closure for
the K2 flatness test.

## Ranked candidate levers

### 1. Frobenius-folded k-sum on Koblitz (Table A / K1 GAE rows) — best gain per cost

- **Mechanism.** Table A spends 98.5-99.5% of S in the reusable set-up, i.e. relation collection on an
  m = 3 aimed pair-table oracle. On random binary curves the generic k-sum (one probe of target into a
  negation-folded table of all signed m-sums of the base) cut the strict pair-table incumbent to
  0.46-0.68x at n21-27, and the gain grew with n (0.68 -> 0.64 -> 0.46). On a Koblitz curve the base can be
  taken as <tau, -1> orbit representatives and the table canonicalised under the same group, so the table
  and the column count both shrink by about 2n while each probe tests the whole orbit. Today's binary
  Frobenius oracles (`mitm-frobenius*`, `FrobeniusMitmOracle`) are two-summand only; `ksum` has no fold.
  This is non-generic in the same sense as signed-Frobenius rho (uses tau), so the honest ceiling is the
  sqrt(r/n) class: it narrows the gap, it cannot cross rho.
- **Why not already done.** The binary k-sum log names it as an open, separate question; the K1 lane sized
  the pair table only; ECC2K L6 closed table *engineering* for the m = 4 2+2 MITM, not a k >= 4 sum table.
- **Cheapest discriminating pilot (seconds to minutes, no new code).** `ksum` already dispatches on
  `BinaryGroup` on this branch, so first measure the *unfolded* k-sum against the K1 recipe on K1 n23
  (`icv1-f2m23-tm5197-1f85e9e1`), 8 targets, 1 round, L0 counted, with matched strong signed-Frobenius rho:
  ```
  cargo build --release --bin ecbench
  # spec arms: rho-sfstrong-v2; ic.pipeline koblitz-standard-subspace:dimension=9 + pdp3-koblitz:m=3,engine=rootfind-negfold;
  #            ic.pipeline <smallest-abscissa base, size from research/ic_tableb_binary_ksum_20261115/pilot> + ksum:m=4 and m=5
  target/release/ecbench run --spec <pilot spec> --out <scratch> && target/release/ecbench verify --replay-all --exit-code <scratch>
  target/release/ecbench band <scratch> --ranks   # w = 0 / 0.1 / 1
  ```
  Decision rule: if unfolded ksum5 / K1 recipe < 1 at w = 0 already, the fold (expected extra factor ~n in
  table size, ~sqrt(n) in relations-to-rank) is worth writing; if it is > 2, drop it. Disclose as a sizing
  pilot (BENCHREC_SKIP), as the binary study did.
- **Expected effect.** If the random-binary trend carries over, relation phase x 0.5-0.7 at n23-29, i.e.
  S x ~0.5-0.7 since set-up is ~99% of S: K1 n23 gap 5.6 -> ~4.6-5.1 bits, Table A 4-11x -> roughly 2.5-7x.
  The fold is the speculative part: rough upside another sqrt(n)-ish on relations. Prior that the unfolded
  pilot beats the K1 recipe: about 0.4 (the K1 recipe is already a tuned m = 3 rootfind, a stronger
  incumbent than Table B's pair table).
- **Collision risk: medium.** Pilot: none (spec + scratch output only). Fold implementation edits
  `src/cryptanalysis/ic_framework/plugins.rs` and `ecbench/methods.rs`, which the rootfind lane also edits;
  needs `conductor check --scope path:src/cryptanalysis/ic_framework/plugins.rs` and a new oracle type
  (e.g. `FrobeniusKSumOracle`) rather than edits to `KSumOracle`. Tell the K1 lane (ml-cryptanalysis) via agent-bus.

### 2. First non-generic point on Table B binary: `descent-algebraic` m = 3 vs ksum5 in one session

- **Mechanism.** Table B binary is now measured only with generic oracles (pair table, ksum) and the
  subspace pair table. The only route whose exponent can differ is an m >= 3 Semaev decomposition solved
  algebraically on a subspace base (Weil descent; `descent-algebraic`, FES-then-batched by default since
  d81dc1241b). It already dispatches on binary curves.
- **Why not already done.** ECC2K L4/L7 and `fes_k13_validated` measured algebraic arms on Koblitz /
  degree-13 curves, not on Table B's random-binary rows against the new k-sum comparator; Table B's
  recorded recipe is m = 2.
- **Cheapest pilot (minutes).** One ecbench session on n21 (`icv1-f2m21-t2591-de54d72e`), 8 targets:
  arms rho-neg, ksum:m=5 (the study's size), and `binary-subspace:dimension=ceil(21/3)+1` +
  `descent-algebraic:m=3`; report per-relation cost and trials per relation separately, then repeat at n24
  to read the slope (L7's criterion: a win must grow with r).
- **Expected effect.** Most likely negative: L4a priced m = 3 subspace algebra at 300-1500x the pair-table
  oracle and L7 found algebraic arms drift away from rho. Its value is the slope reading against ksum on
  the same rows (closes or opens the Table B binary non-generic cell). Prior of an S improvement: <= 0.1.
- **Collision risk: low** for the oracle (no code); conceptual overlap with the SAT-PDP lane (#4240) — do
  not use a SAT engine arm.

### 3. K2: native GF(2) closure flatness at m = 3, 4 (exponent question, expensive)

- **Mechanism / source.** leads_20261112 K2: if m = 3/4 binary descent systems stall at degree D <= 4, an
  m = 4 Weil-descent IC changes exponent; nobody has started the native GF(2) degree-<= 4 closure.
- **Pilot.** Native Rust closure at D <= 4 on N = 27, 30, 33 (m = 3) and 24, 28, 32 (m = 4); registered
  prior of flatness 0.15; CPU hours, new code under `src/`.
- **Expected effect on S/gap.** None directly at Table B sizes (the crossover at D = 4 is n* ~ 205 per the
  lead); it decides whether any non-generic binary lever can ever close the gap.
- **Collision risk:** low on files (new module), medium on ownership (fall-degree sessions #4098, #4291).

## Recommendation

Run lever 1's no-code pilot first (one ecbench session, K1 n23, unfolded ksum4/5 vs the K1 recipe vs
strong signed-Frobenius rho). It is the only candidate with a plausible measured S improvement on a
validated Koblitz row, it reuses this branch's binary `ksum` dispatch, and its outcome decides whether the
Frobenius fold is worth a conductor-reserved oracle. Lever 2 is a cheap side arm in the same session family
if a non-generic data point on Table B binary is wanted for its own sake. Before either becomes a task
card: the research-kb pass and the atlas cards for `LEAD-ICB-*` k-sum / Frobenius-MITM ids.
