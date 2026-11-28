# EXP11d: msolve DEFAULT path (no field equations) on n=4 symmetrised S5 systems at a given q.
# usage: python3 exp11d_run.py DIR q PTO
# Per system: GB-only time + degree (grevlex), then full solve with -v 2 (generic-position diagnostics, param time),
# recover F_q-rational sigma from the rational parametrisation, verify against the input, count F_q-split quartics,
# compare to the meet-in-the-middle ground truth in DIR/manifest.txt.
import subprocess, time, os, re, sys, ast, statistics
D, p, PTO = sys.argv[1], int(sys.argv[2]), int(sys.argv[3])
env = dict(os.environ, DYLD_LIBRARY_PATH="/opt/homebrew/Cellar/flint/3.6.0/lib", OMP_NUM_THREADS="1")
def peval(c, x):
    r = 0
    for a in reversed(c): r = (r*x + a) % p
    return r
def parse_polys(path):
    body = "".join(open(path).read().split("\n")[2:]).split(",")
    out = []
    for b in body:
        b = b.strip()
        if not b: continue
        T = []
        for t in b.split("+"):
            parts = t.split("*"); c = int(parts[0]); ex = [0]*4
            for v in parts[1:]:
                if "^" in v: n, e = v.split("^"); ex[int(n[1])-1] = int(e)
                else: ex[int(v[1])-1] = 1
            T.append((c, ex))
        out.append(T)
    return out
def evalp(T, s): return sum(c*pow(s[0],e[0],p)*pow(s[1],e[1],p)*pow(s[2],e[2],p)*pow(s[3],e[3],p) for c,e in T) % p
def nroots_mult(c):
    c = c[:]; n = 0
    for x in range(p):
        while len(c) > 1 and peval(c, x) == 0:
            q = [0]*(len(c)-1); acc = 0
            for k in range(len(c)-1, 0, -1): acc = (acc*x + c[k]) % p; q[k-1] = acc
            c = q; n += 1
    return n
g = lambda pat, s: (lambda m: int(m.group(1)) if m else None)(re.search(pat, s))
rows = []
print("file type curve truth | gb_time gb_deg | status param_time elim sqfr generic tries | sols split bad", flush=True)
for line in open(f"{D}/manifest.txt"):
    f, ty, cv, bf = line.split(); bf = int(bf)
    gbf = f.replace(".ms", ".gb"); out = f.replace(".ms", ".out")
    REP = os.environ.get("REPARSE") == "1"
    t0 = time.time()
    if not REP: subprocess.run(["msolve","-f",f,"-o",gbf,"-g","2","-t","1"], env=env, capture_output=True, timeout=3600)
    gbt = time.time()-t0
    gdeg = g(r"degree (\d+)", subprocess.run(["python3","gb_stats.py",gbf], capture_output=True, text=True).stdout)
    t0 = time.time(); log = ""
    try:
        if REP:
            if not os.path.exists(out) or os.path.getmtime(out) < os.path.getmtime(f): raise subprocess.TimeoutExpired("reparse", PTO)
            data = ast.literal_eval(open(out).read().strip().rstrip(":")); status = data[0]; info = data[1]; raise StopIteration
        r = subprocess.run(["msolve","-f",f,"-o",out,"-t","1","-v","2"], env=env, capture_output=True, text=True, timeout=PTO)
        log = r.stdout + r.stderr
        data = ast.literal_eval(open(out).read().strip().rstrip(":")); status = data[0]; info = data[1]
    except subprocess.TimeoutExpired as e:
        status, info = "TIMEOUT", None
        log = (e.stdout.decode() if isinstance(e.stdout, bytes) else (e.stdout or "")) + (e.stderr.decode() if isinstance(e.stderr, bytes) else (e.stderr or ""))
    except StopIteration:
        pass
    except Exception as e:
        status, info = f"ERR:{type(e).__name__}", None
    dt = time.time()-t0
    de, ds = g(r"deg\. elim\. pol\.\s+(\d+)", log), g(r"deg\. sqfr\. elim\. pol\.\s+(\d+)", log)
    tries = log.count("not in generic position")
    gen = "no" if tries else ("yes" if de is not None else "?")
    nsol = nsplit = nbad = 0
    if status == 0:
        order = info[3]; lin = info[4]; par = info[5][1]; elim, den, params = par[0][1], par[1][1], par[2]
        polys = parse_polys(f); dw = [(k*elim[k]) % p for k in range(1, len(elim))]
        for th in range(p):
            if peval(elim, th) != 0: continue
            ok = False
            for dd in (peval(den, th), peval(dw, th)):
                if dd % p == 0: continue
                # msolve variable order may be permuted, or carry an extra variable 'A' (a linear form);
                # params give the first nv-1 variables, the last follows from theta = sum lin_i y_i
                nv = len(order); inv = pow(dd, p-2, p); y = [None]*nv
                for i, pr in enumerate(params): y[i] = (-peval(pr[0][1], th)*inv) % p
                if lin[nv-1] % p == 0: continue
                y[nv-1] = ((th - sum(lin[i]*y[i] for i in range(nv-1))) * pow(lin[nv-1] % p, p-2, p)) % p
                s = [None]*4
                for i, nm in enumerate(order):
                    if nm.startswith("s"): s[int(nm[1])-1] = y[i]
                if all(evalp(P, s) == 0 for P in polys): ok = True; break
            if not ok: nbad += 1; continue
            nsol += 1
            if nroots_mult([s[3], (-s[2]) % p, s[1], (-s[0]) % p, 1]) == 4: nsplit += 1
    row = (f.split("/")[-1], ty, cv, bf, round(gbt,2), gdeg, status, round(dt,1), de, ds, gen, tries, nsol, nsplit, nbad)
    rows.append(row); print(*row, flush=True)
for ty in "PR":
    R = [r for r in rows if r[1] == ty]; ok = [r for r in R if r[6] == 0]
    if not R: continue
    print(f"SUMMARY q={p} {ty}: n={len(R)} | GB time median {statistics.median(r[4] for r in R):.2f}s, degrees {sorted(set(r[5] for r in R))} | "
          f"solve finished {len(ok)}/{len(R)}, param time median {statistics.median(r[7] for r in R):.1f}s (cap {PTO}) | "
          f"generic {sum(r[10]=='yes' for r in R)}/{len(R)} | finished: split {sum(r[13] for r in ok)} vs truth {sum(r[3] for r in ok)}, bad {sum(r[14] for r in ok)}", flush=True)
