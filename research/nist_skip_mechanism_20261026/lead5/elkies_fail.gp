\\ L5-elkies-failure: native GP. Run from this directory: gp -q elkies_fail.gp
\\ Implements PROTOCOL.md (sha256 in out/PROTOCOL.sha256). Writes out/*.tsv and out/summary.txt.
default(parisizemax, 2^30);
system("rm -f out/summary.txt out/cohort_E.tsv out/power.tsv");
OUT(s[..]) = write("out/summary.txt", concat(vector(#s, i, Str(s[i]))));

{readtsv(f) = my(L = readstr(f), R = List());
  for (i = 2, #L, if (#L[i], listput(R, strsplit(L[i], "\t")))); Vec(R);}
H = readtsv("../inputs/historical.tsv"); \\ comments start with '#': skip below
R = readtsv("inputs/cohort.tsv");
\\ columns: stream run index role class side p order h
{C = vector(#R, i, my(r = R[i], p = eval(r[7]), u = eval(r[8]));
  [r[1], eval(r[2]), eval(r[3]), r[4], r[5], p, u, eval(r[9]), p + 1 - u]);}
\\ C[i] = [stream, run, index, role, class, p, u, h, t]

\\ ---------- C0 integrity ----------
c0ok = 1;
{cnt = Map();
 for (i = 1, #C, my(k = C[i][4]); mapput(cnt, k, (if (mapisdefined(cnt, k), mapget(cnt, k), 0)) + 1));
 want = ["published", 10, "skipped", 11, "extra69", 48, "extra99", 30];
 for (j = 1, 4, my(g = if (mapisdefined(cnt, want[2*j-1]), mapget(cnt, want[2*j-1]), 0));
   if (g != want[2*j], c0ok = 0; OUT("C0 FAIL count ", want[2*j-1], " = ", g)));}
{nmatch = 0;
 for (i = 1, #C, if (C[i][4] == "published" || C[i][4] == "skipped",
   my(found = 0);
   for (j = 1, #H, my(h = H[j]); if (eval(h[4]) == C[i][2] && eval(h[5]) == C[i][3],
     found = 1; if (eval(h[6]) != C[i][6] || eval(h[8]) != C[i][7], c0ok = 0; OUT("C0 FAIL (p,n) mismatch ", h[1]));
     if ((h[3] == "published") != (C[i][4] == "published"), c0ok = 0; OUT("C0 FAIL role mismatch ", h[1]))));
   if (found, nmatch++, c0ok = 0; OUT("C0 FAIL missing historical row"))));
 OUT("C0 historical rows matched on (p, n, role): ", nmatch, " of 21");}
{for (i = 1, #C, if (C[i][8] == 1 && (!isprime(C[i][7]) || C[i][9]^2 > 16*C[i][6]), c0ok = 0; OUT("C0 FAIL prime/Hasse ", i));
   if (C[i][4] != "extra99" && C[i][8] != 1, c0ok = 0; OUT("C0 FAIL cofactor ", i));
   if (C[i][4] == "extra99" && C[i][8] == 1, c0ok = 0; OUT("C0 FAIL extra99 h=1 ", i));
   if (C[i][9]^2 >= 4*C[i][6], c0ok = 0; OUT("C0 FAIL Hasse ", i)));}
OUT("C0 integrity: ", if(c0ok, "PASS", "FAIL"));
if (!c0ok, error("C0 failed; stop"));

\\ ---------- statistic ----------
Ecount(c, L) = #select(l -> kronecker(c[9]^2 - 4*c[6], l) != -1, primes([3, L]));
\\ pool selection by role set
poolof(roles) = select(i -> setsearch(Set(roles), C[i][4]), [1..#C]);
runsof(P) = vecsort(Set(vector(#P, a, C[P[a]][2])), , 8);
\\ groups: vector of [positions..] per run (positions are indices into C), sorted by seed index
{groupsof(P) = my(rs = Set(vector(#P, a, C[P[a]][2])), G = List());
  for (j = 1, #rs, my(g = select(i -> C[i][2] == rs[j], P)); g = vecsort(g, i -> C[i][3]); listput(G, g)); Vec(G);}
\\ doubled midranks of E within group
{rank2(Ev) = vector(#Ev, i, 2*sum(j = 1, #Ev, Ev[j] < Ev[i]) + sum(j = 1, #Ev, Ev[j] == Ev[i]));}
\\ A pool object: [groups, E vectors per group, rank2 vectors, published count per group, published flags]
{mkpool(P, L) = my(G = groupsof(P), Ev = vector(#G, j, vector(#G[j], a, Ecount(C[G[j][a]], L))),
   rk = vector(#G, j, rank2(Ev[j])), kk = vector(#G, j, sum(a = 1, #G[j], C[G[j][a]][4] == "published")),
   fl = vector(#G, j, vector(#G[j], a, C[G[j][a]][4] == "published")));
  [G, Ev, rk, kk, fl];}
{Wobs(pl) = sum(j = 1, #pl[1], sum(a = 1, #pl[1][j], if (pl[5][j][a], pl[3][j][a], 0)));}
\\ random k-subset sum of ranks
{subsum(rk, k) = my(s = #rk, tot = 0, chosen = vector(k), c, ok);
  for (m = 1, k, until (ok, c = random(s) + 1; ok = 1; for (z = 1, m-1, if (chosen[z] == c, ok = 0; break))); chosen[m] = c; tot += rk[c]); tot;}
{nullW(pl, N, seed) = setrand(seed); my(V = vector(N));
  for (s = 1, N, my(t = 0); for (j = 1, #pl[1], if (pl[4][j] > 0 && pl[4][j] < #pl[1][j], t += subsum(pl[3][j], pl[4][j]), t += if(pl[4][j] == 0, 0, vecsum(pl[3][j])))); V[s] = t);
  vecsort(V);}
\\ number of entries <= w in sorted V
{cntle(V, w) = my(lo = 0, hi = #V); while (lo < hi, my(mid = (lo + hi + 1) \ 2); if (V[mid] <= w, lo = mid, hi = mid - 1)); lo;}
pval(V, w) = (1 + cntle(V, w)) / (#V + 1.);

\\ ---------- model M: conditional Bernoulli sampling and likelihood ----------
\\ suffix elementary symmetric polynomials e[i][m+1] = e_m(w_i..w_s)
{esuf(w, k) = my(s = #w, e = vector(s + 1, i, vector(k + 1)));
  for (i = 1, s + 1, e[i][1] = 1.);
  forstep (i = s, 1, -1, for (m = 1, k, e[i][m+1] = e[i+1][m+1] + w[i]*e[i+1][m])); e;}
{drawset(w, k) = my(s = #w, e = esuf(w, k), m = k, fl = vector(s));
  for (i = 1, s, if (m == 0, break); my(pin = w[i]*e[i+1][m]/e[i][m+1]); if (random(1.) < pin, fl[i] = 1; m--)); fl;}
{loglik(pl, q) = my(ll = 0);
  for (j = 1, #pl[1], my(w = vector(#pl[2][j], a, (1 - q)^pl[2][j][a]), k = pl[4][j]);
    if (k > 0 && k < #w, ll += sum(a = 1, #w, if (pl[5][j][a], log(w[a]), 0)) - log(esuf(w, k)[1][k+1])));
  ll;}
{plant(pl, q) = my(f = vector(#pl[1], j, drawset(vector(#pl[2][j], a, (1 - q)^pl[2][j][a]), pl[4][j]))); [pl[1], pl[2], pl[3], pl[4], f];}
{Wof(pl2) = Wobs(pl2);}

\\ ---------- primary ----------
PRIM = ["published", "extra69"];
pl = mkpool(poolof(PRIM), 100);
w0 = Wobs(pl);
OUT("PRIMARY pool: ", vecsum(vector(#pl[1], j, #pl[1][j])), " positions, runs ", #pl[1], ", published ", vecsum(pl[4]));
NP = 1000000;
Vp = nullW(pl, NP, 20261026);
pp = pval(Vp, w0);
OUT("PRIMARY (L=100, N=", NP, "): W_obs (doubled midranks) = ", w0, ", null mean ", (vecsum(Vp)/NP*1.), ", P(W* <= W_obs) = ", pp, "  [", if(pp <= 0.05, "SUPPORTED at 0.05", "NOT SUPPORTED"), "]");
OUT("  count(W* <= W_obs) = ", cntle(Vp, w0), " of ", NP);

\\ ---------- PC1 deterministic planted ----------
{dflags = vector(#pl[1], j, my(o = vecsort(vector(#pl[1][j], a, [pl[2][j][a], a])), f = vector(#pl[1][j]));
   for (m = 1, pl[4][j], f[o[m][2]] = 1); f);
 dp = [pl[1], pl[2], pl[3], pl[4], dflags];
 wd = Wobs(dp); pd = pval(Vp, wd);
 OUT("PC1 deterministic planted (lowest E per run published): W = ", wd, ", p = ", pd, "  [", if(pd < 0.001, "PASS", "FAIL"), "]");}

\\ ---------- PC2 planted model draws ----------
setrand(20261027);
QS = [0, 0.05, 0.10, 0.20, 0.35];
system("rm -f out/power.tsv");
{
 write("out/power.tsv", "q\tdatasets\trejections\trate");
 for (z = 1, #QS, my(q = QS[z], rej = 0);
   for (d = 1, 1000, my(p2 = plant(pl, q)); if (pval(Vp, Wobs(p2)) <= 0.05, rej++));
   OUT("PC2 q = ", q, ": rejection rate ", rej/1000., " (", rej, "/1000)");
   write("out/power.tsv", q, "\t1000\t", rej, "\t", rej/1000.));}

\\ ---------- secondary ----------
{sec(name, roles, L, seed) = my(p = mkpool(poolof(roles), L), w = Wobs(p), V = nullW(p, 200000, seed), pv = pval(V, w));
  OUT(name, ": pool ", vecsum(vector(#p[1], j, #p[1][j])), ", published ", vecsum(p[4]), ", W_obs = ", w, ", P(<=) = ", pv, ", P(>=) = ", (1 - cntle(V, w - 1)/200001.)); pv;}
sec("S1 C99 pool L=100", ["published", "extra69", "extra99"], 100, 20261027);
sec("S2 all-69 incl. skipped (in-sample) L=100", ["published", "extra69", "skipped"], 100, 20261028);
sec("S3a primary pool L=50", PRIM, 50, 20261029);
sec("S3b primary pool L=200", PRIM, 200, 20261030);

\\ S4 q-hat on the 21 in-sample curves
{pl21 = mkpool(poolof(["published", "skipped"]), 100); best = [-1e9, 0];
 for (i = 0, 60, my(q = i/100., ll = loglik(pl21, q)); if (ll > best[1], best = [ll, q]));
 OUT("S4 in-sample (21 curves, L=100): q-hat = ", best[2], ", loglik = ", best[1], ", loglik(q=0) = ", loglik(pl21, 0), ", LR = ", 2*(best[1] - loglik(pl21, 0)));
 qh = best[2];
 \\ same-model power at q-hat on the primary pool
 setrand(20261031); rej = 0;
 for (d = 1, 1000, my(p2 = plant(pl, qh)); if (pval(Vp, Wobs(p2)) <= 0.05, rej++));
 OUT("S4 power of the primary test at q-hat = ", qh, ": ", rej/1000., " (", rej, "/1000)");}

\\ S5 descriptives and per-curve table
{write("out/cohort_E.tsv", "stream\tindex\trole\tclass\tE50\tE100\tE200");
 for (i = 1, #C, write("out/cohort_E.tsv", C[i][1], "\t", C[i][3], "\t", C[i][4], "\t", C[i][5], "\t", Ecount(C[i], 50), "\t", Ecount(C[i], 100), "\t", Ecount(C[i], 200)));
 foreach(["published", "skipped", "extra69", "extra99"], r, my(S = select(i -> C[i][4] == r, [1..#C]), v = vector(#S, a, Ecount(C[S[a]], 100)));
   OUT("S5 mean E_100 ", r, " (n=", #S, "): ", strprintf("%.2f", vecsum(v)/#S), "  (24 odd primes <= 100; half would be 12.0)"));}
quit;
