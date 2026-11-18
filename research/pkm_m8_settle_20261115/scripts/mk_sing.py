#!/usr/bin/env python3
"""Write a Singular script for the degree-c truncated-basis certificate (PRESET I2/I3).

Usage: mk_sing.py SYS.ms c OUT.sing RESULT.json [mode]
mode = "cert" (default): std with degBound c, then a truncated-Buchberger completion over all pairs with
deg lcm <= c (product criterion skipped pairs only), generator reduction, then 1-test and the rank deficit
delta_c = #standard monomials of degree <= c of lead(G_c).
mode = "ub": std with degBound c only; reports whether 1 is in the result (upper-bound certificate).
"""
import sys

ms, c, out, res = sys.argv[1], int(sys.argv[2]), sys.argv[3], sys.argv[4]
mode = sys.argv[5] if len(sys.argv) > 5 else "cert"
lines = open(ms).read().split("\n")
names = lines[0].strip()
p = int(lines[1].strip())
body = "\n".join(lines[2:]).strip().rstrip(",")
polys = [q.strip() for q in body.split(",\n") if q.strip()]
s = []
s.append(f'ring R = {p},({names}),dp;')
s.append('option(redSB); option(redTail);')
s.append('ideal I = ' + ",\n".join(polys) + ';')
s.append(f'int c = {c};')
s.append('int t0 = timer; int rt0 = rtimer;')
s.append('degBound = c;')
s.append('ideal G0 = std(I);')
s.append('degBound = 0;')
s.append('int tstd = timer - t0;')
s.append('int i; int j; int k; int one0 = 0;')
s.append('for (i = 1; i <= ncols(G0); i++) { if (deg(G0[i]) == 0 and G0[i] != 0) { one0 = 1; } }')
s.append('int maxdeg0 = 0; for (i = 1; i <= ncols(G0); i++) { if (deg(G0[i]) > maxdeg0) { maxdeg0 = deg(G0[i]); } }')
if mode == "ub":
    s.append(f'write(":w {res}", "{{\\"mode\\": \\"ub\\", \\"c\\": " + string(c) + ", \\"one_in_std\\": " + '
             'string(one0) + ", \\"sizeG\\": " + string(ncols(G0)) + ", \\"maxdegG\\": " + string(maxdeg0) + '
             '", \\"std_cpu_s\\": " + string(tstd) + "}");')
    s.append('quit;')
else:
    s += [
        # G_c: elements of degree <= c, monic
        'ideal G; int n = 0;',
        'for (i = 1; i <= ncols(G0); i++) { if (G0[i] != 0 and deg(G0[i]) <= c) { n++; G[n] = G0[i] / leadcoef(G0[i]); } }',
        'int one = one0; poly h; ideal S; ideal H; int ns;',
        'attrib(G, "isSB", 1);',
        'H = reduce(I, G);',
        'int addedgen = 0;',
        'for (i = 1; i <= ncols(H); i++) { if (H[i] != 0) { if (deg(H[i]) > c) { ERROR("gen deg > c"); } n++; G[n] = H[i] / leadcoef(H[i]); addedgen++; if (deg(H[i]) == 0) { one = 1; } } }',
        'int addedsp = 0; int npairs = 0; int nprod = 0; int nhigh = 0; int rounds = 0;',
        'intvec ei; intvec ej; intvec el; poly L; int dl; int cop;',
        'int nold = 0; int nnew;',
        'while (nold < n and one == 0) {',
        '  rounds++; nnew = n; S = 0; ns = 0;',
        '  for (j = nold + 1; j <= nnew; j++) {',
        '    ej = leadexp(G[j]);',
        '    for (i = 1; i < j; i++) {',
        '      ei = leadexp(G[i]); el = ei; dl = 0; cop = 1;',
        '      for (k = 1; k <= nvars(R); k++) { if (ej[k] > el[k]) { el[k] = ej[k]; } dl = dl + el[k]; if (ei[k] > 0 and ej[k] > 0) { cop = 0; } }',
        '      if (dl > c) { nhigh++; continue; }',
        '      if (cop == 1) { nprod++; continue; }',
        '      L = monomial(el);',
        '      ns++; S[ns] = (L / leadmonom(G[i])) * G[i] - (L / leadmonom(G[j])) * G[j];',
        '    }',
        '  }',
        '  npairs = npairs + ns;',
        f'  write(":a {res}.progress", "round " + string(rounds) + " n=" + string(n) + " spolys=" + string(ns) + " cpu=" + string(timer - t0));',
        '  attrib(G, "isSB", 1);',
        '  if (ns > 0) { H = reduce(S, G); H = simplify(H, 2); } else { H = 0; }',
        '  nold = nnew;',
        '  if (size(H) > 0) {',
        '    H = interred(H);',
        '    for (i = 1; i <= ncols(H); i++) { if (H[i] != 0) { if (deg(H[i]) > c) { ERROR("spoly deg > c"); } n++; G[n] = H[i] / leadcoef(H[i]); addedsp++; if (deg(H[i]) == 0) { one = 1; } } }',
        '  }',
        '}',
        'int delta = -1; int dimR = 0;',
        'if (one == 0) {',
        '  ideal LG = lead(G); LG = simplify(LG, 2); attrib(LG, "isSB", 1); delta = 0;',
        '  for (k = 0; k <= c; k++) { delta = delta + size(kbase(LG, k)); }',
        '}',
        'int ttot = timer - t0;',
        f'write(":w {res}", "{{\\"mode\\": \\"cert\\", \\"c\\": " + string(c) + ", \\"one_in_std\\": " + string(one0) + '
        '", \\"one_in_Vc\\": " + string(one) + ", \\"sizeG0\\": " + string(ncols(G0)) + ", \\"maxdegG0\\": " + string(maxdeg0) + '
        '", \\"sizeGc\\": " + string(n) + ", \\"added_from_generators\\": " + string(addedgen) + '
        '", \\"added_from_spolys\\": " + string(addedsp) + ", \\"pairs_reduced\\": " + string(npairs) + '
        '", \\"pairs_product_criterion\\": " + string(nprod) + ", \\"pairs_lcm_above_c\\": " + string(nhigh) + '
        '", \\"delta_c\\": " + string(delta) + ", \\"std_cpu_s\\": " + string(tstd) + ", \\"total_cpu_s\\": " + string(ttot) + "}");',
        'quit;',
    ]
open(out, "w").write("\n".join(s) + "\n")
