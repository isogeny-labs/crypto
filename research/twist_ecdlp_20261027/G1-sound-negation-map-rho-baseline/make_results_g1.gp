default(parisizemax, 2000000000);
\\ make_results_g1.gp -- POST-FREEZE ADDENDUM (DEVIATIONS.md #2).  Assembles results.json from out/solve.csv, out/ratios.csv, out/opcounts.csv, out/RC.txt,
\\ out/summary.txt, PREREG_SHA256.txt.  Every number is copied (rounded to 6 decimals) from those files; the booleans are comparisons of those numbers.
sp(s, ch) = strsplit(s, ch);
q(s) = Str("\"", s, "\"");
n6(x) = strprintf("%.6f", x);
\\ ratios.csv: "name",point,lo,hi
RAT = Map();
{
my(L = externstr("cat out/ratios.csv"));
foreach(L[2..#L], ln,
  my(i2 = 0, name, rest, v);
  for(k = 2, #ln, if(Vecsmall(ln)[k] == 34, i2 = k; break));
  name = Strchr(Vecsmall(ln)[2..i2-1]);
  rest = Strchr(Vecsmall(ln)[i2+2..#ln]);
  v = sp(rest, ",");
  mapput(RAT, name, [eval(v[1]), eval(v[2]), eval(v[3])]));
}
rat(name) = my(r); if(mapisdefined(RAT, name, &r), r, error("missing ratio ", name));
{
my(L = externstr("cat out/solve.csv"), SOL = List());
foreach(L[2..#L], ln, my(v = sp(ln, ",")); listput(SOL, Str("{\"arm\":", q(v[1]), ",\"b\":", v[2], ",\"solved\":", v[3], ",\"trials\":", v[4], ",\"wilson_lo\":", n6(eval(v[5])), ",\"wilson_hi\":", n6(eval(v[6])), "}")));
SOLJ = strjoin(Vec(SOL), ",\n  ");
}
{
my(L = externstr("cat out/opcounts.csv"), H = sp(L[1], ","), OP = List());
foreach(L[2..#L], ln, my(v = sp(ln, ","), s = "{");
  for(k = 1, #H, s = Str(s, if(k > 1, ",", ""), q(H[k]), ":", if(k == 1, q(v[k]), v[k])));
  listput(OP, Str(s, "}")));
OPJ = strjoin(Vec(OP), ",\n  ");
}
{
RJ = "";
my(names = vecsort(Vec(Mat(RAT)[, 1]~)), parts = List());
foreach(names, nm, my(r = rat(nm)); listput(parts, Str(q(nm), ":[", n6(r[1]), ",", n6(r[2]), ",", n6(r[3]), "]")));
RJ = strjoin(Vec(parts), ",\n  ");
}
\\ control lines copied verbatim from summary.txt
{
my(L = externstr("cat out/summary.txt"), keep = List());
foreach(L, ln, if(#ln >= 4 && (Strchr(Vecsmall(ln)[1..4]) == "PC1 " || Strchr(Vecsmall(ln)[1..4]) == "PC2 " || Strchr(Vecsmall(ln)[1..4]) == "NC1 " || Strchr(Vecsmall(ln)[1..4]) == "NC2 "), listput(keep, q(ln))));
CTL = strjoin(Vec(keep), ",\n  ");
}
\\ falsifier / band booleans computed from ratios.csv
{
my(f = List(), h2 = 1, h3 = 1, ver = List(), unres = List());
foreach([3, 10, 30, 100], rho,
  foreach(["kmix", "kpure"], X,
    my(ra = rat(Str(X, "/n1 I/M=", rho, " w0 all")), re = rat(Str(X, "/n1 I/M=", rho, " w0 e5")));
    if(X == "kmix", if(ra[2] < 1 || re[2] < 1, h2 = 0));
    listput(f, Str("{\"arm\":", q(X), ",\"I_over_M\":", rho, ",\"w0_lower_all\":", n6(ra[2]), ",\"w0_lower_e5\":", n6(re[2]), ",\"lower_above_1_both\":", if(ra[2] > 1 && re[2] > 1, "true", "false"), "}"))));
FALS = strjoin(Vec(f), ",\n  ");
H2 = if(h2, "true", "false");
my(rows = List());
foreach([3, 10, 30, 100], rho,
  foreach(["w0", "w0.1(B)", "w1(B)", "w0.1(A)", "w1(A)"],  w,
    my(ra = rat(Str("kmix/n1 I/M=", rho, " ", w, " all")), re = rat(Str("kmix/n1 I/M=", rho, " ", w, " e5")),
       lowb = (ra[2] < 1 || re[2] < 1), upb = (ra[3] < 1 && re[3] < 1), st);
    st = if(upb, "verified_improvement", if(ra[2] > 1 && re[2] > 1, "N1_cheaper_CI_above_1", "unresolved_CI_straddles_1"));
    listput(rows, Str("{\"I_over_M\":", rho, ",\"pricing\":", q(w), ",\"ratio_all\":", n6(ra[1]), ",\"lo_all\":", n6(ra[2]), ",\"hi_all\":", n6(ra[3]), ",\"ratio_e5\":", n6(re[1]), ",\"lo_e5\":", n6(re[2]), ",\"hi_e5\":", n6(re[3]), ",\"status\":", q(st), "}"))));
KMIXJ = strjoin(Vec(rows), ",\n  ");
my(h3 = rat("kmix/n1 I/M=100 w1(B) all"), h3e = rat("kmix/n1 I/M=100 w1(B) e5"));
H3 = if(h3[2] < 1 && h3e[2] < 1, "true", "false");
}
{
my(rc = externstr("cat out/RC.txt"), bad = List(), pre = readstr("PREREG_SHA256.txt"));
foreach(rc, ln, if(#ln > 0 && Strchr(Vecsmall(ln)[#ln - 3..#ln]) != "rc=0", listput(bad, q(ln))));
BAD = strjoin(Vec(bad), ", "); NCELLS = #rc; PRE = pre[1];
}
{
my(s = Str("{\n \"experiment\": \"G1-sound-negation-map-rho-baseline\",\n \"toolchain\": \"PARI/GP 2.15.4 only; no Python, no Rust; fixed seeds (PREREGISTRATION.md section 4)\",\n",
 " \"preregistration_sha256_line\": ", q(PRE), ",\n",
 " \"cells_run\": ", NCELLS, ",\n \"cells_nonzero_exit\": [", BAD, "],\n",
 " \"note_nonzero\": \"rc=124 is the preregistered 840 s per-cell cap; records written before the kill are kept and missing trials are reported as missing, never imputed\",\n",
 " \"primary_cost_metric\": \"operation counts: M, S, I per walk step and totals (out/opcounts.csv); wall time is secondary and contention-affected\",\n",
 " \"controls_verbatim_from_summary_txt\": [\n  ", CTL, "],\n",
 " \"solve_rates\": [\n  ", SOLJ, "],\n",
 " \"opcounts\": [\n  ", OPJ, "],\n",
 " \"ratios_point_lo95_hi95\": {\n  ", RJ, "},\n",
 " \"kmix_over_n1_status_by_pricing\": [\n  ", KMIXJ, "],\n",
 " \"G1_H2_falsifier_table\": [\n  ", FALS, "],\n",
 " \"G1_H2_kmix_lower_limit_above_1_at_w0_all_four_IM_both_instance_sets\": ", H2, ",\n",
 " \"G1_H3_falsified_kmix_lower_limit_at_IM100_w1B_not_below_1\": ", if(H3 == "true", "false", "true"), ",\n",
 " \"gap_distance_remaining\": \"K-mix and K-pure cost ratios to the matched N1 are the ratios_point_lo95_hi95 rows named 'kmix/n1 ...' and 'kpure/n1 ...'; no improvement over N1 is claimed\"\n}\n"));
system("rm -f results.json");   \\ idempotent (write() appends; DEVIATIONS.md #6)
write("results.json", s);
}
print("results.json written");
