# Plots recorded values only (evidence/checks.txt); computes nothing new.
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
fig,(a,b)=plt.subplots(1,2,figsize=(10,4),gridspec_kw={"width_ratios":[1.2,1]})
# census of 7875 pairs over F_2, n=3, dmax=6 (recorded)
labels=["d_F > d'_F\n(Lemma 4 violated)","d_F < d'_F\n(Lemma 4 holds strictly)"]
out=[6138,0]; inn=[4830,224]
x=[0,1]; w=0.38
a.bar([i-w/2 for i in x],out,w,label="field eqs outside",color="#1f77b4")
a.bar([i+w/2 for i in x],inn,w,label="field eqs inside",color="#ff7f0e")
for i,(o,n) in enumerate(zip(out,inn)):
    a.text(i-w/2,o+80,str(o),ha="center",fontsize=8); a.text(i+w/2,n+80,str(n),ha="center",fontsize=8)
a.set_xticks(x); a.set_xticklabels(labels,fontsize=8); a.set_ylabel("pairs (of 7875)")
a.set_title("Exhaustive census, F_2, 3 vars, deg<=2 pairs",fontsize=9); a.legend(fontsize=8)
# witnesses: first fall degrees (recorded)
W=["A","B","C"]; dout=[3,3,3]; din=[3,3,2]; dfake=[2,2,2]
xs=range(3); w=0.27
b.bar([i-w for i in xs],dout,w,label="d_F outside",color="#1f77b4")
b.bar(list(xs),din,w,label="d_F inside",color="#ff7f0e")
b.bar([i+w for i in xs],dfake,w,label="d'_F (fake)",color="#2ca02c")
b.set_xticks(list(xs)); b.set_xticklabels(["witness "+k for k in W],fontsize=8)
b.set_ylabel("first fall degree"); b.set_ylim(0,3.6); b.set_title("Published witnesses (exact)",fontsize=9); b.legend(fontsize=7,loc="upper right",ncol=3)
fig.suptitle("BLINDSPOT-19 S5: Nagao Lemma 4 (d_F <= d'_F) re-derived natively; n=1 exact run, no sampling; source evidence/checks.txt",fontsize=8)
fig.tight_layout(); fig.savefig("reports/lemma4_census.svg"); fig.savefig("reports/lemma4_census.png",dpi=150)
