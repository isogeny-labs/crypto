# Red-team review: `research/split_prime_ecdlp_20261107/` (sections 1–5, 7) and `problems/split-prime-pattern-scalar-only.md`

Reviewer: red-team (adversarial), 2026-11-07. No file under review was edited.

## Evidence reviewed (pinned)

- Branch `claude/novel-ecdlp-algorithm-iams4j`, HEAD `5e9c756773ba01b3ce750c06e61e6eb0fec56068` (local, committed during the
  review; the dispatch described an uncommitted tree). Sections 1–5 of `README.md` hash
  `79c5dde5…f353e` and sections 7–9 hash `5de1e04f…cfbcc` in both HEAD and the working tree I read; §6 is out of scope and
  was being edited during the review. Whole-file sha256 at HEAD: README `2176ad4f…178b`, problem record `11e02570…bada8a`.
- `tools/splitcount.rs` sha256 `cb1dc02c…7550` (matches §8); `data/recount.json` `e84f89d3…7d98` (matches §8);
  `data/uploaded_report_values.json` `df7d9ac4…0896`; `pcc/claims/PCC-20261107-3403f9.json` (standing unverified).
- Report text: `/tmp/claude-0/-home-user/0af71fd6-cb66-5cbf-9292-d617eb53cca4/scratchpad/pdf/report.txt`.
- Repository files opened for cross-checks: `RESEARCH_ISOGENY_FIBERS_ECDLP.md` §15.1 (Theorem 15.1),
  `research/isogeny_class_difficulty_20261008/README.md` (framing theorem),
  `research/isogeny_class_ic_yield_20261102/p2_prime_analogues/RESULTS.md`,
  `research/nist_skip_criterion_20261025/strict_elkies_20261029/results/at691.tsv`, `problems/README.md`,
  `docs/session-logs/2026-11-07.md` (entry 21:05Z). research-kb was unavailable; no KB citation is made.
- My own scripts (scratchpad, not in the repo): `rt/indep.py` (independent big-integer recount in Python),
  `rt/gl2check.py` (brute-force GL2 count), `rt/delta.py` (Δ factorisation, conductor, embedding-degree bound).

Durable-evidence note: the review is of commit `5e9c7567`. Anything later in the working tree is not covered.

## Verified mechanics (confirmed)

- **Rebuild.** `rustc -O --edition 2021 tools/splitcount.rs` → output byte-identical to `data/recount.json`
  (sha256 `e84f89d3…7d98`). Confirmed.
- **Hex parameters.** All ten (p, n, cofactor) match `openssl ecparam -name <curve> -param_enc explicit -text -noout`
  (OpenSSL 3.0.13; leading `00`/`01` is the DER sign byte). Confirmed.
- **Independent recount.** Python big-integer implementation (exact Legendre symbol of Δ, not of Δ mod ℓ by Horner)
  reproduces every split/ramified/inert count, expectation, sd, z and L1 ratio of `recount.json` at all 12 cutoffs for all
  10 curves (0 mismatches at 1e-3). Confirmed.
- **Baseline formula.** Brute force over GL2(F_ℓ) for ℓ ∈ {3,5,7,11,13} and every determinant d confirms
  #{trace τ, det d} = ℓ² + ℓ·χ(τ² − 4d), P(split) = (ℓ − 2 − χ(d))/(2(ℓ − 1)), P(ramified) = (1 + χ(d))ℓ/(ℓ² − 1).
  The removed trace class τ ≡ p + 1 has discriminant (p − 1)², mass (ℓ + 1)/(ℓ² − 1) if p ≢ 1 (split class) and
  ℓ/(ℓ² − 1) if p ≡ 1 (ℓ | Δ class); the code (`classify`) implements exactly that and renormalises. The forced case
  (p ≡ 2 mod 3 ⇒ ℓ = 3 always inert for a prime-order curve) comes out as P(split) = 0, variance 0, as it must. Confirmed.
- **Transcription.** All 10 × 5 per-curve values and the 12 pooled values in `uploaded_report_values.json` match the
  report's §3.2 table and §3.3 list. Confirmed. (The report itself says P-256 "×1.30" in §4 against ×1.29 in its table;
  the transcription took the table.)
