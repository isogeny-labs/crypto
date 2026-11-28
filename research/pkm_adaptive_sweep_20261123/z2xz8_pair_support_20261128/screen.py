#!/usr/bin/env python3
"""Exact signed-pair support on the six frozen PKM held-out curves."""

import argparse
import hashlib
import json
import subprocess
from collections import defaultdict
from pathlib import Path

MANIFEST_SHA = "99140281d9a94c288b8993b503cf91e4226879052e25c63dfdad0fccd20c8bc8"
EXTRACT_SHA = "0809632efd249cb6539c91d8dfc23cfa9fc108abc7af880508ddb1bb40bb7191"
SOURCE_SHA = "4524579faca7f6960d54450df70ce7d80fbfb62692402b7a5c292a0259676d1e"
LIB_SHA = "00b3f89920eb9877070584c0016e0c6626f4e978926823d1795d4fdedab83544"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def parsehex(value):
    return int(value, 16)


def add(left, right, p, a):
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2 and (y1 + y2) % p == 0:
        return None
    if x1 == x2:
        slope = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
    else:
        slope = (y2 - y1) * pow(x2 - x1, -1, p) % p
    x3 = (slope * slope - x1 - x2) % p
    return x3, (slope * (x1 - x3) - y1) % p


def roots_by_square(p):
    roots = [[] for _ in range(p)]
    for y in range(p):
        roots[y * y % p].append(y)
    return roots


def is_point(point, p, a, b):
    return point is None or (point[1] * point[1] -
                             (point[0] ** 3 + a * point[0] + b)) % p == 0


