import json, sys, os
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
here = os.path.dirname(os.path.abspath(__file__))
c = json.load(open(os.path.join(here, "calibration.json")))
fig, (a, b) = plt.subplots(1, 2, figsize=(10, 4))
for key, col, lab in [("word", "#4c72b0", "word GF(p^D)"), ("tower", "#dd8452", "tower over GF(p^10)")]:
    xs, ys = zip(*c[key]["rows"]); a.loglog(xs, ys, "o-", color=col, label=lab)
a.set_xlabel("degree D over GF(p)"); a.set_ylabel("seconds per 256-bit scalar multiplication"); a.legend(); a.set_title("Measured cost, same degree")
xs = [r[0] for r in c["word"]["rows"]]
ratio = [t[1] / w[1] for w, t in zip(c["word"]["rows"], c["tower"]["rows"])]
b.semilogx(xs, ratio, "o-", color="#55a868", label="measured tower / word")
b.axhline(3, color="#333", ls="--", label="charged now: 3")
b.set_ylim(0, 4); b.set_xlabel("degree D over GF(p)"); b.set_ylabel("overhead factor"); b.legend(); b.set_title("Tower overhead (64 was charged before)")
fig.tight_layout()
for ext in ("svg", "png"): fig.savefig(os.path.join(here, f"tower_overhead.{ext}"), dpi=110)
print([round(r, 2) for r in ratio])
