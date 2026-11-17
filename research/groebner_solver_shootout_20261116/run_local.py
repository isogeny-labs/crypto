#!/usr/bin/env python3
"""Local driver for GB-SOLVER-SHOOTOUT-20261116, amendment-1 (PA-4).

LOCAL DIAGNOSTIC, NON-ADMITTED, NEVER A LEADERBOARD OR benchmarks/README ROW.
Owner decision 2026-11-17: AWS does not work and Magma is not available, so
this runs in the current container (isolation L0) with 3 single-threaded
workers. Every figure it writes is [pcc:unverified].

The protocol (protocol.amend1.json, canonical sha256 c521e04c...e858) names the
driver ``run_amend1.py``; this file is that driver under the name the main
session asked for (``run_local.py``). Amendment-2 must pin it by hash (PA-4).

What it implements, section by section of amendment-1:

* Cell K17-m3-plain-l5-a1: K1 y^2+xy=x^3+x^2+1 over GF(2^17), modulus
  z^17+z^3+1 (131081), m=3 plain chained-S3, l=5, 32 Boolean variables.
* Seed rule (M4): seed(q, i) = first 8 bytes big-endian of
  SHA-256('<v1 canonical hash>:amend1:K17-m3-plain-l5-a1:' + q + ':' + i).
  ``--pilot`` replaces the domain by ':amend1-pilot:' so a pilot never
  consumes a main-run seed.
* fb-enum oracle (P2/C1): every 3-multiset with repetition of the 14 nonzero
  liftable factor-base abscissae, every sign pattern, by group addition;
  table x_R -> set of decomposing multisets; asserts 1757/65572.
* Natural targets: SHA-256 counter stream keyed by seed('natural', i), x mod
  2^17, rejected unless x >= 2^5 and liftable; sign bit from the same stream;
  labelled by the oracle into SAT / UNSAT strata until each holds N.
  Planted: semaev.generate(3,17,5,'plain',a=1,b=1,seed=seed('planted',i)).
  Warm-up: natural draw keyed seed('warmup', 0), discarded.
* Arms: msolve-f4 (-g 2 -v 2 -t 1 + certificate.basis_roots extraction),
  cms-xor (--threads 1 --random 0 --verb 1 --maxsol 1000000, full enumeration),
  inherited-f4 (examples/gc_native_solve, op solve, engine defaults). Pass B
  of each is its A/A arm. trace-f4, julia-f4-warm, magma-f4 and
  repo-boolean-f4 are UNAVAILABLE/dropped per the amendment and never run.
* Williams order (M2): all 6 sequences of the 3 pipelines, assigned by a
  hash-shuffled round robin (seed('order', k)); pass B runs the reversed
  sequence immediately after pass A on the same worker.
* Caps (H3): RLIMIT_CPU 60 s (hard 62) and 4 GiB on every solver child;
  cgroup v2 memory.max is used when writable, else RLIMIT_AS (recorded).
  SIGXCPU / SIGKILL / CPU >= cap -> censored_cpu; allocation failure ->
  censored_mem; extraction overflow -> censored_extraction; inherited-f4 node
  budget -> censored_budget; CMS stopping at --maxsol -> censored_maxsol.
  Censored calls are charged at the cap.
* Projection (C1): each Boolean root -> abscissa multiset of its X blocks;
  a multiset containing x = 0 is counted separately (zero_abscissa_roots);
  otherwise verified by group re-addition (semaev.readdition), memoised per
  (call, multiset). Verified relation set compared with the oracle set.
  M1: on oracle-SAT targets, whether each oracle multiset is a root of the
  encoded system is computed and recorded.
* Tamper (negative control): 2 per arm, one bit of a returned planted root
  flipped at index seed('tamper:<arm>:<k>') mod nvars.
* Matched rho (PA-2): examples/gc_rho_match.rs, 1000 DLPs d = 1 + seed('rho',
  i) mod (r-1), walk seed = seed('rho', i); t_GA best of 7 x 10^6 FastCurve
  additions; modulus checked against 131081.
* Startup baselines (M6): 21 empty-input runs per tool at start, middle, end.
* Cost guard and wall ceiling (stopping rules): after the planted phase, if
  the projected total wall exceeds 100 min, N per stratum is reduced
  symmetrically to the largest N >= 40 whose projection fits; whole-run
  ceiling 105 min (no new target starts after it).
* Rows: rows.jsonl, one experiment.export/v1 ``runs``-shaped object per call
  with study fields under ``study``; arm labels masked (arm-NN) with the key in
  blinding_key.json (sha256 recorded); cells.json (oracle, rho, t_GA, startup,
  host, identities); manifest.json (sha256 of every output).

Usage (from the repository root):
  python3 research/groebner_solver_shootout_20261116/run_local.py pilot --out DIR
  python3 research/groebner_solver_shootout_20261116/run_local.py run --out DIR   # main run
"""

import argparse
import concurrent.futures as cf
import hashlib
import itertools
import json
import os
from pathlib import Path
import platform
import re
import resource
import secrets
import shutil
import signal
import subprocess
import sys
import tempfile
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
sys.path.insert(0, str(ROOT))

from groebner_compare import semaev  # noqa: E402
from groebner_compare.certificate import basis_roots, terms, vanishes  # noqa: E402
from groebner_compare.external_worker import (  # noqa: E402
    parse_cms, parse_msolve_basis, parse_msolve_log, vmhwm)

# ----------------------------------------------------------------- constants

V1_CANONICAL = "116b33f754becbd9d7e76cbd80bfff8e323cb79ab0ee0c57481c0616d3d9bfee"
AMEND1_CANONICAL = "c521e04c643ff8b3028426b1d9c8b85a3f92fabc5e6e91e124498c13524fe858"
CELL = "K17-m3-plain-l5-a1"
MAIN_DOMAIN = f"{V1_CANONICAL}:amend1:{CELL}:"
PILOT_DOMAIN = f"{V1_CANONICAL}:amend1-pilot:{CELL}:"
M, N_FIELD, L, A, B = 3, 17, 5, 1, 1
MODULUS = 131081
P_SAT_EXPECTED = (1757, 65572)
FB_SIZE = 14
CPU_CAP_S = 60
CPU_HARD_S = 62
MEM_CAP_BYTES = 4 << 30
WALL_WATCHDOG_S = 240
WORKERS = 3
N_PER_STRATUM = 80
N_MIN_PER_STRATUM = 40
N_PLANTED = 5
N_RHO = 1000
COST_GUARD_S = 100 * 60
WALL_CEILING_S = 105 * 60
STARTUP_RUNS = 21
CMS_MAXSOL = 1_000_000
CMS_RANDOM = 0
MSOLVE_FLAGS = ["-g", "2", "-v", "2", "-t", "1"]
PIPELINES = ("msolve-f4", "cms-xor", "inherited-f4")
AA_OF = {"msolve-f4": "msolve-f4-aa", "cms-xor": "cms-xor-aa",
         "inherited-f4": "inherited-f4-aa"}
