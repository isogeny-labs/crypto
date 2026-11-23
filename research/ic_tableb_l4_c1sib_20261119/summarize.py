"""Collect paired ratios (B/A) with 95% intervals at w = 0, 0.1, 1 and in bitop/v1. Re-runs ecbench compare --weight (cheap)."""
import json, re, subprocess, sys, os
os.chdir(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", ".."))
D = "research/ic_tableb_l4_c1sib_20261119"
BIN = "target/release/ecbench"
pairs = [(a, b) for a in ("rho-neg", "bsgs-neg", "sib8-p256") for b in ("c1", "c1-sib8-p256", "c1-sib8-p256-sg")]
pairs += [("rho-neg", "bsgs-neg"), ("rho-neg", "sib8-p256"), ("c1", "c1-sib8-p256"), ("c1-sib8-p256", "c1-sib8-p256-sg")]
pat = re.compile(r"= ([\d.]+) \[([\d.]+), ([\d.]+)\]")
out = {}
for bits in (20, 22, 24):
    out[bits] = {}
    for a, b in pairs:
        row = {}
        for key, extra in (("w0", ["--weight", "0"]), ("w01", ["--weight", "0.1"]), ("w1", ["--weight", "1"]), ("bitop", ["--unit", "bitop/v1"])):
            t = subprocess.run([BIN, "compare", "--dir", f"{D}/sessions/P{bits}", "--a", a, "--b", b] + extra, capture_output=True, text=True).stdout
            first = t.splitlines()[0] if t else ""
            m = pat.search(first)
            row[key] = [float(m.group(i)) for i in (1, 2, 3)] if m else None
        out[bits][f"{a}|{b}"] = row
json.dump(out, open(f"{D}/summary.json", "w"), indent=1)
