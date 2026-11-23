#!/usr/bin/env python3
"""Freeze the N83 padd-rank panel: five sealed holdout points plus fifteen new.

New points: scalar = 1 + LE(SHA256(domain || u32_le(i))) mod (r-1), i = 0..14,
checked disjoint from every earlier committed N83 point file and the five
sealed holdout points, on the curve, and in the order-r subgroup.
"""
import argparse, hashlib, importlib.util, json
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
HOLDOUT = ROOT / "research/n83_e2e_holdout_20261016"
DOMAIN = b"n83-padd-rank-fifteen-20261019-v1"
NEW = 15
SEALED_TARGETS_SHA256 = "722bc60cd010e533cd715a9bfb321fba52d950352c99684cd422b23af7f93d26"
SEALED_ANSWERS_SHA256 = "b4c2ae858770b7c8842703558984326bc0002b0f1deb3c898d5ac30562de8b96"

spec = importlib.util.spec_from_file_location(
    "n83_cold_fixture", ROOT / "research/n83_cold_factor_base_20261016/generate.py")
cold = importlib.util.module_from_spec(spec); spec.loader.exec_module(cold)
group = cold.group


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def lines(path):
    return [json.loads(l) for l in Path(path).read_text().splitlines()]


def prior_set():
    """Every committed N83 point file plus the derived wide3 points.

    Earlier archived fixtures that are not in the repository cannot be
    replayed here; a hash-derived collision with them has probability about
    (archived points) / r ~ 1e-12 and is recorded as a limit, not checked.
    """
    prior = set()
    for path in sorted((ROOT / "research").glob("n83*/*.jsonl")):
        if path.parent == HERE:
            continue
        for line in path.read_text().splitlines():
            try:
                row = json.loads(line)
            except ValueError:
                continue
            if isinstance(row, list) and len(row) == 2 and all(isinstance(v, str) for v in row):
                prior.add(tuple(row))
    previous = cold.previous
    for index in range(1_000):
        seed = hashlib.sha256(previous.WIDE3_DOMAIN + index.to_bytes(4, "little")).digest()
        scalar = 1 + int.from_bytes(seed, "little") % (group.R - 1)
        point = group.multiply(group.G, scalar)
        prior.add((str(point[0]), str(point[1])))
    return prior


def build():
    if sha(HOLDOUT / "targets.jsonl") != SEALED_TARGETS_SHA256 or \
            sha(HOLDOUT / "answers.jsonl") != SEALED_ANSWERS_SHA256:
        raise ValueError("sealed holdout fixture changed")
    sealed_t, sealed_a = lines(HOLDOUT / "targets.jsonl"), lines(HOLDOUT / "answers.jsonl")
    seen = prior_set() | {tuple(p) for p in sealed_t}
    targets, answers = list(sealed_t), list(sealed_a)
    for i in range(NEW):
        seed = hashlib.sha256(DOMAIN + i.to_bytes(4, "little")).digest()
        scalar = 1 + int.from_bytes(seed, "little") % (group.R - 1)
        point = group.multiply(group.G, scalar)
        enc = [str(point[0]), str(point[1])]
        if tuple(enc) in seen or not group.on_curve(point) or \
                group.multiply(point, group.R) is not None:
            raise ValueError(f"new point {i} repeats or fails replay")
        seen.add(tuple(enc))
        targets.append(enc)
        answers.append({"fixture_index": 5 + i, "point": enc, "scalar": str(scalar),
                        "seed_sha256": seed.hex()})
    return targets, answers


def write(path, rows):
    Path(path).write_text("".join(json.dumps(r) + "\n" for r in rows))


def main(check):
    targets, answers = build()
    manifest = {"domain": DOMAIN.decode(), "sealed_points": 5, "new_points": NEW,
                "sealed_targets_sha256": SEALED_TARGETS_SHA256,
                "sealed_answers_sha256": SEALED_ANSWERS_SHA256}
    if check:
        for name, rows in (("targets.jsonl", targets), ("answers.jsonl", answers)):
            if lines(HERE / name) != rows:
                raise ValueError(f"{name} differs from regeneration")
        m = json.loads((HERE / "fixture_manifest.json").read_text())
        if m["targets_sha256"] != sha(HERE / "targets.jsonl") or \
                m["answers_sha256"] != sha(HERE / "answers.jsonl"):
            raise ValueError("manifest hashes differ")
        print(json.dumps({"mode": "check", "points": len(targets),
                          "targets_sha256": m["targets_sha256"]}))
        return
    write(HERE / "targets.jsonl", targets)
    write(HERE / "answers.jsonl", answers)
    manifest["targets_sha256"] = sha(HERE / "targets.jsonl")
    manifest["answers_sha256"] = sha(HERE / "answers.jsonl")
    (HERE / "fixture_manifest.json").write_text(json.dumps(manifest, indent=2, sort_keys=True) + "\n")
    print(json.dumps(manifest, sort_keys=True))


if __name__ == "__main__":
    ap = argparse.ArgumentParser(); ap.add_argument("--check", action="store_true")
    main(ap.parse_args().check)
