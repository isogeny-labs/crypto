# Sources checked: prior-art pass for the 2026-11-12 closure note (2026-11-15)

Standing: every statement here is `[pcc:unverified]`. No pcc claim was filed.

## KB status

KB unavailable: `scripts/research-kb status` in crypto and in cryptanalysis both exit 127 with
"research-kb: no executable found; set RESEARCH_KB_BIN or put research-kb on PATH". No research-kb
MCP server is configured (`.mcp.json` lists conductor, isolab, isolab-eks, ecdb, cairn). The server
source in `crypto-autoresearcher/research-kb/` is vault ciphertext (`VLT1` header), and there is no
`catalog.sqlite` on the host. So `kb_status`, `search_knowledge`, `search_theorems`, `get_source` and
`get_context` were **not** run, and no KB `source_id` exists to record.

Fallback used (skill section 6): the lead atlas (`crypto-autoresearcher` @ `69d906dc6`,
`knowledge/leads/ecdlp/` plus `knowledge/frontiers/ecdlp/`), queried with
`python3 tools/lead_atlas.py --match "<q>" --limit 6`. It needed a PyYAML shim for the missing
vault-encrypted `tools/fast_yaml.py`. Other fallbacks: rg over crypto and cryptanalysis, and a few
web searches for primary-source abstracts. A miss here is not evidence of absence.

## Per route (atlas id, repo:path, effect)

