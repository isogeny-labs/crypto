#!/usr/bin/env python3
"""Merge: v2 holdout ic4 and bsgs2 rows (their code is unchanged) + the corrected rho-family rows.
usage: merge.py v2_rows.jsonl fixed_rho_rows.jsonl out.jsonl"""
import json, sys
RHO = {"rho", "rho2", "rho2x", "rhoseq"}
keep = [l for l in open(sys.argv[1]) if json.loads(l)["arm"] not in RHO]
new = list(open(sys.argv[2]))
open(sys.argv[3], "w").writelines(keep + new)
print(len(keep), "kept +", len(new), "corrected")
