"""Write three presentations of the same ring-form quotient: dp in (s,w), wp(2..2,1) in (s,w),
and dp in (s, s_m, w) with the quadric s_m - w^2. Times Singular std on each."""
import sys, subprocess, time
sys.path.insert(0, __file__.rsplit("/", 1)[0])
import ringu
p, m, seed, out = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3]), sys.argv[4]
res = ringu.build(p, m, seed, "rand")
def polys(form):
    var = [f"s{k}" for k in range(1, m)] + ([f"s{m}"] if form == "quad" else []) + ["w"]
    out = []
    for eq in res["eqs"]:
        terms = []
        for (sexp, pe), c in eq.items():
            parts = [f"s{k+1}^{e}" for k, e in enumerate(sexp) if e]
            if form == "quad":
                if pe // 2: parts.append(f"s{m}^{pe//2}")
                if pe % 2: parts.append("w")
            elif pe: parts.append(f"w^{pe}")
            terms.append(f"{c}*" + ("*".join(parts) if parts else "1"))
        out.append("+".join(terms))
    if form == "quad": out.append(f"s{m}-w^2")
    return var, out
for form, order in (("ring", "dp"), ("ring", "wp(" + ",".join(["2"]*(m-1)) + ",1)"), ("quad", "dp")):
    var, ps = polys(form)
    path = f"{out}_{form}_{order[:2]}.sing"
    with open(path, "w") as f:
        f.write(f"ring r = {p},({','.join(var)}),{order};\nideal I = " + ",\n".join(ps) + ";\n")
        f.write('option(prot); int t=timer; ideal G = std(I); print("vdim"); print(vdim(G)); print("ms"); print(timer-t); quit;\n')
    t = time.time()
    r = subprocess.run(["timeout", "300", "nice", "Singular", "-q", path], capture_output=True, text=True)
    o = r.stdout
    degs = [int(x) for x in __import__("re").findall(r"\[(\d+):", o)]
    print(form, order, "wall %.2fs" % (time.time() - t), "vdim", o.split("vdim")[-1].split()[0] if "vdim" in o else None,
          "maxdeg-in-prot", max(degs) if degs else None)
    # also msolve format for the dp forms
    if order == "dp":
        with open(path.replace(".sing", ".ms"), "w") as f:
            f.write(",".join(var) + "\n" + str(p) + "\n" + ",\n".join(ps) + "\n")
