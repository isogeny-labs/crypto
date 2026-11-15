# LEAD-ICB-free-relations-self-collision

## 1. Lead as recorded (disposition, family, title, next-step excerpt; files read with paths)
- Disposition: exhausted_at_scale (reviewed 2026-10-24; 2026-11-01 survey agrees). Family: ic-binary.
- Title: Free relations from self-collisions in the pair-root table (no-probe relations).
- Next-step excerpt (truncated in leads.json): "Revisit when: A wide-path (n > 63) harvest is implemented, or a table structure yields independe..."
- Recorded outcome (docs/session-logs/2026-11-01.md l.1408): production duplicate-bucket harvest 0.960x pooled, then a +5..9% loss under rule-v4 cold accounting.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; research/prime_field_lead_assessment_20261107/atlas_survey.md (l.77, 304); docs/session-logs/2026-11-01.md (l.1400-1416). Other hits (atlas_tally.json, cryptanalysis atlas-inputs JSON) not read. The lead's own atlas note is encrypted; mechanism taken from these excerpts plus own knowledge.

## 2. Hypothesis
In a table of T pair sums {F_i +- F_j} from a random factor base, the number of distinct-pair collisions (free relations) equals the birthday count of T uniform draws over the n/2 x-classes (no structural excess), so free relations per table operation are about 0.5x probe relations per probe operation (Y < 1). This is the generic-model reason the recorded harvest stays at or below 1x; an excess would need a table structure giving independent relations (the "revisit" condition).

## 3. Preregistered prediction and falsification criterion
- Statistics: R = C_obs / C_null (collision count over matched uniform-draw null, same T); Y = (C/T) / (probe hits / probes), probes = T.
- Prediction (per size, 12 replicates): random FB has |R-1| < 3 SE + 0.05; Y mean + 2 SE < 1.
- Falsified: random-FB R excludes 1, or Y mean - 2 SE > 1, while positive control and A/A pass. That would be an excess/free-relation yield beating probes.
- Consistent: all of null, A/A, positive control, Y<1 pass at all sizes. Otherwise inconclusive.
- Null: uniform random classes (same T). Positive control: FB an arithmetic progression {i*d} (a+b=c+d collisions), requires R > 3. A/A: second independent random-FB arm with different derived seeds; |R1-R2| < 3 SE-combined + 0.02.

## 4. Method
Per size log2 p in {12,14,16,17}: seeded search for prime-order curve y^2=x^3+ax+b (brute point count, stdlib); table x(kG) built by affine addition (checked against curve addition for k<50). FB = B ~ sqrt(n) random multiples; table = all B(B-1)/2 pair sums keyed by canonical class min(k,-k) (discrete-log shortcut for the group law, exact in a cyclic group). Collisions counted per key bucket. Probes: random classes tested against the key set. Seed0 20261110, derived per cell/arm/replicate. Cost: under ~2 min, <150 MB (xt list of n<=2^17).

## 5. Classification
runnable_local: stdlib toy decides the generic-birthday claim. Reason: only the model-level yield is testable.

## 6. What a run cannot establish
It does not address binary fields, the wide-path (n>63) harvest, production duplicate-bucket accounting, rule-v4 cold costs, or any structured table. It uses a prime-order prime-field toy, shortcut group arithmetic, tiny sizes, and says nothing about beating rho. Labelled [pcc:unverified].
