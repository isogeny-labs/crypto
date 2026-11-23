#set page(margin: 1.5cm)
#set text(size: 9pt)
#show table: set text(size: 6.5pt)
#set table(stroke: 0.4pt + gray, inset: 3pt)
#set image(width: 100%)
#show figure.where(kind: image): set figure(supplement: none)
= Novel strategies for the point decomposition problem: a desk evaluation against the lead atlas
<novel-strategies-for-the-point-decomposition-problem-a-desk-evaluation-against-the-lead-atlas>
#strong[Date.] 2026-10-31. #strong[Question (owner).] "Can we evaluate any novel strategies here for PDP?" #strong[Status.] Desk evaluation. Nothing in this directory is a measured improvement, and none is claimed (success criterion: a verified, measured improvement over the current index-calculus baseline; the gap to rho is distance remaining, never a gate). #strong[Tools.] No build of the crate was possible: the container’s disk allowance was spent, so every number below is either read from a committed record, derived from one, or taken from a primary source.

== 1. Answer in six lines
<answer-in-six-lines>
+ #strong[Nothing here is new to the repository.] Of 28 candidate strategies I could name (the three I suggested earlier, ideas from the PR \#2423 survey, and further brainstorm), 17 collide with an existing atlas lead or known-result row, 7 partly overlap one, and 4 have no real match (§2). The four with no match are the ones I rate least promising (§2.2).
+ #strong[The atlas does not contain the merged PR \#2423 work] (LEAD-19, the 19-lead survey): a search for `lead19`, `projected base` and `pdp_literature_leads` in `knowledge/leads/ecdlp/` on `main` (2026-10-31) finds nothing. A draft lead row for it is in §6.
+ #strong[Why replacing summation polynomials cannot move today’s baseline.] The ladder rows are not solved with summation polynomials at all. They use a pair table and a walk, and on arm B at the published rung the walk is 61–82 % of S and the matrix is at most 0.10 % (§3, measured). An algebraic-oracle lever (solving degree, hybrid guessing, SAT, a tensor encoding) cannot touch those terms. On the committed algebraic-oracle rows the oracle’s share of S is exactly zero when native operations are unpriced, so its ceiling is 1.00× (#link("cards/e1.md") appendix, derived).
+ #strong[Of 14 lead cards (13 atlas leads plus my LEAD-15): 1 PROMOTE, 6 REVISE, 7 CLOSE] (§4). The one promotion is LEAD-15, the only candidate that changes an exponent on a family the repository can build, with a decisive first test that costs under one CPU-hour once an instance builder exists.
+ #strong[The cheapest decisive item changes an existing baseline, not a lead.] The committed volcano study runs its IC arm on a raw (unprojected) factor base over curves with cofactor 4 and 12, which is the regime LEAD-19 showed wastes a fraction `1 - 1/h` of every folded table (§5.2). I verified the premise; the size of the effect is unmeasured.
+ #strong[Corrections found] in the atlas, in my LEAD-15 dossier and in my own brief to the evaluators (§7).

== 2. Novelty audit
<novelty-audit>
Method: each candidate was run through `python3 tools/lead_atlas.py --match` (153 leads, reviewed 2026-10-30; sparse clone of crypto-autoresearcher `main`). The matcher is lexical, so I read every top hit and recorded my own reading in #link("candidate_judgements.tsv")[candidate\_judgements.tsv]; the raw hits are in #link("atlas_match_log.tsv")[atlas\_match\_log.tsv].

#figure([#image("figures/novelty_and_verdicts.png")],
  caption: [
    Novelty audit and verdicts
  ]
)

=== 2.1 What the atlas already holds on the three directions I suggested earlier
<what-the-atlas-already-holds-on-the-three-directions-i-suggested-earlier>
My earlier answer to "other areas of PDP" proposed odd-characteristic covers, degree of regularity as a search objective, and hybrid guessing. All three are atlas leads: `LEAD-TRF-odd-prime-ghs-cover-floor` (open), `LEAD-ICE-linearization-excess-fb-prior` (open\_unexamined) and `LEAD-ICE-hybrid-guess-determine-l7` (open\_next\_step). That answer was given before I had read the atlas; it was not new.

=== 2.2 The four with no match
<the-four-with-no-match>
These are my first-principles notes, #strong[not verified and not evaluated by an agent]:

