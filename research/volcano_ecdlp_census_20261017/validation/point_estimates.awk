# Validator point estimates from measured.tsv (see join.awk for columns):
# 1 V 2 fam 3 level 4 vertex 5 slug 6 workload 7 target 8 arm 9 round
# 10 ops_gae 11 S 12 solve_wall_ns 13 status 14 verified 15 iso
# 16 total_gae 17 s_unc 18 unc
# Prints "key<TAB>value" lines. Only verified rows enter cost statistics.
BEGIN { FS = "\t"; iso_fam["D"]; iso_fam["Ua"]; iso_fam["Ub"]; iso_fam["S"]; iso_fam["N"]
        P[1] = 0; P[2] = 0.1; P[3] = 1 }
$14 != "true" { next }
{
  V = $1; fam = $2; L = $3; vx = $4; arm = $8; s = $11 + 0
  isog = (fam in iso_fam)
  # (a) positive control: V3 crater, D, N, ctl rows of rho-cubic / rho-neg
  if (V == "V3" && L == 0 && (fam == "D" || fam == "N" || fam == "ctl") && (arm == "rho-cubic" || arm == "rho-neg")) {
    pc_sum[arm] += s; pc_n[arm]++
  }
  if (isog) {
    # (b) level means, pooled rows and per-vertex means
    k = V SUBSEP arm SUBSEP L; ls[k] += s; ln[k]++
    kv = V SUBSEP arm SUBSEP L SUBSEP vx; vs[kv] += s; vn[kv]++
    if (!(kv in seenv)) { seenv[kv] = 1; nv[k]++ }
    sq[V SUBSEP arm] += s * s; sm[V SUBSEP arm] += s; sn[V SUBSEP arm]++
    # (c) IC vs rho-neg priced, per level and all
    if (arm == "ic" || arm == "rho-neg") {
      for (i = 1; i <= 3; i++) {
        ps = s * ($16 + P[i] * $18) / $16
        ck = V SUBSEP arm SUBSEP L SUBSEP i; cs[ck] += ps; cn[ck]++
        ca = V SUBSEP arm SUBSEP "all" SUBSEP i; cs[ca] += ps; cn[ca]++
        cv = V SUBSEP arm SUBSEP vx SUBSEP i; cvs[cv] += ps; cvn[cv]++
      }
      uk = V SUBSEP arm; unc_s[uk] += $18; unc_g[uk] += $16; unc_n[uk]++
    }
    # paired source ratio point estimates (family mean S)
    fk = V SUBSEP arm SUBSEP fam; fs[fk] += s; fn[fk]++
    # per-family level means (descriptive)
    fl = V SUBSEP arm SUBSEP fam SUBSEP L; fls[fl] += s; fln[fl]++
    # (e) BSGS identity: ops per (vertex, target, round) across D/Ua/Ub/S
    if ((arm == "bsgs" || arm == "bsgs-neg") && fam != "N") {
      t = V SUBSEP arm SUBSEP vx SUBSEP $7 SUBSEP $9
      if (!(t in bmin)) { bmin[t] = $10 + 0; bmax[t] = $10 + 0; bfam[t] = 1; tkeys[t] = 1 }
      else { if ($10 + 0 < bmin[t]) bmin[t] = $10 + 0; if ($10 + 0 > bmax[t]) bmax[t] = $10 + 0; bfam[t]++ }
    }
    # wall: isolation fraction and A/A pooled wall ratio
    if (arm == "rho-neg" || arm == "ic") { isn[V SUBSEP arm]++; if ($15 == "L2") isl2[V SUBSEP arm]++ }
    if (arm == "rho-neg" || arm == "rho-neg-aa") { wk = V SUBSEP arm; ws[wk] += $12; wn[wk]++ }
  }
}
END {
  OFMT = "%.17g"
  printf "pc.ratio\t%.17g\npc.n_rho_cubic\t%d\npc.n_rho_neg\t%d\n", (pc_sum["rho-cubic"] / pc_n["rho-cubic"]) / (pc_sum["rho-neg"] / pc_n["rho-neg"]), pc_n["rho-cubic"], pc_n["rho-neg"]
  split("V1 V2 V3", VV, " "); split("rho-neg rho-plain bsgs bsgs-neg ic", AA, " ")
  for (vi = 1; vi <= 3; vi++) for (ai = 1; ai <= 5; ai++) {
    V = VV[vi]; arm = AA[ai]
    k0 = V SUBSEP arm SUBSEP 0; m0 = ls[k0] / ln[k0]
    # vertex-mean means at level 0
    vm0 = 0; for (kv in vs) { split(kv, z, SUBSEP); if (z[1] == V && z[2] == arm && z[3] == 0) vm0 += vs[kv] / vn[kv] } vm0 /= nv[k0]
    for (L = 1; L <= 4; L++) {
      kL = V SUBSEP arm SUBSEP L; if (!(kL in ln)) continue
      vmL = 0; for (kv in vs) { split(kv, z, SUBSEP); if (z[1] == V && z[2] == arm && z[3] == L) vmL += vs[kv] / vn[kv] } vmL /= nv[kL]
      printf "R.%s.%s.%d\t%.17g\nRvm.%s.%s.%d\t%.17g\nRn.%s.%s.%d\t%d %d\n", V, arm, L, (ls[kL] / ln[kL]) / m0, V, arm, L, vmL / vm0, V, arm, L, nv[kL], ln[kL]
      for (ff = 1; ff <= 5; ff++) { split("D Ua Ub S N", FF, " "); a1 = V SUBSEP arm SUBSEP FF[ff] SUBSEP L; a0 = V SUBSEP arm SUBSEP FF[ff] SUBSEP 0
        if ((a1 in fln) && (a0 in fln)) printf "Rfam.%s.%s.%s.%d\t%.17g\n", V, arm, FF[ff], L, (fls[a1] / fln[a1]) / (fls[a0] / fln[a0]) }
    }
    mu = sm[V SUBSEP arm] / sn[V SUBSEP arm]; sd = sqrt((sq[V SUBSEP arm] - sn[V SUBSEP arm] * mu * mu) / (sn[V SUBSEP arm] - 1))
    printf "cv.%s.%s\t%.17g\n", V, arm, sd / mu
    for (ff = 1; ff <= 5; ff++) { split("D Ua Ub S N", FF, " ")
      a = V SUBSEP arm SUBSEP FF[ff]; d = V SUBSEP arm SUBSEP "D"
      if (FF[ff] != "D" && (a in fn)) printf "pair.%s.%s.%s_over_D\t%.17g\n", V, arm, FF[ff], (fs[a] / fn[a]) / (fs[d] / fn[d]) }
  }
  for (vi = 1; vi <= 3; vi++) { V = VV[vi]
    for (L = 0; L <= 4; L++) for (i = 1; i <= 3; i++) {
      a = V SUBSEP "ic" SUBSEP L SUBSEP i; b = V SUBSEP "rho-neg" SUBSEP L SUBSEP i
      if (a in cn) printf "icrho.%s.L%d.p%s\t%.17g\n", V, L, P[i], (cs[a] / cn[a]) / (cs[b] / cn[b]) }
    for (i = 1; i <= 3; i++) {
      a = V SUBSEP "ic" SUBSEP "all" SUBSEP i; b = V SUBSEP "rho-neg" SUBSEP "all" SUBSEP i
      printf "icrho.%s.all.p%s\t%.17g\nicrho_n.%s.all\t%d %d\n", V, P[i], (cs[a] / cn[a]) / (cs[b] / cn[b]), V, cn[a], cn[b]
      # mean of vertex means variant
      ma = 0; na = 0; mb = 0; nb = 0
      for (cv in cvs) { split(cv, z, SUBSEP); if (z[1] == V && z[4] == i) { if (z[2] == "ic") { ma += cvs[cv] / cvn[cv]; na++ } else { mb += cvs[cv] / cvn[cv]; nb++ } } }
      printf "icrho_vm.%s.all.p%s\t%.17g\n", V, P[i], (ma / na) / (mb / nb)
    }
    for (ai = 1; ai <= 2; ai++) { arm = (ai == 1 ? "ic" : "rho-neg"); uk = V SUBSEP arm
      printf "unc_per_run.%s.%s\t%.17g\nunc_per_gae.%s.%s\t%.17g\nl2frac.%s.%s\t%.17g\n", V, arm, unc_s[uk] / unc_n[uk], V, arm, unc_s[uk] / unc_g[uk], V, arm, isl2[uk] / isn[uk] }
    printf "aa_wall.%s\t%.17g\n", V, (ws[V SUBSEP "rho-neg-aa"] / wn[V SUBSEP "rho-neg-aa"]) / (ws[V SUBSEP "rho-neg"] / wn[V SUBSEP "rho-neg"])
  }
  for (t in tkeys) { split(t, z, SUBSEP); bk = z[1] SUBSEP z[2]
    if (bfam[t] >= 2) { bt[bk]++; if (bmin[t] == bmax[t]) be[bk]++ } else b1[bk]++ }
  for (bk in bt) { split(bk, z, SUBSEP); printf "bsgs_id.%s.%s\t%.17g\nbsgs_id_n.%s.%s\t%d\nbsgs_single_family_triples.%s.%s\t%d\n", z[1], z[2], be[bk] / bt[bk], z[1], z[2], bt[bk], z[1], z[2], b1[bk] }
}
