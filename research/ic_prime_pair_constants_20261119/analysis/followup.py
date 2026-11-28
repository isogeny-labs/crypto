#!/usr/bin/env python3
"""Post-hoc k-sum sweep: ratio to inc [95%] at w = 0, 0.1, 1 and distance to rho. -> analysis/followup.json/.md"""
import json, subprocess, pathlib
here = pathlib.Path(__file__).resolve().parent.parent
B = "/home/user/crypto/target/release/ecbench"
def cmp(c, a, b, w):
    d = json.loads(subprocess.run([B, "compare", "--dir", str(here/"sessions"/c), "--a", a, "--b", b, "--weight", str(w), "--json"], capture_output=True, text=True).stdout)["ops"]
    ci = d.get("ci95_bootstrap") or d["ci95"]
    return [d["ratio_b_over_a"], ci[0], ci[1]]
out, md = {}, []
for c in ("x24", "x28"):
    plan = json.load(open(here/"sessions"/c/"spec.json"))
    arms = [a["name"] for a in plan["arms"] if a["name"] not in ("rho", "inc")]
    out[c] = {}
    md.append(f"\n### {c}: ratio to inc [95%] / distance to rho at w = 0, 0.1, 1\n\n| arm | w=0 | w=0.1 | w=1 | dist w=0 / 0.1 / 1 | band holds |\n|:--|:--|:--|:--|:--|:--|")
    for a in ["inc"] + arms:
        e = {"inc": {str(w): ([1, 1, 1] if a == "inc" else cmp(c, "inc", a, w)) for w in (0, 0.1, 1)}, "rho": {str(w): cmp(c, "rho", a, w) for w in (0, 0.1, 1)}}
        out[c][a] = e
        v = e["inc"]
        ok = all(v[str(w)][2] < 1 for w in (0, 0.1, 1))
        md.append(f"| {a} | " + " | ".join(f"{v[str(w)][0]:.3f} [{v[str(w)][1]:.3f}, {v[str(w)][2]:.3f}]" for w in (0, 0.1, 1)) +
                  " | " + " / ".join(f"{e['rho'][str(w)][0]:.1f}" for w in (0, 0.1, 1)) + f" | {'yes' if ok and a != 'inc' else '-'} |")
json.dump(out, open(here/"analysis"/"followup.json", "w"), indent=1)
open(here/"analysis"/"followup.md", "w").write("\n".join(md) + "\n")
print("\n".join(md))
