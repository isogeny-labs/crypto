import sys, json, random, math
def curve(p, rng):
    while True:
        a, b = rng.randrange(p), rng.randrange(p)
        if (4*a**3 + 27*b*b) % p: return a, b
def points(p, a, b):
    sq = {}
    for y in range(p): sq.setdefault(y*y % p, []).append(y)
    pts = []
    for x in range(p):
        for y in sq.get((x**3 + a*x + b) % p, []): pts.append((x, y))
    return pts
def add(P, Q, p, a):
    if P is None: return Q
    if Q is None: return P
    if P[0] == Q[0]:
        if (P[1] + Q[1]) % p == 0: return None
        l = (3*P[0]*P[0] + a) * pow(2*P[1], -1, p) % p
    else:
        l = (Q[1]-P[1]) * pow(Q[0]-P[0], -1, p) % p
    x = (l*l - P[0] - Q[0]) % p
    return (x, (l*(P[0]-x) - P[1]) % p)
def trial(p, a, b, pts, rng, st):
    A = [rng.randrange(p) for _ in range(3)] + [rng.randrange(1, p)]  # a0..a3, a3!=0
    B = [rng.randrange(p), rng.randrange(p)]
    zeros = [P for P in pts if (A[0] + A[1]*P[0] + A[2]*P[0]**2 + A[3]*P[0]**3 + (B[0] + B[1]*P[0])*P[1]) % p == 0]
    roots = []
    for x in range(p):
        Av = (A[0] + A[1]*x + A[2]*x*x + A[3]*x**3) % p
        Bv = (B[0] + B[1]*x) % p
        if (Av*Av - Bv*Bv*(x**3 + a*x + b)) % p == 0 and Bv: roots.append(x)
    st['trials'] += 1
    zeros = [P for P in zeros if (B[0] + B[1]*P[0]) % p]  # B(x)=0 zeros are the degenerate chart (A=B=0 at x), excluded
    if sorted(P[0] for P in zeros) != roots:
        st['chart_viol'] += 1
    if len(roots) == 6 and len(set(P[0] for P in zeros)) == 6 and len(zeros) == 6:
        st['split'] += 1
        S = None
        for P in zeros: S = add(S, P, p, a)
        if S is not None: st['sum_viol'] += 1
        Z = list(zeros); Z[0] = (Z[0][0], (-Z[0][1]) % p)
        S = None
        for P in Z: S = add(S, P, p, a)
        if S is None: st['neg_ctrl_hit'] += 1
def arm(p, trials, seed):
    rng = random.Random(seed)
    a, b = curve(p, rng)
    pts = points(p, a, b)
    st = dict(trials=0, chart_viol=0, split=0, sum_viol=0, neg_ctrl_hit=0)
    for _ in range(trials): trial(p, a, b, pts, rng, st)
    st.update(p=p, a=a, b=b, order_affine_pts=len(pts), seed=seed,
              split_rate=st['split']/trials, expected_rate=1/720)
    return st
def main():
    smoke = '--smoke' in sys.argv
    cfg = [(101, 300, 1)] if smoke else [(521, 60000, 11), (521, 60000, 12), (1031, 20000, 13)]
    arms = [arm(*c) for c in cfg]
    T = sum(x['trials'] for x in arms); S = sum(x['split'] for x in arms)
    cv = sum(x['chart_viol'] for x in arms); sv = sum(x['sum_viol'] for x in arms)
    nh = sum(x['neg_ctrl_hit'] for x in arms)
    sig = math.sqrt(T/720*(1-1/720)); z = (S - T/720)/sig
    aa = None
    if len(arms) >= 2:
        s1, s2 = arms[0]['split'], arms[1]['split']
        aa = (s1 - s2)/math.sqrt(max(s1 + s2, 1))
    if cv or sv: v = "falsified"
    elif S < 30 or smoke: v = "inconclusive"
    elif abs(z) < 4 and nh <= 0.02*S and (aa is None or abs(aa) < 4): v = "consistent_with_prediction"
    else: v = "falsified"
    print(json.dumps(dict(lead="LEAD-ICP-shared-coefficient-membership", smoke=smoke, arms=arms,
        total_trials=T, total_split=S, chart_violations=cv, abel_sum_violations=sv,
        negative_control_false_sums=nh, split_rate_z_vs_1_over_720=z, aa_z=aa, verdict=v)))
main()