WILLIAMS = list(itertools.permutations(PIPELINES))  # all 6 sequences (M2)
UNAVAILABLE = {
    "magma-f4": "UNAVAILABLE: Magma not available to the owner (2026-11-17)",
    "trace-f4": "UNAVAILABLE: Groebner.jl learn/apply needs Julia (absent; download host "
                "proxy-blocked); the native GF(2) replay returns unknown on every grid target "
                "(problems/gf2-fixed-schedule-f4-unavailable.md); removed from the arm list by "
                "amendment-1",
    "julia-f4-warm": "UNAVAILABLE: Julia absent, download host proxy-blocked",
    "repo-boolean-f4": "dropped by amendment-1 (red-team H1)",
    "msolve-0.11-simd": "not provisioned in this container (amendment-1 C-out)",
}
HARNESS_PINS = {
    "groebner_compare/semaev.py": "d71ab7c999ad5128635032a74ea85ce023ea2e56d78ffcf3e97582b4aae9756b",
    "groebner_compare/external_worker.py": "556267555fcb075b8ab050672403700ba6ce653d35214c318b9671f4e2b83e75",
    "groebner_compare/semaev_verifier.py": "e2d137a6af26a7df185fbc795e01e4958e31a36646e8261a3a457748ec795521",
    "groebner_compare/runner.py": "9dd66b115830dfeca3cd23e1024c48d6e849f9096b469d7e148acbf6d89aa299",
    "groebner_compare/certificate.py": "27a41eee36da407b596f9da0642dbe9e09e8ae87dbb7361280127eb62760c862",
}
BINARY_PINS = {
    "msolve": "c2722288d22c3a1eab00c0b818d0204d4a98104d059b2fd4ecb7343e8ef759fa",
    "cryptominisat5": "00db7e59c634842acbfc6676bf0f5d30a3fdabbd0ec2fa17b20bb0bf683c7207",
}
LABEL = "LOCAL DIAGNOSTIC, non-admitted, [pcc:unverified]; diagnostic tier (local, non-admitted)"


# ----------------------------------------------------------------- utilities

def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def canonical_json(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), allow_nan=False)


def seed(domain, q, i):
    """Amendment-1 seed rule (M4)."""
    raw = hashlib.sha256(f"{domain}{q}:{i}".encode()).digest()
    return int.from_bytes(raw[:8], "big")


class CounterStream:
    """SHA-256 counter stream keyed by a 64-bit seed."""

    def __init__(self, key):
        self.key = int(key).to_bytes(8, "big")
        self.counter = 0

    def word(self):
        self.counter += 1
        raw = hashlib.sha256(self.key + self.counter.to_bytes(8, "big")).digest()
        return int.from_bytes(raw, "big")


def now_utc():
    return time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())


def host_load():
    out = {}
    try:
        parts = Path("/proc/loadavg").read_text().split()
        out["loadavg"] = [float(x) for x in parts[:3]]
        out["procs_running_total"] = parts[3]
    except OSError:
        pass
    try:
        out["psi_cpu_some_avg10"] = float(
            Path("/proc/pressure/cpu").read_text().split()[1].split("=")[1])
    except (OSError, IndexError, ValueError):
        out["psi_cpu_some_avg10"] = None
    return out


# ----------------------------------------------------------------- the cell

_CELL = None


def cell():
    global _CELL
    if _CELL is None:
        field, curve = semaev.setup(N_FIELD, A, B, MODULUS)
        _CELL = (field, curve)
    return _CELL


def factor_base():
    _, curve = cell()
    return [x for x in range(1, 1 << L) if curve.lift(x)]


def build_oracle():
    """fb-enum oracle: x_R -> set of decomposing multisets (sorted tuples)."""
    field, curve = cell()
    t0 = time.process_time()
    fb = factor_base()
    if len(fb) != FB_SIZE:
        raise AssertionError(f"factor base has {len(fb)} abscissae, expected {FB_SIZE}")
    table = {}
    additions = 0
    for ms in itertools.combinations_with_replacement(fb, M):
        for choice in itertools.product(*[curve.lift(x) for x in ms]):
            total = None
            for point in choice:
                total = curve.add(total, point)
                additions += 1
            if total is None:
                continue
            table.setdefault(total[0], set()).add(ms)
    build_cpu = time.process_time() - t0
    # Stratum weights over liftable abscissae outside V (trace test).
    t1 = time.process_time()
    liftable = 0
    sat = 0
    decomps = 0
    for x in range(1 << L, 1 << N_FIELD):
        c = field.mul(field.sq(x), x) ^ field.mul(A, field.sq(x)) ^ B
        if field.trace(field.mul(c, field.inv(field.sq(x)))) == 0:
            liftable += 1
            if x in table:
                sat += 1
                decomps += len(table[x])
    weight_cpu = time.process_time() - t1
    if (sat, liftable) != P_SAT_EXPECTED:
        raise AssertionError(f"oracle weights {sat}/{liftable} != {P_SAT_EXPECTED}")
    return {
        "table": table, "factor_base": fb, "group_additions": additions,
        "build_cpu_s": build_cpu, "weight_cpu_s": weight_cpu,
        "liftable_outside_V": liftable, "sat_abscissae": sat,
        "p_sat": sat / liftable, "mean_decompositions_given_sat": decomps / sat,
        "table_entries": len(table),
        "relations_in_table": sum(len(v) for v in table.values()),
    }


def build_instance(xr, tid):
    """Plain chained-S3 system for target abscissa xr (same code path as semaev.generate)."""
    field, _ = cell()
    shape = semaev.layout("plain", M, N_FIELD, L)
    sym = semaev.Symbolic(field)
    X = [sym.variables(o, L) for o in shape["x"]]
    U = [sym.variables(o, N_FIELD) for o in shape["u"]]
    chain = X[:1] + U
    rights = X[1:]
    outs = U + [sym.const(xr)]
    equations = []
    for left, right, out in zip(chain, rights, outs):
        equations.extend(sym.s3(left, right, out, B))
    rows = [sorted(p) for p in equations]
    meta = {"generator": semaev.GENERATOR, "variant": "plain", "m": M, "n": N_FIELD,
            "l": L, "a": A, "b": B, "modulus": field.modulus, "x_R": xr, "layout": shape,
            "equations_sha256": hashlib.sha256(json.dumps(rows, separators=(",", ":"))
                                               .encode()).hexdigest()}
    return {"id": tid, "equations": rows, "semaev": meta}


def natural_draw(domain, q, i):
    """One natural target from seed(q, i): (x_R, sign, draws used)."""
    _, curve = cell()
    stream = CounterStream(seed(domain, q, i))
    draws = 0
    while True:
        draws += 1
        x = stream.word() % (1 << N_FIELD)
        if x < (1 << L):
            continue
        lifts = curve.lift(x)
        if not lifts:
            continue
        sign = stream.word() & 1
        return x, sign, draws


def natural_targets(domain, oracle, n_sat, n_unsat, max_draws=200000):
    sat, unsat, discarded = [], [], []
    i = 0
    while len(sat) < n_sat or len(unsat) < n_unsat:
        if i >= max_draws:
            raise RuntimeError("natural stream exhausted")
        x, sign, draws = natural_draw(domain, "natural", i)
        label = "sat" if x in oracle["table"] else "unsat"
        rec = {"stream_index": i, "x_R": x, "sign": sign, "stratum": label,
               "seed": seed(domain, "natural", i), "rejections": draws - 1}
        bucket, cap = (sat, n_sat) if label == "sat" else (unsat, n_unsat)
        if len(bucket) < cap:
            rec["target_id"] = f"nat-{label}-{len(bucket):03d}"
            bucket.append(rec)
        else:
            discarded.append({"stream_index": i, "stratum": label})
        i += 1
    return sat, unsat, discarded, i


def planted_targets(domain, count):
    out = []
    for i in range(count):
        s = seed(domain, "planted", i)
        instance, answer = semaev.generate(M, N_FIELD, L, variant="plain", a=A, b=B,
                                           seed=s, index=0, modulus=MODULUS)
        out.append({"target_id": f"planted-{i}", "stratum": "planted", "x_R": answer["target"][0],
                    "sign": None, "seed": s, "planted_multiset": sorted(answer["abscissae"]),
                    "planted_assignment": answer["assignment"],
                    "equations_sha256": instance["semaev"]["equations_sha256"]})
    return out


