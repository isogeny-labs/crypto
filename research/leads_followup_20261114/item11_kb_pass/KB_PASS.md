# KB pass for the 2026-11-12 lead survey (item 11)

Date 2026-11-15. All statements [pcc:unverified]: lexical search of a snapshot plus reading, no computation on a
curve. Nothing here changes an atlas verdict, leaderboard row or benchmark row.

## 0. Instrument and provenance

| item | value |
|---|---|
| release | isogeny-labs/research-kb `snapshot-2026-11-14` (published 2026-11-14T18:41:14Z), found with `gh api repos/isogeny-labs/research-kb/releases` (the `gh release` subcommands are blocked here: they use GraphQL) |
| asset download | `gh api -H "Accept: application/octet-stream" repos/.../releases/assets/748417989` (catalog) and `.../748417990` (manifest) |
| catalog.sqlite.zst | 239,357,790 B, sha256 `8bc55122b1d257297311e022f6cc45b223423bc123c344061662a2c80054db32` = manifest `assets[0].sha256` (match, `sha256sum`) |
| decompressed | 909,070,336 B = manifest `sqlite_uncompressed_bytes`; sha256 prefix `2c3e1587d863808d`; python zstandard in a scratch venv |
| row counts (measured by SQL) | documents 23,053; versions 23,171; locations 23,053; passages 166,071; declarations 1,936 (all `source-only`: no kernel-checked Lean receipt) = manifest counts |
| catalog revision | `a340f0566afc4abb83dd81cd3eef5b01` (state table = manifest) |
| location of the catalog | `/tmp/claude-0/-home-user/9dbbf9e8-7c86-599b-a188-83c0330b1243/scratchpad/kb/cat/catalog.sqlite` (outside every repository) |
| repo heads at this pass | crypto 44d35033fa, ml-cryptanalysis e8aecdd, cryptanalysis fb3f88a0, research-kb d5c8468 |

Search tooling: `tools/q.py` (FTS5 bm25 over `passage_fts`), `tools/run.py` with `queries/q1..q4.json` (about 60
queries) plus about 25 inline queries (listed in the shell history only, patterns named in the rows below). The
MCP tools (`kb_status`, `search_knowledge`, `search_theorems`, `get_source`) were not used: no server in this
session. Differences: no hybrid ranking (`hybrid_current: false`), so this is a lexical pass with its misses.

### Coverage limits that bound every "not found"

