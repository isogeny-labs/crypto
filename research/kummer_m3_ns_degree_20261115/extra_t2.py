import sys, os, json
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from run_all import *
rows = []
for p in (40961, 12289):
    for idx in range(3):
        C, meta, F = make_instance(p, 2, idx)
        r = dict(p=p, t=2, idx=idx, d_by_E={})
        for E in range(1, 9):
            d, info, _ = d_ns_desc(C, F, E, 11)
            r["d_by_E"][str(E)] = d
        rows.append(r); log(str(r))
    Cp, mp, Fp = planted_instance(p, 2)
    nu = 3; degF = [max(Cp.popdeg_of(Fp[i][v]) + v for v in range(nu) if Fp[i][v].any()) for i in range(2)]
    ks, tot = Cp.monos()
    ok, st = span_at(Cp, Fp, 8, 13, degF, ks, tot, nu, p)
    rows.append(dict(planted_p=p, t=2, E=8, c=13, one_in_span=ok, step=st)); log(str(rows[-1]))
dump("extra_t2_Esweep_and_planted", rows)