- **Agreement claim of §4, as scoped.** Counts at 691, per-curve z at 691/5,000/50,000, pooled z at 12 cutoffs: all
  equal the report's printed values after rounding. The `strict691` column of `at691.tsv` gives the same ten counts and
  its `ramified691` column (3,1,1,3,1,2,1,2,0,1) equals the recount's ramified counts at 691. Confirmed.
- **Stouffer vs count-pooled z** agree to two decimals at every cutoff (borderline at B = 100: −2.708 vs −2.715).
- **"No ℓ ≤ 200,000 divides any n"**, "124 odd primes at 691", `plot.py` reads only the two JSON files. Confirmed.
- **Embedding degree.** p^k ≢ 1 (mod n) for all k ≤ 20,000 on all ten curves, so the §3 remark "small embedding degree
  … which these curves do not have" is supported to that bound. gcd(n, Δ) = 1 on all ten (needed for s_α on
  O ⊋ Z[π] and for the ray-class-field step). Confirmed.

## Findings, ordered by severity

### F1. (medium, unsupported) "The only ECDLP-adjacent effect" (§5) and "the pattern alone … cannot" (§3 corollary) are proved only inside the generic model; the one coordinate-level route that could carry a *pattern* effect is untested, and the cited null cannot test it.

- Evidence: README §3 "What the theorem does not cover" closes the coordinate route by citing the P2 within-class null
  (`isogeny_class_ic_yield_20261102`). But Δ, hence the whole split/inert pattern, is an isogeny-class invariant
  (`isogeny_class_difficulty_20261008/README.md`: "Class invariants that cannot vary … `D_π`"). A within-class null
  (fixed Δ, varying model) says nothing about whether the pattern, which varies only *between* classes, correlates
  with coordinate-method yield. The README's §5 claim "that is the whole effect" is therefore supported only for
  algorithms covered by Theorem 15.1 / §3, and should be scoped that way.
- What is actually established: within the GGM augmented with isogeny and scalar oracles, no effect (Theorem 15.1,
  [pcc:unverified] there). Outside it: nothing measured; no known mechanism either way.
- Cheapest discriminating test: at 24–32 bits sample prime-order curves (not a class), bin by the recount statistic
  (split count ≤ B or the L1 ratio), run the P2 `ic.pipeline` yield measurement with its A/A and planted controls, and
  report the between-bin ICC with a pre-registered kill line. The P2 tooling already exists; this is one protocol
  amendment, not new code.

### F2. (medium, unsupported/omitted) The coverage list in §3 drops an exception that the cited prior theorem names: the p-adic canonical lift.

- Evidence: `RESEARCH_ISOGENY_FIBERS_ECDLP.md` §15.1 "Reading": "The two exceptions sit outside its hypotheses on [pcc:unverified]
  purpose: the Weil/Tate pairing on the [r]-fiber … and Smart's attack uses a lift to Q_p." README §3 carries the
  pairing exception but not the lift. Yet §3's own Shimura paragraph introduces the canonical-lift torsion points
  P̃, Q̃ and says "the group is presented by coordinates, never by residues in O/N". That is incomplete: since
  (π) ≡ 1 mod N, the prime above p splits completely in K_N, so Ẽ[N] ⊂ Ẽ(Z_p) and every P ∈ E(F_p) has a *computable*
  canonical lift (Hensel to the unique n-torsion point of the Serre–Tate lift). This is a third presentation of the
  group — p-adic coordinates — that is neither one of the theorem's maps nor an F_p-coordinate read, and it is not
  covered by the measured null. It is Z-linear (an isomorphism), so it does not break the theorem; it breaks the
  README's claim to have listed what the theorem does not cover.
- Status: believed closed by known results (p-adic log vanishes on torsion; Ẽ(Z_p) ≅ Z_p ⊕ E(F_p) canonically, so
  the lift re-encodes the same group; Silverman's SAC 2008 survey on lifting). Not a route to √n, but the note should
  name it, as Theorem 15.1 did. No test needed; one sentence and a citation.

### F3. (medium, wrong in the summary, right in the body) "Every number agreeing with the report" is false for the L-column; §4's scoped statement is correct, the one-paragraph answer and the problem record are not.

- Evidence: README answer paragraph "every number agreeing with the report"; problem record "Every number matches the
  uploaded report"; `report/REPORT.typ` line 20 "every number agrees". The README's own last column differs from the
  report's "× typical" by a factor 0.78–0.80 for seven curves (P-192 0.93 vs 0.74; prime192v2 1.20 vs 0.95; prime192v3
  0.80 vs 0.63; prime239v1 0.56 vs 0.44; prime239v2 0.86 vs 0.68; prime239v3 0.93 vs 0.73; P-384 0.62 vs 0.49) and
  agrees for three (P-224 0.64/0.64, P-256 1.27/1.29, P-521 1.01/1.04). The calibration (4,267 curves) and percentiles
  were not reproduced, as §4 says.
