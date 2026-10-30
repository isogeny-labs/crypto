"""Validator's shared loaders (TASK-20261029-654f73). Own code; imports nothing
from research/ic_packed_filter_20261029/cards/tools/ and reads no analysis/ file.

Counted sessions are loaded through `load_counted`, which removes every wall or
timing field before any other code sees the record (PROTOCOL section 10: no wall
field of a counted session, no rho wall time). The only isolation data kept are
`level` and `blockers`; a blocker that begins with the too-short prefix is cut
back to the prefix, so the solve time printed inside it is never read.
"""
import hashlib
import json
import math
import os

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
REPO = os.path.abspath(os.path.join(ROOT, "..", ".."))
TOO_SHORT = "L2: solve too short to admit wall time"
IC_ARMS_WALL = ("dgb", "dgb-aa", "dgbf", "dgbf-edge")


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def session_dir(b, kind):
    return os.path.join(ROOT, f"P{b}", kind, "eks-results")


def check_sums(eks):
    """Check every line of SHA256SUMS; return (ok, n, bad list)."""
    bad, n = [], 0
    with open(os.path.join(eks, "SHA256SUMS")) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            want, name = line.split(None, 1)
            n += 1
            p = os.path.join(eks, name)
            if not os.path.exists(p) or sha256_file(p) != want:
                bad.append(name)
    return (not bad), n, bad


def _is_timing_key(k):
    k = str(k)
    return ("wall" in k or k == "time" or k == "online" or k.endswith("_ns")
            or k.endswith("_us") or k.endswith("_ms") or k.endswith("_seconds")
            or k == "unix_ms")


def _strip(o):
    if isinstance(o, dict):
        return {k: _strip(v) for k, v in o.items() if not _is_timing_key(k)}
    if isinstance(o, list):
        return [_strip(v) for v in o]
    return o


def _mask_blockers(bl):
    out = []
    for s in bl:
        out.append(TOO_SHORT if s.startswith(TOO_SHORT) else s)
    return out


def counted_view(rec):
    iso = rec.get("isolation") or {}
    keep_iso = {"level": iso.get("level"), "blockers": _mask_blockers(iso.get("blockers") or [])}
    r = {k: v for k, v in rec.items() if k != "isolation"}
    r = _strip(r)
    r["isolation"] = keep_iso
    return r


def load_counted(b, kind="counted"):
    p = os.path.join(session_dir(b, kind), "session", "records.jsonl")
    out = []
    with open(p) as f:
        for line in f:
            out.append(counted_view(json.loads(line)))
    return out


def load_wall(b):
    """Wall session records in full. The wall session runs only IC arms."""
    p = os.path.join(session_dir(b, "wall"), "session", "records.jsonl")
    out = []
    with open(p) as f:
        for line in f:
            r = json.loads(line)
            if r["arm"] not in IC_ARMS_WALL:
                raise SystemExit(f"STOP: wall session arm {r['arm']} is not an IC arm")
            out.append(r)
    return out


def load_wall_counts_only(b):
    """Wall session records with every wall field removed (used where only
    counts are compared, e.g. the P28 wall records against P28 counted)."""
    return load_counted(b, "wall")


def key(r):
    w = r["workload"]
    return (w["curve"]["slug"], w["target_index"], r["round"])


def by_arm(records, measured_only=True):
    d = {}
    for r in records:
        if measured_only and r["warmup"]:
            continue
        d.setdefault(r["arm"], {})
        k = key(r)
        if k in d[r["arm"]]:
            raise SystemExit(f"STOP: duplicate key {k} on arm {r['arm']}")
        d[r["arm"]][k] = r
    return d


def unpriced_total(r):
    """U of band.rs at 3271a25d02: for each counter named in cost.unpriced,
    the top-level counters value, else the sum over phases of native[name] or
    native[name without _uncharged]; None if any is missing."""
    total, missing = 0, False
    counters = r.get("counters") or {}
    for name in r["cost"]["unpriced"]:
        base = name[: -len("_uncharged")] if name.endswith("_uncharged") else name
        if name in counters:
            total += counters[name]
            continue
        found, t = False, 0
        for p in r["phases"]:
            nat = p.get("native") or {}
            if name in nat:
                t += nat[name]; found = True
            elif base in nat:
                t += nat[base]; found = True
        if not found and base.startswith("solver_") and r.get("solver"):
            total += r["solver"]["ops"]
            continue
        if not found:
            missing = True
        total += t
    return total, missing


def calibrated(r):
    pinned = (r.get("detail") or {}).get("calibration_pinned_units") or []
    if pinned:
        raise SystemExit(f"STOP: record {r['record_id']} carries pinned calibration units {pinned}")
    return 0.0, 0


def priced_gae(r, w):
    g = r["cost"]["total_gae"]
    u, missing = unpriced_total(r)
    if missing and w > 0:
        raise SystemExit(f"STOP: record {r['record_id']} has an uncounted unpriced counter")
    c, nc = calibrated(r)
    return g - c + w * (u + nc)


def tfbs(r):
    """table_filter_bit_sets summed over phases (0 if absent) and the phases carrying it."""
    t, where = 0, []
    for p in r["phases"]:
        wn = p.get("work_native") or {}
        if "table_filter_bit_sets" in wn:
            t += wn["table_filter_bit_sets"]
            where.append(p["name"])
    return t, where


def sprime(r, w):
    t, _ = tfbs(r)
    return (priced_gae(r, w) + w * t) / math.sqrt(r["workload"]["curve"]["r"])


def dump(obj, path):
    with open(path, "w") as f:
        json.dump(obj, f, indent=1, sort_keys=True)
        f.write("\n")
