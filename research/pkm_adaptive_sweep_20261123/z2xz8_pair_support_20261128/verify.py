#!/usr/bin/env python3
"""Replay the pair-support panel using scalar labels rather than pair addition."""

import hashlib
import json
from collections import defaultdict
from pathlib import Path


def sha(data):
    return hashlib.sha256(data).hexdigest()


def h(value):
    return int(value, 16)


def plus(u, v, p, a):
    if u is None:
        return v
    if v is None:
        return u
    if u[0] == v[0] and (u[1] + v[1]) % p == 0:
        return None
    if u[0] == v[0]:
        numerator, denominator = 3 * u[0] ** 2 + a, 2 * u[1]
    else:
        numerator, denominator = v[1] - u[1], v[0] - u[0]
    lam = numerator * pow(denominator, p - 2, p) % p
    x = (lam * lam - u[0] - v[0]) % p
    return x, (lam * (u[0] - x) - u[1]) % p


def group_table(workload):
    curve = workload["curve"]
    p, a, b = h(curve["field"]["modulus"]), h(curve["a"]), h(curve["b"])
    n = h(curve["group_order"])
    generator = h(curve["generator"]["x"]), h(curve["generator"]["y"])
    if (generator[1] ** 2 - (generator[0] ** 3 + a * generator[0] + b)) % p:
        raise ValueError("generator not on curve")
    points = {}
    by_x = defaultdict(list)
    current = None
    for scalar in range(n):
        if current in points:
            raise ValueError("generator order below claimed prime order")
        points[current] = scalar
        if current is not None:
            by_x[current[0]].append((current[1], scalar))
        current = plus(current, generator, p, a)
    if current is not None or len(points) != n:
        raise ValueError("generator order differs")
    targets = []
    for item in workload["targets"]:
        target = h(item["point"]["x"]), h(item["point"]["y"])
        if points.get(target) != item["scalar"]:
            raise ValueError("frozen scalar differs from target")
        targets.append(item["scalar"])
    return p, a, b, n, by_x, targets


def by_labels(xs, by_x, n, targets):
    labels = []
    for col, x in enumerate(xs):
        two = sorted(by_x[x])
        if len(two) != 2 or (two[1][1] + two[0][1]) % n != 0:
            raise ValueError("abscissa does not give two points")
        labels.extend(((two[0][1], col, 1), (two[1][1], col, -1)))
    raw = defaultdict(int)
    rows = defaultdict(set)
    for i, (k1, c1, s1) in enumerate(labels):
        for k2, c2, s2 in labels[i:]:
            key = (k1 + k2) % n
            coeff = defaultdict(int)
            coeff[c1] += s1
            coeff[c2] += s2
            row = tuple((c, value) for c, value in sorted(coeff.items()) if value)
            raw[key] += 1
            rows[key].add(row)
    expected = len(labels) * (len(labels) + 1) // 2
    if sum(raw.values()) != expected:
        raise ValueError("raw pair count differs")
    sizes = [len(rows.get(t, set())) for t in targets]
    return {"columns": len(xs), "signed_points": len(labels),
            "raw_pairs": expected, "distinct_coefficient_rows": sum(map(len, rows.values())),
            "identity_raw_pairs": raw.get(0, 0),
            "identity_distinct_rows": len(rows.get(0, set())),
            "group_support": len(rows), "group_empty": n - len(rows),
            "group_single": sum(len(v) == 1 for v in rows.values()),
            "group_multi": sum(len(v) >= 2 for v in rows.values()),
            "max_rows_per_group_point": max(map(len, rows.values()), default=0),
            "target_rows": sizes, "target_hits": sum(v > 0 for v in sizes),
            "target_multi": sum(v >= 2 for v in sizes),
            "target_total_rows": sum(sizes)}


def main():
    here = Path(__file__).resolve().parent
    root = here.parent
    result_path = here / "RESULT.json"
    result = json.loads(result_path.read_bytes())
    manifest = json.loads((root / "z2xz8_holdout/manifest.json").read_bytes())
    entries = {f"b{e['bits']}_c{e['curve_index']}": e for e in manifest["entries"]}
    checked = []
    for row in result["rows"]:
        label = row["label"]
        entry = entries[label]
        workload_path = root / "z2xz8_holdout" / entry["path"]
        if sha(workload_path.read_bytes()) != entry["sha256"]:
            raise ValueError("workload hash differs")
        workload = json.loads(workload_path.read_bytes())
        p, a, b, n, by_x, targets = group_table(workload)
        if (row["p"], row["a"], row["b"], row["order"], row["target_scalars"]) != (
                p, a, b, n, targets):
            raise ValueError(f"curve or target identity differs: {label}")
        raw_path = here / "raw" / f"{label}.json"
        if sha(raw_path.read_bytes()) != row["extract_sha256"]:
            raise ValueError(f"extractor raw differs: {label}")
        bases = json.loads(raw_path.read_bytes())
        for kind in ("selected", "interval"):
            rebuilt = by_labels(bases[kind], by_x, n, targets)
            expected = row["candidate" if kind == "selected" else "interval"]
            if rebuilt != expected:
                raise ValueError(f"scalar-label pair replay differs: {label}:{kind}")
        checked.append(label)
    if checked != ["b11_c0", "b11_c1", "b11_c2", "b14_c0", "b14_c1", "b14_c2"]:
        raise ValueError("six-curve order differs")
    receipt = {"schema": "pkm-z2xz8-pair-support-verify/v1",
               "result_sha256": sha(result_path.read_bytes()),
               "manifest_sha256": result["manifest_sha256"],
               "curves_checked": checked,
               "method": "enumerate kG for all k; add pair scalar labels modulo group order"}
    dest = here / "VERIFY.json"
    dest.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"curves_checked": len(checked),
                      "verify_sha256": sha(dest.read_bytes())}, sort_keys=True))


if __name__ == "__main__":
    main()
