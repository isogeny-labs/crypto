"""Native (Rust) solver arms for the Semaev solver shootout: inherited-f4 and trace-f4.

Run from the repository root. The Rust side is the example binary
``examples/gc_native_solve.rs``, located through GC_NATIVE_BINARY or
``target/release/examples/gc_native_solve``. Build it with
``groebner_compare/build_native.sh OUT_DIR``: on 2026-11-17 the library at HEAD
does not compile (sparse-merge damage), so the script builds the driver on the
last recorded working engine commit (eb01341d3a) plus ``f4_trace.rs`` from HEAD
and writes ``OUT_DIR/build_identity.json`` (commits, file and binary SHA-256,
build wall time). It takes the instance exactly as
``groebner_compare.semaev`` writes it (``nvars`` unknowns, equations as lists
of monomial bitmasks over GF(2)[x]/(x_i^2+x_i), bit i = x_i), so the solver
sees the same variables and equations as every other arm; nothing is
re-encoded.

Arms
----
``inherited-f4``  the Koblitz decomposition oracle's engine
                  (``ic_framework::solvers::HybridF4``, kind InheritedF4: boolean
                  Macaulay matrices to degree 3, propagation, splitting, each
                  child specialising its parent's reduced basis), the incumbent
                  of research/m4gb_descent_ladder_20261110. Defaults unchanged
                  (max_degree 3, split auto, node_budget 4096). A run that hits
                  the node budget is ``unknown``, never "no root".
``trace-f4``      Joux-Vitse record/replay of the dense degree-D Macaulay step
                  over GF(2) (``koblitz_groebner::f4_trace``): ``learn`` records
                  the surviving rows on training systems of one cell and merges
                  them; ``solve(..., arm="trace-f4", trace=...)`` replays them
                  (audit 16), falls back to the full kernel on refusal (both
                  charged), and reads the roots off the affine part of the
                  reduced Macaulay basis. Not a full Groebner basis and not
                  Groebner.jl's learn/apply; see
                  problems/gf2-fixed-schedule-f4-unavailable.md. The learning
                  cost is returned by ``learn`` and must be charged to the arm
                  (``charge_learning``).

Result dict (mirrors external_worker's measured-child conventions): status
(``ok`` decided, ``unknown`` undecided/budget, ``timeout`` killed at
timeout_s, ``error``, ``unavailable``), roots (sorted list of 0/1 tuples,
x0 first, or None when undecided; every root is checked by the Rust side
against the input equations and again here), wall_s / cpu_s / peak_rss_bytes
of the solver child process (sampled /proc VmHWM, as run_measured),
degree (highest Macaulay degree built), max_matrix ([rows, cols] where the
engine exposes it, else None), notes (engine telemetry, reasons, identity).

Correctness controls only in the tests; nothing here is performance evidence.
"""

import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import tempfile
import time

from .certificate import terms, vanishes
from .external_worker import vmhwm

ROOT = Path(__file__).resolve().parent.parent
DEFAULT_BINARY = ROOT / "target" / "release" / "examples" / "gc_native_solve"
ARMS = ("inherited-f4", "trace-f4")


def locate():
    path = os.environ.get("GC_NATIVE_BINARY") or str(DEFAULT_BINARY)
    return path if Path(path).is_file() else None


def binary_identity(path):
    return {"path": str(path), "sha256": hashlib.sha256(Path(path).read_bytes()).hexdigest()}


