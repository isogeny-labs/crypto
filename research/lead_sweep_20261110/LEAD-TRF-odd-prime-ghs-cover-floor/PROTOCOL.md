# LEAD-TRF-odd-prime-ghs-cover-floor

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- disposition: open_next_step (last reviewed 2026-10-30); family: transfer; source atlas_survey_20261107 --pursuit.
- title: "Odd-characteristic GHS cover genus floors at prime extension degree and their isogeny-class reach".
- next-step excerpt (truncated): "Run the approved n=7 EXP-PEXT-212d5c plan with its Lean finite lemmas, exhaustive placement spec ..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 58-60, 269: lead listed under
  extension-field/Jacobian transfers, "apply to F_{p^k} curves, not E(F_p)"; sibling lead small-genus-cover-general records genus floors 9/49/41 at k=6/10/14);
  research/ecdlp_subexp_desk_20261101/data/atlas_tally.json (id only). The EXP-PEXT-212d5c plan and Lean lemmas were not found in
  the allowed search paths; not read. Nothing in cryptanalysis was searched.

## 2. Hypothesis
Let q=5, n a prime (5 and 7), E/F_{q^n} with j(E) not in F_q, t its Frobenius trace. The Weil restriction A=Res E has F_q-Frobenius
polynomial x^(2n) - t x^n + q^n. A GHS-type cover C has Jac(C) containing A (up to isogeny), so the genus floor is dim A = n, and A
is simple (floor cannot be lowered by a factor) unless x^2 - s x + q divides that polynomial, i.e. t = u_n(s) (n-th power trace of a
subfield Frobenius, |s|<=4). Hypothesis H: this "isogeny-class reach" set B = {u_n(s)} is hit by random E (j not in F_q) at no
higher rate than a same-size shifted decoy set, i.e. f_B / mean(f_decoy) in [0.5, 2]; the reachable fraction is ordinary-density,
about |B|/(trace range), not an enriched class.

## 3. Preregistered prediction and falsification criterion
- Statistic: ratio R = f_B / mean_d f_{B+d}, d in {-6..6}\{0} (decoys disjoint from B), on n=5, N=3000 random curves (j not in F_5).
- Prediction: R in [0.5, 2.0] with at least 5 hits in B, A/A halves of the sample agreeing (|f1-f2| <= 0.5 f_B + 0.01).
  Outcome consistent_with_prediction supports "reach is ordinary density"; R outside the band: falsified (B is enriched or depleted);
  fewer than 5 hits: inconclusive.
- Positive control: curves over F_5 viewed over F_{5^n} must all have t in B and pass exact polynomial divisibility by x^2 - s x + 5;
  any failure marks the instrument broken: verdict falsified.
- Negative control / null: decoy shifted sets B+d (same size, same magnitude); also every hit of random curves must pass divisibility.
- A/A arm: the first and second halves of the random sample, same instrument and seed stream.
- n=7 (N=300) is a secondary, descriptive arm: reported, not used in the verdict.

## 4. Method
Pure Python 3, F_{5^n} from a searched primitive polynomial, log/antilog tables, digit-wise addition tables. Curves y^2=x^3+ax+b,
a,b uniform in F_Q (nonsingular, j computed exactly, j in F_5 detected as code < 5 and j^5=j). Trace by full character sum of
x^3+ax+b via log parity. B from u_{k+1}=s u_k - 5 u_{k-1}, s in -4..4. Divisibility by exact integer polynomial division.
Seeds 20261110 (n=5), 20261111 (n=7); control curves 40 (n=5), 10 (n=7). Cost: about 1-3 minutes, <60 MB, one core.

## 5. Classification: runnable_local
A stdlib toy decides the isogeny-class-reach rate at prime n and re-derives the n-dimensional floor; the full n=7 Lean/placement plan
is not needed for this sharpened, checkable question.

## 6. What a run cannot establish
- No actual GHS cover is built; the genus floor n is only the dimension bound, true genus of explicit covers (e.g. 9/49/41 at k=6/10/14) is not computed.
- Unit multiples (i, omega) of subfield Frobenius for K=Q(i) etc. are ignored; B may be slightly incomplete (n odd so -1 is covered).
- Says nothing about E(F_p) prime-field ECDLP, about the EXP-PEXT-212d5c plan, or about attack cost versus rho.
- Toy size q=5, n=5,7 only; results are [pcc:unverified].