| Route | Atlas / repo hits read | Effect on the closure note |
|---|---|---|
| A1, A2 Weil descent | `LEAD-ICP-extension-field-embedding` (closed_obstructed): crypto-autoresearcher:knowledge/leads/ecdlp/ic-prime/LEAD-ICP-extension-field-embedding.yaml; `KR-IC-58213c` (HKY last fall degree does not apply to summation systems) | Consistent. The embedding cost p^{2-2/n} matches A2's "≈ p". Prior art already covers this, so it is not new. |
| A3 trace / Galois orbit | `LEAD-ICB-frobenius-orbit-factor-base` (closed_obstructed; gain exactly n, prime-field φ-stable at most 6) | Consistent. Not new. |
| A4, A5 algebraic membership / root bases | `LEAD-ICP-algebraic-membership-factor-base` (exhausted_at_scale; "size-B algebraic base has membership degree ≥ B/3", IDEA-20260801-021; KN-FIND-007 yield conservation) | **Pre-empts K1 / Lemma 1** (the bounded-degree factor-base theorem already existed, before 2026-11-12). |
| Section 3 Theorems A, B | crypto:research/notes/ecdlp-general/RESEARCH_ALGEBRAIC_IC_LOWER_BOUND.md (2026-10-31; report crypto:research/prime_field_leads_20261031/L1_lower_bound/REPORT.md); `KR-IC-401ebf` (Yokoyama–Yasuda–Takahashi–Kogure, JMC 14 (2020) 460–485, doi 10.1515/jmc-2019-0029) | **Pre-empted, not contradicted.** The 10-31 note's A2 (D_sol ≥ deg L), A3 (rank ≥ C(m+v−s, m)) and Theorem 1 (direct presentation, T ≥ c_m r) are Lemma 5(U) and Theorem B (univariate case) under another name. Its C2 ("relation count alone excludes m ≤ 2", T ≥ √(2r)) is Theorem A. Its hypotheses H_C and H_E are the closure checker's hidden assumptions. YYTK prove the dense case under statistical assumptions (Conj. 10, Assumption 11) with F4 charging. The closure note cites neither. New in the closure note, as far as this pass can see: Lemma 4 (exact XL degree), the Γ/XL^J graph family, and Theorem C with hypothesis G. |
| Section 3.4 F4 escapes XL on univariate bases | `LEAD-ICP-split-graph-secant-presentation`: crypto:research/prime_factor_base_degree_20261031/SECANT_GRAPH.md (F4 handled degree d+1 against 2d or 2d+2) | Pre-empts the checker's "not mentioned by the producer" observation. It was on record since 2026-10-31. |
| A4-PKM | `LEAD-ICP-pkm-tower-algebraic-oracle` (open_blocked, reviewed 2026-11-15); crypto:research/prime_leads_chase_20261113/README.md and CONTINUATION_PROTOCOL.md; RESEARCH_ALGEBRAIC_IC_LOWER_BOUND.md section 5 | **Superseded in part, and the m ≥ 8 reading is contradicted.** (1) The 11-13 chase measured m = 8: D(8,1) = 7, D(8,2) = 8 (exact), D(8,3) ≥ 9 (certified), with uncertified evidence of ≥ 10. (2) Its continuation states that charging one whole solve removes the advertised m = 8 gain even if D(8,3) = 9. (3) The 10-31 bar c ≥ (m−1)/2 gives a per-unknown σ* of 0.0871 at m = 8, against the preset's α*(8,1) = 0.156. Recomputed in `calc/bars2.out`: σ = 1/8 is **KILL** under the 10-31 bar and below-bar under the preset. So the closure note's "m = 8, w = 1 gives exponent 0.43, below rho" depends on the preset's s < m − 3 parity rule. That rule leaves out the max(1, ·) attempt floor, and it is not robust. |
| A6 interval / Boolean | `KR-IC-401ebf`; `LEAD-ICP-digit-presentation-solving-degree` (Theorem F, D ≥ ffd; settles G clause (ii) for digit presentations, 7,835/7,835) | Consistent. Theorem F partly answers the note's "clause (ii) never tested" for digit presentations, but not for towers. |
| A8 lifting | `KR-IC-cd159a` (JKSST, conditional on Lang and the selected-lift hypotheses); `LEAD-TRF-global-lift-small-height`, `LEAD-ICP-char0-lifting-xedni-smoothness`, `LEAD-EXO-global-lift-height` (open_next_step); crypto:research/prime_leads_chase_20261113/L4-lifting/PROOF.md (Theorem X); Huang–Kueh–Tan, ANTS 2000, LNCS 1838, pp. 377–384 (abstract only) | No contradiction. HKT already name the barrier: lifting a growing number of points at bounded height. Theorem X of the 11-13 chase supersedes the Prop 3 sketch (per-trial ≤ \|M\|/q, Q-only after checking). No source found that already replaces Lang by Petsche in JKSST, so the novelty of that step is **unconfirmed, not established**. |
| A9 BKK | `KR-IC-5a43e5` only | Nothing relevant. |
| A10 Richelot / genus 2 | `LEAD-TRF-same-field-genus-g-embedding` (closed_obstructed by theorem: Tate multiplicativity, q^{2−2/g}) | Consistent. Not new. |
| A11 isogeny pushforward | `LEAD-ICP-isogeny-transported-relation-yield` (exhausted_at_scale); chase L5 (crypto:research/prime_leads_chase_20261113/L5-fibre) | Consistent; L5 confirms the relabelling. |
| B1 CM fold | RESEARCH_ALGEBRAIC_IC_LOWER_BOUND.md D3 (fold changes the bound by at most g² ≤ 36) | Consistent with 1.25–1.62x. |
| Section 6 lower bound open | `KR-IC-58213c` (Caminata–Gorla: solving degree = max(lfd, GB degree)); `LEAD-EXO-low-degree-query-lower-bound` (open_next_step); `LEAD-EXO-lange-winterhof-interpolation` (open_blocked); `KR-IC-28c357` | Consistent: no lower bound for F4 or polynomial calculus on Semaev systems was found. Two related open leads are not cited in section 6. |

## Not done

- No research-kb search, so there are no `source_id`s and no `search_theorems` on `ICCounting.lean`.
- Primary papers were not re-read (YYTK, PKM 2016, JKSST, HKT, Petsche). Only abstracts or
  repo summaries were used.
- The lead-atlas `--delta` reports 11,357 changed paths since its 2026-10-30 review.
