#!/usr/bin/env python3
"""Replay source and order certificates against the exact-main registry delta."""

from __future__ import annotations

import hashlib
import json
import subprocess
from pathlib import Path

from order_arithmetic import certify, mul, require, selftest

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
BASE = "a53a7e2bfa20080a424b9a2484a16932b6e3c116"
REGISTRY = ROOT / "docs/curves/registry.json"
ALIASES = ROOT / "src/cryptanalysis/curve_aliases.json"
EXPECTED_ADDED = 13219


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def git_file(path: str) -> bytes:
    return subprocess.check_output(["git", "show", f"{BASE}:{path}"], cwd=ROOT)


def load_lines(path: Path) -> dict[str, dict]:
    with path.open("rt", encoding="utf8") as stream:
        records = [json.loads(line) for line in stream]
    by_id = {record["icv1"]: record for record in records}
    require(len(by_id) == len(records), f"duplicate identity in {path}")
    return by_id


def pointer(doc: object, path: str) -> object:
    node = doc
    for raw in path.split("/")[1:]:
        part = raw.replace("~1", "/").replace("~0", "~")
        node = node[int(part)] if isinstance(node, list) else node[part]
    return node


def integer(value: object) -> int:
    if isinstance(value, bool):
        raise ValueError("boolean in numeric source field")
    if isinstance(value, int):
        return value
    if isinstance(value, float) and value.is_integer() and abs(value) < 2**53:
        return int(value)
    if isinstance(value, str):
        return int(value, 0)
    raise ValueError("noninteger numeric source field")


def source_rows(doc: object):
    """Find model/order tuples and exact JSON pointers without the builder."""
    stack = [(doc, "")]
    while stack:
        node, where = stack.pop()
        if isinstance(node, list):
            stack.extend((item, f"{where}/{index}") for index, item in enumerate(node)
                         if isinstance(item, (dict, list)))
            continue
        if not isinstance(node, dict):
            continue
        stack.extend((item, where + "/" + key.replace("~", "~0").replace("/", "~1"))
                     for key, item in node.items() if isinstance(item, (dict, list)))
        try:
            p, a, b = (integer(node[key]) for key in ("p", "a", "b"))
            order = integer(node.get("group_order", node.get("order")))
        except (KeyError, TypeError, ValueError):
            continue
        if p >= 5:
            yield (p, a % p, b % p, order), where


