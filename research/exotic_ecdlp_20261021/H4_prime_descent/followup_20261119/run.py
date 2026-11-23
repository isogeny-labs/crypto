#!/usr/bin/env python3
"""Bounded replay of the frozen H4 n=13 builder and Macaulay profile."""

from sage.all import *
import ast
import hashlib
import itertools
import json
import math
import os
from pathlib import Path
import random
import resource
import time
import numpy as np

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
SOURCE = ROOT / "research/exotic_ecdlp_20261021/H4_prime_descent/run.py"
SPEC = json.loads((HERE / "protocol.json").read_text())
assert hashlib.sha256(SOURCE.read_bytes()).hexdigest() == SPEC["source_sha256"]

needed = {
    "gfmul", "gfmul_s", "to_int", "popcount", "build_instance",
    "eval_eqs_at", "enumerate_solutions", "hashvec", "row_proj",
    "rank_rows", "squarefree_masks", "macaulay_profile",
}
tree = ast.parse(SOURCE.read_text(), filename=str(SOURCE))
functions = [node for node in tree.body
             if isinstance(node, ast.FunctionDef) and node.name in needed]
assert {node.name for node in functions} == needed
M64 = np.uint64
SEED = 20261022
ROWCAP = 30000
C1 = M64(0x9E3779B97F4A7C15)
C2 = M64(0xBF58476D1CE4E5B9)
C3 = M64(0x94D049BB133111EB)
C4 = M64(0x2545F4914F6CDD1D)
C5 = M64(0xD1B54A32D192ED03)
exec(compile(ast.Module(body=functions, type_ignores=[]), str(SOURCE), "exec"), globals())

limit_enforced = False
try:
    _, hard = resource.getrlimit(resource.RLIMIT_AS)
    resource.setrlimit(resource.RLIMIT_AS,
                       (SPEC["address_space_soft_limit_bytes"], hard))
    limit_enforced = True
except (OSError, ValueError):
    pass

result = {
    "schema": "h4-n13-macaulay-followup-result-v1",
    "protocol_sha256": hashlib.sha256((HERE / "protocol.json").read_bytes()).hexdigest(),
    "source_sha256": SPEC["source_sha256"],
    "address_space_limit_enforced": limit_enforced,
    "status": "running",
    "arms": {},
}

def checkpoint():
    p = HERE / "result.json"
    tmp = p.with_suffix(".json.tmp")
    tmp.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    os.replace(tmp, p)

class Record:
    def __init__(self, arm):
        self.arm = arm
        self.messages = []

    def send(self, pair):
        tag, payload = pair
        rec = {"arm": self.arm, "tag": tag, "payload": payload}
        self.messages.append(rec)
        with (HERE / "events.jsonl").open("a") as f:
            f.write(json.dumps(rec, sort_keys=True) + "\n")
        print(json.dumps(rec, sort_keys=True), flush=True)

checkpoint()
for arm in SPEC["stage_order"]:
    started = time.time()
    result["arms"][arm] = {"status": "running"}
    checkpoint()
    sd = build_instance(13, 5, arm)
    if sd is None:
        result["arms"][arm] = {"status": "degenerate_instance"}
        checkpoint()
        continue
    sols = enumerate_solutions(sd)
    residual = eval_eqs_at(sd["eqs"], sd["planted"]) if arm == "planted" else None
    planted_in = (sd["planted"] in set(sols)) if arm == "planted" else None
    if arm == "planted":
        assert residual == 0 and planted_in
    conn = Record(arm)
    dff, dsolv, profile = macaulay_profile(
        sd, len(sols), conn, time.time() + SPEC["per_arm_deadline_seconds"], True)
    stop = next((m["payload"]["status"] for m in conn.messages
                 if m["tag"] == "D" and "status" in m["payload"]), None)
    status = ("completed_degree" if dsolv is not None else
              "censored_" + stop if stop else "censored_deadline")
    result["arms"][arm] = {
        "status": status, "field_degree": 13, "instance_index": 5,
        "variables": sd["N"], "equations": len(sd["eqs"]),
        "solution_count": len(sols), "planted_residual": residual,
        "planted_in_solution_set": planted_in,
        "d_ff": dff, "d_solv": dsolv,
        "last_degree_completed": profile[-1]["D"] if profile else None,
        "profile": profile, "wall_seconds": time.time() - started,
        "peak_rss_bytes": resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
    }
    checkpoint()
result["status"] = "complete_with_censoring" if any(
    a["status"].startswith("censored") for a in result["arms"].values()
) else "complete"
checkpoint()
print(json.dumps({"status": result["status"], "arms": result["arms"]},
                 sort_keys=True), flush=True)
