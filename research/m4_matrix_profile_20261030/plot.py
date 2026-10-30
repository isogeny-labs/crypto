import json, math, os
os.environ.setdefault("MPLCONFIGDIR", os.environ.get("TMPDIR", "/tmp") + "/mpl")
import numpy as np
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
R = [r for r in json.load(open("data/recorded_closure_rows.json")) if not r["fixed"] and r["degree"] >= 3 and r["closure_word_ops"] > 0]
fits = {f["name"]: f for f in json.load(open("data/fit_w.json"))}
rp = json.load(open("data/reprice.json"))["rows"]
fig, ax = plt.subplots(1, 2, figsize=(11, 4.4))
col = {3: "#2a6fbb", 4: "#d9822b", 5: "#8a3ab9"}
for d in (3, 4, 5):
    for m, mk in ((3, "o"), (4, "s")):
        pts = [r for r in R if r["degree"] == d and r["m"] == m and not r["closure_refuted"] and r["saturated"]]
        ax[0].scatter([math.log2(r["columns"]) for r in pts], [math.log2(r["closure_word_ops"]) for r in pts], s=8, marker=mk, color=col[d], alpha=.5, label=f"deg {d}, m={m}")
    f = fits[f"m3+m4 deg{d} saturated-not-refuted"]
    xs = np.linspace(f["log2cols"][0], f["log2cols"][1], 10)
    ax[0].plot(xs, f["a"] + f["w"] * xs, color=col[d], lw=1.5)
    ax[0].annotate(f"w={f['w']:.2f} [{f['w_ci'][0]:.2f},{f['w_ci'][1]:.2f}]", (xs[-1], f["a"] + f["w"] * xs[-1]), fontsize=8, color=col[d], xytext=(-95, -14), textcoords="offset points")
ax[0].set_xlabel("log2 columns"); ax[0].set_ylabel("log2 counted word ops"); ax[0].legend(fontsize=6, ncol=2); ax[0].set_title("Dense MutantXL closure: counted ops vs columns\n(recorded saturating runs; lines: OLS with intercept, per degree)", fontsize=9)
ws = np.linspace(1, 3.1, 40)
import importlib.util
spec = importlib.util.spec_from_file_location("rp", "reprice.py")
# re-evaluate B bits along w with a=0 from the printed rows: use stored grid + a quick port
exec(open("reprice.py").read().split("fits = json.load")[0].split("# positive controls")[0].split('"""', 2)[2])
CM = -8.44
ax[1].plot(ws, [min(cost(4, l, 5, 0, w, CM)[0] for l in range(4, 70)) - RHO for w in ws], color="#444", label="D=5, a=0 (existing table convention), rho-step units")
ax[1].axhline(30, color="red", ls="--", lw=1); ax[1].text(1.02, 31, "+30 bits: downgrade rule", color="red", fontsize=8)
for r in rp:
    if r["label"].startswith("deg-5") or r["label"].startswith("deg-4 block fit (m3"):
        ax[1].errorbar([r["w"]], [r["B_bits"]], yerr=[[r["B_bits"] - r["B_lo"]], [r["B_hi"] - r["B_bits"]]], fmt="o", capsize=3, label=r["label"][:44])
ax[1].set_xlabel("w"); ax[1].set_ylabel("m=4 D=5 row, bits over rho"); ax[1].legend(fontsize=6); ax[1].set_title("Repriced ECC2K-130 m=4 degree-5 row", fontsize=9)
plt.tight_layout(); plt.savefig("w_fit_and_reprice.png", dpi=160); plt.savefig("w_fit_and_reprice.svg")
print("ok")
