# RESULT: LEAD-TRF-ghs-binary-composite

## Outcome
- status: ran (single run, no retries). exit code 0, wall 1 s (run.status: exit=0 wall=1).
- RESULT.json is one JSON object; run.stderr is empty.

## Statistic vs preregistered prediction
Prediction (PROTOCOL.md s3): per (l,n), fraction of T=400 random b with F_q-span dimension n, mean of arms A,B, within 4*sqrt(0.25/T)+0.02 = 0.12 of the exact normal density for every odd-n config. Positive control (b in F_q gives dim 1) passes, A/A arms agree within 2*tol. Secondary: proxy table has ic_below_rho false for all n>=4.

Measured (odd n, vs theory):
- (l=4,n=3): A 0.840, B 0.8125, mean 0.826 vs 0.8240. Within.
- (l=5,n=3): A 0.9725, B 0.9675, mean 0.970 vs 0.9678. Within.
- (l=3,n=5): A 0.880, B 0.870, mean 0.875 vs 0.8748. Within.
Even n (no closed form, reported only): (3,4) A 0.895 B 0.830; (2,6) 0.430/0.4225; (4,4) 0.94/0.9275; (6,2) 0.985/0.99.

Controls: positive control passed in all 7 configs; A/A ok in all 7 (largest gap |fA-fB| = 0.065 at (3,4), below 0.24). Null/reference is the theoretical density. All controls passed.

Verdict (run.py): consistent_with_prediction. The decisive statistic agrees with my reading of the numbers.

## Reading
- The toy tests only the proxy: span dimension of Galois conjugates of random b over F_q equals the normal-element density. That is a standard fact about random elements and the toy confirms it. It does not compute the GHS parameter m, genus, covers, or any cost.
- Discrepancy in the secondary (non-decisive) table: the prediction said ic_below_rho false for all n>=4, but the row N=176, l=11, n=16 (genus 32768) has log2_ic_proxy 83.1 < log2_rho 88.0, so ic_below_rho is true. run.py's verdict field ignores the secondary table, so it does not reflect this. The proxy (2l + log2(g!)) is an assumption and ignores the real cost of a genus-32768 cover (construction, linear algebra, factor base), so this is not evidence of a gain, but the preregistered secondary expectation was not met literally.
- The cost table covers only 8 (N,l,n) rows (N=155,162,176,185,210); other rows all show proxy IC well above rho.
- Even-n rows are not tested against theory (null). With T=400 the odd-n tests have only 3 configs; the 0.12 tolerance is loose (about 4.8 sigma at p=0.5).

## Bearing on the sub-sqrt(p) question
closes or confirms the recorded obstruction: generic b gives full-size span at the normal density, consistent with the genus blow-up being the obstruction, though the toy does not measure true GHS cost and the one n=16 proxy row below rho is an unvalidated artefact of the crude formula.

## Standing
[pcc:unverified], single local toy run, no independent replay.
