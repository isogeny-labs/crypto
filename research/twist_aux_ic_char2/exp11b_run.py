# EXP11b: per system (a) msolve diagnostics on the plain sigma system: ideal degree, elimination-poly degree,
# squarefree degree, generic-position flag (first FGLM attempt only, 40 s cap);
# (b) solve with field equations s_i^11 - s_i added (F_q-rational sigma only): time, #solutions, #split, vs brute force.
import subprocess, time, os, re, ast
env = dict(os.environ, DYLD_LIBRARY_PATH="/opt/homebrew/Cellar/flint/3.6.0/lib", OMP_NUM_THREADS="1")
p = 11; T = os.environ.get("TMPDIR", "/tmp")
def peval(c, x):
    r = 0
    for a in reversed(c): r = (r*x + a) % p
    return r
def nroots_mult(coeffs):
    cnt = 0; c = coeffs[:]
    for x in range(p):
        while len(c) > 1 and peval(c, x) == 0:
            q = [0]*(len(c)-1); acc = 0
            for k in range(len(c)-1, 0, -1): acc = (acc*x + c[k]) % p; q[k-1] = acc
            c = q; cnt += 1
    return cnt
rows = []
print("file type curve bruteforce | deg elim sqfr generic | fieldeq_time nsol nsplit status")
for line in open("msolve_runs/manifest.txt"):
    f, ty, cv, bf = line.split()
    # (a) diagnostics
    try:
        r = subprocess.run(["msolve", "-f", f, "-o", T + "/diag.out", "-t", "1", "-v", "2"], env=env, capture_output=True, text=True, timeout=40)
        log = r.stdout + r.stderr
    except subprocess.TimeoutExpired as e:
        log = (e.stdout or b"").decode() if isinstance(e.stdout, bytes) else (e.stdout or "")
    g = lambda pat: (lambda m: int(m.group(1)) if m else None)(re.search(pat, log))
    deg, de, ds = g(r"degree of ideal\s+(\d+)"), g(r"deg\. elim\. pol\.\s+(\d+)"), g(r"deg\. sqfr\. elim\. pol\.\s+(\d+)")
    generic = "no" if "not in generic position" in log else ("yes" if de is not None else "?")
    # (b) field equations
    txt = open(f).read().rstrip("\n")
    ff = T + "/fe.ms"; open(ff, "w").write(txt + ",\n" + ",\n".join(f"1*s{i}^{p}+{p-1}*s{i}" for i in range(1,5)) + "\n")
    t0 = time.time()
    try:
        subprocess.run(["msolve", "-f", ff, "-o", T + "/fe.out", "-t", "1"], env=env, capture_output=True, text=True, timeout=600)
        out = open(T + "/fe.out").read().strip().rstrip(":"); data = ast.literal_eval(out); st = data[0]
    except subprocess.TimeoutExpired:
        st = "TIMEOUT"; data = None
    dt = time.time() - t0
    nsol = nsplit = 0
    if st == 0:
        info = data[1]; par = info[5][1]; elim = par[0][1]; den = par[1][1]; params = par[2]
        dw = [(k*elim[k]) % p for k in range(1, len(elim))]
        for th in range(p):
            if peval(elim, th): continue
            for dd in (peval(den, th), peval(dw, th)):
                if dd % p == 0: continue
                s = [(-peval(pr[0][1], th) * pow(dd, p-2, p)) % p for pr in params] + [th]
                break
            else: continue
            nsol += 1
            if nroots_mult([s[3], (-s[2]) % p, s[1], (-s[0]) % p, 1]) == 4: nsplit += 1
    elif st == -1 or (isinstance(st, int) and st != 0):
        st = f"status{st}"
    rows.append((f, ty, cv, int(bf), deg, de, ds, generic, round(dt, 2), nsol, nsplit, st))
    print(*rows[-1], flush=True)
for ty in "PR":
    R = [r for r in rows if r[1] == ty]
    print(f"SUMMARY {ty}: n={len(R)} ideal deg {sorted(set(r[4] for r in R if r[4]))}, elim deg {sorted(r[5] for r in R if r[5])}, "
          f"sqfr {sorted(r[6] for r in R if r[6])}, generic position {sum(r[7]=='yes' for r in R)}/{len(R)} | "
          f"field-eq solve mean {sum(r[8] for r in R)/len(R):.2f}s, statuses {sorted(set(str(r[11]) for r in R))}, split {sum(r[10] for r in R)} vs brute force {sum(r[3] for r in R)}")
