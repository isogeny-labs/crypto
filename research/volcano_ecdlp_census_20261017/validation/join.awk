# Join measured extract rows with the slug->vertex map and the uncharged
# counters. Files, in order: slug_vertex.tsv, unc_all.csv, extract_all.csv.
# Output (TSV): V fam level vertex slug workload target arm round ops_gae S
#               solve_wall_ns status verified isolation total_gae s_unc unc
FNR == 1 { fileno++ }
fileno == 1 { split($0, f, "\t"); vx[f[1] SUBSEP f[2]] = f[4]; next }
fileno == 2 { split($0, f, ","); k = f[1] SUBSEP f[2] SUBSEP f[3] SUBSEP f[4] SUBSEP f[5]
              if (k in ug) dupu++; ug[k] = f[6]; us[k] = f[7]; uu[k] = f[8]; nunc++; next }
{
  gsub(/"/, ""); split($0, f, ",")
  nall++
  if (f[12] == "true") { nwarm++; next }
  if (f[12] != "false") { badwarm++; next }
  nmeas++
  sk = f[1] SUBSEP f[6]
  if (!(sk in vx)) { nov++; v = "NA" } else v = vx[sk]
  k = f[1] SUBSEP f[7] SUBSEP f[8] SUBSEP f[9] SUBSEP f[11]
  if (!(k in ug)) { nounc++; tg = "NA"; s2 = "NA"; un = "NA" }
  else { tg = ug[k]; s2 = us[k]; un = uu[k]; used[k] = 1
         if (tg + 0 != f[13] + 0) dg++
         if (s2 + 0 != f[14] + 0) ds++ }
  print f[2], f[3], f[4], v, f[6], f[7], f[8], f[9], f[11], f[13], f[14], f[16], f[17], f[18], f[19], tg, s2, un
}
END {
  for (k in ug) if (!(k in used)) uunused++
  printf "rows_all %d\nrows_warmup %d\nrows_measured %d\nwarmup_not_bool %d\nno_vertex %d\nno_uncharged %d\nunc_rows %d\nunc_dup_keys %d\nunc_unused %d\ntotal_gae_ne_ops_gae %d\ns_ne_S %d\n", nall, nwarm, nmeas, badwarm, nov, nounc, nunc, dupu, uunused, dg, ds > "/dev/stderr"
}