- #strong[Sieving over a lattice of x-coordinates instead of solving.] Closest lead: `LEAD-TRF-jv-sieve-fp6-weak-class` (a sieve inside one weak family). For a prime-degree binary extension there is no smoothness notion on `x` to sieve, so I see no object to sieve.
- #strong[Relations on the quadratic twist.] A twist point gives a relation in the twist’s group, which does not involve the target’s discrete log; I see no use.
- #strong[List decoding (Guruswami–Sudan) as a formulation.] There is no received word. "All roots in the subspace V" is the divisibility `h | L_V` of the linearised polynomial, which is the form `LEAD-ICP-shared-coefficient-membership` examines (verdict REVISE, expected null, §4).
- #strong[Hankel / Berlekamp–Massey rank method.] I find no linear recurrence to exploit; the matrix methods that apply (Macaulay, bilinear XL) are already in the repository.

== 3. Where S goes
<where-s-goes>
Recomputed from `research/lead19_projected_base/char2_repeats20.json` by #link("phase_shares.py")[phase\_shares.py] (output in #link("phase_shares.json")[phase\_shares.json] and #link("phase_shares.md")[phase\_shares.md]). Arm B at the published rung, mean of 20 repeats:

#figure([#image("figures/s_phase_shares.png")],
  caption: [
    Phase shares
  ]
)

#align(center)[#table(
  columns: 7,
  align: (col, row) => (left,left,right,right,right,right,right,).at(col),
  inset: 6pt,
  [row], [variant], [S], [factor base], [walk], [matrix], [verify],
  [f2m15 (h\=2)],
  [`…_walk_projected`],
  [12.35],
  [17.2 %],
  [81.5 %],
  [0.103 %],
  [1.21 %],
  [f2m21 (h\=2)],
  [`…_walk_projected`],
  [10.11],
  [38.3 %],
  [61.4 %],
  [0.038 %],
  [0.26 %],
  [f2m27 (h\=6)],
  [`…_walk_projected_balanced`],
  [14.75],
  [24.9 %],
  [75.0 %],
  [0.010 %],
  [0.05 %],
)
]

