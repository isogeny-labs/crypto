import itertools, random, sys, time
import fpeval as FE, smith as SM
S = "/tmp/claude-0/-home-user/c9b2fe43-c085-5dfa-9cde-b0311430c5d1/scratchpad/twist/"
p = int(sys.argv[1])
fp_, PAIRS, rp = FE.load_formula(S + "f_000000.json")
tab = FE.sigma_table(rp["L_subsets"])
print("pair lagrangians", len(tab))
rng = random.Random(5)
T = FE.Tower(p, [0, 1], rng)
for cur in range(3):
    u = rng.sample(range(p), 8)
    c = rng.randrange(1, p)
    F = [c]
    for x in u:
        F = SM.polmul(F, [(-x) % p, 1], p)
    es_all = [T.K.c(x) for x in u]
    agree = dis = 0
    for key, sigs in tab.items():
        pairs = [tuple(sorted(s)) for s in key if len(s) == 2]
        pr = [(u[a], u[b]) for a, b in pairs]
        maps = SM.maps_for_pairing(pr, p)
        if not maps or maps == "degenerate":
            continue
        sy = [SM.prop4_symbols(N, D, F, p) for N, D in maps]
        flat = set(x for s in sy for x in s)
        sig = sigs[0]
        es = [es_all[sig[i]] for i in range(8)]
        t0 = time.time()
        v = FE.eval_formula(T, fp_, PAIRS, es, rng)
        v2 = FE.eval_formula(T, fp_, PAIRS, [es_all[sigs[7][i]] for i in range(8)], rng)
        x = FE.in_Fp(T, v)
        x2 = FE.in_Fp(T, v2)
        mine = FE.legendre(x, p) * FE.legendre(c, p) if x is not None else None
        print(cur, sorted(pairs), "smith", sy, "val", x, x2, "mine", mine, "%.2fs" % (time.time() - t0))
