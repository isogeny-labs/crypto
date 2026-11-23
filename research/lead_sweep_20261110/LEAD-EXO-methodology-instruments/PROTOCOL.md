# LEAD-EXO-methodology-instruments
## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: open_next_step (last reviewed 2026-10-29). Family: exotic.
- Title: "Program methodology: instrument calibration, nulls, findings promotion (not a cost mechanism)".
- Next-step excerpt (truncated in leads.json): "Inject the three planted-signal families of IDEA-20260807-efc8de into the real pipeline and repl..."
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json (entry);
  /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 72-74, 74 and 193 only).
  The text of IDEA-20260807-efc8de was NOT found, so its three families are not known; the families below are my stand-ins (A window bias, B low-frequency sinusoid, C comb), not the idea's.
- Context from the survey: the character-sum instrument was re-read under a symmetric null (R about 1.0), and the weak-class calibration found structure instruments blind to known weak classes (min detectable shift about 0.13). This lead asks whether the instruments can see planted signals at all.
## 2. Hypothesis
The max-over-64-frequencies character-sum statistic S = max_{k<=64} |sum_x cos(2 pi k d_x/N)| / sqrt(n), on the DL labels d_x of the small-x points (x < p/2) of toy curves y^2=x^3+x+b (p about 1500-4100, N about 2^10-2^12), thresholded at the 95th percentile of a uniform-label null, (i) has false-positive rate in [0.01, 0.10] and (ii) reaches 80% power at planted contamination fraction q <= 0.10 for each of three planted families.
## 3. Preregistered prediction and falsification criterion
- Statistic: null FPR (pooled over 3 curves x 60 fresh null draws) and power per family and q in {0.02, 0.05, 0.1, 0.2} (60 trials per curve per cell).
- Consistent: FPR in [0.01,0.10], power(q=0.2) >= 0.8 for all families, and min q with power >= 0.8 is <= 0.1 for all families.
- Falsified: FPR outside [0.01,0.10] (instrument miscalibrated) or power(q=0.2) < 0.8 for any family (instrument blind to a gross planted signal; every null from it is uninformative).
- Inconclusive: calibrated and gross signals seen, but some family needs q > 0.1.
- Negative control: uniform random labels (symmetric null). Positive control: planted families A/B/C at q=0.2. A/A arm: real (shuffled) curve DL labels versus the same threshold, expected reject rate near the FPR; thresholds come from separate draws than the FPR draws.
## 4. Method
Pure stdlib. For each of three primes (search from 1500, 2600, 4100 for p = 3 mod 4), take the first b with a point of order >= half the group order, tabulate DL of x-classes by repeated addition, keep classes with x < p/2 and labels min(d, N-d). Null = 200 uniform-label draws, threshold = 95th percentile. Planted: each label replaced w.p. q by a draw from A uniform [0,N/8), B density 1+cos(2 pi 17 d/N), C comb |41 d mod N| < N/20. Seeds 1000, 1001, 1002 (random.Random). Cost about 3000 statistic evaluations of about 64k cosines each: estimated 1 to 3 minutes, memory under 20 MB.
## 5. Classification: runnable_local
A stdlib toy decides the calibration question (this is an instrument-calibration lead, not a cost mechanism). The three families are stand-ins because the idea text was unavailable; faithful injection into the real pipeline is not done here.
## 6. What a run cannot establish
Nothing about the real pipeline, the actual IDEA-20260807-efc8de families, large p, or any ECDLP cost. Labels are real DLs only through their marginal distribution after shuffling, so structural (x-to-d) signals are not tested. Planted effects are artificial; power says the character-sum instrument sees these shapes only. Results are labelled [pcc:unverified].
