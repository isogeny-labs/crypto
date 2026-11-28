# Sources checked — hybrid-IC directions D1–D4 (2026-11-20)

"Knowledge base first" receipts for the four directions (D1 transport graph,
D2 shared-x twist twin, D3 MaxSAT selection, D4 auxiliary data). Everything
cited is `[pcc:unverified]` unless its own record shows otherwise.

## KB / atlas availability — UNAVAILABLE in this container

- **research-kb: unavailable.** No MCP `research-kb` server is connected, and the
  wrapper `scripts/research-kb` finds no executable
  (`$workspace/crypto-autoresearcher/research-kb/.venv/bin/research-kb` absent)
  and no catalog (`$workspace/research-kb/data/catalog.sqlite` absent). Same state
  independently recorded by `research/cheon_aux_leads_20261112/README.md`
  ("Sources checked").
- **Lead atlas: unavailable.** `tools/lead_atlas.py` is not in this checkout and
  the sibling `crypto-autoresearcher/knowledge/leads/ecdlp/` is absent; per
  `cheon_aux_leads_20261112`, the atlas files are AES-SIV vault ciphertext with no
  `KEY` set, so the atlas cannot be read even when the sibling is cloned.
- **Fallback used** (per the research-kb SKILL §6 "mark the KB review unavailable
  and use the lead atlas and repository paths"): a direct sweep of `research/` and
  `docs/` by `grep`/`ls`, reading the headers/READMEs of every candidate dir.
  Per the skill, **a miss here is not proof of absence** — only the in-repo record
  was searched, not the ~22k-source corpus.

## Sources checked (fallback: in-repo records)

| query / mechanism | top source (repo:path) | class / status | relevance |
|:--|:--|:--|:--|
| best-first / A* / beam / frontier search over decomposition states | (none found in `src/` per E1 grounding; no research dir implements it) | — | D1 search mechanism appears unbuilt |
| graph-based index calculus (cycle/pair oracle) | crypto:research/graph_cycle_ic_20261007/README.md | boundary (negative result); dominated by rho | D1 adjacent — prior graph-IC, loses to rho |
| generalized-birthday / partial-join "one relation" route | crypto:research/single_relation_reduction_20261115/SELECTOR-JOINS.md | [pcc:unverified] hand derivation | D1 adjacent — Wagner k-sum tree, fixed-slope buckets; no A*/frontier |
| "transport" over ECDLP structure | crypto:research/ecc2k130_{dual,oriented}_transport_2026092{4,5}, boolean_basis_transport_20260923 | correctness prereqs / null | NAME-ONLY collision — isogeny/Vélu instance transport & Boolean row-space transport, not a search |
| graph-wide IC over isogeny graph | crypto:research/full_volcano_graphwide_ic_20261017/README.md | workload prep | name-only (graph = isogeny vertices) |
| Koblitz twist pair, shared x-only base | crypto:research/twist_ecdlp_20261027/E4-koblitz-twist-pair/{RESULTS,PREREGISTRATION}.md | measured; `prediction_confirmed` | **D2 direct prior** (see below) |
| quartic/prime twist IC | crypto:research/twist_index_calculus_20261020/, e34_twist24_cheap_rep_20261031, e29_twists_20261030 | "no priced row moves" | D2 — prime-field twist lineage |
| MaxSAT for relation filtering/selection | crypto:research/sat_smt_leads_20261114/12-maxsat-filtering/{README,PROTOCOL,SKEPTIC}.md | pilot measured; skeptic "kill-criterion-met" | **D3 direct prior** (see below) |
| binary SAT decomposition/norm | crypto:research/full_binary_norm_sat_20261017_v1 | [pcc:unverified] | D3 — SAT-as-oracle lineage (distinct from selection) |
| Cheon / auxiliary inputs leads | crypto:research/cheon_aux_leads_20261112/README.md | desk only; [pcc:unverified] | **D4 direct prior** |
| auxiliary curve / point setup | crypto:research/auxiliary_halving_setup_20261115/README.md; cheon_ecbench_{20261022,wide_20261106} | PCC-20261115-3ff0b8 (unverified) | D4 — auxiliary-input domain |
| multi-target amortization vs batch rho | crypto:benchmarks/20261112-mt-fieldops-v5/ (PCC-20261112-22e5d2) | unranked across §5a band | D4 — precomputation reading |

## Novelty verdicts

**D1 — transport graph + A\* + cross-target frontier sharing: GENUINELY-NEW mechanism,
partial conceptual overlap.** No `src/` code and no research dir implements a
best-first/beam/A\* search over partial decomposition states or a cross-target
residual frontier (E1 grounding; no dir found). The idea is conceptually adjacent
to two priors that are both dominated by rho: the (A,P) graph-cycle IC
(`graph_cycle_ic_20261007`, explicit boundary negative result) and the
generalized-birthday / partial-join one-relation route
(`single_relation_reduction_20261115`, Wagner k-sum tree with fixed-slope buckets,
no A\*/frontier). The three "transport" dirs are isogeny/Vélu instance transport
and Boolean row-space transport — name-only collisions. Verdict: the search
mechanism is new; its surrounding framing is not, and the nearest priors lose to
rho — consistent with D1's own self-refuted sharing sub-claim.

**D2 — shared-x twist twin (binary Koblitz a=0/a=1): ALREADY-DONE for m=2.**
`twist_ecdlp_20261027/E4-koblitz-twist-pair/` preregistered and ran exactly this on
the pair K0:y²+xy=x³+1 / K1:y²+xy=x³+x²+1 over F_{2^m}, m∈{11,13,17,19}, Semaev
m=2, x-only F2-subspace bases. Prediction **H0 (separation; no gain from the twist
half of a shared x-only base) held — `prediction_confirmed`; "the shared base is a
pure cost increase (accounting, not a gain)."** It states the same exact identity
our D2 fork derived (rational third abscissa iff the two abscissae share type).
Untested slice only: m≥3 (S3/S4) decompositions, non-x-only bases, a complete solve,
and a matched rho gap (none was run). Prime-field twist lineage
(`twist_index_calculus_20261020`, "no priced row moves") concurs. Verdict:
D2's near-null is already the measured result for m=2; the binary m≥3 case is the
only unmeasured remainder and is strongly predicted null.

**D3 — MaxSAT on relation selection: ALREADY-DONE (killed).**
`sat_smt_leads_20261114/12-maxsat-filtering/` ran the pilot (RC2 exact + Glucose4
model-improving ANY, cardinality-network excess constraint, greedy G0–G3 baselines,
planted-dense positive control, independent verifier). Independent `SKEPTIC.md`
verdict: **"kill-criterion-met,"** scoped to subset selection on uniform random
weight-4 matrices at 5 s/arm. Matches our D3 fork's no-go and the E3 grounding
(LA is 0.11–1.9% of a run; row_ops unpriced in S). Untested slice: merges/fill-in
and realistic (non-uniform, factor-base-skewed) matrices — the one branch the
skeptic left open. Verdict: not novel; already killed in scope, open only on the
skewed-matrix merge question.

**D4 — auxiliary data: ALREADY-DONE (desk) / DROP.** `cheon_aux_leads_20261112`
(desk survey of every auxiliary-curve/point/Cheon lead), `auxiliary_halving_setup_20261115`
(auxiliary-curve point-only cold recovery), and the `cheon_ecbench_*` bound records
cover reading (A); the frontier rule that an oracle domain is a different problem is
confirmed. Reading (B) is `benchmarks/20261112-mt-fieldops-v5` (unranked across the
§5a band). Verdict: fully covered by prior work; nothing new for the single-target
goal, consistent with D4 → drop.

## Receipt for the handoff / PR

KB review: **UNAVAILABLE** (research-kb + lead atlas both absent in this container);
fallback = in-repo `research/`+`docs/` sweep only, so absence is not proven. Net:
**D2 and D3 are already measured in-repo** (`E4-koblitz-twist-pair` prediction_confirmed;
`12-maxsat-filtering` kill-criterion-met); **D4 is already desk-covered**; **D1 is the
only mechanism without an in-repo implementation**, though adjacent to two rho-dominated
priors. Any PR must cite these and must not re-assert D2/D3/D4 as untried.