def oracle_roots_check(target, oracle):
    """M1: for each oracle multiset, is some ordering/sign assignment a root of the encoding?"""
    field, curve = cell()
    instance = build_instance(target["x_R"], target["target_id"])
    eqs = terms(instance["equations"], instance["semaev"]["layout"]["nvars"])
    shape = instance["semaev"]["layout"]
    out = {}
    for ms in sorted(oracle["table"].get(target["x_R"], ())):
        found = False
        for perm in set(itertools.permutations(ms)):
            for choice in itertools.product(*[curve.lift(x) for x in perm]):
                partial = curve.add(choice[0], choice[1])
                if partial is None:
                    continue
                total = curve.add(partial, choice[2])
                if total is None or total[0] != target["x_R"]:
                    continue
                assignment = 0
                for o, x in zip(shape["x"], perm):
                    assignment |= x << o
                assignment |= partial[0] << shape["u"][0]
                if vanishes(eqs, assignment):
                    found = True
                    break
            if found:
                break
        out[",".join(map(str, ms))] = found
    return out


# ----------------------------------------------------------------- projection

def project_roots(instance, roots):
    """C1 projection of Boolean roots (int masks) to verified abscissa multisets."""
    meta = instance["semaev"]
    shape = meta["layout"]
    t0 = time.process_time()
    memo = {}
    verified, zero, spurious = set(), 0, 0
    per_multiset = {}
    for mask in roots:
        xs = [semaev.field_value(mask, o, L) for o in shape["x"]]
        ms = tuple(sorted(xs))
        if 0 in ms:
            zero += 1
            continue
        if ms not in memo:
            # semaev.readdition semantics on a canonical assignment of this multiset.
            assignment = 0
            for o, x in zip(shape["x"], ms):
                assignment |= x << o
            memo[ms] = semaev.readdition(meta, assignment) is not None
        if memo[ms]:
            verified.add(ms)
            key = ",".join(map(str, ms))
            per_multiset[key] = per_multiset.get(key, 0) + 1
        else:
            spurious += 1
    return {"verified": sorted(verified), "zero_abscissa_roots": zero,
            "spurious_roots": spurious, "roots_per_multiset": per_multiset,
            "readditions": len(memo), "projection_cpu_s": time.process_time() - t0}


# ----------------------------------------------------------------- capped child

def memory_cap_method():
    """cgroup v2 memory.max if a per-call cgroup is creatable, else RLIMIT_AS."""
    base = Path("/sys/fs/cgroup")
    if (base / "cgroup.controllers").is_file():
        try:
            probe = base / f"gc-probe-{os.getpid()}"
            probe.mkdir()
            ok = (probe / "memory.max").exists()
            probe.rmdir()
            if ok:
                return "cgroup-v2 memory.max"
        except OSError:
            pass
    return "RLIMIT_AS"


def _limits():
    resource.setrlimit(resource.RLIMIT_CPU, (CPU_CAP_S, CPU_HARD_S))
    resource.setrlimit(resource.RLIMIT_AS, (MEM_CAP_BYTES, MEM_CAP_BYTES))
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))


def run_capped(argv, stdout_path, stderr_path, stdin_path=None, poll=0.002):
    """Run one solver child under the per-call caps; rusage from wait4 of the child."""
    begin = time.perf_counter()
    peak = None
    watchdog = False
    with open(stdout_path, "wb") as out, open(stderr_path, "wb") as err, \
            (open(stdin_path, "rb") if stdin_path else open(os.devnull, "rb")) as inp:
        process = subprocess.Popen(argv, stdin=inp, stdout=out, stderr=err,
                                   preexec_fn=_limits)
        while True:
            pid, status, usage = os.wait4(process.pid, os.WNOHANG)
            if pid:
                break
            sample = vmhwm(process.pid)
            if sample is not None:
                peak = max(peak or 0, sample)
            if time.perf_counter() - begin > WALL_WATCHDOG_S:
                watchdog = True
                process.send_signal(signal.SIGKILL)
                _, status, usage = os.wait4(process.pid, 0)
                break
            time.sleep(poll)
        process.returncode = os.waitstatus_to_exitcode(status)
    wall = time.perf_counter() - begin
    signalled = os.WIFSIGNALED(status)
    return {"exit_code": process.returncode,
            "signal": os.WTERMSIG(status) if signalled else None,
            "wall_s": wall, "user_s": usage.ru_utime, "sys_s": usage.ru_stime,
            "cpu_s": usage.ru_utime + usage.ru_stime,
            "peak_rss_bytes": peak,
            "rss_source": "sampled /proc/PID/status VmHWM of the exec'd solver (2 ms)",
            "ru_maxrss_upper_bound_bytes": usage.ru_maxrss * 1024,
            "wall_watchdog_killed": watchdog}


def classify_kill(child, stderr_text):
    sig = child["signal"]
    if sig in (signal.SIGXCPU, signal.SIGKILL) or child["cpu_s"] >= CPU_CAP_S:
        if child["wall_watchdog_killed"] and child["cpu_s"] < CPU_CAP_S:
            return "censored_wall_watchdog"
        return "censored_cpu"
    low = stderr_text.lower()
    if any(k in low for k in ("alloc", "out of memory", "cannot allocate", "bad_alloc",
                              "memory allocation", "memoryerror")):
        return "censored_mem"
    if sig in (signal.SIGSEGV, signal.SIGABRT, signal.SIGBUS):
        # Under RLIMIT_AS an unchecked failed allocation can surface as a crash.
        if child["peak_rss_bytes"] and child["peak_rss_bytes"] > MEM_CAP_BYTES // 2:
            return "censored_mem"
    return None


# ----------------------------------------------------------------- arms

def tools():
    return {
        "msolve": os.environ.get("MSOLVE_BINARY") or shutil.which("msolve"),
        "cryptominisat5": os.environ.get("CRYPTOMINISAT_BINARY") or shutil.which("cryptominisat5"),
        "gc_native_solve": os.environ.get("GC_NATIVE_BINARY")
        or str(ROOT / "target/release/examples/gc_native_solve"),
        "gc_rho_match": os.environ.get("GC_RHO_BINARY")
        or str(ROOT / "target/release/examples/gc_rho_match"),
    }


INTERNAL_CPU = re.compile(r"overall\(cpu\)\s+([\d.]+)\s*sec")
CMS_TOTAL = re.compile(r"^c Total time \(this thread\)\s*:\s*([\d.]+)", re.M)
CMS_PROPS = re.compile(r"^c propagations\s*:\s*([\d.]+)([KMG]?)", re.M)
CMS_CONFLICTS = re.compile(r"^c conflicts\s*:\s*(\d+)", re.M)
CMS_DECISIONS = re.compile(r"^c decisions\s*:\s*(\d+)", re.M)
CMS_GOOD = re.compile(r"\[matrix\] Good\s+matrix\s+\d+\s+(\d+)\s*x\s*(\d+)")
CMS_USING = re.compile(r"\[matrix\] Using (\d+) matrices")
SUFFIX = {"": 1, "K": 10 ** 3, "M": 10 ** 6, "G": 10 ** 9}


def msolve_argv(binary, source, output):
    return [binary, *MSOLVE_FLAGS, "-f", str(source), "-o", str(output)]


def cms_argv(binary, cnf):
    return [binary, "--verb", "1", "--threads", "1", "--random", str(CMS_RANDOM),
            "--maxsol", str(CMS_MAXSOL), str(cnf)]


