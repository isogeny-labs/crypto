#!/usr/bin/env python3
"""Replay plan from sample.json: per sampled unit REF, BOTH (CAND NRM=1 LZ_REUSE=1), ORACLE (oracle build, both env);
A0 (CAND, no env) on the two A/A units; extra heavier-row unit q=524287 seed 7 c_D=1.55 (REF, BOTH, ORACLE);
ORACLE on every producer cell (q in 5 sizes, seeds 1-6) not already sampled.
Columns: idx role arm bin env q seed c. Usage: make_plan.py sample.json > plan.tsv"""
import json, sys
sm = json.load(open(sys.argv[1]))
Q = [262111, 524287, 1048507, 2097091, 4194271]
rows = []
def add(role, arm, b, env, q, s, c="1.7"):
    rows.append((role, arm, b, env, q, s, c))
for u in sm["sample"]:
    add("cnt", "REF", "ref", "-", u["q"], u["seed"])
    add("cnt", "BOTH", "cand", "NRM=1,LZ_REUSE=1", u["q"], u["seed"])
    add("orc", "ORACLE", "oracle", "NRM=1,LZ_REUSE=1", u["q"], u["seed"])
for u in sm["aa"]:
    add("aa", "A0", "cand", "-", u["q"], u["seed"])
add("cd", "REF", "ref", "-", 524287, 7, "1.55")
add("cd", "BOTH", "cand", "NRM=1,LZ_REUSE=1", 524287, 7, "1.55")
add("cd", "ORACLE", "oracle", "NRM=1,LZ_REUSE=1", 524287, 7, "1.55")
done = {(u["q"], u["seed"]) for u in sm["sample"]}
for q in Q:
    for s in range(1, 7):
        if (q, s) not in done:
            add("orcp", "ORACLE", "oracle", "NRM=1,LZ_REUSE=1", q, s)
for i, r in enumerate(rows, 1):
    print(i, *r, sep="\t")