def pairs(xs, p, a, b, roots, order, targets):
    signed = []
    for col, x in enumerate(xs):
        rhs = (x ** 3 + a * x + b) % p
        ys = roots[rhs]
        if len(ys) != 2:
            raise ValueError(f"expected two signed points at x={x}")
        low, high = sorted(ys)
        if (low + high) % p != 0:
            raise ValueError("roots are not opposite")
        signed.extend(((x, low, col, 1), (x, high, col, -1)))
    support = defaultdict(set)
    multiplicity = defaultdict(int)
    for i, (x1, y1, c1, s1) in enumerate(signed):
        for x2, y2, c2, s2 in signed[i:]:
            point = add((x1, y1), (x2, y2), p, a)
            if not is_point(point, p, a, b):
                raise ValueError("pair sum off target curve")
            coeff = defaultdict(int)
            coeff[c1] += s1
            coeff[c2] += s2
            row = tuple((c, v) for c, v in sorted(coeff.items()) if v)
            support[point].add(row)
            multiplicity[point] += 1
    raw_pairs = len(signed) * (len(signed) + 1) // 2
    row_total = sum(map(len, support.values()))
    if sum(multiplicity.values()) != raw_pairs or row_total > raw_pairs:
        raise ValueError("pair count does not reconcile")
    if len(support) > order:
        raise ValueError("more sums than group points")
    target_rows = [len(support.get(point, set())) for point in targets]
    return {"columns": len(xs), "signed_points": len(signed),
            "raw_pairs": raw_pairs, "distinct_coefficient_rows": row_total,
            "identity_raw_pairs": multiplicity.get(None, 0),
            "identity_distinct_rows": len(support.get(None, set())),
            "group_support": len(support),
            "group_empty": order - len(support),
            "group_single": sum(len(rows) == 1 for rows in support.values()),
            "group_multi": sum(len(rows) >= 2 for rows in support.values()),
            "max_rows_per_group_point": max(map(len, support.values()), default=0),
            "target_rows": target_rows,
            "target_hits": sum(v > 0 for v in target_rows),
            "target_multi": sum(v >= 2 for v in target_rows),
            "target_total_rows": sum(target_rows)}


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, required=True)
    ap.add_argument("--extract-bin", type=Path, required=True)
    ap.add_argument("--lib", type=Path, required=True)
    args = ap.parse_args()
    root = args.root.resolve(strict=True)
    here = Path(__file__).resolve().parent
    extract = args.extract_bin.resolve(strict=True)
    lib = args.lib.resolve(strict=True)
    if (sha(extract.read_bytes()) != EXTRACT_SHA
            or sha((here / "extract.rs").read_bytes()) != SOURCE_SHA
            or sha(lib.read_bytes()) != LIB_SHA):
        ap.error("source or executable hash differs")
    manifest_path = root / "z2xz8_holdout/manifest.json"
    if sha(manifest_path.read_bytes()) != MANIFEST_SHA:
        ap.error("held-out manifest differs")
    manifest = json.loads(manifest_path.read_bytes())
    entries = sorted(manifest["entries"], key=lambda e: (e["bits"], e["curve_index"]))
    labels = [f"b{e['bits']}_c{e['curve_index']}" for e in entries]
    if labels != ["b11_c0", "b11_c1", "b11_c2", "b14_c0", "b14_c1", "b14_c2"]:
        ap.error("six-curve roster differs")
    receipt_path = root / "z2xz8_preflight_archive/receipts.jsonl"
    receipts = {row["id"]: row for row in map(json.loads, receipt_path.read_text().splitlines())}
    if receipts["header"]["manifest_sha256"] != MANIFEST_SHA:
        ap.error("preflight manifest differs")
    raw_dir = here / "raw"
    raw_dir.mkdir(exist_ok=True)
    rows = []
    for entry, label in zip(entries, labels):
        workload_path = root / "z2xz8_holdout" / entry["path"]
        if sha(workload_path.read_bytes()) != entry["sha256"]:
            raise ValueError(f"workload hash differs: {label}")
        workload = json.loads(workload_path.read_bytes())
        curve = workload["curve"]
        p = parsehex(curve["field"]["modulus"])
        a, b = parsehex(curve["a"]), parsehex(curve["b"])
        order = parsehex(curve["group_order"])
        output = subprocess.check_output([str(extract), str(p), str(a), str(b)])
        raw_path = raw_dir / f"{label}.json"
        raw_path.write_bytes(output)
        extracted = json.loads(output)
        archived = root / "z2xz8_preflight_archive/raw" / f"{label}.stdout.json"
        preflight = receipts[f"result:{label}"]
        if sha(archived.read_bytes()) != preflight["stdout"]["sha256"]:
            raise ValueError(f"preflight raw hash differs: {label}")
        previous = json.loads(archived.read_bytes())["rows"][0]["factor_base"]
        native = previous["cost"]["native"]
        if (extracted["p"], extracted["a"], extracted["b"]) != (p, a, b):
            raise ValueError(f"extractor curve differs: {label}")
        if (extracted["m"] != native["aux_selection_m"]
                or extracted["cosets_tested"] != native["aux_selection_cosets_tested"]
                or extracted["total_adds"] != previous["cost"]["group_ops"]["adds"]
                or extracted["total_doubles"] != previous["cost"]["group_ops"]["doubles"]
                or len(extracted["selected"]) != previous["abscissae"]):
            raise ValueError(f"map ledger differs: {label}")
        coset, selected, interval = (extracted[k] for k in ("coset", "selected", "interval"))
        if (len(coset) != 32 or coset != sorted(set(coset))
                or selected != sorted(set(selected))
                or interval != sorted(set(interval))
                or not set(selected) <= set(coset)
                or len(selected) != len(interval)):
            raise ValueError(f"base size or coset differs: {label}")
        roots = roots_by_square(p)
        for base in (selected, interval):
            for x in base:
                if len(roots[(x ** 3 + a * x + b) % p]) != 2:
                    raise ValueError(f"nonlifting abscissa: {label}")
        if interval != [x for x in range(32)
                        if len(roots[(x ** 3 + a * x + b) % p]) == 2]:
            raise ValueError(f"interval differs: {label}")
        targets = [(parsehex(t["point"]["x"]), parsehex(t["point"]["y"]))
                   for t in workload["targets"]]
        if len(targets) != 16 or any(not is_point(t, p, a, b) for t in targets):
            raise ValueError(f"target identity differs: {label}")
        candidate = pairs(selected, p, a, b, roots, order, targets)
        control = pairs(interval, p, a, b, roots, order, targets)
        rows.append({"label": label, "p": p, "a": a, "b": b, "order": order,
                     "target_scalars": [t["scalar"] for t in workload["targets"]],
                     "extract_sha256": sha(output), "preflight_sha256": sha(archived.read_bytes()),
                     "candidate": candidate, "interval": control})
    result = {"schema": "pkm-z2xz8-pair-support/v1",
              "manifest_sha256": MANIFEST_SHA,
              "extractor_source_sha256": SOURCE_SHA,
              "extractor_binary_sha256": EXTRACT_SHA,
              "library_sha256": LIB_SHA,
              "preflight_receipt_sha256": sha(receipt_path.read_bytes()),
              "rows": rows}
    dest = here / "RESULT.json"
    dest.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps({"result": str(dest),
                      "sha256": sha(dest.read_bytes()), "curves": len(rows)}, sort_keys=True))


if __name__ == "__main__":
    main()
