"""Small standard-library helpers; importing this module executes no CLI."""
import hashlib
import json
import os
from pathlib import Path

HERE = Path(__file__).resolve().parent
PLAN = HERE.parent / "CLI-FIXTURE-PLAN.md"
SCRIPT_NAMES = ("run.sh", "common.py", "prepare.py", "verify.py", "driver.py", "adapter.py", "workflow.py", "protocol_population.py")
LIMITS = {
    "driver_seconds": 1800, "attempt_bytes": 3 * 1024**3,
    "generate_seconds": 60, "verify_seconds": 30, "status_seconds": 10,
    "export_seconds": 60, "worker_seconds": 30, "worker_memory_mib": 2048,
    "worker_trace_mib": 64, "work_limit": 10000,
    "cells": 16, "pattern_solves": 24, "corruption_exports": 4,
}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False,
                      allow_nan=False).encode("utf-8")


def digest_bytes(value):
    return hashlib.sha256(value).hexdigest()


def sha(path):
    h = hashlib.sha256()
    with Path(path).open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def pairs(items):
    result = {}
    for key, value in items:
        require(key not in result, "duplicate JSON key: " + key)
        result[key] = value
    return result


def decode(raw):
    return json.loads(raw, object_pairs_hook=pairs,
                      parse_constant=lambda value: (_ for _ in ()).throw(ValueError(value)))


def load(path, maximum=32 * 1024**2):
    path = Path(path)
    require(path.is_file() and not path.is_symlink(), "not an ordinary file: " + str(path))
    require(path.stat().st_size <= maximum, "JSON input too large: " + str(path))
    return decode(path.read_bytes())


def jsonl(path, maximum=64 * 1024**2):
    path = Path(path)
    require(path.is_file() and not path.is_symlink(), "not an ordinary JSONL file")
    require(path.stat().st_size <= maximum, "JSONL file exceeds verifier bound")
    result = []
    with path.open("rb") as handle:
        for line in handle:
            require(line.endswith(b"\n"), "incomplete exported JSONL line")
            result.append(decode(line))
    return result


def write(path, value, exclusive=False):
    path = Path(path)
    if exclusive:
        with path.open("xb") as handle:
            handle.write(canonical(value) + b"\n")
            handle.flush()
            os.fsync(handle.fileno())
        return
    temporary = path.with_name(path.name + ".tmp")
    with temporary.open("wb") as handle:
        handle.write(canonical(value) + b"\n")
        handle.flush()
        os.fsync(handle.fileno())
    os.replace(temporary, path)


def relative_file(root, name):
    rel = Path(name)
    require(not rel.is_absolute() and rel.parts and ".." not in rel.parts,
            "unsafe artifact path: " + str(name))
    current = Path(root)
    for part in rel.parts:
        current = current / part
        require(not current.is_symlink(), "artifact symlink: " + str(current))
    require(current.is_file(), "missing artifact: " + str(current))
    return current


def inventory(root):
    root = Path(root)
    require(not root.is_symlink(), "inventory root is a symlink")
    result = {}
    for path in sorted(root.rglob("*")):
        require(not path.is_symlink(), "inventory contains a symlink: " + str(path))
        if path.is_file():
            result[path.relative_to(root).as_posix()] = {
                "sha256": sha(path), "bytes": path.stat().st_size}
        else:
            require(path.is_dir(), "nonordinary inventory entry")
    return result


def artifacts(root, manifest):
    result = {}
    for item in manifest["artifacts"]:
        name = item["path"]
        require(name not in result, "duplicate artifact path")
        path = relative_file(root, name)
        require(item["bytes"] == path.stat().st_size and item["sha256"] == sha(path),
                "artifact digest/size mismatch: " + name)
        require(item["role"] and item["encoding"], "empty artifact metadata")
        result[name] = item
    return result


def config(cell):
    require(0 <= cell < 16, "invalid candidate index")
    return {"family": ["koblitz", "binary"][cell // 8], "n": 7,
            "m": [2, 3][(cell // 2) % 2], "twist": bool(cell % 2),
            "curve_seed": 0, "generator_index": 0, "key_index": (cell // 4) % 2,
            "seed": 20261030, "factor_base_size": 8, "degree_bound": 3,
            "work_limit": 10000, "solver": "sat"}


def file_bindings():
    return {**{name: sha(HERE / name) for name in SCRIPT_NAMES},
            "../CLI-FIXTURE-PLAN.md": sha(PLAN)}
