# CT237 — independent raw-relation and public-target replay (2026-11-19)

**Standing: [pcc:unverified].** This is a post-hoc independent arithmetic replay of the already frozen CT235/236 F101 experiment, **not a new timing panel**, not original authenticated 695/701 evidence, and not a signed PCC.

## Source and non-duplication
- Continue draft [crypto #4715](https://github.com/isogeny-labs/crypto/pull/4715) under merged/root [#3877](https://github.com/isogeny-labs/crypto/pull/3877) and queue [#3904](https://github.com/isogeny-labs/crypto/issues/3904).
- Inputs: unchanged [CT235 native panel](ct235_native_panel.jsonl), native source SHA `58179c8befdaa5137281fcdd8caf3cbb4fd050c4`, public `E/F101:y²=x³+x+46`, `G=(2,37)`, `Q=(19,37)`, `r=109` and `C:y²=u⁹+u³+46`.
- Reuse the **independently written Python polynomial/Cantor implementation** in [CT223](ct223_extension_norm_oracle.py), not the production Rust Jacobian implementation or Rust outputs as arithmetic inputs. CT223's source was itself producer-authored; no independent reviewer has signed its correctness.
- Original norm-point 695/701 corpus and physics C169/C13a/C13b/N2_actual/D are **not** this F101 model; remain gated. No duplicate cryptanalysis CT172/CT189 hardware or catalog lanes.
- Literature checked: Gaudry–Thomé–Thériault–Diem, *A double large prime variation for small genus hyperelliptic index calculus* (2004/2007), https://eprint.iacr.org/2004/153. Weighted large-prime elimination is prior art; CT237 claims no new index-calculus algorithm.

## Acceptance tests (frozen existing data, no prospectively registered cost)
1. Regenerate all F101 rational signed factor-base points, first-eight offsets, Cantor walks and fully split squarefree degree-four supports **without** reading the native relation witnesses. For B=24/36/48/54, independently reproduce counts, raw accepted two-LP coefficients, the weighted LP Schur complement, factor-base rank, and determined LP logs.
2. For **every** raw accepted relation independently verify in the elliptic source group, without computing a logarithm, that the sum of support points minus the offset-point sum equals `[3k]G`. The factor 3 comes from the degree-three pullback norm.
3. Recompute factor-base and recovered LP logs using a separate modular linear solver and verify each via source elliptic scalar multiplication. Independently search for `Q=(19,37)` and recover its discrete log with the CT235 factor base; verify the recovered result by elliptic multiplication **without supplying 73 as input to the solver**.
4. Validate four distinct public target solves and compare all available rank/candidate/full/LP/target counts with **all 16 original CT235 records**, not CI grep echoes. Reject wrong group equations, wrong target, missing/altered panels. Treat timing from CT235 as recorded observations, not independently reproduced.
5. Same algorithm and offset ordering, bounded 60×109 relation candidates per base. Run Python normal mode with assertions enabled on an exact-branch scoped GitHub-hosted runner. `python -O` is forbidden by CT223's own guard.

## Reportable standing
This is one new independent-algorithm correctness/replay axis. A pass does **not** establish an independently timed solver, statistical advantage, performance on an ordinary large prime-field curve, authenticated original-model reuse, PCC or rho/BSGS/IC parity. CT236's 16,404–38,222× cold toy slowdown stands unchanged. The next **speed** path is a different cover/collector with lower full cold cost, subject to matched controls and original data/cost gates.
