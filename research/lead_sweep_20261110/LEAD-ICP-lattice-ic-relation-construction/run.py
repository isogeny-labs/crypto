import sys, json, math, random
SMOKE = "--smoke" in sys.argv

def is_prime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17):
        x = pow(a,d,n)
        if x in (1,n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def safe_prime(bits):
    n = (1 << bits) + 1
    while True:
        if is_prime(n) and is_prime(2*n+1): return n
        n += 2

def trial(n, seed, planted=False, null=False):
    """Oracle-free relation construction in the order-n subgroup of Z_p^*, p=2n+1.
    Cost unit: one group multiplication per pair product. Returns ops to R>=m relations."""
    p = 2*n+1
    rng = random.Random(seed)
    m = max(8, int(2*(8*n)**(1/3)))
    exps = [rng.randrange(1, n) for _ in range(m)]
    if planted:  # positive control: plant e0+e1 == e2+e3 mod n
        exps[3] = (exps[0]+exps[1]-exps[2]) % n
    els = [pow(4, e, p) for e in exps]   # 4 generates the order-n subgroup
    if null:  # null: elements with random independent values (no group structure test of sums beyond chance)
        els = [pow(4, rng.randrange(1,n), p) for _ in range(m)]
    seen = {}; R = 0; ops = 0; valid = 0
    for i in range(m):
        for j in range(i+1, m):
            ops += 1
            v = els[i]*els[j] % p
            if v in seen:
                a, b = seen[v]
                if len({a,b,i,j}) == 4:
                    R += 1
                    if not null and (exps[a]+exps[b]-exps[i]-exps[j]) % n == 0: valid += 1
                    if R >= m: return dict(ops=ops, R=R, m=m, valid=valid, censored=False)
            else:
                seen[v] = (i, j)
        if planted and R >= 1: return dict(ops=ops, R=R, m=m, valid=valid, censored=False)
    return dict(ops=ops, R=R, m=m, valid=valid, censored=True)

def median(x):
    x = sorted(x); k = len(x)
    return x[k//2] if k % 2 else 0.5*(x[k//2-1]+x[k//2])

def slope(xs, ys):
    mx, my = sum(xs)/len(xs), sum(ys)/len(ys)
    return sum((a-mx)*(b-my) for a,b in zip(xs,ys)) / sum((a-mx)**2 for a in xs)

def main():
    bits_list = [12] if SMOKE else [12, 14, 16, 18, 20, 22]
    T = 3 if SMOKE else 30
    rows = []
    for bits in bits_list:
        n = safe_prime(bits)
        rho = math.sqrt(math.pi*n/4)
        armA = [trial(n, 1000+s) for s in range(T)]
        armB = [trial(n, 5000+s) for s in range(T)]   # A/A: disjoint seed set, same instrument
        ctrl = [trial(n, 9000+s, planted=True) for s in range(T)]
        nul = [trial(n, 7000+s, null=True) for s in range(T)]
        rA = median([r["ops"] for r in armA])/rho
        rB = median([r["ops"] for r in armB])/rho
        rows.append(dict(bits=bits, n=n, rho_ops=rho, ratio_A=rA, ratio_AA=rB,
            aa_rel_diff=abs(rA-rB)/max(rA,rB),
            censored_A=sum(r["censored"] for r in armA),
            invalid_relations=sum(r["valid"] != r["R"] for r in armA),
            positive_control_found=sum((not r["censored"]) and r["valid"]>=1 for r in ctrl),
            null_ratio=median([r["ops"] for r in nul])/rho))
    xs = [math.log2(r["n"]) for r in rows]; ys = [math.log2(r["ratio_A"]) for r in rows]
    sl = slope(xs, ys) if len(rows) > 1 else None
    pc_ok = all(r["positive_control_found"] == T for r in rows)
    aa_ok = all(r["aa_rel_diff"] <= 0.25 for r in rows)
    valid_ok = all(r["invalid_relations"] == 0 for r in rows)
    if not (pc_ok and valid_ok): v = "inconclusive"
    elif any(r["ratio_A"] < 1.0 for r in rows): v = "falsified"
    elif SMOKE or not aa_ok or sl is None: v = "inconclusive"
    elif sl >= 0.05: v = "consistent_with_prediction"
    else: v = "inconclusive"
    print(json.dumps(dict(lead="LEAD-ICP-lattice-ic-relation-construction", smoke=SMOKE,
        statistic="median ops to collect m=2(8n)^(1/3) four-term relations / sqrt(pi n/4); slope of log2 ratio vs log2 n",
        threshold="ratio>=1 at all sizes and slope>=0.05 (expected ~1/6)",
        seeds=dict(armA=1000, AA=5000, positive=9000, null=7000, trials=T),
        params=dict(bits=bits_list, group="order-n subgroup of Z_(2n+1)^*, n safe-prime"),
        rows=rows, slope=sl, positive_control_ok=pc_ok, aa_ok=aa_ok, relations_valid=valid_ok,
        verdict=v)))
main()
