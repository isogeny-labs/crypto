"""Write the m=2 tower decomposition system (S_3 in x, tower chain per summand) for msolve and report the
F4 step degrees, next to the exact XL degree dS + delta from the interpolation lemma.  [pcc:unverified]"""
import sys, random, subprocess, re, json, time
import numpy as np
import icmodel as M, fastdelta as F
p = 786433
k = int(sys.argv[1]); seed = int(sys.argv[2]) if len(sys.argv) > 2 else 1
rng = random.Random(f"msolve-tower-{k}-{seed}")
while True:
    a = rng.randrange(p); b = rng.randrange(1, p)
    if (4 * a ** 3 + 27 * b ** 2) % p: break
base = M.tower_base(p, k, rng.randrange(2, p))
roots = [w[0] for w in base.pts]
while True:
    xR = rng.randrange(p)
    SR = M.semaev_special(2, p, a, b, xR)
    G = F.semaev_grid(SR, 2, roots, p)
    if not (G == 0).any(): break
# exact XL refutation degree via the lemma (S input in x-variables: dS = 4)
d = 2 ** k
Vinv = F.mat_inv_mod(F.vandermonde(roots, p), p)
C = F.coeffs(F.inv_mod_array(G, p), Vinv, p)
delta = F.delta_of(C, F.weight_grid(d, 2, "tower"))
names = [f"x{i}" if j == 0 else f"t{i}_{j}" for i in (1, 2) for j in range(k)]
def poly_str(P, N):
    terms = []
    for e, c in P.items():
        mon = "*".join(f"{names[v]}^{q}" if q > 1 else names[v] for v, q in enumerate(e) if q)
        terms.append(f"{c}" + (f"*{mon}" if mon else ""))
    return "+".join(terms) if terms else "0"
N = 2 * k
polys = [poly_str(M.S_global(SR, 2, k), N)] + [poly_str(M.lift_block(g, i, k, N), N) for i in range(2) for g in base.gens]
fn = f"../out/msolve_tower_k{k}_s{seed}.ms"
open(fn, "w").write(",".join(names) + "\n" + str(p) + "\n" + ",\n".join(polys) + "\n")
t0 = time.time()
out = subprocess.run(["msolve", "-v", "2", "-g", "1", "-t", "1", "-f", fn, "-o", fn + ".out"], capture_output=True, text=True, timeout=1200)
txt = out.stdout + out.stderr
open(fn + ".log", "w").write(txt)
steps = re.findall(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s*x\s*(\d+)", txt, re.M)
degs = [int(s[0]) for s in steps]
maxcols = max((int(s[4]) for s in steps), default=0)
maxrows = max((int(s[3]) for s in steps), default=0)
nosol = "No solution" in txt
print(json.dumps({"k": k, "N": N, "xR": xR, "XL_degree_lemma": 4 + delta, "delta": delta, "max": 2 * k,
                  "msolve_rc": out.returncode, "F4_max_pair_degree": max(degs) if degs else None,
                  "F4_rounds": len(degs), "F4_max_rows": maxrows, "F4_max_cols": maxcols, "no_solution": nosol,
                  "XL_cols_at_lemma_degree": __import__("math").comb(N + 4 + delta, N), "sec": round(time.time() - t0, 2), "log": fn + ".log"}))