def main() -> None:
    selftest()
    current_bytes = REGISTRY.read_bytes()
    current = json.loads(current_bytes)["curves"]
    baseline = json.loads(git_file("docs/curves/registry.json"))["curves"]
    now = {row["icv1"]: row for row in current}
    old = {row["icv1"]: row for row in baseline}
    require(len(now) == len(current) and len(old) == len(baseline), "duplicate ICV1")
    require(set(old) <= set(now), "previous model identity removed")
    require(len(current) == len(baseline) + EXPECTED_ADDED, "unexpected registry growth")
    for identity, before in old.items():
        after = now[identity]
        require(before["slug"] == after["slug"], f"slug changed: {identity}")
        require(set(before["aliases"]) <= set(after["aliases"]), f"alias removed: {identity}")
        prior_uids = {record["curve_uid"] for record in before["representations"]}
        after_uids = {record["curve_uid"] for record in after["representations"]}
        require(prior_uids <= after_uids, f"EC1 UID removed: {identity}")
    current_aliases = json.loads(ALIASES.read_bytes())["aliases"]
    old_aliases = json.loads(git_file("src/cryptanalysis/curve_aliases.json"))["aliases"]
    require(all(current_aliases.get(name) == slug for name, slug in old_aliases.items()),
            "existing alias mapping changed")

    added = {identity: row for identity, row in now.items() if identity not in old}
    certs = load_lines(HERE / "candidate-order-certificates.jsonl")
    bindings = load_lines(HERE / "candidate-source-bindings.jsonl")
    require(set(added) == set(certs) == set(bindings), "certificate identity set differs")
    source_cache: dict[str, object] = {}
    source_indexes: dict[str, dict[tuple[int, int, int, int], list[str]]] = {}
    for identity, entry in added.items():
        certificate = certs[identity]
        binding = bindings[identity]
        require(entry["family"] == "prime", f"nonprime new model: {identity}")
        require(certificate["status"] == "certified" and certify(entry) == certificate,
                f"arithmetic certificate changed: {identity}")
        require(binding["matching_rows"] == 1, f"source not unique: {identity}")
        source = binding["source"]
        raw = (ROOT / source).read_bytes()
        require(sha(raw) == binding["source_sha256"], f"source digest changed: {source}")
        if source not in source_cache:
            source_cache[source] = json.loads(raw)
            index: dict[tuple[int, int, int, int], list[str]] = {}
            for key, location in source_rows(source_cache[source]):
                index.setdefault(key, []).append(location)
            source_indexes[source] = index
        row = pointer(source_cache[source], binding["pointer"])
        p = integer(row["p"])
        model = [p, integer(row["a"]) % p, integer(row["b"]) % p,
                 integer(row.get("group_order", row.get("order")))]
        require(model == binding["model"], f"source pointer changed: {identity}")
        require(source_indexes[source].get(tuple(model)) == [binding["pointer"]],
                f"source row is not unique: {identity}")
        require([int(entry["params"][key]) for key in ("p", "a", "b")] +
                [int(entry["order"])] == model, f"registry model/order changed: {identity}")
        require(source in entry["sources"], f"source membership changed: {identity}")

    require(len(source_cache) == 152, "audited source-file count changed")
    by_model = {(int(row["params"]["p"]), int(row["params"]["a"]),
                 int(row["params"]["b"]), int(row["order"])): row
                for row in current if row["family"] == "prime" and
                row["params"].get("a") is not None and row["params"].get("b") is not None}
    conductor_rows = 0
    conductor_files = sorted((ROOT / "research/conductor_gap_cm_orbits_20261015/prepared")
                             .glob("*curve_ids.json"))
    require(len(conductor_files) == 4, "conductor prepared-source count changed")
    for path in conductor_files:
        rel = str(path.relative_to(ROOT))
        require(sha(path.read_bytes()) == sha(git_file(rel)),
                f"conductor source differs from baseline: {rel}")
        for source_row in json.loads(path.read_bytes()):
            p, a, b = (integer(source_row[key]) for key in ("p", "a", "b"))
            order, r = (integer(source_row[key]) for key in ("order", "r"))
            generator = tuple(int(coordinate, 0) for coordinate in source_row["generator"])
            require(p > 3 and order == r and
                    (4 * a**3 + 27 * b**2) % p != 0, f"invalid conductor model: {rel}")
            require((generator[1]**2 - generator[0]**3 - a * generator[0] - b) % p == 0,
                    f"conductor generator off curve: {rel}")
            require(mul(p, a, r, generator) is None, f"conductor order claim failed: {rel}")
            entry = by_model[(p, a % p, b % p, order)]
            require(any(rel in rep["sources"] and
                        rep["curve"]["subgroup_order"] == str(r) and
                        [int(v, 0) for v in rep["curve"]["generator"]] == list(generator)
                        for rep in entry["representations"]),
                    f"conductor EC1 representation not registered: {rel}")
            conductor_rows += 1
    require(conductor_rows == 298, "conductor representation count changed")
    first = dict(next(iter(added.values())))
    first["order"] = str(int(first["order"]) + 1)
    try:
        certify(first)
    except ValueError:
        pass
    else:
        raise ValueError("wrong-order negative control unexpectedly passed")
    print(json.dumps({"baseline": BASE, "registry_sha256": sha(current_bytes),
                      "models": len(current), "added_source_bound_and_order_certified": len(added),
                      "source_files": len(source_cache), "old_aliases_preserved": len(old_aliases),
                      "conductor_generators_checked": conductor_rows,
                      "negative_order_control": "rejected"}, sort_keys=True))


if __name__ == "__main__":
    main()
