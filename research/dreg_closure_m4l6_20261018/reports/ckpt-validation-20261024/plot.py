"""Draws ckpt-validation.svg/png from the recorded files in data/ (no new computation)."""
import json, re, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = pathlib.Path(__file__).parent
d1 = here / "data/m4-12-3-d1"
d2 = here / "data/m4-13-4-d1"
S = json.loads((d1 / "summary.json").read_text())
wall = lambda t: float((d1 / f"{t}.wall").read_text().split()[1])

fig, ax = plt.subplots(1, 3, figsize=(13.2, 4.2), gridspec_kw={"width_ratios": [1.15, 1.0, 1.1]})

# (a) m4-12-3 draw 1: wall seconds per scenario, segments stacked
rows = [("plain\n(no checkpoint)", [wall("plain")]),
        ("checkpoint,\nnever stopped", [wall("ckpt")]),
        ("SIGTERM at 25 s,\nthen resumed", [wall("term1"), wall("term2")]),
        ("SIGKILL at 25 s,\nthen resumed", [wall("kill1"), wall("kill2")])]
cols = ["#4c78a8", "#f58518"]
for i, (name, segs) in enumerate(rows):
    left = 0
    for j, w in enumerate(segs):
        ax[0].barh(i, w, left=left, color=cols[j], edgecolor="white")
        ax[0].text(left + w / 2, i, f"{w:.0f} s", ha="center", va="center", color="white", fontsize=8)
        left += w
ax[0].set_yticks(range(len(rows)), [r[0] for r in rows], fontsize=8)
ax[0].invert_yaxis()
ax[0].set_xlabel("wall seconds (segment 1 blue, segment 2 orange)")
ax[0].set_title("(a) m=4 (12,3) draw 1, degree 4:\nall four give the same result", fontsize=9)

# (b) m4-13-4 draw 1
# wall times of the two segments are in data/m4-13-4-d1/walls.txt (read from the shell)
seg1, seg2 = 308.0, 132.0
plain = json.loads((d2 / "plain.out").read_text())["scan"][-1]["secs"]
ax[1].barh(0, plain, color="#4c78a8")
ax[1].text(plain / 2, 0, f"{plain:.0f} s", ha="center", va="center", color="white", fontsize=8)
ax[1].barh(1, seg1, color="#4c78a8", edgecolor="white")
ax[1].barh(1, seg2, left=seg1, color="#f58518", edgecolor="white")
ax[1].text(seg1 / 2, 1, f"{seg1:.0f} s", ha="center", va="center", color="white", fontsize=8)
ax[1].text(seg1 + seg2 / 2, 1, f"{seg2:.0f} s", ha="center", va="center", color="white", fontsize=8)
ax[1].set_yticks([0, 1], ["plain", "SIGTERM inside\nround 5's elimination,\nthen resumed"], fontsize=8)
ax[1].invert_yaxis()
ax[1].set_xlabel("wall seconds (4 cores shared with a compile)")
ax[1].set_title("(b) m=4 (13,4) draw 1, degree 4:\nidentical result, 4.47 GB saved", fontsize=9)

# (c) save size vs seconds, every committed generation in the recorded logs
pts = []
for f in list(d1.glob("*.err")) + list(d2.glob("*.err")):
    for m in re.finditer(r"committed, ([0-9.]+) GB in ([0-9.]+)s", f.read_text()):
        gb, s = float(m.group(1)), float(m.group(2))
        if gb > 0.005:
            pts.append((gb, s, f.parent.name))
for name, c in [("m4-12-3-d1", "#4c78a8"), ("m4-13-4-d1", "#e45756")]:
    xs = [p[0] for p in pts if p[2] == name]
    ys = [p[1] for p in pts if p[2] == name]
    ax[2].scatter(xs, ys, s=14, color=c, label=f"{name} ({len(xs)} saves)")
ax[2].set_xlabel("generation size (GB)")
ax[2].set_ylabel("seconds to write, hash and fsync")
ax[2].legend(fontsize=7)
ax[2].set_title("(c) cost of one save", fontsize=9)
fig.suptitle("Native closure checkpoints: stop, kill, resume (data/ in this directory)", fontsize=10)
fig.tight_layout()
fig.savefig(here / "ckpt-validation.svg")
fig.savefig(here / "ckpt-validation.png", dpi=140)
print(len(pts), "save points;", f"plain secs {plain}")
