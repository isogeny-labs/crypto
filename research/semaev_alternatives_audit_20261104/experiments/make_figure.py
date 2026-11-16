import json, math
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

# LLL recovery (this audit, sympy LLL, 24 trials/cell)
d72 = [(0.06,24),(0.07,24),(0.08,24),(0.09,21),(0.10,13),(0.11,9),(0.12,4)]
d96 = [(0.06,24),(0.07,24),(0.08,24),(0.09,21),(0.10,14),(0.11,5),(0.12,3)]

mitm = json.load(open("mitm_m4.json"))["rows"]

fig, ax = plt.subplots(1, 2, figsize=(11, 4.2))

for data, lab, mk in ((d72, "p = 2^72", "o"), (d96, "p = 2^96", "s")):
    ax[0].plot([x for x, _ in data], [y/24 for _, y in data], marker=mk, label=lab)
ax[0].axvline(1/12, ls="--", c="k", lw=1)
ax[0].text(1/12 + 0.001, 0.05, "1/12", fontsize=9)
ax[0].set_xlabel(r"$\delta$  ($B=p^\delta$)")
ax[0].set_ylabel("recovery rate")
ax[0].set_title("3-dim form lattice, LLL (audit reproduction)")
ax[0].legend(); ax[0].grid(alpha=.3)

ax[1].loglog([r["B"] for r in mitm], [r["ops"] for r in mitm], "o-", label="measured ops")
b = [r["B"] for r in mitm]
ax[1].loglog(b, [b[0]**2 * (x/b[0])**2 for x in b], "--", label=r"$B^2$")
ax[1].set_xlabel("B")
ax[1].set_ylabel("group operations")
ax[1].set_title("2+2 MITM m=4 oracle (fitted s = 1.93)")
ax[1].legend(); ax[1].grid(alpha=.3, which="both")

fig.tight_layout()
fig.savefig("../figures/audit.svg")
fig.savefig("../figures/audit.png", dpi=160)
print("wrote figures/audit.svg")
