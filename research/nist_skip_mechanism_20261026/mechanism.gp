\\ nist_skip_mechanism_20261026: native port of addendum A and the addendum B tests.
\\ Run from this directory: gp -q mechanism.gp
\\ Writes out/curves.tsv, out/binary.tsv, out/b2_cover.tsv and out/summary.txt.
default(parisizemax, 2^30);
LMAX = 3000;
PR = primes([3, LMAX]);
NNULL = 20000;
setrand(20261026);

\\ ---------- inputs ----------
{readtsv(f) = my(L = readstr(f), R = List());
  for (i = 1, #L, my(s = L[i]); if (#s && Vecsmall(s)[1] != 35, listput(R, strsplit(s, "\t"))));
  Vec(R);}
\\ historical.tsv columns: label bits role run index p b n seed
H = readtsv("inputs/historical.tsv");
{C = vector(#H, i, my(r = H[i], p = eval(r[6]), n = eval(r[8]), t = p + 1 - n);
  [r[1], r[3] == "published", r[4], p, n, t, t^2 - 4*p]);}

\\ L_E: smallest l such that the product of odd primes <= l with (D/l) != -1 exceeds 4 sqrt(q)
{LE(q, Dq) = my(bound = 4*sqrtint(q), pr = 1, res = 0);
  for (k = 1, #PR, if (kronecker(Dq, PR[k]) != -1, pr *= PR[k]); if (pr > bound, res = PR[k]; break));
  if (!res, error("LMAX too small")); res;}

\\ ---------- streams, exact enumeration of count-preserving label assignments ----------
runs = Set(vector(#C, i, C[i][3]));
S = vector(#runs, j, select(i -> C[i][3] == runs[j], [1..#C]));
TRUE = select(i -> !C[i][2], [1..#C]);
{assignments() = my(A = [[]]);
  for (j = 1, #S, my(s = S[j], k = #select(i -> !C[i][2], s), B = List());
    forsubset([#s, k], u, my(sk = vector(#u, a, s[u[a]])); for (a = 1, #A, listput(B, concat(A[a], sk))));
    A = Vec(B));
  A;}
AS = assignments();

avgrank(v) = vector(#v, i, sum(j = 1, #v, v[j] < v[i]) + (sum(j = 1, #v, v[j] == v[i]) + 1)/2);
{streamranks(val) = my(R = vector(#C));
  for (j = 1, #S, my(s = S[j], r = avgrank(vector(#s, a, val[s[a]]))); for (a = 1, #s, R[s[a]] = r[a]));
  R;}
\\ [observed skipped rank-sum, #assignments, P(>= obs), P(<= obs)]
{permtest(val) = my(R = streamranks(val), obs = sum(k = 1, #TRUE, R[TRUE[k]]), ge = 0, le = 0);
  for (a = 1, #AS, my(x = sum(k = 1, #AS[a], R[AS[a][k]])); ge += (x >= obs); le += (x <= obs));
  [obs, #AS, ge/#AS, le/#AS];}

\\ ---------- Gekeler-model null of L_E ----------
\\ Frobenius mod l uniform in GL2(F_l) with det q, independent over l, given l does not divide #E.
\\ Matrices with trace a and det q: l^2 + l*chi(a^2 - 4q).
{atkinprob(q) = vector(#PR, k, my(l = PR[k], w = 0, wa = 0);
  for (a = 0, l - 1, if ((1 - a + q) % l, my(c = kronecker(a^2 - 4*q, l), m = l^2 + l*c); w += m; if (c == -1, wa += m)));
  wa/w);}
{nullLE(q, N) = my(pa = atkinprob(q), bound = 4*sqrtint(q), V = vector(N));
  for (s = 1, N, my(pr = 1, k = 0); until (pr > bound, k++; if (random(1.) >= pa[k], pr *= PR[k])); V[s] = PR[k]);
  vecsort(V);}
pct(V, x) = (sum(i = 1, #V, V[i] < x) + sum(i = 1, #V, V[i] == x)/2.) / #V * 100;

system("rm -f out/curves.tsv out/summary.txt out/binary.tsv out/b2_cover.tsv");
OUT(x) = write("out/summary.txt", x);

le = vector(#C, i, LE(C[i][4], C[i][7]));
NUL = Map();
for (i = 1, #C, if (!mapisdefined(NUL, C[i][4]), mapput(NUL, C[i][4], nullLE(C[i][4], NNULL))));
pp = vector(#C, i, pct(mapget(NUL, C[i][4]), le[i]));
twistl = vector(#C, i, #select(l -> (C[i][4] + 1 + C[i][6]) % l == 0, primes([3, 199])));
othere = vector(#C, i, #select(l -> (C[i][4] + 1 + C[i][6]) % l && kronecker(C[i][7], l) != -1, primes([3, 199])));
write("out/curves.tsv", "label\trole\trun\tt\tL_E\tnull_pct\tl_div_twist_lt200\tother_elkies_lt200");
{for (i = 1, #C, write("out/curves.tsv", C[i][1], "\t", if (C[i][2], "published", "skipped"), "\t", C[i][3], "\t",
  C[i][6], "\t", le[i], "\t", strprintf("%.1f", pp[i]), "\t", twistl[i], "\t", othere[i]));}

\\ ---------- B0: port of addendum A ----------
OUT(Str("B0 port of addendum A (exact enumeration of ", #AS, " count-preserving assignments)"));
r = permtest(le); OUT(strprintf("  L_E: skipped rank-sum %s, P(>=obs) = %.4f, P(<=obs) = %.5f", r[1], r[3], r[4]));
r = permtest(vector(#C, i, C[i][6] < 0)); OUT(strprintf("  t<0: skipped rank-sum %s, P(>=obs) = %.4f, P(<=obs) = %.5f", r[1], r[3], r[4]));
r = permtest(twistl); OUT(strprintf("  #l<200 dividing the twist order: P(>=obs) = %.4f, P(<=obs) = %.4f", r[3], r[4]));
r = permtest(othere); OUT(strprintf("  #other Elkies l<200: P(>=obs) = %.4f, P(<=obs) = %.4f", r[3], r[4]));
pub = select(i -> C[i][2], [1..#C]);
{OUT(strprintf("  Gekeler null (N = %d per field): mean percentile published %.1f (n = %d), skipped %.1f (n = %d)",
  NNULL, vecsum(vector(#pub, k, pp[pub[k]]))/#pub, #pub, vecsum(vector(#TRUE, k, pp[TRUE[k]]))/#TRUE, #TRUE));}

\\ ---------- B1: NSA binary curves ----------
\\ L_E is discrete, so midrank percentiles are not uniform under the null and Irwin-Hall does not
\\ apply. The tail of the mean is calibrated by Monte Carlo instead: each replicate draws one
\\ independent L_E per primary curve from its field's model and scores it against the same reference.
{pctmap(V) = my(M = Map(), u = Set(V));
  for (k = 1, #u, mapput(M, u[k], pct(V, u[k])));
  M;}
\\ midrank percentile of a value absent from the reference sample
pctof(M, V, x) = if (mapisdefined(M, x), mapget(M, x), pct(V, x));
NCAL = 20000;
BIN = readtsv("inputs/binary.tsv");
write("out/binary.tsv", "label\trole\tm\tt\tL_E\tnull_pct");
OUT("B1 NSA binary curves (q = 2^m, #E = h*n)");
bp = List(); bref = List();
{for (i = 1, #BIN, my(r = BIN[i], m = eval(r[2]), q = 2^m, N = eval(r[3])*eval(r[4]), t = q + 1 - N,
    L = LE(q, t^2 - 4*q), V = nullLE(q, NNULL), x = pct(V, L));
  write("out/binary.tsv", r[1], "\t", r[5], "\t", m, "\t", t, "\t", L, "\t", strprintf("%.1f", x));
  OUT(strprintf("  %s (%s): L_E = %d, null median %d, percentile %.1f", r[1], r[5], L, V[NNULL\2], x));
  if (r[5] == "primary", listput(bp, x); listput(bref, [q, V, pctmap(V)])));}
mb = vecsum(Vec(bp))/#bp;
\\ calibration draws: column k holds NCAL independent model draws of L_E for primary curve k
caldraws = vector(#bref, k, nullLE(bref[k][1], NCAL));
\\ nullLE sorts its output; shuffle each column so that replicate s pairs independent draws
{for (k = 1, #caldraws, my(v = caldraws[k]);
  forstep (i = #v, 2, -1, my(j = random(i) + 1, tmp = v[i]); v[i] = v[j]; v[j] = tmp);
  caldraws[k] = v);}
calmean = vector(NCAL, s, sum(k = 1, #bref, pctof(bref[k][3], bref[k][2], caldraws[k][s]))/#bref);
calge = sum(s = 1, NCAL, calmean[s] >= mb);
{OUT(strprintf("  primary mean percentile %.1f; Monte Carlo P(mean >= obs) = %.4f (add-one, %d replicates; calibrated null mean %.1f)",
  mb, (1 + calge)/(NCAL + 1.), NCAL, vecsum(calmean)/NCAL));}

\\ ---------- B2: Elkies-failure covering rule ----------
L100 = primes([3, 100]);
elk(i, l) = kronecker(C[i][7], l) != -1;
\\ [S_max, number of skipped curves not covered] for a skip set
{covers(skipset) = my(ispub = vector(#C, i, 1), Smax, unc);
  for (k = 1, #skipset, ispub[skipset[k]] = 0);
  Smax = select(l -> sum(i = 1, #C, ispub[i] && elk(i, l)) == 0, L100);
  unc = #select(i -> !ispub[i] && sum(k = 1, #Smax, elk(i, Smax[k])) == 0, [1..#C]);
  [Smax, unc];}
cv = covers(TRUE);
OUT(Str("B2 Elkies-failure covering: S_max = ", cv[1], "; skipped curves not covered = ", cv[2], " of ", #TRUE));
write("out/b2_cover.tsv", "label\trole\telkies_primes_in_Smax");
{for (i = 1, #C, write("out/b2_cover.tsv", C[i][1], "\t", if (C[i][2], "published", "skipped"), "\t",
  select(l -> elk(i, l), cv[1])));}
cal = sum(a = 1, #AS, covers(AS[a])[2] == 0);
OUT(Str("  calibration: ", cal, " of ", #AS, " assignments admit a covering S (rate ", strprintf("%.4f", cal/#AS), ")"));

\\ ---------- Amendment 2 controls (added after the data; no new inputs) ----------
\\ PC1: plant labels that the L_E permutation test must find. In every stream the skipped
\\ candidates are relabelled as the ones with the smallest L_E (ties broken by row order).
{planted = concat(vector(#S, j, my(s = S[j], k = #select(i -> !C[i][2], s),
    o = vecsort(vector(#s, a, [le[s[a]], s[a]]), 1)); vector(k, a, o[a][2])));}
{permtest_set(val, skipset) = my(R = streamranks(val), obs = sum(k = 1, #skipset, R[skipset[k]]), le_ = 0);
  for (a = 1, #AS, le_ += (sum(k = 1, #AS[a], R[AS[a][k]]) <= obs));
  le_/#AS;}
pmin = vecmin(vector(#AS, a, sum(k = 1, #AS[a], streamranks(le)[AS[a][k]])));
R_le = streamranks(le);
pminp = sum(a = 1, #AS, sum(k = 1, #AS[a], R_le[AS[a][k]]) <= pmin)/#AS;
{OUT(strprintf("PC1 planted smallest-L_E labels: P(<=obs) = %.5f (smallest attainable %.5f); detected = %d",
  permtest_set(le, planted), pminp, permtest_set(le, planted) <= 0.01));}
\\ PC2: B1 power. P(calibrated mean >= 74.6), the published prime curves' mean percentile;
\\ a detector with power gives a small value when the binary curves sit at that effect size.
{OUT(strprintf("PC2 B1 power at the prime-published effect size (mean 74.6): Monte Carlo P = %.4f",
  (1 + sum(s = 1, NCAL, calmean[s] >= 74.6))/(NCAL + 1.)));}
quit;