- Fix is editorial: the summary and the problem record must carry §4's scope ("counts and z at the listed cutoffs").

### F4. (medium, plausible; the README conceals a likely defect of the uploaded report) The 0.78× systematic gap in the L-column is explained by the report normalising all ten curves to one field's typical value; the README attributes it to "calibrated median vs exp E[log]".

- Evidence (my `rt/indep.py`): report_L1 / recount_partial_L1 implies a single "typical" of 0.833–0.843 for all ten
  curves, independent of field. The field-specific model expectation `model_expected_L1` is 0.6624 for the three
  p ≡ 2 (mod 3) fields (192, 239, 384) and 0.8448 / 0.8526 / 0.8636 for P-224 / P-256 / P-521 (p ≡ 1 mod 3). Mechanism:
  for a prime-order curve over p ≡ 2 (mod 3), ℓ = 3 is forced inert (Δ ≡ t² + 1 ≢ 0, and t ≡ 0 is the excluded
  class), so the Euler factor at 3 is exactly 3/4; for p ≡ 1 (mod 3) it is 3/4 with probability 2/5 and 1 otherwise.
  The report's Figure 4 caption says its bands "come from the calibrated baseline (P-224 field)". So the report's
  "prime239v1 ×0.44 and P-384 ×0.49 outside the 95% band" is at least partly a field-mismatch artefact; under the
  recount's own field-matched normalisation they are 0.56 and 0.62.
- The README's description of its column is honest about what it computes, but "the ordering agrees" (see F5) and the
  explanation offered steer the reader away from this. The study is not obliged to audit the report's L-claims, but
  it may not imply agreement where its own numbers show a 20–25% systematic disagreement with a visible cause.
- Cheapest test: none needed beyond what is already in `recount.json`; print `model_expected_L1` per field next to the
  column. A one-line statement settles it.

### F5. (low, wrong) "the ordering agrees (prime239v1, P-384 and P-224 lowest; P-256 highest)" misstates the report.

- Evidence: report table, ascending: prime239v1 0.44, P-384 0.49, prime192v3 0.63, P-224 0.64, prime239v2 0.68,
  prime239v3 0.73, P-192 0.74, prime192v2 0.95, P-521 1.04, P-256 1.29. Recount: prime239v1 0.56, P-384 0.62,
  P-224 0.64, prime192v3 0.80, prime239v2 0.86, P-192 0.93 = prime239v3 0.93, P-521 1.01, prime192v2 1.20, P-256 1.27.
  The report's third-lowest is prime192v3, not P-224; the orderings also swap (P-192, prime239v3) and
  (prime192v2, P-521). Only "bottom two and top one agree" is true.

### F6. (low, mislabelled/unpriced) "Ramified" and "L(1, χ)" are used for (Δ/ℓ), which is not the field character at conductor primes; the report's caveat on this is dropped.

- Evidence: §1 defines χ(ℓ) = (Δ/ℓ) and calls 0 "ramified"; `splitcount.rs` doc comment and JSON keys say
  "partial Euler product for L(1, χ)". My `rt/delta.py`: 3³ ‖ Δ for P-224 and 5² ‖ Δ for prime239v2, so 3 | f (P-224)
  and 5 | f with 5 ∤ D_K (prime239v2); those ℓ are *not* ramified in K, yet enter the product as factor 1. The report
  says so ("primes dividing the conductor f are miscounted"); the README does not. The z-statistic is unaffected
  (the model is about Δ mod ℓ, not D_K), so this is a labelling issue in §1 and the column header, not a numerical one.

### F7. (low, wrong citation detail) The P2 power quote is misread.

- Evidence: README §3 "planted-20 % power 0.84–1.00 in every informative cell". `RESULTS.md`: informative cells have
  power ≥ 0.99; the 0.84 cell (c24B_m3) and the 0.38 cell are explicitly "not informative at 20%" and "not counted as
  nulls". Should read "≥ 0.99 in every informative cell; two 24-bit m = 3 cells not informative".

