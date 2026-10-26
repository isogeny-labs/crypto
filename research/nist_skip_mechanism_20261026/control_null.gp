\\ Validation (not preregistered): the Gekeler-model null of L_E against exact point counts of
\\ random prime-order curves y^2 = x^3 - 3x + b over 64- and 96-bit primes.
\\ Run from this directory: gp -q control_null.gp   Writes out/control_null.tsv and appends to out/summary.txt.
default(parisizemax, 2^30);
PR = primes([3, 3000]);
{LE(q, Dq) = my(bound = 4*sqrtint(q), pr = 1, res = 0);
  for (k = 1, #PR, if (kronecker(Dq, PR[k]) != -1, pr *= PR[k]); if (pr > bound, res = PR[k]; break)); res;}
{atkinprob(q) = vector(#PR, k, my(l = PR[k], w = 0, wa = 0);
  for (a = 0, l - 1, if ((1 - a + q) % l, my(c = kronecker(a^2 - 4*q, l), m = l^2 + l*c); w += m; if (c == -1, wa += m)));
  wa/w);}
{nullLE(q, N) = my(pa = atkinprob(q), bound = 4*sqrtint(q), V = vector(N));
  for (s = 1, N, my(pr = 1, k = 0); until (pr > bound, k++; if (random(1.) >= pa[k], pr *= PR[k])); V[s] = PR[k]);
  vecsort(V);}
\\ two-sample Kolmogorov-Smirnov statistic on sorted integer samples
ks(A, B) = my(v = vecsort(concat(A, B), , 8)); vecmax(vector(#v, i, abs(sum(j = 1, #A, A[j] <= v[i])/#A - sum(j = 1, #B, B[j] <= v[i])/#B)));
setrand(20261026);
system("rm -f out/control_null.tsv");
write("out/control_null.tsv", "bits\tp\tsample\tmedian_exact\tmedian_model\tmean_exact\tmean_model\tKS_D\tKS_crit_5pct");
{foreach([nextprime(2^63 + 12345), nextprime(2^95 + 6789)], p,
  my(E = List(), K = 2000, b, N);
  while (#E < K, b = random(p); if (Mod(4*27 + 27*b^2 - 4*27, p) == 0 && b == 0, next);
    N = ellcard(ellinit([-3, b], p)); if (isprime(N), listput(E, LE(p, (p + 1 - N)^2 - 4*p))));
  E = vecsort(Vec(E)); my(M = nullLE(p, 20000), D = ks(E, M), crit = 1.36*sqrt((#E + #M)/(#E*#M*1.)));
  write("out/control_null.tsv", #binary(p), "\t", p, "\t", K, "\t", E[K\2], "\t", M[10000], "\t",
    strprintf("%.2f", vecsum(E)/K*1.), "\t", strprintf("%.2f", vecsum(M)/20000.), "\t", strprintf("%.4f", D), "\t", strprintf("%.4f", crit));
  write("out/summary.txt", strprintf("C1 null-model validation, %d-bit p: exact median %d vs model %d, mean %.1f vs %.1f, KS D = %.4f (5%% critical %.4f)",
    #binary(p), E[K\2], M[10000], vecsum(E)/K*1., vecsum(M)/20000., D, crit)));}
quit;
