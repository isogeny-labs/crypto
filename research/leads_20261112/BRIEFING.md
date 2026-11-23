# Briefing: new index-calculus techniques for ECDLP (2026-11-12)

Owner's request (verbatim intent): "look at some of these techniques for prime and Koblitz and everything. Some are
still open; think of other ones too, techniques for index calculus; for the ones that look promising figure out next
steps."  Deliverable: falsifiable proposals with the cheapest discriminating test, for a main-session triage.

## The lead atlas brief (image, "Factor-base families tried, by field regime", verdicts as recorded, 2026-11-12)
Legend: REF = refuted/negative; ENG = engineering/constant factor only; OPEN = open/inconclusive; JAC = genus>=2
Jacobian (not an ECDLP route alone); WEAK = below rho on a weak class only. Flagship families (black border): 0.1, 0.2.

Prime fields E(F_p):
 0.1 Kummer g*mu_{2^t}: F4 at P-224 ........ OPEN (flagship)
 0.2 Isogeny x(R'+<Q>): V at P-192 ........... OPEN (flagship)
 A1 small abscissae (baseline) ............... REF
 A2 geometry family (FB3) .................... REF
 A3 automorphism orbits ...................... ENG
 A4 subgroup/coset, one binomial ............. REF
 A5 Dickson; P-256 torus (24 rds) ............ REF
 A6 P-256 affine bit-box ..................... REF
 A7 P-256 PGL2 root orbit .................... REF
 A8 Eisenstein-smooth (j=0) .................. REF
 A9 arith. progression; mod-l ................ REF
 A10 fixed-degree / map images ............... REF
 A11 isogeny-augmented, class numbers ........ REF
 A12 height box, degree-d maps ............... OPEN
 A13 coset vs low-x/random (stage) ........... REF
 A14 hashed affine-threshold (tensor) ........ REF
Extension fields E(F_{p^k}):
 B1 Gaudry-Diem x in F_p, k=2..5 ............. REF
 B2 psi / GLS / eigenline folds .............. ENG
 B3 exotic coordinates ....................... ENG
Binary fields E(F_{2^n}):
 C1 coordinate window (FPPR) ................. REF
 C2 random subspace .......................... REF
 C3 subfield F_{2^l} ......................... REF
 C4 shapes: G1,G3,G5 killed .................. REF
 C5 trace-zero ............................... ENG
 C6 Hamming ball ............................. REF
 C7 quasi-subfield ........................... REF
 C8 GG / Shantz-Teske ........................ ENG
 C9-C11 affine, adapted, quotient ............ REF
Koblitz curves:
 D1 Frobenius-invariant subspace ............. ENG
 D2 Frobenius-orbit unions ................... ENG
 D3 Z[tau] multiplier closure ................ ENG
 D4 cyclotomic: yield-neutral ................ OPEN
 D5 u-frame symmetrised ...................... REF
 D6 prefix Frobenius closure ................. ENG
 D7 isogeny-transported ...................... REF
 D8 N83 orbit bases: online only ............. ENG
 D9 one/two large primes ..................... ENG
 D10 solver-cost selection ................... ENG
Jacobians and covers:
 E1 degree-1 places, g=2..4 .................. JAC
 E2 Joux-Vitse g3 cover ...................... WEAK
 E3 P-192 CM class ideals .................... OPEN
 E4 GHS covers ECC2K-130 ..................... REF
 E5 Smith quartic codomain ................... REF
 E6 genus-3 cover, IC-2LP .................... OPEN

## What is settled today about the fall degree of the flagship family 0.1 (two-summand prime chains, x_i in a
coset of mu_{2^k}; repository isogeny-labs/ml-cryptanalysis, PR #101, clone at /home/user/ml-cryptanalysis)
- kernel (Lean, 547 declarations, no axioms): LFD <= k + 2 for every k >= 2, every field with 2 != 0, every
  nonsingular curve, every target (`SelectorExtension.lfd_le_depth_add_two_of_two_ne`). Bound attained at k = 2.
- kernel relative to four GPU closures as named hypotheses: LFD = 6 at k = 10..13, 7 at k = 14, >= 7 at k = 15
  (unit instance over p = 65537).  Measured law LFD(k) = floor((k+14)/4) to k = 15: conjecture beyond.
- written proof, reviewed today (docs/proofs/N8-growth.md, review in N8-growth-review.md): LFD is unbounded,
  >= floor((k-10)/14) + 1 for unit instances with t != 0, by a characteristic-p abc/Wronskian locality argument on
  the curve S_3(x1, x2, t) = 0: a low-weight polynomial vanishing on the curve cannot connect exponents 2^k apart,
  so wrap-around (x^N = 1) cannot be used at low degree.  Open: slope between 1/14 and 1 (measured 1/4), t = 0,
  m >= 3 summands (needs a higher-dimensional abc theorem), singular curves.
- Consequence for 0.1: F4 degree grows linearly in t = log2 of the factor-base size: cost superpolynomial in the
  base size.  Anything proposed for prime fields must say how it escapes this (different presentation, different
  base structure, different solver, or exploits the known exact degree).
- Binary deep chain today (interim, 16 of 57 systems, unverified): at k = 11 the discriminating cell gives LFD 4 on
  all 7 affine systems (law B predicted 5); trace-zero k = 11 gives LFD 5 (every law); units at n = 2k - 1 give 4.

## Already proposed (do not re-propose; name the nearest one and the difference)
- 176 atlas lead ids: atlas_lead_ids.txt (the atlas bodies are vault-encrypted; only ids are known here).
- 173 ids of the 2026-11-10 sweep: EXISTING_LEADS.txt.
- 13 leads of PR isogeny-labs/crypto#3938 (2026-11-11): lane_A_falldegree.md, lane_B_binary_ic.md,
  lane_C_prime_ext_cover.md.  Two were tested 2026-11-12: Hilbert early stop at Nagao scale held (counted S x 0.528,
  below matched rho at p = 1451, 1601, unvalidated); selector eta-kernel conjecture falsified at e_min.

## Evidence on disk (cite paths; quote numbers, never re-derive from memory)
- /home/user/crypto/docs/ecdlp-hardness-corpus/ATTACK_GRAPH.md ("Frontier"), WEAKNESSES.md (live obstructions),
  LEMMA_LINKAGE.md, LITERATURE_SEARCH.md, fall-degree-lemmas.json, lit-groebner.json.
- /home/user/crypto/docs/ic/LEADERBOARD.md (every IC row is above its matched rho except the GHS-weak family;
  best cold matched ratios 1.27x-1.57x on f2m29 Frobenius-orbit IC, 2.59x on fp36 residual collector).
- /home/user/crypto/AGENTS.md: "Success criterion", "Name the obstruction", "The rule: boundary, table, ratio",
  section 5a (price unpriced work at 0, 0.1, 1 GAE), validation gate.
- /home/user/ml-cryptanalysis/docs/proofs/*.md, experiments/ic-bounds-k14/RESULTS.md,
  experiments/ic-bounds-binary-deep/PLAN.md.
- /home/user/cryptanalysis (Rust engines: src/cryptanalysis/jv_cover.rs, jv_sieve.rs, gaudry_quartic.rs).
Constraints: research-kb is NOT reachable from this session (say so in "Sources checked"); implementation
language for anything landed is not Python; every proposal names its Lean obligation if the program would rely on a
mathematical claim; every cost band prices every arm including the matched rho reference.

## Output
Write ONLY to the file named in your prompt under this directory's out/ subfolder.  Format: a short "Sources
checked" list; then 3 to 6 proposals as YAML records (id, title, claim, mechanism, prediction, discriminating_test,
falsification, cost_band, controls, risks, closest_baseline, nearest_existing_lead_and_difference,
lean_obligation, next_steps as an ordered list of at most 5 concrete actions with owner tier and time); then a
ranking with one sentence each; then "Considered and not proposed (with the reason)".  Do not edit any repository file.
