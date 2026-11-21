#!/usr/bin/env python3
"""Negative control for check_v3_certs.py: every single-field tamper of every
certificate must be rejected.  Usage: tamper_v3_certs.py certs.jsonl results.json"""
import json, sys, os, subprocess, tempfile, copy
here = os.path.dirname(os.path.abspath(__file__))
certs = [json.loads(l) for l in open(sys.argv[1]) if l.strip()]
res = sys.argv[2]
def mutations(c):
    p = c["p"]
    m = copy.deepcopy(c); m["claimed_k"] = (c["claimed_k"] + 1) % c["r"]; yield "k+1", m
    if c["group"]["kind"] == "ec":
        m = copy.deepcopy(c); m["t"] = [c["t"][0], (-c["t"][1]) % p]; yield "T->-T", m
        m = copy.deepcopy(c); m["group"]["a6"] = (c["group"]["a6"] + 1) % p; yield "a6+1", m
    else:
        m = copy.deepcopy(c); m["t"]["v"] = [(-x) % p for x in c["t"]["v"]]; yield "T->-T", m
        m = copy.deepcopy(c); m["group"]["f"][0] = (c["group"]["f"][0] + 1) % p; yield "f0+1", m
    if c["transfer"] is not None:
        m = copy.deepcopy(c); m["transfer"]["src_t"] = [c["transfer"]["src_t"][0], (-c["transfer"]["src_t"][1]) % p]; yield "src_t->-src_t", m
        if c["transfer"]["kind"] == "pullback":
            m = copy.deepcopy(c); m["transfer"]["c"][0] = (c["transfer"]["c"][0] + 1) % p; yield "c0+1", m
rej = acc = 0; bad = []
for i, c in enumerate(certs):
    for name, m in mutations(c):
        lines = [json.dumps(x) for x in certs]; lines[i] = json.dumps(m)
        with tempfile.NamedTemporaryFile("w", suffix=".jsonl", delete=False) as f:
            f.write("\n".join(lines) + "\n"); path = f.name
        out = subprocess.run([sys.executable, "-I", os.path.join(here, "check_v3_certs.py"), path, res],
                             capture_output=True, text=True)
        os.unlink(path)
        line = [l for l in out.stdout.splitlines() if l.startswith(f"line {i+1:2d} ")]
        ok_line = bool(line) and " OK " in line[0]
        crashed = out.returncode != 0
        if ok_line and not crashed: acc += 1; bad.append((i + 1, c["arm"], name))
        else: rej += 1
print(f"tampered certificates: {rej + acc}, rejected {rej}, accepted {acc}")
for b in bad: print("  ACCEPTED", b)
sys.exit(1 if acc else 0)
