"""Lay out triage.json as a grid chart (rendering only; no result is computed)."""
import json, collections
import matplotlib; matplotlib.use("svg")
import matplotlib.pyplot as plt
d = json.load(open("triage.json"))
cells = collections.defaultdict(list)
for it in d["items"]:
    cells[(it["tract"], it["rel"])].append(it)
colors = {"ATTACK-NOW": "#c0392b", "ATTACK-LATER": "#d68910", "RECORD-ONLY": "#7f8c8d"}
fig, ax = plt.subplots(figsize=(10, 6.2))
for (t, r), items in cells.items():
    for k, it in enumerate(sorted(items, key=lambda i: i["id"])):
        cols = 2 if len(items) > 4 else 1
        x = t + ((k % cols) - (cols - 1) / 2) * 0.42
        y = r + 0.36 - (k // cols) * 0.16
        ax.text(x, y, it["id"], ha="center", va="center", fontsize=7.5,
                color=colors[it["verdict"]], weight="bold" if it["verdict"] != "RECORD-ONLY" else "normal")
ax.set_xlim(0.5, 3.5); ax.set_ylim(-0.5, 3.5)
ax.set_xticks([1, 2, 3], ["low", "medium", "high"]); ax.set_yticks([0, 1, 2, 3], ["none", "low", "medium", "high"])
for v in (1.5, 2.5): ax.axvline(v, color="#ddd", lw=0.8)
for v in (0.5, 1.5, 2.5): ax.axhline(v, color="#ddd", lw=0.8)
ax.set_xlabel("tractability (a bounded attack in hours to days exists)")
ax.set_ylabel("relevance to IC speedups / non-generic ECDLP")
handles = [plt.Line2D([], [], color=c, marker="s", ls="", label=f"{v} ({sum(i['verdict']==v for i in d['items'])})") for v, c in colors.items()]
ax.legend(handles=handles, loc="upper right", fontsize=8, frameon=False)
ax.set_title(f"Triage of the {len(d['items'])} open math-st items (upstream f631bcc), 2026-11-19", fontsize=10)
fig.tight_layout(); fig.savefig("triage.svg")
