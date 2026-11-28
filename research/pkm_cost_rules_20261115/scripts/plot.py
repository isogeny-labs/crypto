import sys, numpy as np, matplotlib; matplotlib.use("Agg"); import matplotlib.pyplot as plt
out=sys.argv[1]; m=8
s=np.linspace(0,7,701)
cl=np.where(s<m-1, 2/(m+1-s), 1.0)
fl=np.where(s<=2, 2/(m+1-s), s/(m-1))
fig,ax=plt.subplots(figsize=(7,4.2))
ax.plot(s,cl,label="closure rule: 2/(m+1-s), no attempt floor",color="#b0413e")
ax.plot(s,fl,label="floored: max(1, attempts), quad LA",color="#2a6f97")
ax.axhline(0.5,color="grey",ls="--",lw=1); ax.text(0.1,0.51,"rho (1/2)",color="grey",fontsize=8)
H=lambda x:-x*np.log2(x)-(1-x)*np.log2(1-x)
for sig,lab in ((1/8,"sigma=1/8 (D 7->8, N 8->16, certified)"),):
    sv=m*H(sig); ax.axvline(sv,color="k",lw=0.8,ls=":"); ax.text(sv+0.05,0.9,lab,fontsize=8,rotation=90,va="top")
ax.axvline(3.5,color="#2a6f97",lw=0.8,ls="-."); ax.axvline(5,color="#b0413e",lw=0.8,ls="-.")
ax.set_xlabel("per-attempt exponent s (cost B^s), w=1 multilinear: s = m H(sigma)"); ax.set_ylabel("total exponent E (log_r T)")
ax.set_title("PKM m=8: the two cost rules [pcc:unverified]"); ax.set_ylim(0,1.05); ax.legend(fontsize=8,loc="lower right")
fig.tight_layout(); [fig.savefig(f"{out}/m8_rules.{e}") for e in ("svg","png","pdf")]
