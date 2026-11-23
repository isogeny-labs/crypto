"""Independent check of the 'decomposable' flag: brute-force sums of m points of B by group law.  [pcc:unverified]"""
import sys, json, itertools
import icmodel as M
for f in sys.argv[1:]:
    D = json.load(open(f)); s = D["summary"]
    m, k, p, a, b, g = s["m"], s["k"], s["p"], s["a"], s["b"], s["g"]
    roots = [w[0] for w in M.tower_base(p, k, g).pts]
    pts = M.curve_points(p, a, b)
    B = [P for P in pts if P[0] in set(roots)]
    sums = set()
    for combo in itertools.combinations_with_replacement(B, m):
        R = None
        for P in combo: R = M.ec_add(R, P, p, a)
        if R is not None: sums.add(R[0])
    flagged = {r["xR"] for r in D["rows"] if r["dec"]}
    print(json.dumps({"file": f.split("/")[-1], "B": len(B), "brute_dec_abscissas": len(sums),
                      "flagged": len(flagged), "agree": sums == flagged}))
