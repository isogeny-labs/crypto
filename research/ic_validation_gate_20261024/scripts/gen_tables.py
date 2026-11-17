#!/usr/bin/env python3
"""Emit the per-row Markdown tables used by README.md (reads the committed standing and report JSON). stdlib only."""
import json, sys
from pathlib import Path
here = Path(sys.argv[1])
Bv = json.load(open(here / "B/validator-report.json")); Bs = json.load(open(here / "B/standing.json"))
Cv = json.load(open(here / "C/validator-report.json")); Cs = json.load(open(here / "C/standing.json"))
Ca = json.load(open(here / "C/rho-reference-assessment.json"))
L = ["| table | row | standing | S_IC | S_ref | x ref (board) | IC/rho at w = 0 / 0.1 / 1 (formula) | smallest IC/rho, extended band | artifacts |", "|:--|:--|:--|--:|--:|--:|:--|--:|:--|"]
for r in Bv["rows"]:
    a = r["analysis"]; st = Bs[r["slug"]]
    L.append(f'| B | `{r["slug"]}` | **{"validated" if st["pass"] else "FAILED"}** (staged) | {a["S_IC"]:.4g} | {a["S_ref"]:.4g} | {a["x_reference"]:.2f} | '
             + " / ".join(f'{b["ratio"]:.2f}' for b in a["band"]) + f' | {min(b["ratio_ext"] for b in a["band"]):.2f} | [B/](B/) |')
for r in Cv["rows"]:
    rc = r["recomputed"]; st = Cs[r["slug"]]; ass = Ca[r["slug"]]
    L.append(f'| C | `{r["slug"]}` | **failed validation** (R4) | {rc["S_IC"]:.4g} | {rc["S_ref"]:.4g} | {rc["x_reference"]:.2f} | no band (rho not in the same process) | n/a | [C/](C/) |')
print("\n".join(L))
