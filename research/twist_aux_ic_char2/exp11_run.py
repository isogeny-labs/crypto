# EXP11 driver: run msolve on each symmetrised system; record status, ideal degree, wall time;
# recover F_q-rational sigma solutions from the parametrisation, verify them against the input equations,
# and count those whose quartic splits over F_q (candidate decompositions) vs the brute-force count.
import subprocess, time, os, re, sys, itertools, ast
env = dict(os.environ, DYLD_LIBRARY_PATH="/opt/homebrew/Cellar/flint/3.6.0/lib", OMP_NUM_THREADS="1")
p = 11
PTO = 120
def peval(coeffs, x):  # coeffs low->high
    r = 0
    for c in reversed(coeffs): r = (r*x + c) % p
    return r
def parse_polys(path):
    lines = open(path).read().split("\n")
    body = "".join(lines[2:]).split(",")
    polys = []
    for b in body:
        b = b.strip()
        if not b: continue
        terms = []
        for t in b.split("+"):
            parts = t.split("*"); c = int(parts[0]); ex = [0,0,0,0]
            for v in parts[1:]:
                if "^" in v: n, e = v.split("^"); ex[int(n[1])-1] = int(e)
                else: ex[int(v[1])-1] = 1
            terms.append((c, ex))
        polys.append(terms)
    return polys
def evalp(terms, s): return sum(c*pow(s[0],e[0],p)*pow(s[1],e[1],p)*pow(s[2],e[2],p)*pow(s[3],e[3],p) for c,e in terms) % p
rows = []
for line in open("msolve_runs/manifest.txt"):
    f, ty, cv, bf = line.split(); out = f.replace(".ms", ".out")
    gb = f.replace(".ms", ".gb")
    t0 = time.time(); subprocess.run(["msolve", "-f", f, "-o", gb, "-g", "2", "-t", "1"], env=env, capture_output=True, text=True, timeout=1800); gbt = time.time() - t0
    gs = subprocess.run(["python3", "gb_stats.py", gb], capture_output=True, text=True).stdout
    gdeg = re.search(r"degree (\d+)", gs); gdeg = int(gdeg.group(1)) if gdeg else -1
    t0 = time.time()
    try:
        subprocess.run(["msolve", "-f", f, "-o", out, "-t", "1"], env=env, capture_output=True, text=True, timeout=PTO)
        txt = open(out).read().strip().rstrip(":"); data = ast.literal_eval(txt); status = data[0]; info = data[1]
    except subprocess.TimeoutExpired:
        status = "TIMEOUT"; info = None
    dt = time.time() - t0
    deg = info[2] if status == 0 else None
    nsol = nsplit = nbad = 0
    if status == 0:
        lin = info[4]; par = info[5][1]
        elim = par[0][1]; den = par[1][1]; params = par[2]
        assert lin == [0,0,0,1], lin
        for th in range(p):
            if peval(elim, th) != 0: continue
            d = peval(den, th)
            if d == 0: nbad += 1; continue
            polys = parse_polys(f)
            # msolve: x_i = -v_i(th) / den(th), den given (or w'(th) if den is the constant 1); try both, keep the one that verifies
            dw = [(k*elim[k]) % p for k in range(1, len(elim))]
            ok = False
            for dd in (d, peval(dw, th)):
                if dd % p == 0: continue
                s = [None]*4; s[3] = th
                for i, pr in enumerate(params):
                    s[i] = (-peval(pr[0][1], th) * pow(dd, p-2, p)) % p
                if all(evalp(P, s) == 0 for P in polys): ok = True; break
            if not ok: nbad += 1; continue
            nsol += 1
            # quartic X^4 - s1 X^3 + s2 X^2 - s3 X + s4 : count roots with multiplicity over F_p
            coeffs = [s[3], (-s[2]) % p, s[1], (-s[0]) % p, 1]
            roots = [x for x in range(p) if peval(coeffs, x) == 0]
            # multiplicity: divide out
            cnt = 0; c = coeffs[:]
            for x in range(p):
                while len(c) > 1 and peval(c, x) == 0:
                    # synthetic division
                    q = [0]*(len(c)-1); acc = 0
                    for k in range(len(c)-1, 0, -1): acc = (acc*x + c[k]) % p; q[k-1] = acc
                    c = q; cnt += 1
            if cnt == 4: nsplit += 1
    rows.append((f, ty, cv, int(bf), round(gbt,2), gdeg, status, deg, round(dt,2), nsol, nsplit, nbad))
    print(*rows[-1], flush=True)
print("columns: file type curve bruteforce gb_time gb_degree param_status param_degree param_time Fq_sigma_sols split bad")
for ty in "PR":
    R = [r for r in rows if r[1] == ty]; ok = [r for r in R if r[6] == 0]
    print(f"SUMMARY {ty}: systems {len(R)}, GB time mean {sum(r[4] for r in R)/len(R):.2f}s, GB degrees {sorted(r[5] for r in R)}, "
          f"param finished {len(ok)}/{len(R)} (timeouts {sum(1 for r in R if r[6]=='TIMEOUT')}), "
          f"on finished: split sols {sum(r[10] for r in ok)} vs brute force {sum(r[3] for r in ok)}")
