# LEAD-ICP-elliptic-code-list-decoding

## 1. Lead as recorded
- Disposition: open_next_step. Family: ic-prime. Title: empty in leads.json.
- Next-step excerpt (truncated): "Read the full Zhang-Liu text and record which received words and parameters it uses and the comp..."
- Atlas survey: AG-code minimum-weight codewords are k-subsets of the factor base summing to a prescribed point (Cheng cs/0507026; Zhang-Liu ePrint 2018/795). Desk check: Guruswami-Sudan (GS) needs agreement > ~sqrt(Bk) > k when B>k. Atlas expectation: "zero hits except near k ~ B", reduces to enumeration. Author's slide: "not even of square-root time". Zhang-Liu full text unread.
- Files read: /home/user/crypto/research/lead_sweep_20261110/leads.json; /home/user/crypto/research/prime_field_lead_assessment_20261107/atlas_survey.md (lines 31, 234); .../README.md (line 163). Nothing found in /home/user/cryptanalysis.

## 2. Hypothesis
At toy sizes (group order n ~2^12..2^16, B=24, k=2..5), the number of signed k-subsets of an IC-style factor base summing to a random target is Poisson-like with mean C(B,k)2^k/n, no better than for random labels (no code structure helps). And the GS radius condition sqrt(Bk) <= k fails for every k<B, so list decoding cannot output weight-k words. Hence the lead reduces to plain enumeration.

## 3. Preregistered prediction and falsification criterion
- Statistic R = pooled observed/expected mean solution count over cells with expected >= 1.
- Consistent: R_fb in [0.75,1.33], |R_fb - R_null| <= 0.25 R_null, GS fails for all k, positive control passes.
- Falsified: R_fb > 1.5 or < 0.5 (structure in real factor base changes solution density: a lead worth pursuing). Otherwise inconclusive.
- Negative control / null: uniform random labels in Z_n, same B and n. Positive control: planted 3-subset must be found. A/A: two seeds (11,12) on identical settings; max spread of per-cell ratio is reported (noise floor).
- A pass decides only that the counting heuristic holds; it does not test a decoder.

## 4. Method
For primes p ~ 4000, 16000, 64000 (seeds 11,12) build a random short-Weierstrass curve by point enumeration, take a random point P and its cyclic subgroup (order n, discrete logs by table). Factor base: B=24 smallest-x subgroup points. DP over factor-base elements counts signed j-subset sums in Z_n (exact), read off at 2000 random targets. Also tabulate GS sqrt(Bk) vs k. Pure stdlib, one core, estimated < 3 min, < 100 MB. Smoke: tiny p=307.

## 5. Classification
runnable_local. The counting question is decidable by stdlib toy; the full-text step (Zhang-Liu) is separate.

## 6. What a run cannot establish
- No Reed-Solomon/AG list decoder is implemented; only the counting premise and the GS arithmetic are checked.
- Toy sizes, small B, discrete logs known (counting only, not solving). Says nothing about Zhang-Liu's actual parameters or p~2^64 cost.
- Results are [pcc:unverified].
