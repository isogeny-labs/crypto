"""Shared instrumentation for the total verified-relation cost benchmark.

Stage timers (wall + CPU), incremental Gaussian elimination over Z/r, the
factor-base orbit lookup used to build relation vectors, receipts plumbing and
source hashing.  No cryptographic arithmetic lives here.
"""
from __future__ import annotations

import contextlib
import hashlib
import json
import os
from pathlib import Path
import platform
import resource
import subprocess
import sys
import time

STAGES = ("fb_setup", "input_gen", "eq_build", "engine_load", "solver_setup", "solve",
          "model_extraction", "lifting", "verification", "rank_update")
SOLVER_STAGES = ("solver_setup", "solve", "model_extraction")
CORE_STAGES = ("fb_setup", "eq_build", "solver_setup", "solve", "model_extraction",
               "lifting", "verification", "rank_update")


class Stages:
    """Accumulating wall/CPU timers; a stage is charged even if the body raises."""

    def __init__(self):
        self.t = {}

    @contextlib.contextmanager
    def stage(self, name):
        assert name in STAGES, name
        w0, c0 = time.perf_counter(), time.process_time()
        try:
            yield
        finally:
            d = self.t.setdefault(name, {"wall_s": 0.0, "cpu_s": 0.0, "calls": 0})
            d["wall_s"] += time.perf_counter() - w0
            d["cpu_s"] += time.process_time() - c0
            d["calls"] += 1

    def dump(self):
        return {k: dict(v) for k, v in self.t.items()}


def rank_add(pivots, row, prime):
    """Incremental Gaussian elimination over Z/prime.

    `pivots` maps a pivot column to a row normalised to leading entry 1 with zeros
    in all earlier columns.  Returns True (and records the reduced row) when `row`
    is independent of the rows already present; returns False and changes nothing
    otherwise.
    """
    row = [x % prime for x in row]
    for col in range(len(row)):
        c = row[col]
        if not c:
            continue
        if col in pivots:
            pr = pivots[col]
            row = [(a - c * b) % prime for a, b in zip(row, pr)]
        else:
            inv = pow(c, -1, prime)
            pivots[col] = [(a * inv) % prime for a in row]
            return True
    return False


def build_lookup(curve, reps, lam, n, r):
    """Signed Frobenius-orbit table: point -> (column, coefficient mod r).

    A representative already inside an earlier column's signed orbit is skipped, so
    columns are distinct orbit classes and relation vectors are unique.
    """
    lookup, columns = {}, []
    for rep in reps:
        if rep in lookup or curve.neg(rep) in lookup:
            continue
        col = len(columns)
        columns.append(rep)
        point, coeff = rep, 1
        for _ in range(n):
            for signed, value in ((point, coeff), (curve.neg(point), (-coeff) % r)):
                assert lookup.get(signed, (col, value)) == (col, value)
                lookup[signed] = (col, value)
            point = curve.frob(point)
            coeff = coeff * lam % r
    return lookup, columns


def relation_row(lookup, width, witness, r):
    row = [0] * width
    for point in witness:
        col, coeff = lookup[tuple(point)]
        row[col] = (row[col] + coeff) % r
    return row


def digest(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def sha256_file(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def peak_rss_bytes():
    return resource.getrusage(resource.RUSAGE_SELF).ru_maxrss * 1024


def cpu_model():
    try:
        for line in Path("/proc/cpuinfo").read_text().splitlines():
            if line.startswith("model name"):
                return line.split(":", 1)[1].strip()
    except OSError:
        pass
    return None


def versions():
    out = {"python": sys.version, "platform": platform.platform(), "machine": platform.machine(),
           "cpu_model": cpu_model(), "cpu_count": os.cpu_count()}
    for pkg in ("z3", "pycryptosat"):
        try:
            m = __import__(pkg)
            out[pkg] = m.get_version_string() if pkg == "z3" else m.__version__
        except Exception as error:  # record, never hide
            out[pkg] = f"unavailable: {type(error).__name__}"
    return out


def git_state(repo):
    def run(*args):
        try:
            return subprocess.run(["git", "-C", str(repo), *args], capture_output=True,
                                  text=True, timeout=20).stdout.strip()
        except Exception:
            return None
    return {"head": run("rev-parse", "HEAD"), "dirty_files": (run("status", "--porcelain") or "").splitlines()[:40]}


def source_hashes(paths, root):
    return {str(Path(p).resolve().relative_to(root)): sha256_file(p) for p in sorted(paths)}


def run_child(cmd, timeout, env):
    """Run one benchmark child, charging parent-side wall and child CPU (incl. startup)."""
    ru0 = resource.getrusage(resource.RUSAGE_CHILDREN)
    w0 = time.perf_counter()
    load = os.getloadavg()
    try:
        proc = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout, env=env)
        rec = {"returncode": proc.returncode, "timeout_killed": False,
               "stdout": proc.stdout, "stderr": proc.stderr[-2000:]}
    except subprocess.TimeoutExpired as error:
        rec = {"returncode": None, "timeout_killed": True,
               "stdout": (error.stdout.decode(errors="replace") if isinstance(error.stdout, bytes) else error.stdout) or "",
               "stderr": ((error.stderr.decode(errors="replace") if isinstance(error.stderr, bytes) else error.stderr) or "")[-2000:]}
    ru1 = resource.getrusage(resource.RUSAGE_CHILDREN)
    rec["parent_wall_s"] = time.perf_counter() - w0
    rec["child_cpu_s"] = (ru1.ru_utime - ru0.ru_utime) + (ru1.ru_stime - ru0.ru_stime)
    rec["loadavg_1m_at_start"] = load[0]
    result = None
    for line in reversed(rec["stdout"].splitlines()):
        line = line.strip()
        if line.startswith("{"):
            try:
                result = json.loads(line)
                break
            except json.JSONDecodeError:
                continue
    rec["result"] = result
    del rec["stdout"]
    return rec