Consequences (the arithmetic is in #link("cards/e1.md") and #link("cards/e3.md")):

- The levers that remain on these rows are cost per candidate in the walk, the number of targets K and target sharing, and walk restarts. The yield per group addition is fixed (KN-FIND-007).
- On the three-summand rows the same record gives `S(m=3 walk) / S(m=2 walk)` \= 1.62, 1.70, 2.53, 2.23, 3.90 on the raw base (ratio of means), so three-summand probes are not a free upgrade.
- An oracle-only gain has a ceiling of exactly 1.00× when native operations are unpriced, and the oracle’s share exceeds 50 % only above about 3e-6 to 7e-5 GAE per word XOR (E1 appendix; derived).

== 4. Verdicts
<verdicts>
Fourteen cards from five evaluators (Sonnet 5.5, desk only; cards are unedited in #link("cards/")). "Decisive test" is the evaluator’s, with its frozen kill criterion.

#align(center)[#table(
  columns: 4,
  align: (col, row) => (left,left,left,left,).at(col),
  inset: 6pt,
  [lead], [verdict], [one-line reason], [decisive test (cost)],
  [#strong[LEAD-15] Diem–Thomé plane quartic on the cubic-GHS cover],
  [#strong[PROMOTE]],
  [the one exponent candidate on a buildable family (Õ(q) vs Õ(q^{4/3}) for the hyperelliptic genus-3 baseline); model predicts 9–40× below the hyperelliptic incumbent at q\=2²²],
  [instance builder (q ∈ {257, 2053, 32771, 2²²}, 6 curves each), then the full harvest; kills K1–K4 on pair yield, cycle count, nullity, projected total; run \< 1 CPU-h, builder 2–3 person-days],
  [LEAD-TRF-odd-char-descent-smith-quartic],
  [REVISE],
  [dominated by LEAD-15; atlas text misstates the exponent; blocker stale],
  [W3a Q1 exhaustive for p ∈ {7…23}; kill pass(E) \< 10 % (hours)],
  [LEAD-ICE-linearization-excess-fb-prior],
  [REVISE],
  [baseline subspace already is the top-e one; gain over it is 0; a stage diagnostic cannot move S],
  [redraw the ℓ\=7 cells (m\=3, n\=19, 21) with prefix and geometric bases, 8 UNSAT draws each; kill ≤ 3/16 (1–2 h, 5–8 GB)],
  [LEAD-ICP-shared-coefficient-membership],
  [REVISE],
  [expect a null: per-root tower grows 2.4–2.5 bits per unknown vs a ~0.30 budget],
  [ladder over t \= 3, 4, 5; kill ≥ 1.5 bits per unknown],
  [LEAD-ICE-hybrid-guess-determine-l7],
  [REVISE (low)],
  [arithmetic reproduced; E8 can close the lead but never promote it],
  [extend R1 past 3 variables, ℓ\=8 on ≥ 2 UNSAT draws; kill any k\* ≥ 3 (6–12 h, 7–12 GB)],
  [LEAD-ICB-nonlinear-frob-stable-bases],
  [REVISE (low)],
  [zero effect on any current row; uncited prior art refuted the same idea],
  [scaling-first diagnostic on n ∈ {19, 29, 37, 53}; kill R(n) ≥ 0.5 by n\=37 (CPU-hours)],
  [LEAD-STR-class-representative-ic-selection],
  [REVISE],
  [cofactor is an isogeny invariant; the 4.5 % S effect may be an artefact of the raw base],
  [rerun the ℓ\=2 volcano exports with `prime-projected-abscissa`; kill CI inside \[0.97, 1.03\] (≈ 2.4–7.7 CPU-h)],
  [LEAD-ICE-decomposition-probability-audit],
  [CLOSE],
  [yield is size-only (KN-FIND-007); headroom ≈ 0.1 % at the Table B rows],
  [none for S (batch BATCH-79a580 only retires an instrument)],
  [LEAD-ICE-semaev-yield-model-coset-typing],
  [CLOSE (typed half)],
  [typing is +0.4 to +1.7 bits worse under the committed model; the trace-parity half is already realised by LEAD-19],
  [new engineering test: `geomtrace` base in the algebraic arms; kill \< 1.10× paired S across the pricing band],
  [LEAD-ICP-xline-symmetry-special-j],
  [CLOSE],
  [automorphism fold is a measured constant (÷3 at j\=0); rank can only drop unfavourably],
  [none worth running (optional fibre-rank check, \< 1 CPU-h)],
  [LEAD-EXO-four-list-ktree-join],
  [CLOSE (ladder)],
  [only homomorphic filter is the cofactor class, already spent by LEAD-19; predicts S ≥ 1.00×],
  [none on the ladder],
  [LEAD-ICB-kic-golden-base-exact-decomp],
  [CLOSE (baseline lever)],
  [coverage cannot exceed the null; at most ≈ 2 % of S],
  [none for the baseline],
  [LEAD-EXO-hashed-factor-base-tensor-ecdl],
  [CLOSE (watch item)],
  [conditional on an unreviewed theorem; saving needs ≥ 2#super[{2]17} edges],
  [none; the 84-case panel cannot discriminate],
  [LEAD-TRF-odd-prime-ghs-cover-floor],
  [CLOSE (improvement)],
  [n\=7 floor already known; for g\=n Diem’s exponent equals Gaudry’s],
  [none; census is formalisation hygiene],
)
]

=== 4.1 LEAD-15, the promotion, with the evaluator’s corrections
<lead-15-the-promotion-with-the-evaluators-corrections>
- Diem’s plane-quartic result is Õ(q) (heuristic in #link("https://eprint.iacr.org/2005/119")[Diem 2006, ePrint 2005/119], a theorem in #link("https://www.numdam.org/articles/10.5802/jtnb.815/")[Diem, JTNB 2012]); the hyperelliptic genus-3 bound is Õ(q^{4/3}). I checked these two exponents against the search result; I did not read the JTNB theorem statement myself.
- Corrections to my LEAD-15 dossier (E5): the provable variant costs at least 322q additions, not about 48q; "rho about 10⁵× more work" is against relation collection only (Diem–Thomé’s end-to-end is about 1.6e3); the 12 non-zeros per row is the weakest assumption (Diem–Thomé measured 352 at q \= 2³¹); and the baseline is stale: round 4 has an incumbent slope of 1.408 over 2¹⁶–2²².
- Predicted total at q \= 2¹⁵ is 7.3 M + 0.53 M·w multiplications (derived): 13.6 M, 28 M, 60 M for w \= 12, 40, 100 non-zeros per row, i.e. 0.12 to 0.55 of the hyperelliptic incumbent.
- #strong[Family gap.] It needs `E/F_{q³}` with 4 | \#E, so never prime order, and that family is not on Tables A–C. A promotion creates a new row, not an improvement to an existing one. Its baseline is the round-4 hyperelliptic genus-3 row on the #emph[same instances], which the first test arranges.
- In the tower family `F_{p⁶}/F_{p²}/F_p` Joux–Vitse’s own exponents (p^{5/3}, p^{12/7}) beat Diem’s p², so the quartic route wins only when q has no subfield.
- #strong[A later in-repo measurement exists and the evaluator could not see it.] The shared session log of 2026-10-26 (entry "Hess cubic product-family audit", Conductor T-5446, PR \#2651) transferred two q \= 8 sources (subgroup primes 137 and 139) to smooth canonical plane quartics and ran Magma’s line sieve end to end. The median full transfer cost 0.720 s and 0.870 s against a same-source rho of 0.0003125 s, a loss of 2,304× and 2,784×. That is a characteristic-2, q \= 8 control far below the modelled crossover (q ≈ 2⁸ to 2¹¹·⁵), so it neither supports nor contradicts the promotion; it is the first measured quartic-transfer cost in the repository and should be read before the instance builder is written. I read the log entry, not the audit’s `RESULTS.md`.

== 5. Two findings that do not depend on any lead
<two-findings-that-do-not-depend-on-any-lead>
=== 5.1 No priced oracle-side scenario crosses rho at ECC2K-130
<no-priced-oracle-side-scenario-crosses-rho-at-ecc2k-130>
From the committed cost model (#link("cards/e1.md")): for m\=4, one closure degree is worth 2.7 bits at one guessed variable (+10.32 → +7.62 over rho) and 12.6 bits at two (+45.71 → +33.10); the best case D\=3 reads +5.34 and +19.84. None of these priced rows is below rho. Only a #emph[free] oracle would be: the model’s free-oracle floor at m\=4 is 2^56.46, 4.35 bits below rho, because targets × oracle is 26 % of the D\=4, w\=1 cost and linear algebra is the rest. Note the correction in §7: the `+7.62` is the D\=4, w\=1 row, not a floor.

=== 5.2 The volcano IC arm runs the raw base on cofactor-4 curves
<the-volcano-ic-arm-runs-the-raw-base-on-cofactor-4-curves>
Verified by me from the committed specs: `research/isogeny-volcano-cost-v2_20261017/sessions/*/spec/spec.json` set `"factor_base": "prime-abscissa:size=32"`, and the first curve there has `group_order` 10967284 and `r` 2741821, so h \= 4. `prime-projected-abscissa` is a registered plug-in (`src/cryptanalysis/ecbench/methods.rs:2159`). The evaluator (E4) reports that all 14 ℓ\=2 classes have h ∈ {4, 12} (not re-checked by me) and that the session mean is 11.5 relations from 85,707 targets, which is the dead-weight regime of #link("../lead19_projected_base/README.md")[LEAD-19]. So the volcano study’s 4.5 % S effect between curve models was measured on a base that wastes about 75 % (h \= 4) of each table. Whether the effect survives a projected base is the decisive test of `LEAD-STR-class-representative-ic-selection`. If the LEAD-19 mechanism carries over to the prime abscissa base the baseline of those rows is also improvable; #strong[this is unmeasured].

== 6. Atlas follow-up
<atlas-follow-up>
`LEAD-19` is merged but absent from the atlas, and so are the survey’s LEAD-15 and the findings above. A draft row for the cofactor-projected ladder base is in #link("atlas_drafts/LEAD-ICB-cofactor-projected-ladder-base.yaml")[atlas\_drafts/LEAD-ICB-cofactor-projected-ladder-base.yaml]. The existing `LEAD-ICB-cofactor-torsion-quotients` (closed\_obstructed) is #strong[not contradicted]: its obstruction is about ECC2K-130 and standardized prime-degree curves, where the cofactor carries two bits and gcd(4, l) \= 1; the ladder rows are random binary curves with h \= 2…8. Its `revisit_when` ("a curve with a large smooth cofactor") is the neighbouring regime.

== 7. Corrections
<corrections>
#align(center)[#table(
  columns: 3,
  align: (col, row) => (left,left,left,).at(col),
  inset: 6pt,
  [where], [correction], [source],
  [my brief to the evaluators],
  ["+7.62 floor" is the D\=4, w\=1 row; the free-oracle floor at m\=4 is lower (−4.35 bits)],
  [E1],
  [atlas `LEAD-TRF-odd-char-descent-smith-quartic`],
  [title, mechanism and `discriminates` attribute q^{4/3} to the plane quartic; that is the hyperelliptic bound (quartic: Õ(q)). `blocked_by` is stale: Smith Props 2 and 4 are native],
  [E5, exponent re-checked by me],
  [atlas `LEAD-ICE-semaev-yield-model-coset-typing`],
  [`EV-SEMBIN-36f27d` is cited as "coset typing near finite-k cap"; the record says admission gates, direction neutral],
  [E1],
  [atlas `LEAD-EXO-four-list-ktree-join`],
  [`best_measured` repeats the wording `KN-FIND-ffe1df` withdrew],
  [E3],
  [atlas `LEAD-ICP-xline-symmetry-special-j`],
  [evidence omits the measured ×3 fold at j\=0; `H-PFDR-df0127` contradicts "cheaper decomposition"],
  [E2],
  [atlas `LEAD-ICB-nonlinear-frob-stable-bases`],
  [does not cite `research/hamming_ideal_pdp_20260930` (same idea, H1 refuted, exponent 1.9–2.5 vs 0.8)],
  [E3],
  [atlas `LEAD-ICE-hybrid-guess-determine-l7`],
  ["1/116 to 1/3,400" merges two exponent assumptions; "K unmeasured" is outdated (PR \#2164 measured it: a column touch costs 4.5× a rho step)],
  [E2],
  [my LEAD-15 dossier],
  [four corrections in §4.1],
  [E5],
  [cover census README],
  ["Õ(q) is a heuristic, not a theorem" is outdated (Diem 2012)],
  [E5],
)
]

== 8. What I verified and what I did not
<what-i-verified-and-what-i-did-not>
#strong[Verified by me:] the phase shares and the m\=3/m\=2 ratios (recomputed from the committed record, agree with E3 to about one point); the raw base and h \= 4 of the volcano specs and the existence of the projected plug-in; the Diem exponents (search result only, not the theorem text); the atlas counts (153 leads; 74 closed, 53 open\_next\_step, 17 open\_blocked, 9 open\_unexamined).

#strong[Relayed from evaluators without re-checking:] every other number and quotation in §4, §5.1 and §7. Primary sources they read in full or in the relevant sections are listed on each card (E5: Diem JTNB 2012, Diem arXiv:math/0606607, Diem–Thomé 2008, Smith arXiv:0806.2995, Joux–Vitse, Tian arXiv:2012.07173; E4: the Bochum hashed-factor-base paper and the OpenAI preprint it depends on; E2: Gebregiyorgis’ thesis, Faugère–Huot–Joux–Renault–Vitse, Petit–Kosters–Messeng). Abstract-only or inaccessible: Wagner 2002, ePrint 2026/384, ePrint 2025/783, Corrigan-Gibbs–Kogan 2018, Iijima–Momose–Chao 2015, Gaudry 2009, GGMP 2020, Minder–Sinclair, Bettale–Faugère–Perret.

#strong[Searched, nothing found:] the shared session logs of 2026-10-24 to 2026-10-31 contain no mention of LEAD-19, `prime-projected-abscissa` or a volcano projected-base rerun, and no run of LEAD-15’s harvest; they do contain the 2026-10-26 Hess audit above and a triage of an 8-strategy plan against the atlas (a different set of strategies; its decisions agree with the saturation seen here).

#strong[Not verifiable here:] every atlas `evidence:` path in crypto-autoresearcher or added after 2026-10-24 (the local checkouts are stale); the ml-cryptanalysis `fb-koblitz-*` and `fb-prime-*` reports; whether OpenAI’s Theorem D.1 is correct; whether Koblitz Table A/C bases are class-projected.

== 9. What to run next, by decisive value per cost
<what-to-run-next-by-decisive-value-per-cost>
+ #strong[Volcano rerun with `prime-projected-abscissa`] (≈ 2.4–7.7 CPU-h, no new code): tests whether LEAD-19’s mechanism carries to the prime abscissa base and whether the volcano effect is real. If it carries, it is the next candidate improvement to a baseline, class engineering.
+ #strong[LEAD-15 builder and harvest] (builder 2–3 person-days, run \< 1 CPU-h): the only exponent test.
+ #strong[Validation-gate artefacts for LEAD-19], then the three-summand rows (already listed in its §8).
+ Everything else is a stage diagnostic or a null by derivation; run only when idle capacity exists.

None of these is a claim. Each becomes an improvement only if a measured, verified S beats the baseline named before the run, with the gap to matched rho reported as distance remaining.

== 10. Files
<files>
`README.md` (this), `report.pdf`, `figures/`, `phase_shares.{py,json,md}`, `plot_evaluation.py`, `candidate_judgements.tsv`, `atlas_match_log.tsv`, `cards/` (shared brief and the five unedited evaluator outputs), `evaluator_scripts/` (the exploratory scripts the evaluators wrote; one is a single-file Rust toy, labelled exploratory by its author), `atlas_drafts/`, `build.sh`.