1. **Corpus date.** The newest `crypto` file stamps in the index are 20261104; the indexed source signature for
   `crypto` was taken around 2026-11-06 and the catalog refreshed 2026-11-14T03:49Z. Nothing from the 2026-11-10..14
   work (PR #3938 leads, the 11-12 survey, ml-cryptanalysis PR #101) is in it.
2. **Run directories are not indexed.** `crypto` has 814 files, all notes, docs, papers and problems. Zero hits
   for the paths `research/quartic_n4v2_*`, `research/c1_twisted_line_ladder_*`, `research/ecbench_pair_claw_*`,
   `research/subgroup_trap_*`, `research/leads_2026111*`, `research/weil_restriction_isogeny_census_*`. Results
   that live only in those RESULTS.md files are visible here only when a note, atlas card or paper quotes them.
3. **ml-cryptanalysis is not indexed** (no `ic-bounds*` path). The growth theorem and its engines have no corpus
   copy; only `crypto` papers and notes that predate it.
4. **Papers.** Most literature entries are abstract-level (`KN-LIT-*` say "abstract only" or "full text not
   read"); GGMP eprint 2020/1315 and Tian 2023 are in that state. Brownawell-Masser, Voloch, Mason, Wang, Hsia-Wang
   (the theorem T2 of the growth proof) have no hit at all (queries `Brownawell AND Masser`, `Voloch AND (abc OR
   Wronskian ...)`, `Mason AND abc AND function field`).

## 1. Outcome against the preregistered prediction

Prediction: at least 8 of 15 proposals have a partly-anticipated or stronger hit. Falsifier: fewer than 4.

Measured classification (details in section 3): new 4 (P2, J1, S1, S3); partly anticipated 8 (P1, P5, J2, J3,
J4, K1, M1, S2); already proposed or tried 3 (P3, K2, K3-as-K1). That is 11 of 15 at "partly anticipated or
stronger": prediction holds, falsifier not triggered. "New" here means no in-scope hit, bounded by the limits above.

## 2. Vault-encrypted atlas bodies: plaintext counterparts

Question: do `crypto-autoresearcher knowledge/leads/ecdlp/*.yaml` (VLT1 ciphertext in the checkouts) have
plaintext counterparts in the corpus? **Yes.** Measured by SQL on `locations` and `passages`:

- 173 present files under `knowledge/leads/ecdlp/` (171 lead cards + `README.md` + `notes/exotic-leads-20261103.md`);
  all with passages, first passage text in the clear (`- **id**: LEAD-...`, 171 of 173) and 0 passages beginning
  or containing `VLT1`. Each card is 8 passages with the fields id, family, title, mechanism, disposition,
  best_measured, last_reviewed, next_action, discriminates, blocked_by, evidence.
- Of the 176 ids in `research/leads_20261112/atlas_lead_ids.txt`, 171 have plaintext cards; 5 do not:
  `LEAD-ICP-hecke-cm-class-point-rank-completion` and `LEAD-ICP-prym-witness-relation-diversity` are listed in
  `ingestion` with status `missing` (location row present, no passages); `LEAD-ICE-power-sum-coordinates`,
  `LEAD-ICE-quadratised-chain-representation`, `LEAD-ICE-subspace-frobenius-chain-mod-n` have no `locations` or
  `ingestion` row (probably added after the scan).
- Of the 63 `LEAD-*` ids named in the lane files and README, 40 have plaintext cards; the 23 without are the
  survey's own new proposals (P-, J-, K-, SYN-), `LEAD-C-jv-*` (lane C ids not in the atlas list), the two
  `missing` ids above, `LEAD-ICE-quadratised-chain-representation`, and a glob (`LEAD-ICP-gfp5-*`, which matches
  `LEAD-ICP-gfp5-oa-sdhp-jv` and `LEAD-ICP-gfp5-odd-char-cover`, both present).

Handling consequence (for the owner, not an action taken): the release asset is a plaintext copy of content the
repository keeps vault-encrypted. The catalog stays in the scratchpad outside the repositories. This file quotes
only dispositions and short summaries of cards, not full bodies. Whoever restricts access to the vault should
consider the release assets as a second copy.

The nearest-id reads below replace the "bodies unread" caveat in every lane file.

## 3. Per-proposal record

Format: classification; prior work (source_id | repo:path); what changes. `src:` ids are the first 12 hex digits
of the catalog `source:` id (`document_id`).

### P1 quotient law at D-1 (about 3B per target). Partly anticipated.
- The claim "the algebraic solver's cost is the rank of the Macaulay-type matrix, which is about the number of
  columns" is **GAP A4** of `crypto:research/notes/ecdlp-general/RESEARCH_ALGEBRAIC_IC_LOWER_BOUND.md`
  (src:6cf4fb0f198f, 2026-10-31): Theorem 1 proves cost >= c_m r for the direct presentation; for PKM towers it
  reduces to "solving degree grows linearly in the number of levels and the rank is close to the number of
  columns", stated as an open gap, with H_tower. Theorem 2: in-model sub-r^(1/2) cost iff per-attempt exponent
  c < (m-1)/2.
- `crypto:research/notes/ecdlp-general/RESEARCH_PRIME_FIELD_BARRIER_MAP_20261104.md` (src:1d1daefdad3a): "the
  |F|-degree membership quotient is the cost"; per-attempt algebraic work e_S = 2.19 [2.16, 2.22] against group
  e_G = 1.97; the k-SUM framing says algebraic = B^2 up to log p; slope bar D >~ 0.04 N + c. (P1 quotes a closure
  cost exponent B^2.24; same neighbourhood, different object: whole-closure cost, not per-attempt S4 roots.)
- Atlas: `LEAD-ICP-pkm-tower-algebraic-oracle` (open_blocked, src:0ac2e5271a2e) and
  `LEAD-ICE-prime-field-semaev-solving-degree` (closed_known; beta_work 0.797 / 0.647 against a 0.30 sub-rho gate).
- Negative form searched (`penultimate AND quotient`, `lower bound AND last fall degree AND subgroup AND semaev`):
  nothing states a q/B law at D-1 or its numbers.
- Effect: the idea is not new as a thesis; the measured q/B in [2.5, 5.7] and the out-of-sample test would be the
  first measurement of GAP A4's rank clause. Rank stays 2; word the card as "test of H_tower / GAP A4 rank clause"
  and cite L1 Theorem 2 as the pass line. Also the atlas verdict proposed in README section 4 ("0.1 ... open") must
  be reconciled with the L1 note, which already says the tower case is conditional.

### P2 free-extension lift to k = 15..18. New in scope.
- No hit for `free extension AND closure OR lift`, `V_7 ... kummer closure`, `plateau`. The plateau record on the
  F4 side: RESEARCH_PKM_TOWER_ORACLE.md (src:13c233fff93b) and the encyclopedia section 0.1 table
  (src:601058f8defc): m = 2 F4 D = 5 at N = 12..18, 6 at N = 20 and 22 (N = 2k, so k = 10, 11), width 33,446 and
  51,909 columns, peak memory 473 MB and 2.3 GB.
- Support for the premise "memory is the blocker": `crypto:research/ic_leads_review_20261018/README.md` (quoted
  in the descent-closure atlas card: "one large-memory node is the largest missed opportunity") and the barrier map
  ("decisive cells need 130-380 GB").
- Effect: no prior art; the GPU-route rationale is independently stated. Rank 9 may move up one or two places
  (above S2/S3) because the corpus names large-memory cells as the bottleneck, but P2's own prior (plateau of four
  confirmed, 0.9) is unchanged. Owner's route decision still required.

### P3 radix-2^rho presentation. Already proposed, not run.
- Barrier map section 6.1 "Radix-vs-levels discriminator" (src:1d1daefdad3a): same question (is D set by levels t or
  by bits log2|V|), arms radix 2 / radix 3 / mixed, cells |V| in {2^6, 2^12, 2^18}, pre-registered prediction
  D_radix3 >= D_radix2 + 1 with multiply-adds equal within 2x, falsifier D equal and multiply-adds lower by >= 2x
  at |V| = 2^18. Blocked: `f4_fp_tower` has one rewrite type (degree-2 steps), `Mono = u128` square-free bitmask.
- `LEAD-ICP-digit-presentation-solving-degree` (open_next_step, src:c966bcfca144): digit presentations, closed form
  d_ff = m 2^(m-1) + floor((s - 2^(m-1))/2) + 1, curve-independent and reproduced by a block-factored null;
  `IDEA-20261024-e65d46` lists it as adjacent.
- Effect: P3's "degree cannot drop below the binary chain's" is the barrier map's prediction. Merge P3 into the
  barrier-map card (reuse its prediction and falsifier); do not open a separate card. Rank stays low (10).

### P5 degree-d reach and quotient (A12). Partly anticipated.
- `LEAD-ICP-coppersmith-smallroot-semaev` (open_next_step, src:c31625145e1a): derived reach 0.3293 (3 unknowns) /
  0.3536 (2 unknowns), numerical best 0.4357 / 0.4892, demand 8/9 at (m,w) = (4,2); l = 3 and non-triangular
  lattices open. `LEAD-ICP-algebraic-membership-factor-base` (src:703208c6bf5a): map-image yield within 7% (32 bits)
  and ratio 1.000000 at N 2^14..2^18 (EV-FBG-001).
- Degree-d maps: `H-ECDLP-30a2a1` (src:26e3dcc1ae18, status proposed) states a factor base from a degree-d map
  E -> P^1 with window W, Newton polytope and per-variable degree depending on the map, and notes the reach
  functional "is not resolvable in this program"; `IDEA-20260808-486ae2` (src:db46733960f2) is the degree-2 window
  small-root record whose number is eps(m, tree). `RESEARCH_LATTICE_ECDLP_ROUTES.md` section 6 (src:bc450fb75021)
  has the Coppersmith ceiling and Lemma A (a lattice basis is a log).
- Effect: the reach-at-degree-d computation has a standing hypothesis; P5's new part is only the closure quotient
  test. Rank 12 stays.

### J1 quartic DL1 on random cofactor-one Jacobians. New in scope.
- Direct route on the GHS-weak family only: treatise `prog_transfer.tex` (src:34978a5873b7; DL1 0.022 of rho
  in the table), `LEAD-TRF-odd-char-descent-smith-quartic` (open_blocked, src:14c0d17f93a9): N4-v2 is "a direct
  plane-quartic IC win on the special GHS-weak family; not evidence that the Smith quotient ...". No hit for a
  random or generic plane quartic run (`random AND plane quartic ...`, `quartic AND DL1 AND k = 16`). Theory: Diem
  O~(q) on plane quartics against q^(4/3) hyperelliptic (`prog_transfer.tex`, KN-TECH-36e667 src:c3fcd667546c).
- Effect: no prior art; the atlas itself flags family dependence as the open point, which supports J1 as the
  gate for J3. Rank 4 stays.

### J2 percolation threshold for the DL1 factor-base constant. Partly anticipated.
- The mechanism (large-prime graph percolation, cycle rank, giant component): `LEAD-ICE-relation-graph-rank-large-prime`
  (open_next_step, src:0fe1b605fe94: 2LP study inconclusive at 9/9 toy cells), `H-RELN-4d092c` (src:58ceea63fe14),
  `EXP-RELN-dc761f` (src:fdbdb1956027; "A1 sat below the percolation threshold ... cycle rank = |E| - |V| + c is fixed
  by the edge count once the budget is set"), `RESEARCH_EXTENSION_FIELD_BOUNDARIES.md` (src:79dc360690a3: the 2LP
  graph is Erdos-Renyi on the large-prime set, giant component at the standard threshold), `LEAD-ICB-large-prime-variants`
  (src:fdae6d54ac81: n = 53 paired 0.604 [0.599, 0.610], binary Koblitz, crypto PR #2850).
- Not found: any tuning of the c_D constant of the quartic DL1 row (`c_D` hits are the unrelated decomposition
  count notation of KN-FIND-007).
- Effect: the threshold law is standard theory the corpus already uses; J2's novelty is the c x k grid on the E6
  row. Caution from EXP-RELN-dc761f: cycle rank is determined by the edge count and the component count, so the
  predicted "x0.65 with adaptive cycle-rank stop" must be separated from a plain edge-budget effect in the card.
  Rank 3 stays.

### J3 Tian transport and reach. Partly anticipated (stage 1 census).
- `IDEA-20261024-a29e53` (src:687227c8c486, status proposed): exhaustive existence of genus-3 Jacobians in Res E
  classes at n = 3, "existence is not descent", cites Tian 2023 (KN-LIT-b4a89c, src:bd5f1a8ba888: (l,l,l)-isogeny
  from Res E to a genus-3 Jacobian, O~(q) for some prime-order E(F_{q^3}); abstract only, "not verified here").
  `LEAD-TRF-weil-restriction-jacobian-class` (open_next_step, src:d83cb619b3af), `LEAD-TRF-odd-prime-ghs-cover-floor`
  (credential-blocked branches), `LEAD-TRF-small-genus-cover-general`, `LEAD-TRF-cover-catalog-jacobian-transfer`
  (prime balanced-Jacobian backend missing).
- Effect: J3's census of Tian kernels is a different statistic (kernel presence, odd l) from the class-existence
  table, but both are open and both sit behind the same missing isogeny engine. Keep rank 11, start only after J1;
  coordinate with the PEXT experiments so the same census is not paid for twice.

### J4 k = 4 via genus 2 over F_{p^2} with a four-point Nagao descent. Partly anticipated (neighbours).
- `IDEA-20261018-1bc873` (src:ee18a7f7a565): E(F_{p^4}) linear algebra is 0.495 of the exponent (0.420 with large
  primes); `IDEA-20261018-2c37f5` (src:fb3cafcd9988): double large primes plus D4 symmetry on E(F_{p^4}); Gaudry
  k = 4 ladder with D4 symmetry in the program papers (`intro.tex`, src:b30ee320f303).
- Nagao arm evidence: `LEAD-TRF-jv-nagao-genus2-cover-decomp` (closed_dominated, src:39bc0b130882): the best
  Nagao-only arm is 83-100x costlier than the JV sieve on the genus-3 cover; `RESEARCH_COVER_DECOMPOSITION_LEDGER.md`
  (src:917fc55d6e93). No hit for a genus-2-over-F_{p^2} cover of E(F_{p^4}).
- Effect: no direct prior; the closed_dominated verdict for Nagao arms is a negative prior for the descent. Rank 12.

### K1 twisted-line fold on E/F_16. Partly anticipated (mechanism known).
- GGMP = Galbraith, Gebregiyorgis, Murphy, Petit, "On index calculus algorithms for subfield curves", SAC 2020,
  eprint 2020/1315 (cited in src:883ab40565c1, src:f5294db1917e; record KN-LIT-796 holds the abstract only; three
  ledger records say the full text was not read: IDEA-20260906-6da433, IDEA-20260906-a77711, IDEA-20260915-8fe0ef;
  EXP-BINSTD-178742 lists "an agent reads eprint 2020/1315 at source" as an open clearing condition).
  The literature review `crypto:docs/ecdlp-literature-review-20260926.md` (src:7a2a2d0c1be8) lists
  "Frobenius-invariant factor bases on Koblitz curves" among internal results that were already known.
- The fold arithmetic is already stated: `IDEA-20260922-6cf862` (src:5a347cfdc296): "Relation-search system count
  / k ... matched rho also gains sqrt(2k), so the NET gap movement is sqrt(k/2) and no exponent moves", for
  factor bases as F_q[T]/(T^k - 1)-ideals on the c2pnb rows; K1's R0/sqrt(k) is the same bookkeeping.
  `LEAD-ICB-frobenius-orbit-factor-base` (closed_obstructed, src:d447df8e9760): compact-orbit IC 2.96x at n = 13
  rising to 18.2x at n = 41; `LEAD-ICB-composite-degree-c2pnb` (src:63e3d4412bc3): F_{2^16}-rational base
  structurally empty in G on the c2pnb rows; `LEAD-ICB-nonlinear-frob-stable-bases`, `LEAD-ICB-frobenius-search-symmetry`.
- Effect: the fold is not new; the new object is the twisted line theta F_Q (weak class j not in F_4, n = 4k,
  k odd). The owed GGMP full-text check is still owed (the corpus cannot discharge it). K1's own prior that the fold
  moves the crossover by sqrt(k) should be stated as the already-recorded sqrt(k/2) net. Rank 6 stays; the
  matched-rho arm requirement is real work.

### K2 price of flatness at m = 3, 4 (binary). Already tried (the premise).
- `LEAD-ICE-descent-closure-degree-at-scale` (open_blocked, src:ce06e82065ae) asks exactly K2's premise: closure
  degree 4 through (m,l) = (4,6) at N = 54; (3,7) degree-5 closure capped at 169 GB; "degree rises at l = 7 at
  m = 3"; ECC2K-130 m = 4 priced at +7.6 bits over rho at degree 4, +10.3 at degree 5; blocked on memory, not ideas.
  Runs cited: `crypto:research/dreg_closure_mutantxl_20261016`, `..._driver_...`, `..._m4l6_20261018`,
  `..._specialise_20261017` (run directories, not in the corpus). Also `LEAD-ICE-hybrid-guess-determine-l7`
  (src:e77625cde162: (3,7) degree 4 after fixing k* <= 2 variables), `LEAD-ICE-first-fall-degree-assumptions`,
  `LEAD-ICE-chained-semaev-dreg-vs-semiregular`, treatise `binary.tex` (src:661ee7c02e24: degree 4 to N = 42 at
  m = 4, l <= 4, none of 345 satisfiable draws).
- K2's m = 3 cells N = 27, 30, 33 are l = 9, 10, 11, beyond the l = 7 rise already recorded; its registered
  flatness prior 0.15 is not contradicted, but 15 percent should be restated against the recorded rise at l = 7 and
  the memory cap that stopped (3,7).
- Effect: demote from rank 5 to about 9; what is new is the crossover pricing n* (derived), which needs no run;
  the measurement half duplicates a blocked atlas lead. M1 + K2 should no longer be one card.

### K3 same fold at Gaudry n = 3 over F_8. Partly anticipated (as K1). Expected negative stays.
- Same records as K1; `RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (src:6cfea8b744d2) lists E2/E5 and the F_{p^3} line;
  KOBLITZ_SUBFIELD_EXPERIMENT_TRACKER.md (src:f92d50dd3851). Nothing found that measures the threshold
  3.8e4 F_Q multiplications at k = 13. Effect: none; rank 12.

### M1 arity grid at m = 3 (prime). Partly anticipated.
- F4 solving degrees at m = 3 already exist: encyclopedia section 0.1 (src:601058f8defc): D = 6 (N = 9, 12), 7
  (N = 15, 18) with width 106,288 and 12.7 GB at N = 18; `RESEARCH_PKM_TOWER_ORACLE.md` (src:13c233fff93b: 6 6 7 7
  over N = 9..18; "slope undecided"); `ic-prime` atlas card pkm-tower (m = 3: 5 6 6 7 at N = 6..15). M1's k = 3..7
  is N = 9..21, so it overlaps this range with a different object (certified closure LFD on the chain
  presentation, not F4 D on the tower quotient). The one on-disk LFD record M1 quotes (p = 7681, k = 6: LFD >= 8)
  sits against F4 D = 7 at N = 18 (t = 6); different instruments, so no contradiction is claimed.
- `LEAD-ICP-full-width-arity-grid` (open_blocked, src:b1cf7dc6df32): complete-solve arity ladder at 50-63 bits,
  stops at 32 bits / m = 7, timeout-censored; different aim (complete solves, not fall degree).
- Effect: M1's table row would be the first certified-LFD m = 3 row, not the first m = 3 degree. Cost stays low;
  note the F4 series in the card so the LFD/F4 gap is reported. Rank 5 for the prime half stands.

### S1 characteristic and generator crossing. New in scope.
- No hit for `locality AND fall degree AND characteristic AND multiplicative subgroup AND closure`, `small prime AND
  fall degree AND characteristic AND kummer`, `Frobenius AND x^p AND last fall degree AND characteristic ...`.
- Supporting records for hypothesis F (characteristic links): Lean (source-only, no receipt)
  `CryptoResearch.FallDegree.frobenius_degree_fall` (`formal/CryptoResearch/FallDegree/FirstFallBounded.lean:187`:
  g^(p^e) - Frobenius-twisted g lies in V(field equations, p^e deg g)) and
  `CryptoResearch.Semaev.free_degree_fall` (`formal/.../Semaev/FrobeniusCollapse.lean:107`: degree drop
  (p^e - 1) deg g). `H-PFDR-c88f14` (src:e59d2899ca70): the Frobenius cyclic-shift lemma of arXiv:2103.07282 "has
  no F_p analogue", so bounded-LFD theorems do not transfer to prime-field presentations and LFD must be measured.
  `RESEARCH_FALL_DEGREE_BOUNDS.md` (src:9bc2d68bd836), `KR-IC-fbdb61` (src:4a085d96e6ab: HKY bounded last fall degree
  under stated conditions).
- Effect: no prior art; the surrounding theory is consistent with "characteristic matters", which makes S1 a live
  discriminator rather than a foregone one. Rank 1 stays.

### S2 Velu tower exact LFD. Partly anticipated (F4 data exist).
- Encyclopedia section 0.2 (src:601058f8defc) and PKM oracle (src:13c233fff93b): isogeny-tower F4 D, m = 2:
  4,5,5,5,5,5,5,5,5 (pilot) / 3 5 5 5 5 5 5 5 5 6 (round 2, N = 2..20) against Kummer 3 4 4 4 4 5 5 5 5 6; m = 3:
  6,6,6 at N = 6, 9, 12 (Kummer 5, 6, 6); m = 4: 6, 7 at N = 8, 12; "rises at the same level as Kummer's, t = 10";
  generator-count ladder at N = 12 (D = 5,5,5,5,4 for g = 1,2,4,8,12); the PKM note says isogeny towers are not
  degenerate at infinity ("top(S)" survives), so "the three families may scale differently". The atlas card says
  "isogeny tower one higher".
- Effect: the F4 reading is "same law, at most +1", which is what the README already uses to lower S2's prior;
  S2 adds certified LFD and the graded-algebra test. No new reason to change rank 8.

### S3 no-wrap closedness screen. New in scope.
- No hit for `closedness AND no-wrap ...`, `evaluation matrix AND kernel AND fall degree`. Nearest: PKM oracle
  Proposition 2 (XL solving degree >= d + delta, delta about N) and the top-part algebra remarks (src:13c233fff93b).
- Effect: no prior art. Rank 7 stays; because the growth proof's Proposition 2.2 and T3 are in ml-cryptanalysis only,
  no corpus entry competes.

### Growth theorem N8 (docs/proofs/N8-growth.md, ml-cryptanalysis e8aecdd). Not in corpus; context found.
- Searches: `unbounded AND last fall degree AND kummer OR subgroup AND proof`, `N8 AND growth AND fall degree`,
  `Petit AND Kosters AND Messeng AND fall degree AND tower AND bound`, the vanishing-sum authors above.
  No in-scope statement that the prime chain LFD is unbounded, and no in-scope statement that it is bounded.
- The competing framing is recorded: HKY (arXiv:1505.02532, eprint 2015/573) bounded last fall degree for Weil
  descent (RESEARCH_FALL_DEGREE_BOUNDS.md, KR-IC-fbdb61); `H-PFDR-c88f14` says no bounded theorem transfers to the
  prime digit presentation; PKM (pkm2016) construct Kummer towers whose beat-rho condition is a tower-degree bound
  (RESEARCH_PRIME_FIELD_LEADS_20261031.md src:b8a0363e69a4); Yokoyama-Yasuda-Takahashi-Kogure's lower bound is for
  the naive presentation, not towers (src:13c233fff93b, src:6cf4fb0f198f).
- Prior conjecture the theorem bears on: L1 GAP A4 (linear growth of the solving degree in the number of levels)
  and the barrier map's k-SUM slope bar D >~ 0.04 N + c. Derived here: the proved floor 1/14 per level at m = 2
  (N = 2k) is 1/28 = 0.0357 per N, 0.89 of the 0.04 bar; the measured F4 slope in the barrier map is 0.125-0.15.
  The theorem proves a linear lower bound for the two-summand chain closure, which is the degree half of GAP A4,
  not the rank half (P1 targets that).
- Consistency observation (not verification): the F4 degrees recorded in Oct 2026 for Kummer m = 2, N = 2..20
  (t = 1..10) are 3 4 4 4 4 5 5 5 5 6; the growth note's measured formula LFD = floor((k + 14)/4) gives
  3 4 4 4 4 5 5 5 5 6 for k = 1..10 (computed here). Different quantity (F4 solving degree on the tower quotient
  against certified closure LFD), different engine and date; the match is a cross-check on the formula's low end
  only, and it cannot be claimed as independent verification because the two sequences may share the same
  underlying plateau structure.
- Citation gap: T2 is attributed to Brownawell-Masser, Voloch, Garcia-Voloch, Wang, Hsia-Wang; none is in the corpus.
  The proof is self-contained by the note's own statement; the literature entries should be added to the KB when the
  note is reviewed for a claim id.

## 4. Ranking effects, in one table

| id | classification | rank change | reason |
|---|---|---|---|
| S1 | new | 1 (unchanged) | no prior art; Lean and ledger records support a live characteristic effect |
| P1 | partly anticipated | 2 (unchanged) | measures GAP A4 rank clause of the 2026-10-31 L1 note; reword the card |
| J2 | partly anticipated | 3 (unchanged) | percolation theory used already; the c_D grid is new; separate edge-budget effect |
| J1 | new | 4 (unchanged) | no generic-quartic run in scope; atlas flags family dependence |
| M1 | partly anticipated | 5 (unchanged), split from K2 | F4 D at m = 3 to N = 18 exists; M1 adds certified LFD |
| K1 | partly anticipated | 6 (unchanged) | fold and sqrt(k/2) net already recorded; GGMP full text still owed |
| S3 | new | 7 (unchanged) | no prior art |
| S2 | partly anticipated | 8 (unchanged, prior stays low) | F4 degrees: isogeny = Kummer or +1 |
| P2 | new | 9 -> about 7 (suggested) | corpus independently names large memory as the blocker |
| K2 | already tried | 5 -> about 9 | premise duplicates the blocked closure-degree-at-scale lead |
| P3 | already proposed | 10 -> merge into barrier-map 6.1 | same question, prediction and falsifier already registered |
| J3 | partly anticipated | 11 (unchanged) | census overlaps IDEA-20261024-a29e53; same missing engine |
| J4 | partly anticipated | 12 (unchanged) | Nagao arm closed_dominated at genus 3 |
| P5 | partly anticipated | 12 (unchanged) | degree-d hypothesis H-ECDLP-30a2a1 exists |
| K3 | partly anticipated | 12 (unchanged) | as K1; expected negative |

## 5. Sources checked (receipt)

`kb_status` equivalent: manifest.json and `state` table above. `search_knowledge` equivalent: FTS5 queries in
`queries/` and inline. `search_theorems` equivalent: `declaration_fts` queries `fall AND degree`, `closure AND ideal`,
`semaev`, `vanishing AND ideal`, `unit`: two relevant declarations (frobenius_degree_fall, free_degree_fall), both
`source-only`; no declaration about closure ideals, kummer chains or the growth bound, so no Lean claim here can cite
a kernel-checked counterpart. `get_source` equivalent: `tools/doc.py` / `tools/ctx.py` over the passages of each cited
source. Atlas cards read: pkm-tower-algebraic-oracle, prime-field-semaev-solving-degree, descent-closure-degree-at-scale,
digit-presentation-solving-degree, coppersmith-smallroot-semaev, algebraic-membership-factor-base, full-width-arity-grid,
relation-graph-rank-large-prime, large-prime-variants, TRF-small-genus-cover-general, TRF-odd-prime-ghs-cover-floor,
TRF-weil-restriction-jacobian-class, TRF-cover-catalog-jacobian-transfer, TRF-odd-char-descent-smith-quartic,
TRF-jv-nagao-genus2-cover-decomp, ICP-gfp5-oa-sdhp-jv, ICP-gfp5-odd-char-cover, ICB-frobenius-orbit-factor-base,
ICB-composite-degree-c2pnb, ICB-frobenius-search-symmetry, ICB-nonlinear-frob-stable-bases,
ICE-first-fall-degree-assumptions, ICE-chained-semaev-dreg-vs-semiregular, ICE-curve-presentation-invariance,
ICE-hybrid-guess-determine-l7, ICE-sat-xor-pdp-oracle, ICP-isogeny-transported-relation-yield,
STR-p192-class-cm-relations, STR-endo-scalar-action-generic, STR-isogeny-class-generic-invariance,
ICP-extension-field-embedding.

## 6. Not done, with the next command

- GGMP (eprint 2020/1315) and Tian 2023 full texts: not in the corpus; read the PDFs (the lane agent's fetch returned
  403). Next: `curl` through the proxy is blocked for eprint in the earlier session; use the paper collection
  (`research-pdfs` repo) if the PDF is added, then `research-kb refresh`.
- Brownawell-Masser / Voloch / Hsia-Wang: add to `research-pdfs` and re-index, then re-run
  `python3 -I tools/q.py 'Brownawell AND Masser'`.
- A hybrid-ranked pass with the real MCP server (`kb_status`, `search_knowledge`) once the session has it, for the
  negative forms that returned nothing here (P2, S1, S3, J1).
- Re-run this pass against a snapshot taken after 2026-11-14 that indexes `ml-cryptanalysis` and the dated run
  directories; those would confirm or break the "new" labels for P2, J1, S1, S3.
- Tools in `tools/` hard-code the scratchpad catalog path; edit the `file:` URI in `q.py`, `doc.py` to rerun elsewhere.