def arm_msolve(instance, binary, workdir):
    nvars = instance["semaev"]["layout"]["nvars"]
    eqs = terms(instance["equations"], nvars)
    source, output = workdir / "in.ms", workdir / "out.gb"
    source.write_text(semaev.to_msolve(eqs, nvars))
    child = run_capped(msolve_argv(binary, source, output), workdir / "log", workdir / "err")
    log = (workdir / "log").read_text(errors="replace")
    err = (workdir / "err").read_text(errors="replace")
    tele = parse_msolve_log(log)
    m = INTERNAL_CPU.search(log)
    res = {"child": child, "internal_cpu_s": float(m.group(1)) if m else None,
           "degree": tele["highest_degree"],
           "max_matrix": ([tele["largest_matrix_rows"], tele["largest_matrix_columns"]]
                          if tele["largest_matrix_rows"] is not None else None),
           "native_ops": sum(r * c for _, r, c in tele["f4_round_table"]),
           "native_unit": "F4 matrix cells (sum rows*cols over -v 2 rounds)",
           "telemetry": {k: tele[k] for k in ("f4_rounds", "f4_round_table",
                                              "final_reduction_matrix")},
           "extract_cpu_s": 0.0, "roots": None}
    killed = classify_kill(child, err) if (child["signal"] or child["exit_code"]) else None
    if child["cpu_s"] >= CPU_CAP_S and not killed:
        killed = "censored_cpu"
    if killed:
        res["outcome"] = killed
        res["degree_semantics"] = "lower bound (log parsed after kill)"
        return res
    if child["exit_code"] != 0 or not output.is_file():
        res["outcome"] = "error"
        res["reason"] = f"msolve exit {child['exit_code']}; stderr tail {err[-400:]!r}"
        return res
    t0 = time.process_time()
    try:
        basis = parse_msolve_basis(output.read_text(), nvars)
        roots, info = basis_roots(nvars, basis)
        res["roots"] = roots
        res["extraction"] = info
        res["outcome"] = "ok"
        res["basis_polys"] = len(basis)
        bad = [r for r in roots if not vanishes(eqs, r)]
        if bad:
            res["outcome"] = "error"
            res["reason"] = f"{len(bad)} extracted roots do not vanish on the input"
    except TimeoutError as error:
        res["outcome"] = "censored_extraction"
        res["reason"] = str(error)
    res["extract_cpu_s"] = time.process_time() - t0
    return res


def arm_cms(instance, binary, workdir):
    nvars = instance["semaev"]["layout"]["nvars"]
    eqs = terms(instance["equations"], nvars)
    text, shape = semaev.to_dimacs_xor(eqs, nvars)
    (workdir / "in.cnf").write_text(text)
    child = run_capped(cms_argv(binary, workdir / "in.cnf"), workdir / "out", workdir / "err")
    out = (workdir / "out").read_text(errors="replace")
    err = (workdir / "err").read_text(errors="replace")
    solutions, verdicts, _ = parse_cms(out, nvars)
    total = CMS_TOTAL.search(out)
    props = CMS_PROPS.search(out)
    conflicts = CMS_CONFLICTS.search(out)
    decisions = CMS_DECISIONS.search(out)
    good = [(int(r), int(c)) for r, c in CMS_GOOD.findall(out)]
    using = [int(x) for x in CMS_USING.findall(out)]
    res = {"child": child, "internal_cpu_s": float(total.group(1)) if total else None,
           "degree": None,
           "max_matrix": list(max(good, key=lambda rc: (rc[0] * rc[1], rc))) if good else None,
           "native_ops": (int(float(props.group(1)) * SUFFIX[props.group(2)]) if props else None),
           "native_unit": "propagations (CMS --verb 1 final statistics; printed with K/M/G "
                          "suffix, so resolution is that of the printed figure)",
           "telemetry": {"propagations_raw": (props.group(0).split(":", 1)[1].strip()
                                              if props else None),
                         "conflicts": int(conflicts.group(1)) if conflicts else None,
                         "decisions": int(decisions.group(1)) if decisions else None,
                         "gauss_good_matrices": good, "gauss_using_matrices": using,
                         "gauss_zero_matrices_flag": bool(using) and max(using) == 0,
                         "models": len(solutions), "final_verdict": verdicts[-1] if verdicts else None,
                         **shape},
           "extract_cpu_s": 0.0, "roots": None}
    killed = classify_kill(child, err) if child["signal"] else None
    if child["cpu_s"] >= CPU_CAP_S and not killed:
        killed = "censored_cpu"
    if killed:
        res["outcome"] = killed
        return res
    if child["exit_code"] not in (10, 20):
        res["outcome"] = "error"
        res["reason"] = f"cryptominisat5 exit {child['exit_code']}"
        return res
    if not (verdicts and verdicts[-1] == "UNSATISFIABLE") or len(solutions) >= CMS_MAXSOL:
        res["outcome"] = "censored_maxsol"
        res["roots"] = solutions
        return res
    bad = [r for r in solutions if not vanishes(eqs, r)]
    res["roots"] = solutions
    res["outcome"] = "ok" if not bad else "error"
    if bad:
        res["reason"] = f"{len(bad)} CMS models do not vanish on the input"
    return res


def arm_inherited(instance, binary, workdir):
    nvars = instance["semaev"]["layout"]["nvars"]
    eqs = terms(instance["equations"], nvars)
    request = {"op": "solve", "engine": "inherited-f4", "nvars": nvars,
               "equations": eqs, "params": {}}
    (workdir / "in.json").write_text(json.dumps(request, separators=(",", ":")))
    child = run_capped([binary], workdir / "out", workdir / "err", stdin_path=workdir / "in.json")
    text = (workdir / "out").read_text(errors="replace").strip()
    err = (workdir / "err").read_text(errors="replace")
    res = {"child": child, "internal_cpu_s": None, "degree": None, "max_matrix": None,
           "native_ops": None, "native_unit": "word XORs (SolverCost.ops)",
           "telemetry": {}, "extract_cpu_s": 0.0, "roots": None}
    killed = classify_kill(child, err) if child["signal"] else None
    if child["cpu_s"] >= CPU_CAP_S and not killed:
        killed = "censored_cpu"
    if killed:
        res["outcome"] = killed
        return res
    try:
        answer = json.loads(text.splitlines()[-1])
    except (ValueError, IndexError):
        res["outcome"] = "error"
        res["reason"] = f"no JSON answer; stderr tail {err[-400:]!r}"
        return res
    res["internal_cpu_s"] = (answer.get("solver_wall_ns") or 0) / 1e9 or None
    res["internal_timer_note"] = "adapter-internal solve() WALL ns (no internal CPU timer)"
    res["degree"] = answer.get("degree_reached")
    res["native_ops"] = answer.get("ops")
    res["telemetry"] = {"extra": answer.get("extra"), "params": answer.get("params"),
                        "finds_every_solution": answer.get("finds_every_solution"),
                        "op_unit": answer.get("op_unit"), "timed_out": answer.get("timed_out")}
    res["max_matrix_note"] = "inherited-f4 exposes no matrix dimensions; null"
    status = answer.get("status")
    if status == "unknown":
        res["outcome"] = "censored_budget"
        return res
    if status != "ok":
        res["outcome"] = "error"
        res["reason"] = answer.get("reason") or f"status {status}"
        return res
    roots = answer.get("roots") or []
    bad = [r for r in roots if not vanishes(eqs, r)]
    res["roots"] = sorted(roots)
    res["outcome"] = "ok" if not bad else "error"
    if bad:
        res["reason"] = f"{len(bad)} returned roots do not vanish on the input"
    return res


