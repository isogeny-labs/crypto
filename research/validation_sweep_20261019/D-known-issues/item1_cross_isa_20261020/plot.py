# Lays out recorded audit outcomes only (receipts/*.json problems counts).
import json, matplotlib; matplotlib.use('svg')
import matplotlib.pyplot as plt
rows=[("x86-64, rustc 1.97.0\n(default build)","x86_197"),("x86-64, rustc 1.93.1\n(default build)","x86_193"),
("aarch64 (qemu-user),\nrustc 1.97.0","a64_197"),("x86-64, BMI2 word-wise\nrewrite off (1 line)","bis2"),
("x86-64, all x86-only\npaths in inherited_f4.rs off","bis1")]
v=[len(json.load(open(f"receipts/audit_{k}.json"))['problems']) for _,k in rows]
fig,ax=plt.subplots(figsize=(8.6,3.8))
ax.bar(range(len(rows)),v,color=["#c0504d" if x else "#6b9a5b" for x in v])
for i,x in enumerate(v): ax.text(i,x+1,f"{x}/168 differ",ha="center",fontsize=9)
ax.set_xticks(range(len(rows))); ax.set_xticklabels([r[0] for r in rows],fontsize=7)
ax.set_ylabel("replays differing (of 168)"); ax.set_ylim(0,60)
ax.set_title("Replay-all of session ECBS1hfd5bf1885227 (recorded on Apple M4 Pro), source at 2245b75",fontsize=9)
fig.tight_layout(); fig.savefig("cross_isa.svg"); fig.savefig("cross_isa.png",dpi=150)
