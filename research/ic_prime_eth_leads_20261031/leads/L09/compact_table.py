#!/usr/bin/env python3
import json, sys
rows = json.load(open(sys.argv[1]))
bb = {(r["path"], r["label"]): r for r in rows if r["mode"] == "bb"}
def n(x): return "-" if x is None else f"{x:,}"
print("| encoding | instance | b | m | s (split) | s (bb) | V (split) | Delta split / bb | nonlin ports (split) | s/sqrt(log2 s) [split] | enum (m-1)b/m | ratio | D.1 | k vs b |")
print("|" + "---|" * 14)
for r in rows:
    if r["mode"] == "bb": continue
    q = bb.get((r["path"], r["label"]), r)
    k = r.get("k"); b = r["b"]
    kv = "-" if k is None else (f"{k} >= {b}" if k >= b else f"**{k} < {b}**")
    print("| " + " | ".join([r["group"], r["label"].replace("pdp_", "").replace(".cnf.gz", ""), str(b), str(r["m"]), n(r["s"]), n(q["s"]), n(r["V"]),
        f'{r["delta"]} / {q["delta"]}', n(r["nl_ports"]), f'{r["s_over_sqrtlog"]:,.0f}', f'{r["enum_exp"]:.2f}', f'{r["ratio_vs_enum"]:,.0f}x',
        "no", kv]) + " |")