### F8. (low, plausible conclusion / wrong reason given) The corollary's inheritance of Shoup's bound is right, but not for the reason stated.

- Evidence: README §3 corollary: "the maps cost at least one group operation each to evaluate on a point." Cost is
  not what makes the bound hold; a unit-cost scalar or transport oracle still gives Ω(√n) queries because each answer
  is the encoding of a degree-1 polynomial in the secret (Theorem 15.1's proof, which the README now cites as the
  prior form). Replace the cost sentence with that observation. Also "Shoup's bound" as published covers a single
  encoding with add/negate; the multi-encoding-plus-oracles version is Theorem 15.1 ([pcc:unverified]) or Maurer's
  abstract model, not Shoup verbatim.

### F9. (low, hidden assumptions, all harmless but unstated) Items the dispatch asked about.

- **End(E) = O vs Z[π]**: immaterial for the scalar conclusion (any endomorphism of a cyclic group of prime order is a [pcc:unverified]
  scalar). What is used silently is that every endomorphism of an *ordinary* curve is F_p-rational (End is
  commutative and contains π), so α preserves E(F_p). Lenstra's isomorphism is needed only to *name* the scalar, and
  for α ∉ Z[π] the formula s_α = u + v needs the denominator invertible mod n (holds: gcd(n, Δ) = 1, verified).
- **ℓ = n and ℓ = p exclusions** are unnecessary and leave an apparent gap: the two F_p-rational n-isogenies have
  kernels E[π − 1] = E(F_p) (kills the group) and E[π − p] (the endomorphism π − p, scalar 1 − p); ℓ = p gives
  Frobenius (1) and Verschiebung (p). Theorem 15.1 already includes degree divisible by r. Say so.
- **Ramified / inert / vertical**: covered by item (1) for any ℓ-power degree regardless of χ; see F6 for the label.
- **Non-F_p-rational isogenies** (inert ℓ, or Elkies kernels over F_{p^k}) are not in the theorem's statement; they
  inject E(F_p) into E′(F_{p^k}), still DLP-preserving, no gain known. Worth one sentence.
- **Pairings**: the bilinearity argument (t^ℓ = t^n = 1 ⇒ t = 1) is correct; the ℓ = n case is the MOV pairing and is
  excluded by embedding degree, verified > 20,000 here.
- **Shimura reciprocity paragraph**: correct as stated, given O^× = ±1 (j ∉ {0, 1728}: a = −3, b ≠ 0) and N coprime to
  the conductor (gcd(n, Δ) = 1). One imprecision: the Galois element sends the Weber value h(P̃) to h(Q̃), i.e. P̃ to
  ±Q̃, which the "/±1" absorbs. See F2 for what the paragraph omits.

### F10. (low, process) Status labels and claim record.

- README header says "lead closure"; the problem record says `open`. Both are defensible only if "closure" is scoped to
  the scalar route; the header should say so.
- `PCC-20261107-3403f9` is `kind: statement`, standing unverified; its `producer.commit` `1f9cd6a2` predates the files
  (fine as a timestamp, but the claim is anchored by the recount.json sha256, not by a commit containing the code).
  The README's "[pcc:unverified]" labels are correct. The at691.tsv census is a genuine third implementation of the
  count (its `ramified691` column also matches), which §4 states correctly.

## Not found

- No error in the theorem's three items or in their proof sketch. The content is: homomorphisms between cyclic groups
  of prime order are scalars or isomorphisms; Z-linear data cannot see a point of order n through ℓ-structure with
  ℓ ≠ n. That is correct and is Theorem 15.1 specialised, as the README now says ("it is not new").
- No arithmetic error in the recount, its baseline, its conditioning, its hex inputs, or its transcription.

## Cheapest falsification of the obstruction

There is none for the theorem as stated: it is a statement about group homomorphisms and I could not construct a [pcc:unverified]
counterexample; a toy numerical check (a 24-bit prime-order class, every rational ℓ-isogeny and every α = u + vπ
verified to act as the predicted scalar/isomorphism) would only guard against sign/convention slips in s_α = u + v.
The falsifiable *scope* claims are F1 and F2: the between-class yield experiment in F1 is the cheapest test that
could show the pattern has an ECDLP-adjacent effect outside the expander degree; F2 needs a sentence, not a run.