def run_child(binary, request, timeout_s, poll=0.002):
    """Run the native binary on one JSON request with a hard wall timeout.

    Returns (answer dict or None, wall s, cpu s, peak rss bytes, timed_out, stderr).
    Peak RSS is the largest sampled VmHWM of the exec'd child (None for runs
    shorter than one poll); cpu is ru_utime + ru_stime from wait4.
    """
    with tempfile.TemporaryDirectory(prefix="gc-native-") as directory:
        directory = Path(directory)
        (directory / "in.json").write_text(json.dumps(request, separators=(",", ":")))
        begin = time.perf_counter()
        peak, timed_out = None, False
        with open(directory / "in.json", "rb") as inp, open(directory / "out", "wb") as out, \
                open(directory / "err", "wb") as err:
            process = subprocess.Popen([binary], stdin=inp, stdout=out, stderr=err)
            while True:
                pid, status, usage = os.wait4(process.pid, os.WNOHANG)
                if pid:
                    break
                sample = vmhwm(process.pid)
                if sample is not None:
                    peak = max(peak or 0, sample)
                if timeout_s is not None and time.perf_counter() - begin > timeout_s:
                    timed_out = True
                    process.send_signal(signal.SIGKILL)
                    _, status, usage = os.wait4(process.pid, 0)
                    break
                time.sleep(poll)
        wall = time.perf_counter() - begin
        cpu = usage.ru_utime + usage.ru_stime
        text = (directory / "out").read_text(errors="replace").strip()
        stderr = (directory / "err").read_text(errors="replace")
        answer = None
        if text and not timed_out:
            try:
                answer = json.loads(text.splitlines()[-1])
            except ValueError:
                answer = None
        code = os.waitstatus_to_exitcode(status)
        process.returncode = code  # reaped by wait4; tell Popen
        if answer is not None:
            answer.setdefault("exit_code", code)
        return answer, wall, cpu, peak, timed_out, stderr


def to_tuple(mask, nvars):
    return tuple(mask >> i & 1 for i in range(nvars))


def from_tuple(bits):
    return sum(1 << i for i, b in enumerate(bits) if b)


def _result(status, *, roots=None, wall=None, cpu=None, rss=None, degree=None,
            max_matrix=None, notes=None):
    return {"status": status, "roots": roots, "wall_s": wall, "cpu_s": cpu,
            "peak_rss_bytes": rss, "degree": degree, "max_matrix": max_matrix,
            "notes": notes or {}}


def _checked_roots(answer, equations, nvars, notes):
    """Roots as 0/1 tuples, each replayed on the input equations here too."""
    masks = answer.get("roots")
    if masks is None:
        return None
    rejected = [m for m in masks if not vanishes(equations, m)]
    if rejected:
        notes["rejected_roots"] = rejected
        raise ValueError(f"native solver returned {len(rejected)} non-roots")
    return sorted(to_tuple(m, nvars) for m in masks)


def _instance_system(instance, nvars=None):
    n = nvars if nvars is not None else instance.get("nvars") or \
        instance["semaev"]["layout"]["nvars"]
    equations = terms(instance["equations"], n)
    return n, equations


def solve(instance, timeout_s, *, arm="inherited-f4", trace=None, params=None,
          max_free=20, audit=16, nvars=None):
    """Solve one groebner_compare.semaev instance dict on a native arm.

    ``trace`` (from ``learn``) is required for arm ``trace-f4``. The learning
    cost is NOT in the returned wall/cpu; add it with ``charge_learning``.
    """
    if arm not in ARMS:
        return _result("unavailable", notes={"reason": f"unknown native arm {arm!r}"})
    binary = locate()
    if binary is None:
        return _result("unavailable", notes={
            "reason": "gc_native_solve not built: cargo build --release --example "
                      "gc_native_solve (or set GC_NATIVE_BINARY)"})
    n, equations = _instance_system(instance, nvars)
    if arm == "inherited-f4":
        request = {"op": "solve", "engine": "inherited-f4", "nvars": n,
                   "equations": equations, "params": params or {}}
    else:
        if trace is None or trace.get("status") != "ok":
            return _result("unavailable", notes={
                "reason": "trace-f4 needs a learned trace (learn() status ok)"})
        request = {"op": "replay", "nvars": n, "equations": equations,
                   "trace": trace["trace"], "max_free": max_free, "audit": audit,
                   "fallback": True}
    answer, wall, cpu, rss, timed_out, stderr = run_child(binary, request, timeout_s)
    notes = {"arm": arm, "binary": binary_identity(binary), "instance_id": instance.get("id"),
             "nvars": n}
    if timed_out:
        notes["reason"] = f"killed at timeout {timeout_s} s"
        return _result("timeout", wall=wall, cpu=cpu, rss=rss, notes=notes)
    if answer is None:
        notes["reason"] = "no JSON answer from the native solver"
        notes["stderr_tail"] = stderr[-2000:]
        return _result("error", wall=wall, cpu=cpu, rss=rss, notes=notes)
    status = answer.pop("status", "error")
    roots = None
    try:
        roots = _checked_roots(answer, equations, n, notes) if status == "ok" else None
    except ValueError as error:
        notes["reason"] = str(error)
        return _result("error", wall=wall, cpu=cpu, rss=rss, notes=notes)
    answer.pop("roots", None)
    notes.update({k: v for k, v in answer.items()})
    degree = answer.get("degree_reached")
    max_matrix = None
    if arm == "trace-f4":
        max_matrix = [answer.get("matrix_rows"), answer.get("matrix_cols_replayed")]
    else:
        notes["max_matrix_note"] = ("inherited-f4 exposes no matrix dimensions "
                                    "(SolveStats has none); max_matrix is null")
    return _result(status, roots=roots, wall=wall, cpu=cpu, rss=rss, degree=degree,
                   max_matrix=max_matrix, notes=notes)