ARM_FUNCS = {"msolve-f4": ("msolve", arm_msolve), "cms-xor": ("cryptominisat5", arm_cms),
             "inherited-f4": ("gc_native_solve", arm_inherited)}


def charged_cpu(res):
    """Primary-endpoint charge: solver child CPU + extraction CPU; censored at the cap."""
    if res["outcome"].startswith("censored"):
        return float(CPU_CAP_S)
    return res["child"]["cpu_s"] + (res.get("extract_cpu_s") or 0.0)


# ----------------------------------------------------------------- one target

def run_target(job):
    """Worker entry: pass A in the Williams sequence, then pass B reversed. Returns rows."""
    target, sequence, oracle_set, m1, tool_paths, worker_tag, domain = job
    instance = build_instance(target["x_R"], target["target_id"])
    nvars = instance["semaev"]["layout"]["nvars"]
    rows = []
    for pass_name, seq in (("A", list(sequence)), ("B", list(reversed(sequence)))):
        for position, pipeline in enumerate(seq):
            tool, func = ARM_FUNCS[pipeline]
            started = now_utc()
            load_before = host_load()
            with tempfile.TemporaryDirectory(prefix=f"gc-{pipeline}-") as d:
                res = func(instance, tool_paths[tool], Path(d))
            finished = now_utc()
            roots = res.get("roots")
            proj = project_roots(instance, roots) if roots is not None else None
            verified = {tuple(v) for v in proj["verified"]} if proj else None
            oset = {tuple(v) for v in oracle_set}
            matches = (verified == oset) if (verified is not None
                                             and res["outcome"] == "ok") else None
            m1_violation = (res["outcome"] == "ok" and bool(oset) and not verified
                            and any(m1.values()))
            arm_id = pipeline if pass_name == "A" else AA_OF[pipeline]
            child = res["child"]
            row = {
                "run_id": f"{target['target_id']}:{arm_id}",
                "arm": arm_id,
                "instance_id": target["target_id"],
                "seed": target.get("seed"),
                "replicate": 0 if pass_name == "A" else 1,
                "warmup": target["stratum"] == "warmup",
                "status": "succeeded" if res["outcome"] in ("ok",) else (
                    "censored" if res["outcome"].startswith("censored") else "failed"),
                "outcome_class": res["outcome"],
                "attempts": 1, "resumed": 0, "checkpoint": None,
                "measurements": {
                    "counted": {"verified_relations": len(verified) if verified is not None else None,
                                "native_ops": res.get("native_ops"),
                                "spurious_roots": proj["spurious_roots"] if proj else None},
                    "wall_seconds": child["wall_s"], "wall_valid": False,
                    "cpu_user_seconds": child["user_s"], "cpu_sys_seconds": child["sys_s"],
                    "max_rss_bytes": child["peak_rss_bytes"], "counters": {}},
                "isolation": {"tier": "L0-local-diagnostic", "contended": True,
                              "blockers": ["owner 2026-11-17: AWS does not work; shared 4 vCPU "
                                           "VM, no PMU, no L2; non-admitted"]},
                "verification": {"status": ("verified" if matches else
                                            "mismatch" if matches is False else "not_applicable"),
                                 "verifier": "group re-addition (semaev.readdition) vs fb-enum oracle"},
                "exclusion": {"excluded": target["stratum"] == "warmup",
                              "rule": "warm-up discarded" if target["stratum"] == "warmup" else None},
                "started_at": started, "finished_at": finished, "worker": worker_tag,
                "study": {
                    "label": LABEL, "pipeline": pipeline, "pass": pass_name,
                    "position": position, "sequence": list(sequence),
                    "stratum": target["stratum"], "x_R": target["x_R"], "sign": target.get("sign"),
                    "nvars": nvars, "equations_sha256": instance["semaev"]["equations_sha256"],
                    "outcome": res["outcome"], "reason": res.get("reason"),
                    "solver_cpu_s": child["cpu_s"], "solver_user_s": child["user_s"],
                    "solver_sys_s": child["sys_s"], "solver_wall_s": child["wall_s"],
                    "exit_code": child["exit_code"], "signal": child["signal"],
                    "wall_watchdog_killed": child["wall_watchdog_killed"],
                    "extract_cpu_s": res.get("extract_cpu_s"),
                    "charged_cpu_s": charged_cpu(res),
                    "internal_cpu_s": res.get("internal_cpu_s"),
                    "internal_timer_note": res.get("internal_timer_note"),
                    "peak_rss_bytes": child["peak_rss_bytes"], "rss_source": child["rss_source"],
                    "ru_maxrss_upper_bound_bytes": child["ru_maxrss_upper_bound_bytes"],
                    "memory_cap": memory_cap_method(), "cpu_cap_s": CPU_CAP_S,
                    "degree": res.get("degree"), "max_matrix": res.get("max_matrix"),
                    "native_ops": res.get("native_ops"), "native_unit": res.get("native_unit"),
                    "telemetry": res.get("telemetry"), "extraction": res.get("extraction"),
                    "n_roots": len(roots) if roots is not None else None,
                    "roots": roots,
                    "verified_relations": [list(v) for v in sorted(verified)] if verified is not None else None,
                    "n_verified": len(verified) if verified is not None else None,
                    "zero_abscissa_roots": proj["zero_abscissa_roots"] if proj else None,
                    "spurious_roots": proj["spurious_roots"] if proj else None,
                    "roots_per_multiset": proj["roots_per_multiset"] if proj else None,
                    "projection_cpu_s": proj["projection_cpu_s"] if proj else None,
                    "readditions": proj["readditions"] if proj else None,
                    "oracle_relations": [list(v) for v in sorted(oset)],
                    "oracle_count": len(oset),
                    "matches_oracle": matches,
                    "missing_vs_oracle": ([list(v) for v in sorted(oset - verified)]
                                          if verified is not None else None),
                    "extra_vs_oracle": ([list(v) for v in sorted(verified - oset)]
                                        if verified is not None else None),
                    "m1_oracle_multiset_is_root": m1,
                    "m1_violation": m1_violation,
                    "planted_multiset": target.get("planted_multiset"),
                    "planted_found": (tuple(target["planted_multiset"]) in verified
                                      if target.get("planted_multiset") and verified is not None
                                      else None),
                    "host_load_before": load_before, "host_load_after": host_load(),
                },
            }
            rows.append(row)
    return rows


# ----------------------------------------------------------------- references

