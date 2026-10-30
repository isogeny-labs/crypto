#!/usr/bin/env python3
"""Bind each unpromoted prime-curve candidate to an exact original JSON row."""

from __future__ import annotations

import argparse
import collections
import hashlib
import json
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[2]
BASE = REPO / "docs/curves/registry.json"
RECEIPT = HERE / "source-scope-summary.json"


def value(v):
    if isinstance(v, bool):
        return None
    if isinstance(v, int):
        return v
    if isinstance(v, float):
        return int(v) if v.is_integer() and abs(v) < 2**53 else None
    if isinstance(v, str):
        try:
            return int(v, 0)
        except ValueError:
            return None
    return None


def rows(doc):
    stack = [(doc, "")]
    while stack:
        node, pointer = stack.pop()
        if isinstance(node, list):
            stack.extend((v, f"{pointer}/{i}") for i, v in enumerate(node)
                         if isinstance(v, (dict, list)))
            continue
        if not isinstance(node, dict):
            continue
        stack.extend((v, pointer + "/" + str(k).replace("~", "~0").replace("/", "~1"))
                     for k, v in node.items() if isinstance(v, (dict, list)))
        p, a, b = (value(node.get(k)) for k in ("p", "a", "b"))
        n = value(node.get("group_order", node.get("order")))
        if p is not None and p >= 5 and a is not None and b is not None and n is not None:
            yield (p, a % p, b % p, n), pointer


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("candidate", type=Path)
    ap.add_argument("source_root", type=Path, help="complete checkout at the audited head")
    ap.add_argument("bindings", type=Path)
    args = ap.parse_args()
    receipt = json.loads(RECEIPT.read_text())
    source = args.candidate.read_bytes()
    if hashlib.sha256(source).hexdigest() != receipt["candidate_registry_sha256"]:
        raise ValueError("candidate hash differs from audited scratch output")
    candidate = json.loads(source)["curves"]
    baseline = json.loads(BASE.read_text())["curves"]
    known = {entry["icv1"] for entry in baseline}
    delta = [entry for entry in candidate if entry["icv1"] not in known]
    if len(delta) != receipt["new_count"]:
        raise ValueError("candidate count differs from audited delta")
    tracked = set(subprocess.check_output(
        ["git", "ls-files", "-z", "--", "*.json"], cwd=args.source_root
    ).decode().split("\0"))
    by_source = collections.defaultdict(list)
    for entry in delta:
        by_source[entry["sources"][0]].append(entry)
    bound = 0
    failures = []
    with args.bindings.open("w") as out:
        for rel in sorted(by_source):
            if rel not in tracked:
                failures.append({"source": rel, "error": "not tracked"})
                continue
            path = args.source_root / rel
            raw = path.read_bytes()
            digest = hashlib.sha256(raw).hexdigest()
            indexed = collections.defaultdict(list)
            for key, pointer in rows(json.loads(raw)):
                indexed[key].append(pointer)
            for entry in sorted(by_source[rel], key=lambda x: x["icv1"]):
                p = int(entry["params"]["p"])
                key = (p, int(entry["params"]["a"]),
                       int(entry["params"]["b"]), int(entry["order"]))
                pointers = indexed.get(key, [])
                if not pointers:
                    failures.append({"icv1": entry["icv1"], "source": rel,
                                     "error": "model/order not found in source"})
                    continue
                out.write(json.dumps({"icv1": entry["icv1"], "source": rel,
                                      "source_sha256": digest, "pointer": pointers[0],
                                      "matching_rows": len(pointers), "model": key},
                                     sort_keys=True) + "\n")
                bound += 1
    summary = {"candidate_sha256": receipt["candidate_registry_sha256"],
               "bindings_sha256": hashlib.sha256(args.bindings.read_bytes()).hexdigest(),
               "source_files": len(by_source), "bound": bound,
               "failed": len(failures), "failures": failures[:10]}
    print(json.dumps(summary, sort_keys=True))
    return 0 if not failures and bound == len(delta) else 1


if __name__ == "__main__":
    raise SystemExit(main())
