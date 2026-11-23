# LEAD-ICB-composite-degree-c2pnb

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-30). Family: ic-binary.
- Title: Composite-degree ANSI X9.62 binary curves (c2pnb176/208/272/304/368): Gaudry-Diem n = 4 with a pi_16 orbit base.
- Next step (truncated in leads.json): "Run the C1 toy ladder (NEXT-STEPS item 6): E over F_{2^4} lifted to F_{2^{4k}}, k in {5, 7, 9, 1..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry); /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (one-line listing only). Other hits (atlas_tally.json, docs/known-label-*) are index files, not read. No lead note with the full NEXT-STEPS text was found; the rest of this protocol rests on the mechanism, not on the recorded text.

## 2. Hypothesis
For E: y^2+xy = x^3+ax^2+b over F_{2^m}, m = s*k, with factor base B = {points with x in F_{2^s}} (Frobenius-pi_{2^s}-stable), the k-fold sum set S_k = {P_1+..+P_k, P_i in B} covers a fraction of E(F_{2^m}) close to the Gaudry-Diem heuristic 1-exp(-C(q+k,k)/#E) (q=2^s) when (a,b) are generic in F_{2^m}, but when (a,b) lie in F_{2^s} (curve lifted from the subfield, as the C1 ladder specifies) coverage is at most 0.1 of that heuristic, because B lies in E(F_q) and E(F_{q^2}), a tiny subgroup. So a lifted-subfield toy cannot show decomposition gain; only generic-coefficient curves can, and c2pnb curves (generic coefficients, pi_16 orbit base) must be judged on that second case.

## 3. Preregistered prediction and falsification criterion
- Statistic: ratio = observed coverage / heuristic coverage per (curve, config); heuristic uses nominal base size q+1 (about q points on average).
- Prediction: median generic ratio in [0.6, 1.3]; every subfield-lifted ratio <= 0.1; A/A median |ratio(A1)-ratio(A2)| <= 0.3.
- Falsified: any subfield-lifted ratio > 0.5 (lifting would then give decomposition power, contradicting the hypothesis). Inconclusive: anything between, or an incomplete run (time guard).
- Positive control: generic-coefficient curves (detector must see the heuristic coverage). Negative control/null: the heuristic itself and the subfield-lifted arm (predicted near zero). A/A: two independently seeded generic curves per config.
- Outcome decides: consistent -> the C1 ladder as literally specified (lifted from F_{2^4}) is uninformative and should be redone with generic coefficients; falsified -> lifted curves carry real decomposition yield and the ladder stands.

## 4. Method
Pure Python GF(2^m) with log tables (primitive polynomial found by search), affine group law, point counting by half-trace table. Configs (s,k): (2,3),(2,5),(2,7),(3,3),(3,4),(3,5),(4,3); m=6..15. Per config two generic curves (a random in F_{2^m}, b random nonzero, both outside the subfield) and two subfield curves (a,b in F_{2^s}, b != 0), seeds from the string "20261110-s-k-arm". S_k built by iterating S_j = S_{j-1}+B (exactly k summands, O start). Cost: about 5e6 group additions per curve set, estimated 2-6 min, guarded by an 520 s deadline (run is then inconclusive). One core, <200 MB, stdlib only.

## 5. Classification: runnable_local
A stdlib toy decides the structural question (subfield-lifted vs generic coverage) cheaply.

## 6. What a run cannot establish
- It does not test the actual ladder (E fixed over F_{2^4}, k in {5,7,9,...}, n=4, pi_16 orbit base) at those sizes; m <= 15 only, small base, few curves per config (heuristic ratios are noisy at order ~2^6..2^15).
- Coverage of sums is not relation yield, linear algebra, or an end-to-end DLP cost; no matched rho, no pcc claim, no S-gap. Results are [pcc:unverified].
- It says nothing about c2pnb176..368 themselves or the real Weil-restriction/Semaev degree of regularity.
