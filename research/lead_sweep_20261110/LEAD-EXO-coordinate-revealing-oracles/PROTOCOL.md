# LEAD-EXO-coordinate-revealing-oracles

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- disposition: open_next_step (reviewed 2026-11-04); family: exotic; title: empty in leads.json.
- next_step_excerpt (truncated): "The GGM gives no bound for this class. Two x-queries plus free local work give log_P Q up to sig..."
- next_step_cost: zero-compute review round + literature read first. why_it_might_beat_sqrt_p: "GGM gives no bound; this is the formal statement of 'index calculus itself'".
- Atlas survey text: two x-queries + free local work give log up to sign (Theta(sqrt N') simulation overhead, holant-h1-ggm-lemma Lemma 3, KN-FIND-194294); C_t and degree-D tests non-simulable (KN-FIND-982fdf); open question = time-bounded advantage from coordinate access on prime-order prime-field curves; best toy: 7-8 bit m=3 x-membership-filtered Semaev MITM B/A 0.0557 (preliminary, "Not a bound"). Reading item KN-LIT-7606 (ePrint 2026/384) body not read.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 40-200 region, grep context); README.md in the same directory (grep context). No hits searched in cryptanalysis (not needed).

## 2. Hypothesis
H: For a solver that uses coordinates only as a membership filter (factor base = points with smallest x; relations = random points whose x equals the x of a +-sum of two factor-base points, found via a pair table), the relation-phase cost in counted point additions, optimised over factor-base size B in {0.5,1,2}*p^(1/3), scales with fitted exponent in [0.58,0.85] of N (theory 2/3 for B~p^(1/3)) over prime-order curves with N in about 2^12..2^20, and exceeds BSGS cost at the largest size. I.e. coordinate access used this way does not by itself beat generic sqrt(N); an advantage needs the algebraic (Semaev/degree) structure the filter lacks.

## 3. Preregistered prediction and falsification criterion
- Statistic: s_coord = least-squares slope of log2(adds) vs log2(N) over 5 sizes (bits 12,14,16,18,20), plus ratio R = adds_coord/adds_BSGS at the largest size.
- consistent_with_prediction: 0.58 <= s_coord <= 0.85 AND R > 1 AND negative control ratio in (0.5,2) AND controls valid.
- falsified: s_coord < 0.55 AND R < 1 (the filter-only algorithm shows sub-sqrt(N)-trend and beats BSGS at the largest size; would reopen the lead for this class).
- otherwise inconclusive (also inconclusive if positive control fails or any sampled relation fails validation).
- Positive control: BSGS counted adds must give slope within 0.1 of 0.5 (shows the instrument sees a known sqrt exponent).
- Negative control / null: same algorithm with a random-point factor base of equal size (no coordinate structure); ratio randfb/coord should be ~1 (coordinates give no extra filtering power, since x of a random point is as good a label). A big deviation (outside 0.5..2) would be coordinate structure effect.
- A/A arm: coord arm repeated with a second derived seed; ratio reported, expected within noise (0.7..1.4).

## 4. Method
Curves y^2=x^3+ax+b over F_p, p=3 mod 4, group order N prime (point count by Legendre sum, seeds 20261110*1000+bits; second arm seed 20261111). Relations: build table of x(P_i +- P_j) for all i<j (2*C(B,2) additions, counted); walk R <- R + S from random start, 1 counted addition per step, until B+1 table hits or cap 4p. Up to 20 relations per size are validated against a full log table (sizes <=18 bits). BSGS: m=isqrt(N)+1 baby steps, mean over 3 random targets. Linear algebra is NOT charged (favours IC). Cost: sizes up to 2^20, estimated well under 2 minutes, <200 MB (log table only to 2^18). Output one JSON object with slopes, ratios, controls, verdict.

## 5. Classification
runnable_local: a stdlib toy decides the stated hypothesis; run.py checked in with --smoke.

## 6. What a run cannot establish
- Not a lower bound or GGM-type statement; only one algorithm family (pair-sum x-filter) at toy sizes, with exponents fitted over 5 points (fit noise perhaps +-0.1).
- Does not test non-simulable tests (C_t, degree-D), m>=3 Semaev decomposition, or field-valued access; does not read ePrint 2026/384.
- Linear-algebra and target-phase costs are omitted, so IC is favoured; unit is point additions, not priced under the Section 5a band. Results labelled [pcc:unverified].
