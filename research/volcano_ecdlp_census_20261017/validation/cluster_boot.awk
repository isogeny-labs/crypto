# Two-stage stratified cluster bootstrap (validator-written, independent of
# analysis/analyze.py). Input TSV rows: stratum cluster a b c d.
# Statistic: (sum a / sum b) / (sum c / sum d) over the resampled rows.
# Each replicate: within every stratum draw its clusters with replacement,
# then draw each chosen cluster's rows with replacement.
# Prints the full-sample statistic, then one replicate value per line
# prefixed "rep\t". Variables: seed, reps.
BEGIN { FS = "\t"; srand(seed) }
{
  st = $1; cl = $1 SUBSEP $2
  if (!(cl in ncr)) { ns[st]++; scl[st, ns[st]] = cl; if (!(st in seenst)) { seenst[st] = 1; nst++; sts[nst] = st } }
  ncr[cl]++; i = ncr[cl]
  A[cl, i] = $3; B[cl, i] = $4; C[cl, i] = $5; D[cl, i] = $6
  ta += $3; tb += $4; tc += $5; td += $6
}
END {
  printf "full\t%.17g\n", (ta / tb) / (tc / td)
  for (r = 1; r <= reps; r++) {
    sa = sb = sc = sd = 0
    for (k = 1; k <= nst; k++) {
      st = sts[k]; m = ns[st]
      for (j = 1; j <= m; j++) {
        cl = scl[st, int(rand() * m) + 1]; n = ncr[cl]
        for (q = 1; q <= n; q++) { i = int(rand() * n) + 1; sa += A[cl, i]; sb += B[cl, i]; sc += C[cl, i]; sd += D[cl, i] }
      }
    }
    printf "rep\t%.17g\n", (sa / sb) / (sc / sd)
  }
}
