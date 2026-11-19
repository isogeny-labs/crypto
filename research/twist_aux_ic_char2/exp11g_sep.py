# EXP11g: separation hypothesis. Per system (q=31,43): F_q-rational sigma-points via GB + field equations
# (read off by exhaustive evaluation is q^4 -- too big at 43; instead solve the GB with field eqs by msolve's
# own default path, which is in generic position for such tiny radical ideals, else fall back to GB triangular read).
# Then: does s4 separate them? fraction of F_q linear forms (projective) separating them? vs observed retries (Exp 11d).
import subprocess, os, sys, ast, re, itertools, random
env = dict(os.environ, DYLD_LIBRARY_PATH="/opt/homebrew/Cellar/flint/3.6.0/lib", OMP_NUM_THREADS="1"); T = os.environ["TMPDIR"]
obs = {}
cur = None
for l in open("results/exp11d_default_path.log"):
    if l.startswith("file"): cur = None
    m = l.split()
    if l.startswith("c") and len(m) > 11: obs.setdefault(m[0], []).append((m[1], m[6], m[11], m[10]))
def points_fe(f, p):
    txt = open(f).read().rstrip("\n"); ff = T + "/sep.ms"
    open(ff, "w").write(txt + ",\n" + ",\n".join(f"1*s{i}^{p}+{p-1}*s{i}" for i in range(1, 5)) + "\n")
    subprocess.run(["msolve", "-f", ff, "-o", T + "/sep.gb", "-g", "2", "-t", "1"], env=env, capture_output=True, timeout=600)
    G = open(T + "/sep.gb").read(); body = G[G.index('['):G.rindex(']')].strip('[]:')
    polys = []
    for ps in body.split(','):
        ps = ps.strip().replace(' ', '')
        if not ps: continue
        terms = []
        for c, mon in re.findall(r'([+-]?\d+)((?:\*s\d\^\d+)*)', ps):
            e = [0]*4
            for v, k in re.findall(r's(\d)\^(\d+)', mon): e[int(v)-1] = int(k)
            terms.append((int(c), e))
        polys.append(terms)
    ev = lambda P, x: sum(c*pow(x[0],e[0],p)*pow(x[1],e[1],p)*pow(x[2],e[2],p)*pow(x[3],e[3],p) for c, e in P) % p
    # triangular search: use polys only in s4, then s3,s4 ... (grevlex basis of a tiny radical ideal; brute force by backtracking)
    pts = []
    def rec(x, k):  # assign variables in order s4, s3, s2, s1
        idx = 3 - k
        if k == 4:
            if all(ev(P, x) == 0 for P in polys): pts.append(tuple(x))
            return
        for v in range(p):
            x[idx] = v
            # prune with polys involving only assigned variables
            ok = all(ev(P, x) == 0 for P in polys if all(e[j] == 0 for _, e in P for j in range(idx)))
            if ok: rec(x, k+1)
        x[idx] = 0
    rec([0]*4, 0)
    return pts
for q in (31, 43):
    D = f"msolve_q{q}"
    print(f"--- q={q}")
    # projective forms over F_q: first nonzero coordinate 1
    forms = [l for l in itertools.product(range(q), repeat=4) if any(l) and l[[i for i in range(4) if l[i]][0]] == 1]
    for line in open(f"{D}/manifest.txt"):
        f, ty, cv, bf = line.split(); name = f.split("/")[-1]
        pts = points_fe(f, q)
        diffs = [tuple((a[i]-b[i]) % q for i in range(4)) for a, b in itertools.combinations(pts, 2)]
        s4sep = all(d[3] != 0 for d in diffs)
        good = sum(1 for l in forms if all(sum(l[i]*d[i] for i in range(4)) % q for d in diffs))
        o = obs[name][0] if q == 31 else obs[name][1] if len(obs[name]) > 1 else obs[name][0]
        print(f"{name} {ty} Fq-pts {len(pts)} pairs {len(diffs)} s4-separates {s4sep} sep-form-fraction {good/len(forms):.3f} | observed generic {o[3]} retries {o[2]} status {o[1]}", flush=True)
