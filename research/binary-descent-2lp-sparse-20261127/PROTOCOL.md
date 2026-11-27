# Protocol: arm D2S — 2LP Weil descent + BFS forest + sparse LA, against matched rho

Study `b2lsp-20261127/v1`. Seed policy `isogeny-labs/experiment-seeds/v1`.
Status `[pcc:unverified]`, L0, one shared 4-vCPU / 15 GB cloud container.
**Single target only** (owner instruction): every solve is one cold solve of
one public `Q`; nothing is amortised across targets.

Builds on crypto PR #5291 (`research/binary-descent-2lp-20261127/`, arm D2,
study `b2lp-20261127/v1`, head `73bf5f4e`) and PR #5290
(`research/binary-descent-sparse-la-20261127/`, arm D-S, study
`bdsl-20261127/v1`, head `92f096c1`), both stacked on #5261 (arm D) and #5253.
This branch is based on #5291's head; #5290's study directory is copied
unchanged from its branch head (`git checkout 92f096c1 -- research/binary-descent-sparse-la-20261127/`),
and its `sparse_la.rs` is included by `#[path]`, unedited.

## What D2S is (fixed before the pilot)

`harness/src/main.rs` (`run_d2s`) and `harness/src/bfs.rs`. Reused unchanged
by `#[path]`: #5253's field, curves, factor base, A1 relation-target stream,
group-law resolution, echelon (cross-check only) and rho; #5261's
`descent.rs`; #5291's `graph.rs` (`Comb`, `inv_mod`; `LpGraph` only as an
uncounted shadow); #5290's `sparse_la.rs`.

1. **Relation phase = #5291's D2**: same oracle, same stream, small base =
   columns `1..fs`, large primes = the other subfield points; rows with 0 / 1
   / 2 large primes kept (full row / edge to SV / edge), 3 discarded.
2. **Relations are stored**, with a union-find that counts cycle-closing
   edges: `Y_ub = full rows + cycle-closing edges`, an upper bound on the
   yields (exact in SV's component, at most one too many per other cyclic
   component).
3. **Stopping rule = #5290's**, on `Y_ub` and `fs` in place of the rows and
   the column count: first checkpoint at `Y_ub ≥ ⌈0.70·fs⌉`, then every
   `⌈fs/256⌉` more. At each checkpoint:
   - **BFS forest** rebuilt from scratch: BFS from SV in SV's component, from
     the union-find root in every other component with a cycle; every vertex
     gets a shortest path to its root, so a non-tree edge `(u, w)` closes a
     cycle of length ≤ depth(u) + depth(w) + 1.
   - Each non-tree edge's relation is reduced **deepest column first** along
     BFS parent edges (each vertex eliminated once; signed-graph grounding as
     #5291: the first frustrated cycle of a component without SV grounds it).
     Yields = full rows + these combinations = `E − rank` (the same count as
     #5291's `LpGraph`, checked by an uncounted shadow at every checkpoint).
   - #5290's 2-core excess on the yields; if ≥ 1, #5290's SGE + Wiedemann
     (kmax 64, 4 attempts, |u| = 16, excess ≤ 2) on the yields over `fs + 1`
     columns. `d` found → stop; otherwise collection goes on and the next LA
     call is counted again.
4. **Counting**: all graph work in phase `graph` — classification,
   union-find, BFS adjacency and reduction steps one `h` each; mod-ℓ work by
   #5290's convention (product `z` unless an operand is ±1, addition `za`,
   inversion `2·bits(ℓ)` `z`, cached per build). The ±1 products and the
   LA's structural bookkeeping are counted and reported as sensitivities.
5. **D2Sall** (every class small, `fs = F − 1`) on #5290's LA stream
   `la-ds|n|ci|ti` must reproduce D-S bitwise (d, targets, relations, LA
   counters and report). Pre-freeze engineering check on #5290's own
   self-test instances (n = 27, 33; ti = 1..3): 6/6 identical in d, targets,
   relations, `la_report`, LA calls, LA phase counters and structural count.

## Part A — pilot rule (frozen before any pilot is run)

The pilot chooses only the small-base rule of D2S (exponent β and constant
θ in `fs = min(F − 1, ⌈θ·F^β⌉)`) and the rho lanes of rung n = 60. Held-out
instances only.

- **Small-base pilot.** Curves `d2spilot-curve|n|ci`, targets
  `d2spilot-target|n|ci|ti`, relation stream `d2spilot-stream|n|ci|ti`, LA
  stream `d2spilot-la|n|ci|ti|α`; n ∈ {42, 48, 54}, ci, ti ∈ {0, 1}
  (4 cases per rung); grid `fs = ⌈α·(F − 1)⌉`,
  α ∈ {0.045, 0.067, 0.1, 0.15, 0.22, 0.33, 0.5, 0.75, 1.0}.
  Rule (`b2lsp_harness pilot-summary`):
  1. per rung, `S(α) = ln gmean_cases Meq(s=1,u=1)`; `k*` = argmin on the
     grid (ties to the smaller α);
  2. if `k*` is interior, α̂ = vertex of the parabola through
     `(ln α, S)` at `k*−1, k*, k*+1`, clamped to that bracket; at an edge,
     α̂ = α_{k*};
  3. `fs*(n) = α̂·(F − 1)`; OLS of `ln fs*` on `ln F` over the three rungs
     gives β̂ and θ̂; β̂ is clamped to [0.5, 1.0] and, if clamped, θ̂ is
     refitted as the geometric mean of `fs*/F^β`;
  4. arm **D2S** uses (β̂, θ̂); arm **D2S23** uses β = 2/3 (the textbook k = 3
     2LP balance) with θ = geometric mean over the rungs of `fs*/F^{2/3}`.
- **Rho lanes.** n ≤ 54: unchanged references (part B). n = 57: W = 256,
  the argmin of #5291's complete held-out cells at n = 57
  (`pilot_rho_ext2.jsonl`: 256 → 1.718·10⁹, 128 → 1.822·10⁹, 64 → 2.045·10⁹
  Meq; 16 records each). n = 60: new held-out pilot `pilot-curve|60|ci`,
  `pilot-target|60|ci|ti`, `pilot-rho|60|ci|ti|W`, ci ∈ {0, 1}, 8 targets per
  curve, W ∈ {128, 256, 512}; W* = argmin of mean Meq(s=1,u=1).

Pilot outputs: `data/pilot_alpha.jsonl`, `data/pilot_rho_60.jsonl`. They are
not evaluation data. Part B (arms, rungs, references, predictions, analysis)
is written and frozen after the pilot and before any evaluation data.
