"""Checker's own contrast: F4 (msolve) on the UNIVARIATE-base (U) decomposition system, non-decomposable
target, m=2,3.  Reports F4 max degree and largest matrix vs d^m (grid size). [pcc:unverified]"""
import sys, random, subprocess, re, json, time
import icmodel as M, fastdelta as F
p = 786433
m = int(sys.argv[1]); d = int(sys.argv[2]); seed = int(sys.argv[3]) if len(sys.argv) > 3 else 1
rng = random.Random(f"chk-uni-{m}-{d}-{seed}")
while True:
    a = rng.randrange(p); b = rng.randrange(1, p)
    if (4 * a ** 3 + 27 * b ** 2) % p: break
roots = sorted(rng.sample(range(1, p), d))
while True:
    xR = rng.randrange(p)
    SR = M.semaev_special(m, p, a, b, xR)
    G = F.semaev_grid(SR, m, roots, p)
    if not (G == 0).any(): break
f = [1]
for r in roots: f = M.polmul1(f, [(-r) % p, 1], p)
names = [f"x{i}" for i in range(1, m + 1)]
def ps(P):
    t = []
    for e, c in P.items():
        mon = "*".join(f"{names[v]}^{q}" if q > 1 else names[v] for v, q in enumerate(e) if q)
        t.append(f"{c}" + (f"*{mon}" if mon else ""))
    return "+".join(t)
polys = [ps(SR)]
for i in range(m):
    P = {}
    for kk, c in enumerate(f):
        if c:
            e = [0] * m; e[i] = kk; P[tuple(e)] = c
    polys.append(ps(P))
fn = f"../out/chk_uni_m{m}_d{d}_s{seed}.ms"
open(fn, "w").write(",".join(names) + "\n" + str(p) + "\n" + ",\n".join(polys) + "\n")
t0 = time.time()
out = subprocess.run(["msolve", "-v", "2", "-g", "1", "-t", "1", "-f", fn, "-o", fn + ".out"], capture_output=True, text=True, timeout=1200)
txt = out.stdout + out.stderr
open(fn + ".log", "w").write(txt)
steps = re.findall(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s*x\s*(\d+)", txt, re.M)
print(json.dumps({"m": m, "d": d, "grid": d ** m, "F4_max_deg": max(int(s[0]) for s in steps) if steps else None,
  "F4_max_rows": max((int(s[3]) for s in steps), default=0), "F4_max_cols": max((int(s[4]) for s in steps), default=0),
  "no_solution": "No solution" in txt, "rc": out.returncode, "sec": round(time.time() - t0, 2)}))
