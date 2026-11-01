# Lays out recorded audit outcomes only: rerun problem counts beside the committed first run.
import json, matplotlib; matplotlib.use('svg')
import matplotlib.pyplot as plt
C="../item1_cross_isa_20261020/receipts/audit_%s.json"
rows=[("x86-64\nrustc 1.97.0","x86_197","x86_197"),("x86-64\nrustc 1.93.1","x86_193","x86_193"),("aarch64 (qemu)\nrustc 1.97.0","a64_197","a64_197"),("x86-64, BMI2\nrewrite off","bmi2off","bis2")]
first=[len(json.load(open(C%c))['problems']) for _,_,c in rows]
rerun=[len(json.load(open("receipts/audit_%s.json"%r))['problems']) for _,r,_ in rows]
fig,ax=plt.subplots(figsize=(8,3.8)); w=0.38
ax.bar([i-w/2 for i in range(4)],first,w,label="first run (merged, #2327)",color="#8fa8c8")
ax.bar([i+w/2 for i in range(4)],rerun,w,label="rerun (this PR)",color="#c0504d")
for i,(a,b) in enumerate(zip(first,rerun)): ax.text(i-w/2,a+1,str(a),ha="center",fontsize=8); ax.text(i+w/2,b+1,str(b),ha="center",fontsize=8)
ax.set_xticks(range(4)); ax.set_xticklabels([r[0] for r in rows],fontsize=8); ax.set_ylim(0,60); ax.legend(fontsize=8)
ax.set_ylabel("replays differing (of 168)"); ax.set_title("Cross-ISA replay-all, session ECBS1hfd5bf1885227, source at 2245b75",fontsize=9)
fig.tight_layout(); fig.savefig("rerun.svg"); fig.savefig("rerun.png",dpi=150)
