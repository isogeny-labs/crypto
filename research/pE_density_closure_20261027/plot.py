"""Layout only: plots recorded values from groups.tsv (written by the Rust tool
src/main.rs from the twist-obstruction rows.jsonl). Computes nothing new except
drawing the closed-form curves C(|S|+m-1,m)/|G| and the sigma_p band."""
import csv, math, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

rows = list(csv.DictReader(open("groups.tsv"), delimiter="\t"))
for r in rows:
    for k in ("n","l","m","N_clean","v_E"):
        r[k] = int(r[k])
    for k in ("p_E","G","mean_clean","se_clean","pred","z"):
        r[k] = float(r[k])
cols = {11:"#1f77b4",13:"#ff7f0e",17:"#2ca02c",19:"#d62728"}
mk = {"koblitz_a0_b1":"s"}

def comb(x,k):
    r=1.0
    for i in range(k): r*= (x-i)/(i+1)
    return r

fig, ax = plt.subplots(1,3, figsize=(15,4.6))
for i,m in enumerate((3,2)):
    a = ax[i]
    for n in cols:
        pts=[r for r in rows if r["m"]==m and r["n"]==n]
        if not pts: continue
        l=pts[0]["l"]
        a.errorbar([r["p_E"]**m for r in pts],[r["mean_clean"] for r in pts],
                   yerr=[r["se_clean"] for r in pts], fmt="none", ecolor=cols[n], alpha=.7, capsize=2)
        for r in pts:
            a.scatter(r["p_E"]**m, r["mean_clean"], c=cols[n], marker=mk.get(r["curve"],"o"), s=36, zorder=3)
        ps=[0.38+0.2*j/50 for j in range(51)]
        a.plot([p**m for p in ps],[comb(2*p*(2**l-1)+1+m-1,m)/(2**n+1) for p in ps],
               color=cols[n], lw=1, ls="--", label=f"n={n}, l={l}: C(|S|+m-1,m)/(2^n+1)")
    a.set_xlabel(f"p_E^{m}  (p_E = v_E / (2^l - 1))"); a.set_ylabel("mean solutions per target (A_random, clean targets)")
    a.set_title(f"m={m}: mean solutions vs p_E^{m}"); a.legend(fontsize=6.5); a.grid(alpha=.3)
a=ax[2]
for r in rows:
    a.errorbar(r["pred"], r["mean_clean"], yerr=r["se_clean"], fmt=mk.get(r["curve"],"o"), color=cols[r["n"]], ms=4, alpha=.8, capsize=1.5)
lim=[2.5,70]; a.plot(lim,lim,"k-",lw=1); a.set_xscale("log"); a.set_yscale("log")
a.set_xlabel("identity prediction C(2v_E+m, m) / |G|   (|G| = #E(F_q), census where recorded else 2^n+1)")
a.set_ylabel("observed mean solutions per clean target"); a.set_title("observed vs counting identity (y = x)"); a.grid(alpha=.3, which="both")
a.xaxis.label.set_size(7)
fig.suptitle("Twist study run-20261026T1456Z, recorded values: yield per target is |S|^m/(m!|G|), i.e. p_E^m at fixed V (squares: a=0,b=1; bars: +-1 SE)", fontsize=9)
fig.tight_layout(rect=(0,0,1,.94))
fig.savefig("fig1_mean_vs_pE_pow_m.svg"); fig.savefig("fig1_mean_vs_pE_pow_m.png", dpi=150)

fig, a = plt.subplots(figsize=(6.4,4.4))
for r in rows:
    a.scatter(r["l"], r["p_E"]-0.5, c=cols[r["n"]], s=22, alpha=.8)
ls=[x/10 for x in range(55,115)]
a.plot(ls,[0.5*2**(-l/2) for l in ls],"k--",lw=1,label="+-sigma_p = 2^(-l/2)/2 (random character sum)")
a.plot(ls,[-0.5*2**(-l/2) for l in ls],"k--",lw=1)
a.set_xlabel("l = dim V"); a.set_ylabel("p_E - 1/2"); a.set_title("p_E - 1/2 vs 2^(-l/2): recorded groups (colour = n)")
a.legend(fontsize=7); a.grid(alpha=.3)
fig.tight_layout(); fig.savefig("fig2_pE_deviation_vs_l.svg"); fig.savefig("fig2_pE_deviation_vs_l.png", dpi=150)
