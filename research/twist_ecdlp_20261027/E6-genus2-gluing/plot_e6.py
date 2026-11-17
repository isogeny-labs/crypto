# plots only values recorded in out/summary.txt (no new numbers)
import re, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
D="/tmp/claude-0/-home-user/6c104f56-b34a-5c4d-a978-f322cc83838f/scratchpad/twist_ecdlp/E6-genus2-gluing/"
rows={}
for l in open(D+"out/summary.txt"):
    if not l.startswith("SUMMARY p="): continue
    kv=dict(re.findall(r"(\S+?)=(\S+)",l)); p=int(kv["p"]); rows.setdefault(p,{}).update(kv)
ps=sorted(rows)
f,ax=plt.subplots(1,2,figsize=(10,4))
for k,lab in [("ctie_full_min","min"),("ctie_full_med","median"),("ctie_full_max","max"),("bound_full","bound 1.2533(sqrt p+1)/p")]:
    ax[0].loglog(ps,[float(rows[p][k]) for p in ps],marker="o",label="c_tie_full "+lab)
ax[0].set_xlabel("p");ax[0].set_ylabel("model break-even constant (l=#E)");ax[0].legend(fontsize=7);ax[0].set_title("model cost ratio (NOT measured IC)")
for k,lab in [("ctie_min","min"),("ctie_med","median"),("ctie_max","max")]:
    ax[1].loglog(ps,[float(rows[p][k]) for p in ps],marker="o",label="c_tie "+lab)
ax[1].set_xlabel("p");ax[1].set_ylabel("c_tie (l=largest prime factor)");ax[1].legend(fontsize=7)
plt.tight_layout();plt.savefig(D+"out/ctie_vs_p.png",dpi=130)
f,ax=plt.subplots(figsize=(7,3.5))
keys=["match_target","nJ_eq_prod","quotients_ok","nE_div_nJ","ident_ok"]
for i,k in enumerate(keys):
    ax.plot(ps,[int(rows[p][k])/int(rows[p]["C_target_n"]) for p in ps],marker="o",label=k,ls="none",ms=8-i)
ax.set_xscale("log");ax.set_ylim(0,1.05);ax.set_xlabel("p");ax.set_ylabel("pass rate, target arm");ax.legend(fontsize=7)
plt.tight_layout();plt.savefig(D+"out/passrates.png",dpi=130)
