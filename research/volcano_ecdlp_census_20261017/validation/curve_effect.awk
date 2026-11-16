# Curve effect against the isomorphic control (ANALYSIS_PLAN s4), validator
# re-derivation of the statistic only (no permutation test).
# m_v: mean S of the D-family rows of vertex v; c_v1, c_v2: mean S of each
# ctl slug of v. F = pooled within-level variance of m_v / pooled
# within-vertex variance of {m_v, c_v1, c_v2}. z_v = (m_v - mean(c_v)) /
# sqrt(pooled within-vertex variance). Input: measured.tsv. Variable: V.
BEGIN { FS = "\t" }
$14 == "true" && $1 == V && ($2 == "D" || $2 == "ctl") {
  a = $8; v = $4
  if ($2 == "D") { md[a, v] += $11; nd[a, v]++; lev[v] = $3 }
  else { k = a SUBSEP v SUBSEP $5; cs[k] += $11; cn[k]++; if (!(k in seen)) { seen[k] = 1; nc[a, v]++; cslug[a, v, nc[a, v]] = $5 } }
  arms[a] = 1; verts[v] = 1
}
END {
  for (a in arms) {
    if (a == "rho-neg-aa" || a == "rho-cubic") continue
    delete lm; delete ln; delete mv; wss = 0; wdf = 0; nvx = 0
    for (v in verts) {
      if (!((a, v) in nd) || nc[a, v] != 2) continue
      m = md[a, v] / nd[a, v]; c1 = cs[a, v, cslug[a, v, 1]] / cn[a, v, cslug[a, v, 1]]; c2 = cs[a, v, cslug[a, v, 2]] / cn[a, v, cslug[a, v, 2]]
      mu = (m + c1 + c2) / 3; wss += (m - mu) ^ 2 + (c1 - mu) ^ 2 + (c2 - mu) ^ 2; wdf += 2
      mv[v] = m; cm[v] = (c1 + c2) / 2; L = lev[v]; lm[L] += m; ln[L]++; nvx++
    }
    bss = 0; bdf = 0
    for (v in mv) { L = lev[v]; bss += (mv[v] - lm[L] / ln[L]) ^ 2 }
    for (L in ln) bdf += ln[L] - 1
    wv = wss / wdf
    if (wv == 0) { printf "F.%s.%s\tdegenerate\t%d\n", V, a, nvx; continue }
    printf "F.%s.%s\t%.17g\t%d\n", V, a, (bss / bdf) / wv, nvx
    zmax = 0; for (v in mv) { z = (mv[v] - cm[v]) / sqrt(wv); if (z > zmax || -z > zmax) { zmax = (z < 0 ? -z : z); zv = v; zs = z } }
    printf "zmax.%s.%s\t%d\t%.6g\n", V, a, zv, zs
    if (V == "V2") printf "z69.%s.%s\t%.6g\n", V, a, (mv[69] - cm[69]) / sqrt(wv)
  }
}
