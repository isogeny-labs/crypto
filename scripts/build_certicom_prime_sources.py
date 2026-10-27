#!/usr/bin/env python3
"""Bind the two cited Certicom prime models to their frozen native sources."""
from __future__ import annotations

import argparse
import hashlib
import json
import re
from pathlib import Path

import curve_id as cid

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "docs/curves/sources/certicom-prime-models.json"
P30 = "research/certicom131_ic_program_20261020/pkm_ext/specs/SPEC-p30.json"
ECC = "src/ecc/curve_zoo.rs"


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def multiply(p: int, a: int, point: tuple[int, int], n: int) -> tuple[int, int] | None:
    """Independent affine check of the source generator's stated order."""
    def add(x, y):
        if x is None:
            return y
        if y is None:
            return x
        x1, y1 = x
        x2, y2 = y
        if x1 == x2 and (y1 + y2) % p == 0:
            return None
        if x == y:
            slope = (3 * x1 * x1 + a) * pow(2 * y1, -1, p) % p
        else:
            slope = (y2 - y1) * pow(x2 - x1, -1, p) % p
        x3 = (slope * slope - x1 - x2) % p
        return x3, (slope * (x1 - x3) - y1) % p

    acc = None
    while n:
        if n & 1:
            acc = add(acc, point)
        point = add(point, point)
        n >>= 1
    return acc


def record(name: str, values: dict, source: str) -> dict:
    p, a, b, n = (int(values[k]) for k in ("p", "a", "b", "group_order"))
    gx, gy = int(values["gx"]), int(values["gy"])
    assert p > 3 and n > 1 and (4 * pow(a, 3, p) + 27 * pow(b, 2, p)) % p
    assert 0 <= gx < p and 0 <= gy < p
    assert (gy * gy - gx * gx * gx - a * gx - b) % p == 0
    assert multiply(p, a, (gx, gy), n) is None
    ident = cid.prime_id(p, a, b, n)
    return {
        "name": name,
        "p": str(p), "a": str(a), "b": str(b),
        "group_order": str(n), "subgroup_order": str(n), "cofactor": 1,
        "generator": {"x": str(gx), "y": str(gy)},
        "icv1": ident["icv1"], "slug": ident["slug"],
        "source_path": source, "source_sha256": sha(ROOT / source),
    }


def build() -> dict:
    source = json.loads((ROOT / P30).read_text())["workloads"]["curves"]
    assert len(source) == 1 and source[0]["kind"] == "prime_explicit"
    p30 = {k: source[0][k] for k in ("p", "a", "b", "group_order", "gx", "gy")}
    text = (ROOT / ECC).read_text()
    block = text.split("pub fn certicom_eccp131() -> Self {", 1)[1].split("\n    }", 1)[0]
    fields = {k: int(re.search(r"\b" + k + r': hexp\("([0-9A-F]+)"\)', block).group(1), 16)
              for k in ("p", "a", "b", "gx", "gy", "n")}
    fields["group_order"] = fields.pop("n")
    rows = [record("icv1-fp30-t49927-d70f5de2", p30, P30),
            record("ECCp-131", fields, ECC)]
    assert [row["slug"] for row in rows] == [
        "icv1-fp30-t49927-d70f5de2",
        "icv1-fp131-t29529377007934231835-9681b699",
    ]
    return {"schema": "curve-sources.certicom-prime/v1", "records": rows}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    text = json.dumps(build(), indent=2) + "\n"
    if args.check:
        if not OUT.exists() or OUT.read_text() != text:
            print(f"{OUT.relative_to(ROOT)} is stale")
            return 1
        print("Certicom prime models match both source files and generator orders")
    else:
        OUT.write_text(text)
        print(f"wrote {OUT.relative_to(ROOT)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
