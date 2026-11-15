"""D5: m = 3, random base in E[r]. Exact coverage of +-P1 +- P2 +- P3, and two counted decisions:
(i) direct: S = R -+ P1 for each P1, then x(S -+ P2) tested against the base (P2 index >= P1 index);
(ii) meet in the middle: x(R -+ P1) tested against a precomputed table of pair-sum abscissae
     (setup: all +-Pi +- Pj, about F^2 group additions). Both are generic (additions + equality tests)."""
import json, random, sys, pathlib
sys.path.insert(0, str(pathlib.Path(__file__).parent))
import fp
from fp import p, A, r, mul, CNT, batch_inv
D = pathlib.Path(__file__).resolve().parents[1]
rng = random.Random(202611075)

def addx(R, pts, Bx):  # shared-denominator x(R -+ P) for P in pts; returns hits
    xR, yR = R; invs = batch_inv([(xR - x) % p for x, _ in pts]); hits = []
    for k, ((x1, y1), iv) in enumerate(zip(pts, invs)):
        for sg in (1, -1):
            lam = mul((yR - sg * y1) % p, iv); x3 = (mul(lam, lam) - A - xR - x1) % p
            if x3 in Bx: hits.append((k, sg, x3))
    return hits

def addpt(R, pts):  # full points R -+ P (x and y), counted: 2M + 2S + 2M(y) + batch 3
    xR, yR = R; invs = batch_inv([(xR - x) % p for x, _ in pts]); out = []
    for (x1, y1), iv in zip(pts, invs):
        for sg in (1, -1):
            lam = mul((yR - sg * y1) % p, iv); x3 = (mul(lam, lam) - A - xR - x1) % p
            out.append((x3, (mul(lam, (xR - x3) % p) - yR) % p))
    return out

out = []
for F in (24, 32, 40, 46, 56):
    B = []; seen = set()
    while len(B) < F:
        P = fp.u_rand_Er(rng)
        if P[0] not in seen: seen.add(P[0]); B.append(P)
    Bx = {P[0] for P in B}
    pair_pts = []
    for i in range(F):
        for j in range(i, F):
            for Q in (fp.u_add(B[i], B[j]), fp.u_add(B[i], fp.u_neg(B[j]))):
                if Q is not None: pair_pts += [Q, fp.u_neg(Q)]
    pair_x = {Q[0] for Q in pair_pts}; setup_adds = F * (F + 1)
    pair_x_m2 = pair_x | Bx  # adding the base itself lets MITM also report m <= 2 decompositions
    xs = set()
    uniq = list({Q for Q in pair_pts})
    for Q in uniq:
        for P in B:
            S = fp.u_add(Q, P)
            if S is not None: xs.add(S[0])
    C = 2 * len(xs)
    T = 150; hd = hm = cd = cm = 0; agree = 0
    for t in range(T):
        R = fp.u_rand_Er(rng)
        fp.reset(); h2 = bool(addx(R, B, pair_x)); cm += CNT["M"]; hm += h2; h2b = h2 or bool(addx(R, B, pair_x_m2))
        if t < 30:
            fp.reset(); S = addpt(R, B); h1 = False
            for k, Spt in enumerate(S):
                if Spt[0] in Bx: h1 = True  # m = 2 hit inside the m = 3 decision
                if addx(Spt, B[k // 2:], Bx): h1 = True
            cd += CNT['M']; hd += h1; agree += (h1 == h2b)
    row = dict(F=F, C=C, C_over_r=C / r, pair_table_x=len(pair_x), setup_group_adds=setup_adds,
               hit_rate_mitm=hm / T, c_mitm=cm / T, c_direct=cd / 30, direct_agrees_with_mitm_incl_m2_of_30=agree)
    out.append(row); print(row)
json.dump(out, open(D / "data/d5_m3.json", "w"), indent=1)