def learn(training_instances, degree, timeout_s, *, nvars=None):
    """Record and merge the degree-``degree`` Macaulay trace on training systems.

    The training systems must be disjoint from the measured targets. The
    returned dict carries the trace and its cost (child wall/cpu/rss, kernel
    and recording word XORs) so it can be charged to the arm.
    """
    binary = locate()
    if binary is None:
        return {"status": "unavailable", "reason": "gc_native_solve not built"}
    systems, n = [], None
    for instance in training_instances:
        k, equations = _instance_system(instance, nvars)
        if n is not None and k != n:
            return {"status": "error", "reason": "training systems differ in nvars"}
        n = k
        systems.append(equations)
    request = {"op": "learn", "nvars": n, "degree": degree, "systems": systems}
    answer, wall, cpu, rss, timed_out, stderr = run_child(binary, request, timeout_s)
    if timed_out:
        return {"status": "timeout", "learn_wall_s": wall, "learn_cpu_s": cpu}
    if answer is None:
        return {"status": "error", "reason": "no JSON answer", "stderr_tail": stderr[-2000:]}
    answer.update({"learn_wall_s": wall, "learn_cpu_s": cpu, "learn_peak_rss_bytes": rss,
                   "nvars": n, "degree": degree,
                   "training_ids": [i.get("id") for i in training_instances],
                   "binary": binary_identity(binary)})
    return answer


def charge_learning(results, trace):
    """Amortise the learning wall/cpu evenly over the per-target results of one cell.

    Returns new dicts with wall_s/cpu_s increased by learn_*/len(results) and
    the share recorded in notes; the raw per-call figures are kept in notes.
    """
    if not results:
        return []
    share_wall = (trace.get("learn_wall_s") or 0.0) / len(results)
    share_cpu = (trace.get("learn_cpu_s") or 0.0) / len(results)
    out = []
    for result in results:
        charged = dict(result)
        notes = dict(charged.get("notes") or {})
        notes.update({"call_wall_s": result["wall_s"], "call_cpu_s": result["cpu_s"],
                      "learn_share_wall_s": share_wall, "learn_share_cpu_s": share_cpu,
                      "learn_amortised_over": len(results)})
        if charged["wall_s"] is not None:
            charged["wall_s"] = result["wall_s"] + share_wall
        if charged["cpu_s"] is not None:
            charged["cpu_s"] = result["cpu_s"] + share_cpu
        charged["notes"] = notes
        out.append(charged)
    return out


def main():
    """python -m groebner_compare.native_worker ARM < instance.json (one instance)."""
    import sys
    arm = sys.argv[1] if len(sys.argv) > 1 else "inherited-f4"
    timeout = float(sys.argv[2]) if len(sys.argv) > 2 else 300.0
    instance = json.load(sys.stdin)
    print(json.dumps(solve(instance, timeout, arm=arm), default=list))


if __name__ == "__main__":
    main()