def startup_baseline(tool_paths, runs, phase):
    out = {}
    with tempfile.TemporaryDirectory(prefix="gc-startup-") as d:
        d = Path(d)
        (d / "e.ms").write_text("x0\n2\nx0^2+x0\n")
        (d / "e.cnf").write_text("p cnf 1 0\n")
        (d / "e.json").write_text(json.dumps({"op": "solve", "engine": "inherited-f4",
                                              "nvars": 1, "equations": [[1]], "params": {}}))
        cases = {
            "msolve": lambda: run_capped(msolve_argv(tool_paths["msolve"], d / "e.ms", d / "e.gb"),
                                         d / "o", d / "r"),
            "cryptominisat5": lambda: run_capped(cms_argv(tool_paths["cryptominisat5"], d / "e.cnf"),
                                                 d / "o", d / "r"),
            "gc_native_solve": lambda: run_capped([tool_paths["gc_native_solve"]], d / "o", d / "r",
                                                  stdin_path=d / "e.json"),
        }
        for k in range(runs):
            for name, fn in cases.items():  # interleaved
                child = fn()
                out.setdefault(name, []).append({"cpu_s": child["cpu_s"], "wall_s": child["wall_s"],
                                                 "exit_code": child["exit_code"]})
    summary = {}
    for name, items in out.items():
        cpus = sorted(i["cpu_s"] for i in items)
        walls = sorted(i["wall_s"] for i in items)
        summary[name] = {"runs": len(items), "cpu_median_s": cpus[len(cpus) // 2],
                         "cpu_range_s": [cpus[0], cpus[-1]], "wall_median_s": walls[len(walls) // 2],
                         "wall_range_s": [walls[0], walls[-1]],
                         "exit_codes": sorted({i["exit_code"] for i in items})}
    return {"phase": phase, "at": now_utc(), "summary": summary}


def rho_reference(tool_paths, domain, count):
    binary = tool_paths["gc_rho_match"]
    if not Path(binary).is_file():
        return {"status": "UNAVAILABLE", "reason": f"gc_rho_match not built at {binary}"}
    r = 65587
    dlps = []
    for i in range(count):
        s = seed(domain, "rho", i)
        dlps.append([1 + s % (r - 1), s])
    request = {"a": A, "n": N_FIELD, "dlps": dlps, "tga_batches": 7, "tga_adds": 1_000_000}
    with tempfile.TemporaryDirectory(prefix="gc-rho-") as d:
        d = Path(d)
        (d / "in.json").write_text(json.dumps(request))
        child = run_capped([binary], d / "out", d / "err", stdin_path=d / "in.json")
        text = (d / "out").read_text()
    answer = json.loads(text.strip().splitlines()[-1])
    if answer.get("modulus_int") != MODULUS:
        raise AssertionError(f"rho curve modulus {answer.get('modulus_int')} != {MODULUS}")
    if answer.get("subgroup_order") != r:
        raise AssertionError("rho subgroup order mismatch")
    rows = answer.pop("dlps")
    k = len(rows)
    mean = lambda key: sum(x[key] for x in rows) / k
    answer.update({
        "child": {key: child[key] for key in ("cpu_s", "wall_s", "exit_code", "peak_rss_bytes")},
        "dlps": k,
        "all_verified": all(x["verified"] and x["log_matches"] for x in rows),
        "verified_count": sum(1 for x in rows if x["verified"] and x["log_matches"]),
        "mean_walk_group_additions": mean("walk_group_additions"),
        "mean_setup_group_additions": mean("setup_group_additions"),
        "mean_partition_hashes": mean("partition_hashes"),
        "mean_canonicalizations": mean("canonicalizations"),
        "mean_frobenius_maps": mean("frobenius_maps"),
        "mean_negations_examined": mean("negations_examined"),
        "mean_iterations": mean("iterations"),
        "mean_cpu_s_per_dlp": mean("cpu_s"),
        "unpriced_per_dlp_note": "U_rho = mean partition hashes + canonicalisations (amendment-1 C3); "
                                 "Frobenius maps and negations examined recorded beside",
        "per_dlp": rows,
    })
    return answer


def ldd_closure(paths):
    out = {}
    for name, path in paths.items():
        if not path or not Path(path).is_file():
            out[name] = None
            continue
        try:
            text = subprocess.run(["ldd", path], capture_output=True, text=True).stdout
        except OSError:
            out[name] = None
            continue
        libs = {}
        for line in text.splitlines():
            m = re.search(r"(/\S+)\s+\(0x", line)
            if m and Path(m.group(1)).is_file():
                libs[m.group(1)] = sha256_file(m.group(1))
        out[name] = libs
    return out


def identities(tool_paths):
    out = {"binaries": {}, "harness": {}, "harness_pins_match": True}
    for name, path in tool_paths.items():
        if path and Path(path).is_file():
            out["binaries"][name] = {"path": path, "sha256": sha256_file(path)}
        else:
            out["binaries"][name] = None
    for name, pin in BINARY_PINS.items():
        got = (out["binaries"].get(name) or {}).get("sha256")
        out.setdefault("binary_pins_match", {})[name] = got == pin
    for rel, pin in HARNESS_PINS.items():
        got = sha256_file(ROOT / rel)
        out["harness"][rel] = got
        if got != pin:
            out["harness_pins_match"] = False
    for rel in ("groebner_compare/native_worker.py", "examples/gc_native_solve.rs",
                "examples/gc_rho_match.rs", "groebner_compare/build_native.sh",
                "research/groebner_solver_shootout_20261116/run_local.py",
                "research/groebner_solver_shootout_20261116/protocol.amend1.json"):
        p = ROOT / rel
        out["harness"][rel] = sha256_file(p) if p.is_file() else None
    return out


def host_identity():
    def cmd(argv):
        try:
            return subprocess.run(argv, capture_output=True, text=True, cwd=ROOT).stdout.strip()
        except OSError:
            return None
    cpu = None
    try:
        for line in Path("/proc/cpuinfo").read_text().splitlines():
            if line.startswith("model name"):
                cpu = line.split(":", 1)[1].strip()
                break
    except OSError:
        pass
    mem = None
    try:
        mem = int(Path("/proc/meminfo").read_text().split()[1]) * 1024
    except (OSError, ValueError):
        pass
    def read(p):
        try:
            return Path(p).read_text().strip()
        except OSError:
            return None
    return {
        "cpu_model": cpu, "nproc": os.cpu_count(), "memory_bytes": mem,
        "kernel": platform.release(), "python": sys.version.split()[0],
        "rustc": cmd(["rustc", "--version"]), "cargo": cmd(["cargo", "--version"]),
        "msolve_version_line": None,
        "cms_version": cmd([tools()["cryptominisat5"] or "cryptominisat5", "--version"]),
        "git_head": cmd(["git", "rev-parse", "HEAD"]),
        "git_dirty": bool(cmd(["git", "status", "--porcelain"])),
        "cgroup": {"version": "v2" if Path("/sys/fs/cgroup/cgroup.controllers").is_file() else "v1",
                   "memory_limit_v1": read("/sys/fs/cgroup/memory/memory.limit_in_bytes"),
                   "cpu_quota_v1": read("/sys/fs/cgroup/cpu/cpu.cfs_quota_us"),
                   "cpu_period_v1": read("/sys/fs/cgroup/cpu/cpu.cfs_period_us")},
        "memory_cap_method": memory_cap_method(),
        "pmu": Path("/sys/bus/event_source/devices/cpu/type").exists(),
        "isolation_tier": "L0 local diagnostic (non-admitted)",
        "load_at_start": host_load(),
    }


# ----------------------------------------------------------------- scheduling

def assign_sequences(domain, targets):
    """Hash-shuffled round robin over the 6 Williams sequences (M2)."""
    keyed = sorted(range(len(targets)), key=lambda k: seed(domain, "order", targets[k]["target_id"]))
    out = [None] * len(targets)
    for rank, k in enumerate(keyed):
        out[k] = WILLIAMS[rank % len(WILLIAMS)]
    order = [targets[k] for k in keyed]
    seqs = [out[k] for k in keyed]
    return order, seqs


def run_pool(jobs, workers, deadline=None, log=None):
    rows, started, skipped = [], 0, 0
    with cf.ProcessPoolExecutor(max_workers=workers) as pool:
        pending = set()
        queue = list(jobs)
        while queue or pending:
            while queue and len(pending) < workers:
                if deadline is not None and time.monotonic() > deadline:
                    skipped = len(queue)
                    queue = []
                    break
                job = queue.pop(0)
                pending.add(pool.submit(run_target, job))
                started += 1
            if not pending:
                break
            done, pending = cf.wait(pending, return_when=cf.FIRST_COMPLETED)
            for fut in done:
                got = fut.result()
                rows.extend(got)
                if log:
                    log(f"target {got[0]['instance_id']} done: " + ", ".join(
                        f"{r['arm']}={r['study']['outcome']}/{r['study']['charged_cpu_s']:.2f}s"
                        for r in got))
    return rows, started, skipped


def target_cost(rows):
    """Sum of solver wall + extraction + projection per target (both passes, all arms)."""
    per = {}
    for r in rows:
        s = r["study"]
        per.setdefault(r["instance_id"], 0.0)
        per[r["instance_id"]] += (s["solver_wall_s"] or 0) + (s["extract_cpu_s"] or 0) + (
            s["projection_cpu_s"] or 0)
    return per


def project_wall(per_target_s, n_targets, workers, contention=1.2):
    return per_target_s * n_targets / workers * contention


def cost_guard(elapsed_s, planted_mean_s, workers, n_per_stratum=N_PER_STRATUM):
    """Pre-stated rule: largest N per stratum (>= 40) whose projected total wall <= 100 min."""
    def total(n):
        return elapsed_s + project_wall(planted_mean_s, 2 * n, workers)
    if total(n_per_stratum) <= COST_GUARD_S:
        return n_per_stratum, total(n_per_stratum), False
    for n in range(n_per_stratum, N_MIN_PER_STRATUM - 1, -1):
        if total(n) <= COST_GUARD_S:
            return n, total(n), True
    return N_MIN_PER_STRATUM, total(N_MIN_PER_STRATUM), True


def tamper_checks(rows, domain, oracle):
    out = []
    by_arm = {}
    for r in rows:
        s = r["study"]
        if s["stratum"] == "planted" and s["pass"] == "A" and s["roots"]:
            by_arm.setdefault(s["pipeline"], []).append(r)
    for pipeline in PIPELINES:
        cands = by_arm.get(pipeline, [])
        for k in range(2):
            if not cands:
                out.append({"arm": pipeline, "k": k, "status": "no planted root available"})
                continue
            r = cands[k % len(cands)]
            s = r["study"]
            root = s["roots"][0]
            bit = seed(domain, f"tamper:{pipeline}", k) % s["nvars"]
            tampered = root ^ (1 << bit)
            instance = build_instance(s["x_R"], r["instance_id"])
            eqs = terms(instance["equations"], s["nvars"])
            proj = project_roots(instance, [tampered])
            oset = {tuple(v) for v in oracle["table"].get(s["x_R"], ())}
            accepted_bad = any(tuple(v) not in oset for v in proj["verified"])
            out.append({"arm": pipeline, "k": k, "target": r["instance_id"], "bit": bit,
                        "still_root_of_system": vanishes(eqs, tampered),
                        "projects_to": [list(v) for v in proj["verified"]],
                        "zero_abscissa": proj["zero_abscissa_roots"],
                        "accepted_relation_outside_oracle": accepted_bad,
                        "status": "invalidates arm" if accepted_bad else "rejected or oracle-consistent"})
    return out


# ----------------------------------------------------------------- main flow

def write_jsonl(path, rows):
    with open(path, "w") as f:
        for r in rows:
            f.write(json.dumps(r, sort_keys=True, default=list) + "\n")


def mask_rows(rows, out_dir):
    arms = sorted({r["arm"] for r in rows})
    nonce = secrets.token_hex(16)
    labels = {}
    for arm in arms:
        labels[arm] = "arm-" + hashlib.sha256(f"{nonce}:{arm}".encode()).hexdigest()[:2]
    while len(set(labels.values())) < len(labels):  # collision: extend
        labels = {a: "arm-" + hashlib.sha256(f"{nonce}:{a}".encode()).hexdigest()[:4] for a in arms}
    masked = []
    for r in rows:
        m = json.loads(json.dumps(r, default=list))
        m["arm"] = labels[r["arm"]]
        m["run_id"] = f"{r['instance_id']}:{labels[r['arm']]}"
        m["study"].pop("pipeline", None)
        masked.append(m)
    key = {"nonce": nonce, "labels": labels,
           "note": "phase-2 key; closed until summary.json (blind) is written"}
    (out_dir / "blinding_key.json").write_text(json.dumps(key, indent=2))
    return masked, sha256_file(out_dir / "blinding_key.json")


def preflight(tool_paths):
    missing = [k for k in ("msolve", "cryptominisat5") if not tool_paths[k]
               or not Path(tool_paths[k]).is_file()]
    if missing:
        raise SystemExit(f"run does not start: missing {missing}")
    ident = identities(tool_paths)
    problems = []
    if not ident["harness_pins_match"]:
        problems.append("harness file hash differs from the amendment pin")
    for name, ok in ident.get("binary_pins_match", {}).items():
        if not ok:
            problems.append(f"{name} sha256 differs from the amendment pin")
    return ident, problems


def main_flow(args):
    pilot = args.mode == "pilot"
    domain = PILOT_DOMAIN if pilot else MAIN_DOMAIN
    out = Path(args.out).resolve()
    out.mkdir(parents=True, exist_ok=False)
    logf = open(out / "driver.log", "w")

    def log(msg):
        line = f"{now_utc()} {msg}"
        print(line, flush=True)
        logf.write(line + "\n")
        logf.flush()

    t_start = time.monotonic()
    tool_paths = tools()
    log(f"mode={args.mode} domain={domain!r} out={out}")
    log(LABEL)
    ident, problems = preflight(tool_paths)
    native_ok = Path(tool_paths["gc_native_solve"]).is_file()
    if not native_ok:
        log("inherited-f4 UNAVAILABLE: gc_native_solve binary missing")
    for p in problems:
        log(f"INVALIDATION: {p}")
    if problems and not pilot:
        raise SystemExit("hash mismatch: run invalid; not starting")
    host = host_identity()
    ldd = ldd_closure(tool_paths)
    log(f"host {host['cpu_model']} nproc={host['nproc']} mem_cap={host['memory_cap_method']}")

    log("oracle: building fb-enum table")
    oracle = build_oracle()
    log(f"oracle: {oracle['sat_abscissae']}/{oracle['liftable_outside_V']} "
        f"p_sat={oracle['p_sat']:.6f} build_cpu={oracle['build_cpu_s']:.2f}s "
        f"weights_cpu={oracle['weight_cpu_s']:.2f}s additions={oracle['group_additions']}")

    log("rho reference")
    rho = rho_reference(tool_paths, domain, args.rho_dlps)
    if rho.get("status") == "ok":
        log(f"rho: {rho['verified_count']}/{rho['dlps']} verified, mean walk adds "
            f"{rho['mean_walk_group_additions']:.2f}, expected {rho['expected_steps']:.2f}, "
            f"t_GA {rho['tga_seconds_best']:.3e} s, mean cpu/DLP {rho['mean_cpu_s_per_dlp']:.3e}")
    startups = [startup_baseline(tool_paths, args.startup_runs, "start")]
    log(f"startup(start): {json.dumps(startups[0]['summary'])}")

    pipelines = [p for p in PIPELINES if p != "inherited-f4" or native_ok]
    if len(pipelines) != len(PIPELINES):
        raise SystemExit("inherited-f4 unavailable: this driver requires all three pipelines "
                         "for the Williams design; record UNAVAILABLE and stop")

    def job_for(t, seq, tag):
        oset = sorted(oracle["table"].get(t["x_R"], ()))
        m1 = oracle_roots_check(t, oracle) if oset else {}
        return (t, seq, oset, m1, tool_paths, tag, domain)

    all_rows = []
    # Warm-up (discarded) unless pilot skips it.
    if not args.skip_warmup:
        x, sign, _ = natural_draw(domain, "warmup", 0)
        wt = {"target_id": "warmup-0", "stratum": "warmup", "x_R": x, "sign": sign,
              "seed": seed(domain, "warmup", 0)}
        all_rows += run_target(job_for(wt, WILLIAMS[0], "driver"))
        log("warm-up done (discarded)")

    planted = planted_targets(domain, args.planted)
    for p in planted:
        if tuple(p["planted_multiset"]) not in oracle["table"].get(p["x_R"], set()):
            raise AssertionError(f"planted multiset of {p['target_id']} missing from oracle")
    p_order, p_seqs = assign_sequences(domain, planted)
    t_pl = time.monotonic()
    rows, _, _ = run_pool([job_for(t, s, f"pool-{WORKERS}") for t, s in zip(p_order, p_seqs)],
                          args.workers, log=log)
    planted_wall = time.monotonic() - t_pl
    all_rows += rows
    per = target_cost(rows)
    planted_mean = (sum(per.values()) / len(per)) if per else 0.0
    elapsed = time.monotonic() - t_start
    n_strat, projected, reduced = cost_guard(elapsed, planted_mean, args.workers, args.n_per_stratum)
    guard = {"planted_targets": len(per), "planted_phase_wall_s": planted_wall,
             "planted_mean_target_cost_s": planted_mean, "elapsed_at_guard_s": elapsed,
             "projected_total_wall_s_at_N": projected, "N_per_stratum": n_strat,
             "reduced": reduced, "threshold_s": COST_GUARD_S,
             "rule": "stopping_rules[budget]: largest N >= 40 whose projection fits 100 min"}
    log(f"cost guard: {json.dumps(guard)}")

    if pilot:
        n_sat, n_unsat = args.pilot_sat, args.pilot_unsat
    else:
        n_sat = n_unsat = n_strat
    sat, unsat, discarded, draws = natural_targets(domain, oracle, n_sat, n_unsat)
    naturals = sat + unsat
    n_order, n_seqs = assign_sequences(domain, naturals)
    jobs = [job_for(t, s, f"pool-{WORKERS}") for t, s in zip(n_order, n_seqs)]
    half = len(jobs) // 2
    deadline = t_start + WALL_CEILING_S
    t_nat = time.monotonic()
    r1, st1, sk1 = run_pool(jobs[:half], args.workers, deadline, log)
    startups.append(startup_baseline(tool_paths, args.startup_runs, "middle"))
    r2, st2, sk2 = run_pool(jobs[half:], args.workers, deadline, log)
    natural_wall = time.monotonic() - t_nat
    all_rows += r1 + r2
    startups.append(startup_baseline(tool_paths, args.startup_runs, "end"))
    tamper = tamper_checks(all_rows, domain, oracle)
    ident_end, problems_end = preflight(tool_paths)
    truncated = (sk1 + sk2) > 0
    total_wall = time.monotonic() - t_start

    # Pilot extrapolation to the protocol's full run.
    nat_cost = target_cost(r1 + r2)
    by_stratum = {}
    for t in naturals:
        if t["target_id"] in nat_cost:
            by_stratum.setdefault(t["stratum"], []).append(nat_cost[t["target_id"]])
    mean_sat = sum(by_stratum.get("sat", [0])) / max(1, len(by_stratum.get("sat", [])))
    mean_unsat = sum(by_stratum.get("unsat", [0])) / max(1, len(by_stratum.get("unsat", [])))
    fixed = elapsed - planted_wall  # preflight, oracle, rho, startup, warm-up
    full_planted = project_wall(planted_mean, N_PLANTED, args.workers)
    full_nat = project_wall((mean_sat + mean_unsat) / 2, 2 * N_PER_STRATUM, args.workers)
    startup_cost = sum(sum(s["summary"][k]["wall_median_s"] * s["summary"][k]["runs"]
                           for k in s["summary"]) for s in startups) / len(startups)
    extrap = {
        "method": "mean measured per-target cost (solver wall + extraction + projection, "
                  "both passes, all arms) x targets / workers x 1.2 contention, plus measured "
                  "fixed phases; startup baselines x3",
        "fixed_measured_s": fixed, "startup_phase_s_each": startup_cost,
        "planted_mean_s": planted_mean, "natural_sat_mean_s": mean_sat,
        "natural_unsat_mean_s": mean_unsat,
        "projected_planted_phase_s": full_planted,
        "projected_natural_phase_s_N80": full_nat,
        "projected_total_s_N80": fixed + full_planted + full_nat + 3 * startup_cost,
    }
    if not pilot:
        extrap = None

    cells = {
        "label": LABEL, "mode": args.mode, "seed_domain": domain, "cell": CELL,
        "protocol": {"path": "protocol.amend1.json", "canonical_sha256": AMEND1_CANONICAL,
                     "file_sha256": sha256_file(HERE / "protocol.amend1.json")},
        "host": host, "identities_start": ident, "identities_end": ident_end,
        "identity_problems": problems + problems_end, "ldd_closure": ldd,
        "unavailable_arms": UNAVAILABLE,
        "oracle": {k: v for k, v in oracle.items() if k != "table"},
        "rho": rho, "startup": startups, "cost_guard": guard,
        "natural_stream": {"draws": draws, "discarded": len(discarded), "sat": len(sat),
                           "unsat": len(unsat)},
        "targets": {"planted": planted, "natural": naturals},
        "tamper": tamper, "truncated": truncated, "targets_skipped_at_ceiling": sk1 + sk2,
        "phase_walls_s": {"planted": planted_wall, "natural": natural_wall, "total": total_wall},
        "extrapolation": extrap,
    }
    masked, key_sha = mask_rows(all_rows, out)
    write_jsonl(out / "rows.jsonl", masked)
    cells["blinding_key_sha256"] = key_sha
    (out / "cells.json").write_text(json.dumps(cells, indent=2, sort_keys=True, default=list))
    if pilot:
        write_jsonl(out / "pilot_rows_unmasked.jsonl", all_rows)
    logf.close()
    files = sorted(p for p in out.iterdir() if p.is_file() and p.name != "manifest.json")
    (out / "manifest.json").write_text(json.dumps(
        {"label": LABEL, "files": {p.name: sha256_file(p) for p in files}}, indent=2))
    return cells, all_rows


def parse_args(argv=None):
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("mode", choices=("pilot", "run"))
    ap.add_argument("--out", required=True)
    ap.add_argument("--workers", type=int, default=WORKERS)
    ap.add_argument("--n-per-stratum", type=int, default=N_PER_STRATUM)
    ap.add_argument("--planted", type=int, default=None)
    ap.add_argument("--pilot-sat", type=int, default=1)
    ap.add_argument("--pilot-unsat", type=int, default=1)
    ap.add_argument("--rho-dlps", type=int, default=N_RHO)
    ap.add_argument("--startup-runs", type=int, default=STARTUP_RUNS)
    ap.add_argument("--skip-warmup", action="store_true")
    args = ap.parse_args(argv)
    if args.mode == "run":
        # The main run takes the frozen design only.
        if (args.workers, args.n_per_stratum, args.rho_dlps, args.startup_runs) != (
                WORKERS, N_PER_STRATUM, N_RHO, STARTUP_RUNS) or args.planted not in (None, N_PLANTED) \
                or args.skip_warmup:
            ap.error("the main run uses the frozen design: no overrides")
        args.planted = N_PLANTED
    elif args.planted is None:
        args.planted = 1
    return args


if __name__ == "__main__":
    main_flow(parse_args())
