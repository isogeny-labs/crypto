# RESUME (draft) — D1 Frobenius-transport relation graph

> Draft content for `.resume/RESUME.md`. **Not** yet installed: do NOT run
> `scripts/checkpoint.sh init` from this design pass (it pushes to S3 and refuses
> run launchers). `init` is run at **authorized launch time**, seeded from this
> draft, by the session that actually executes the experiment.

## One-paragraph state

A fresh session can resume the D1 effort from this file alone. D1 is the
"Frobenius-transport relation graph" hybrid IC direction from the 2026-11-20
hybrid-IC sweep (4 directions D1–D4; D1 is the only one still live, and even it is
expected to return a null). The design, grounding, and a frozen-before-data
preregistration are written; **no experiment has run**; the candidate instrument is
**not yet implemented**. Everything is `[pcc:unverified]`.

## Where things are

- Branch: `claude/adoring-fermi-ap4ukn` (crypto).
- Study dir: `research/hybrid_ic_directions_20261120/`.
  - `D1_transport_graph.md` — full design/memo.
  - `SYNTHESIS.md` — all four directions and the meta-finding.
  - `SOURCES.md` — research-kb / lead-atlas receipts (sibling fork).
  - `prereg/D1_n31_sharing_null.protocol.json` — the `experiment.protocol/v1` record.
  - `prereg/D1_PREREG.md` — readable companion.
  - `prereg/RESUME.draft.md` — this file.

## The hypotheses (frozen) and the honest priors

- `D1-SHARE` — cross-target frontier sharing is a **preregistered NULL**:
  `f_share = b·E·n/r` ≈ 0.18 (n=31), ≈ 7.6e-4 (n=41). Sharing dies in `n`.
- `D1-ASTAR` — the one live sub-claim (low confidence): A\*-on cuts forward
  expansions ≥2× at matched yield. **Main risk:** the heuristic (ONB x-weight /
  Frobenius distance) is the exact feature set that gave the learned selector **no
  routing headroom** (SOLVER_SELECTION, 288 queries) — it may be too flat.
- `D1-BEAT` — gate: beat the **equal-envelope control** (wider base `|F∪D|`) at all
  of 0/0.1/1 GAE. (This control cut large-prime from 23.8× to 2.1×.)
- `D1-COLLAPSE` — if sharing null ∧ A\* dead, D1 ≡ batched MITM + compact-orbit
  (already on `main`); record as a measured null.
- Scope: toy rungs `n≤41` only (**no n=131 path**: u64 FastPoint < n=64,
  MAX_MODULUS_BITS=63). No leaderboard / exponent / sub-rho / ECC2K-130 claim.

## The exact next action (in order)

1. **Implement `examples/transport_graph_cell.rs`** — the A\*/beam search over
   canonicalised residual states; a `--sharing on/off` shared-frontier mode; a
   `--heuristic on/off` switch; the equal-envelope control (plain MITM on `|F∪D|`);
   the planted positive control; and same-process `rho.signed_frobenius` — **all
   counting GAE identically** (reuse `FrobeniusPairTable`/`decompose_mitm_frobenius`
   in `ic_boundary.rs`, `compact-orbit-scan:norm=N` in `plugins.rs`, and the
   `group_op_count`/`FieldOps` counters). Add tests: every accepted path re-adds in
   the curve group (fail-closed); planted scalar never solver input.
2. Build release; record `binary_sha256`; run a **preflight** (fill
   `sample_size.preflight_runtime` and the `null` `commit`/`binary_sha256` fields in
   the protocol); freeze the protocol hash and `analyze.py` (sha256) **before data**.
3. research-kb + lead-atlas receipts into `SOURCES.md` before novelty wording lands
   in a PR.
4. Conductor task + scope reservation (Conductor not installed in the design
   session; hooks warn-and-allow).
5. `scripts/checkpoint.sh init` → push `.resume/RESUME.md` (this content) to S3.
6. experiment-director compiles the frozen task card from the protocol;
   executor-mechanical runs it unchanged on **isolab-eks native-L2**; default route.
7. **Phase 0 first:** the `n=31` sharing-null cell (`A-S1H0` vs `A-S0H0` + A/A).
   **Phase 1 gate:** only spend on the `n=41` candidate arms if the `n=31` A\* arm
   shows ≥2× forward-expansion cut; else run only the `n=41` baseline + rho.
8. Analyze masked; unmask only after `ANALYSIS.json` is frozen; report with
   gap-to-rho; `research-visuals` report+SVG+PDF. No leaderboard row (D1-OOS).

## What is explicitly NOT done / outstanding gates

- Instrument not implemented; no preflight; `commit`/`binary_sha256` are `null`.
- No Conductor task; no S3 checkpoint; no task card; no run.
- `SOURCES.md` receipts pending (sibling fork).
- bench.record/VALIDATION.json only if a priced row ever moves (none expected).
