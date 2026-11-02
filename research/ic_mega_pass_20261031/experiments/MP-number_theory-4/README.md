# MP-number_theory-4: outcome INCONCLUSIVE (leans toward no structure; not a refutation)
Files: PROTOCOL.md (pre-registered), run.py, raw_output.txt, RESULTS.json; post-hoc: supp_symnull.py, raw_supp.txt, RESULTS_supp_symnull.json. Runtime 763 s plus about 60 s. numpy 2.0.2, PARI through cypari2. Seeds are listed in PROTOCOL.md.
Scope: prime field only. The run does NOT test KN-OPEN-91d4f6 (char 2, F_2-linear). Toy evidence only, with no claim about deployed curves.

## Spectral test: lambda = max_{j!=0}|FFT(1[f(x(kG))<p/256])|/sqrt(N mu), climbed f, d=1..6
max over d of the REAL/NULL-2 ratio, and of the held-out/NULL-2 ratio:
k=14: 1.53, 1.64 | k=16: 1.32, 1.25 | k=18: 1.50, 1.35 | k=20: 1.31, 1.22 | k=22: 1.42, 1.31
The ratios are flat in p, with no growth. The largest value is 1.64 (held-out, k=14, d=5), which is just above the pre-registered 1.5x SUPPORTS line and far below the 3x REFUTES line.
REAL-climbed is consistently about 1.3x NULL-1-climbed (random x permutation). The post-hoc symmetric null removes this gap. That null permutes x over the pairs {k,-k} and so keeps the identity x(kG)=x(-kG): REAL/NULL-1sym = 0.92-1.14 (k=14,16,18; d=1,2,4,6). The 1.3x is therefore the known evenness of x. It is not polynomial structure.
Identity filter f=x: lambda 3.7 to 5.0, at the same level as the climbed filters.

## Controls (protocol flaw)
The planted 5% Fourier-correlated flip was detected (argmax = j0) only at k=22 (2.08x null-2). At k<=20 it was not detected (1.01-1.23x). By the pre-registered rule this makes the k<=20 spectral arms underpowered for an effect of that size. The planted control is also below 3x null at k=22. I did not re-tune it, because that would be post-hoc.

## Theta sweep (k=20, climbed d=2 f; mass ratio hits/(|A||B|mu), z)
- random A,B at theta 0.25/0.35/0.45/0.55: 0.74/0.89/1.00/0.985. The NULL-1 values are similar.
- adversarial top-s: 3.68/2.87/1.93/1.455. The NULL-1 values are identical (3.68/2.84/1.97/1.455), so this is selection bias.
- interval: 0, 0, 1.83, 0.72 (REAL) against 0, 0, 1.47, 0.86 (NULL-1). The interval sums cover only about 2s distinct k, so the z-scores are not valid. Uninformative.
Below sqrt p (theta<=0.45) no arm separates REAL from NULL-1.

## Verdict
The data show no structured degree<=6 filter: no excess beyond the x-parity artifact, which a matched null reproduces. The verdict is INCONCLUSIVE rather than SUPPORTS for three reasons. (a) The planted control failed at k<=20. (b) One held-out ratio, 1.64, sits slightly above 1.5. (c) The data cannot confirm the proposed rate, because d p^{1/2}/sqrt(|A||B|) is vacuous below |A||B|=p. These data do not establish a theorem, and as the skeptics noted the idea cannot close KN-OPEN-91d4f6 in any case. Not run: the coding_comb-2 even-character / 16|#E / M=2 arm.
## Next step
Use a planted control strength calibrated before the run (for example a dose-response at 5/10/20%). Use the symmetric NULL-1 as the primary null. Use more climber budget, k up to 24 (memory allows about 16M points). Run a char-2 version with Tr(f(x)) and F_2-affine buckets on binary curves, which is the only route that touches KN-OPEN-91d4f6.
