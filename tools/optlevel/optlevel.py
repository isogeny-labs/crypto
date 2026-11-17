#!/usr/bin/env python3
"""optlevel: how close does an implementation run to what its machine can do?

    optlevel.py probe                              what this host can measure
    optlevel.py calibrate --out CAL.json           machine ceiling (roofline) of this host
    optlevel.py measure --out M.json [opts] -- CMD measure a kernel (perf stat with a PMU, else timing)
    optlevel.py score --calibration CAL.json --measurement M.json --out R.json
    optlevel.py plot --calibration CAL.json --out roofline.svg R.json [R2.json ...]
    optlevel.py controls --out DIR                 the synthetic control kernels, end to end
    optlevel.py vector-share BINARY SYMBOL...      static share of packed-vector instructions
    optlevel.py gpu probe|calibrate|measure ...    GPU path; refuses without nvidia-smi
    optlevel.py validate R.json                    check a record against the schema and tier rules
    optlevel.py benchrec R.json                    the record's values as `benchrec.py metric` arguments

Definition, formulas, tiers and failure modes: docs/OPTIMIZATION_LEVEL.md.
Standard library only; the calibration kernels (kernels/optk.c) are compiled
with the system C compiler at first use and the flags are recorded.
Exit status: 0 ok, 2 usage, 3 unsupported on this host, 4 the command failed.
"""
from __future__ import annotations

import argparse
import datetime
import hashlib
import json
import os
import re
import shlex
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import score as S  # noqa: E402
import plot as P   # noqa: E402

BUILDS = {  # name -> extra flags; every build also gets BASE_FLAGS
    "native": [],
    "w512": ["-mprefer-vector-width=512"],
    "scalar": ["-fno-tree-vectorize", "-fno-tree-slp-vectorize"],
}
BASE_FLAGS = ["-O3", "-march=native", "-ffp-contract=fast"]
# operation class -> kernels of optk.c that realise it (array loops, plus a
# register-resident variant where the array loop is load/store bound)
CLASS_KERNELS = {
    "muladd_u32": ["muladd_u32"], "muladd_u64": ["muladd_u64"], "mulwide_u64": ["mulwide_u64"],
    "modmuladd_p31": ["modmuladd_p31"], "modmuladd_p62": ["modmuladd_p62"],
    "xor_u64": ["xor_u64"], "popcnt_u64": ["popcnt_u64"],
    "fma_f64": ["fma_f64", "fma_f64_reg"], "ptr_hop": ["chase"],
}
CLASS_DOC = {
    "muladd_u32": "a <- a*m + c on 32-bit words (wrapping)",
    "muladd_u64": "a <- a*m + c on 64-bit words (wrapping)",
    "mulwide_u64": "a <- hi64(a*m) + c: one 64x64->128 multiply per operation",
    "modmuladd_p31": "a <- a + m*b mod p, p < 2^31 (Montgomery, 32x32->64 multiplies)",
    "modmuladd_p62": "a <- a + m*b mod p, p < 2^62 (Montgomery, 64x64->128 multiplies)",
    "xor_u64": "a <- a xor b on 64-bit words (one F_2 row-add word)",
    "popcnt_u64": "popcount(a and b) on 64-bit words",
    "fma_f64": "a <- a*s + b in binary64 (one fused multiply-add; float-encoded modular arithmetic)",
    "ptr_hop": "one dependent pointer dereference",
}
PMU_GROUPS = {
    "core": "cycles:u,instructions:u,branches:u,branch-misses:u",
    "mem": "cycles:u,instructions:u,cache-misses:u,L1-dcache-load-misses:u,dTLB-load-misses:u",
}
CACHE_LINE = 64


def die(msg, code=2):
    print(f"optlevel: {msg}", file=sys.stderr)
    sys.exit(code)


def utcnow():
    return datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def read(path, default=None):
    try:
        return Path(path).read_text().strip()
    except OSError:
        return default


def sha256_file(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def write_json(path, doc):
    Path(path).parent.mkdir(parents=True, exist_ok=True)
    Path(path).write_text(json.dumps(doc, indent=1, sort_keys=True) + "\n")


def load_json(path):
    doc = json.loads(Path(path).read_text())
    doc["_sha256"] = sha256_file(path)
    return doc


# ---- host ------------------------------------------------------------------
def find_perf():
    cands = [os.environ.get("PERF"), shutil.which("perf")]
    cands += sorted(map(str, Path("/usr/lib").glob("linux-tools*/perf")), reverse=True)
    cands += sorted(map(str, Path("/usr/lib/linux-tools").glob("*/perf")), reverse=True)
    for p in cands:
        if p and os.access(p, os.X_OK):
            try:
                if subprocess.run([p, "--version"], capture_output=True, timeout=10).returncode == 0:
                    return p
            except (OSError, subprocess.SubprocessError):
                pass
    return None


def pmu_probe():
    """Same test as tools/perf_ipc.sh: can this process count cycles:u and
    instructions:u?  Returns (perf_path or None, reason when unavailable)."""
    sources = sorted(os.listdir("/sys/bus/event_source/devices")) if os.path.isdir("/sys/bus/event_source/devices") else []
    perf = find_perf()
    if not perf:
        return None, f"no working perf binary (event sources: {' '.join(sources) or 'none'})"
    try:
        r = subprocess.run([perf, "stat", "-x,", "-e", "cycles:u,instructions:u", "--", "true"],
                           capture_output=True, text=True, timeout=30)
    except (OSError, subprocess.SubprocessError) as e:
        return None, f"perf probe failed: {e}"
    if re.search(r"<not (supported|counted)>", r.stderr):
        return None, "perf cannot count cycles:u/instructions:u here (<not supported>): no hardware PMU is exposed"
    if re.search(r"permission|access to performance monitoring", r.stderr, re.I):
        return None, f"perf_event_paranoid={read('/proc/sys/kernel/perf_event_paranoid', '?')} forbids counting"
    if r.returncode != 0:
        return None, f"perf probe exit {r.returncode}"
    return perf, None


def cache_sizes():
    out = {}
    base = Path("/sys/devices/system/cpu/cpu0/cache")
    for idx in sorted(base.glob("index*")):
        lvl, typ, size = read(idx / "level"), read(idx / "type"), read(idx / "size", "")
        m = re.match(r"(\d+)([KMG]?)", size)
        if lvl and m:
            out[f"L{lvl}{'d' if typ == 'Data' else 'i' if typ == 'Instruction' else ''}"] = \
                int(m.group(1)) * {"": 1, "K": 1 << 10, "M": 1 << 20, "G": 1 << 30}[m.group(2)]
    return out


def host_info():
    cpuinfo = read("/proc/cpuinfo", "")
    model = re.search(r"model name\s*:\s*(.*)", cpuinfo)
    flags = re.search(r"^(?:flags|Features)\s*:\s*(.*)", cpuinfo, re.M)
    flagset = sorted(flags.group(1).split()) if flags else []
    caches = cache_sizes()
    cpus = sorted(os.sched_getaffinity(0))
    mem = re.search(r"MemTotal:\s*(\d+)", read("/proc/meminfo", ""))
    nodes = [d for d in os.listdir("/sys/devices/system/node")] if os.path.isdir("/sys/devices/system/node") else []
    h = {
        "cpu_model": model.group(1) if model else "unknown",
        "arch": os.uname().machine, "kernel": os.uname().release,
        "logical_cpus": os.cpu_count(), "allowed_cpus": cpus,
        "isa": [f for f in flagset if f in ("sse4_2", "avx", "avx2", "fma", "bmi2", "adx", "popcnt", "pclmulqdq",
                                             "avx512f", "avx512dq", "avx512bw", "avx512vl", "avx512ifma",
                                             "avx512_vpopcntdq", "vpclmulqdq", "gfni", "neon", "asimd", "sve", "pmull")],
        "hypervisor": "hypervisor" in flagset,
        "caches_bytes": caches,
        "mem_total_bytes": int(mem.group(1)) * 1024 if mem else None,
        "smt_active": read("/sys/devices/system/cpu/smt/active"),
        "thread_siblings_cpu0": read("/sys/devices/system/cpu/cpu0/topology/thread_siblings_list"),
        "numa_nodes": len([n for n in nodes if re.fullmatch(r"node\d+", n)]) or None,
        "cpufreq_governor": read("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor"),
        "turbo_disabled": read("/sys/devices/system/cpu/intel_pstate/no_turbo"),
        "cpufreq_boost": read("/sys/devices/system/cpu/cpufreq/boost"),
        "event_sources": sorted(os.listdir("/sys/bus/event_source/devices")) if os.path.isdir("/sys/bus/event_source/devices") else [],
        "perf_event_paranoid": read("/proc/sys/kernel/perf_event_paranoid"),
        "cgroup_cpu_max": read("/sys/fs/cgroup/cpu.max"),
    }
    key = json.dumps([h["cpu_model"], h["arch"], h["logical_cpus"], flagset, caches, h["mem_total_bytes"],
                      h["hypervisor"]], sort_keys=True)
    h["fingerprint"] = hashlib.sha256(key.encode()).hexdigest()[:16]
    return h


def proc_stat():
    f = read("/proc/stat", "cpu 0 0 0 0 0 0 0 0").splitlines()[0].split()[1:]
    v = [int(x) for x in f] + [0] * 8
    hz = os.sysconf("SC_CLK_TCK")
    busy = v[0] + v[1] + v[2] + v[5] + v[6]
    return {"busy_s": busy / hz, "steal_s": v[7] / hz}


def throttle_count():
    tot, seen = 0, False
    for p in Path("/sys/devices/system/cpu").glob("cpu[0-9]*/thermal_throttle/*_throttle_count"):
        v = read(p)
        if v is not None and v.isdigit():
            tot, seen = tot + int(v), True
    return tot if seen else None


def conditions_begin():
    return {"t": time.time(), "stat": proc_stat(), "throttle": throttle_count(),
            "loadavg": os.getloadavg(), "psi_cpu": read("/proc/pressure/cpu")}


def conditions_end(begin, own_cpu_s):
    wall = max(time.time() - begin["t"], 1e-9)
    st = proc_stat()
    other = max(0.0, (st["busy_s"] - begin["stat"]["busy_s"]) - own_cpu_s) / wall
    thr = throttle_count()
    psi = re.search(r"some avg10=([\d.]+)", begin["psi_cpu"] or "")
    return {"loadavg_1m_before": begin["loadavg"][0], "loadavg_1m_after": os.getloadavg()[0],
            "psi_cpu_some_avg10_before": float(psi.group(1)) if psi else None,
            "other_cpu_cores": other,  # CPU-seconds per second used by everything else on the host
            "steal_cores": (st["steal_s"] - begin["stat"]["steal_s"]) / wall,
            "thermal_throttle_events": (thr - begin["throttle"]) if thr is not None and begin["throttle"] is not None else None}


# ---- build -----------------------------------------------------------------
def cache_dir():
    d = Path(os.environ.get("OPTLEVEL_CACHE") or (Path(os.environ.get("XDG_CACHE_HOME") or Path.home() / ".cache") / "optlevel"))
    d.mkdir(parents=True, exist_ok=True)
    return d


def build_kernels():
    """Compile kernels/optk.c once per (source, compiler, host) and return
    {build name: {path, flags, sha256}} plus compiler identity."""
    cc = os.environ.get("CC") or shutil.which("cc") or shutil.which("gcc") or shutil.which("clang")
    if not cc:
        die("no C compiler (set CC)", 3)
    src = HERE / "kernels" / "optk.c"
    ccv = subprocess.run([cc, "--version"], capture_output=True, text=True).stdout.splitlines()[0]
    tag = hashlib.sha256((src.read_text() + ccv + host_info()["fingerprint"] + json.dumps(BUILDS) + json.dumps(BASE_FLAGS)).encode()).hexdigest()[:12]
    out = {}
    for name, extra in BUILDS.items():
        path = cache_dir() / f"optk-{tag}-{name}"
        flags = BASE_FLAGS + extra
        if not path.exists():
            tmp = str(path) + f".tmp{os.getpid()}"
            r = subprocess.run([cc, *flags, "-o", tmp, str(src)], capture_output=True, text=True)
            if r.returncode != 0:
                if extra:   # an optional flag this compiler does not know: skip the build, say so
                    print(f"optlevel: build {name} skipped: {r.stderr.strip().splitlines()[-1] if r.stderr.strip() else 'compiler error'}", file=sys.stderr)
                    continue
                die(f"cannot compile {src}: {r.stderr}", 3)
            os.replace(tmp, path)
        out[name] = {"path": str(path), "flags": flags, "sha256": sha256_file(path)}
    return {"compiler": cc, "compiler_version": ccv, "source_sha256": sha256_file(src), "builds": out}


# ---- running one command ---------------------------------------------------
def parse_ops(stdout, stderr, args_ops, ops_regex):
    """Operation count of one process: optlevel.ops/v1 lines on stdout (summed),
    else --ops-regex matches on stdout+stderr (summed), else --ops."""
    ops, ksec, declared, cls, found, wset = 0, 0.0, 0.0, None, False, None
    have_ksec = have_decl = True
    for line in stdout.splitlines():
        line = line.strip()
        if not line.startswith("{") or S.OPS_SCHEMA not in line:
            continue
        try:
            d = json.loads(line)
        except ValueError:
            continue
        if d.get("schema") != S.OPS_SCHEMA:
            continue
        found = True
        ops += int(d["ops"])
        cls = d.get("op_class", cls)
        if d.get("working_set_bytes") is not None:
            wset = max(wset or 0, int(d["working_set_bytes"]))
        if d.get("kernel_seconds") is None:
            have_ksec = False
        else:
            ksec += float(d["kernel_seconds"])
        if d.get("bytes_declared") is None:
            have_decl = False
        else:
            declared += float(d["bytes_declared"])
    if found:
        return {"ops": ops, "kernel_seconds": ksec if have_ksec else None,
                "bytes_declared": declared if have_decl else None, "op_class": cls,
                "working_set_bytes": wset, "source": "optlevel.ops/v1 line"}
    if ops_regex:
        vals = [int(m.group(1).replace(",", "").replace("_", "")) for m in re.finditer(ops_regex, stdout + "\n" + stderr)]
        if vals:
            return {"ops": sum(vals), "kernel_seconds": None, "bytes_declared": None, "op_class": None,
                    "source": f"--ops-regex {ops_regex!r} ({len(vals)} matches summed)"}
    if args_ops is not None:
        return {"ops": int(args_ops), "kernel_seconds": None, "bytes_declared": None, "op_class": None, "source": "--ops (constant)"}
    return None


def parse_perf_csv(text):
    """perf stat -x, rows: value,unit,event,run-time,percent-running,..."""
    counters, multiplexed, not_counted = {}, [], []
    for line in text.splitlines():
        if not line or line.startswith("#"):
            continue
        f = line.split(",")
        if len(f) < 3:
            continue
        ev = re.sub(r":u$", "", f[2])
        if f[0].startswith("<"):
            not_counted.append(ev)
            continue
        try:
            counters[ev] = float(f[0])
        except ValueError:
            continue
        try:
            if len(f) > 4 and float(f[4]) < 100.0:
                multiplexed.append(ev)
        except ValueError:
            pass
    return counters, multiplexed, not_counted


def run_once(cmd, *, copies=1, cpus=None, env=None, cwd=None, perf=None, events=None,
             ops=None, ops_regex=None, timeout=None):
    """Run `copies` concurrent instances of cmd, copy i pinned to cpus[i] when
    cpus is given.  Returns the summed operation count and the rate."""
    procs, tmp = [], tempfile.mkdtemp(prefix="optlevel-")
    begin = conditions_begin()
    ru0 = os.times()
    t0 = time.perf_counter()
    for i in range(copies):
        argv = list(cmd)
        csv = None
        if perf and events:
            csv = os.path.join(tmp, f"perf{i}.csv")
            argv = [perf, "stat", "-x,", "--no-big-num", "-e", events, "-o", csv, "--", *argv]
        pin = (lambda c: (lambda: os.sched_setaffinity(0, {c})))(cpus[i]) if cpus else None
        procs.append((subprocess.Popen(argv, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
                                       env=env, cwd=cwd, preexec_fn=pin), csv))
    outs = []
    for p, csv in procs:
        try:
            so, se = p.communicate(timeout=timeout)
        except subprocess.TimeoutExpired:
            p.kill()
            so, se = p.communicate()
        outs.append((p.returncode, so, se, csv))
    wall = time.perf_counter() - t0
    ru1 = os.times()
    own_cpu = (ru1.children_user - ru0.children_user) + (ru1.children_system - ru0.children_system)
    res = {"copies": copies, "pinned_cpus": cpus, "wall_s": wall, "cpu_s": own_cpu,
           "exit_status": [o[0] for o in outs], "conditions": conditions_end(begin, own_cpu)}
    rate, total_ops, ksec, declared, src, cls = 0.0, 0, [], 0.0, None, None
    decl_ok = True
    for rc, so, se, csv in outs:
        o = parse_ops(so, se, ops, ops_regex)
        if o is None:
            res["ops"] = None
            res["stderr_tail"] = se[-400:]
            shutil.rmtree(tmp, ignore_errors=True)
            return res
        total_ops += o["ops"]
        src, cls = o["source"], o["op_class"] or cls
        if o.get("working_set_bytes") is not None:
            res["working_set_bytes"] = o["working_set_bytes"]
        secs = o["kernel_seconds"] if o["kernel_seconds"] else wall
        ksec.append(o["kernel_seconds"])
        rate += o["ops"] / secs
        if o["bytes_declared"] is None:
            decl_ok = False
        else:
            declared += o["bytes_declared"]
    res.update(ops=total_ops, ops_per_s=rate, ops_source=src, op_class=cls,
               kernel_seconds=max(k for k in ksec) if all(k is not None for k in ksec) else None,
               bytes_declared=declared if decl_ok else None)
    if perf and events:
        c, mux, nc = {}, [], []
        for *_x, csv in outs:
            if csv and os.path.exists(csv):
                ci, m, n = parse_perf_csv(Path(csv).read_text())
                for k, v in ci.items():
                    c[k] = c.get(k, 0.0) + v
                mux += m
                nc += n
        res["counters"], res["multiplexed"], res["not_counted"] = c, sorted(set(mux)), sorted(set(nc))
    shutil.rmtree(tmp, ignore_errors=True)
    return res


def pick_cpus(k):
    allowed = sorted(os.sched_getaffinity(0), reverse=True)
    if k > len(allowed):
        die(f"{k} copies requested but only {len(allowed)} CPUs are allowed")
    return allowed[:k]


# ---- calibrate -------------------------------------------------------------
def cmd_calibrate(a):
    host = host_info()
    build = build_kernels()
    bins = {n: b["path"] for n, b in build["builds"].items()}
    ncpu = len(host["allowed_cpus"])
    scale = sorted({1, ncpu} | {k for k in (2, 4, 8, 16, 32, 64) if k < ncpu})
    flags = []
    begin = conditions_begin()
    own = 0.0
    sec = ["--seconds", str(a.seconds)]

    def run(binary, kernel, k, extra=()):
        nonlocal own
        r = run_once([bins[binary], kernel, *sec, *extra], copies=k, cpus=pick_cpus(k))
        own += r["cpu_s"]
        if r.get("ops") is None or any(r["exit_status"]):
            die(f"calibration kernel {kernel} ({binary}) failed: {r.get('stderr_tail', '')}", 4)
        return r

    clock = [run("native", "clock", 1)["ops_per_s"] for _ in range(a.repeat)]
    compute = {}
    for cls, kernels in CLASS_KERNELS.items():
        variants = {}
        for rep in range(a.repeat):
            for kern in kernels:
                for b in bins:
                    if kern.endswith("_reg") and b == "scalar":
                        continue
                    variants.setdefault(f"{kern}@{b}", []).append(run(b, kern, 1)["ops_per_s"])
        vstats = {k: S.stats(v) for k, v in variants.items()}
        best = max(vstats, key=lambda k: vstats[k]["max"])
        bk, bb = best.split("@")
        total = {"1": vstats[best]}
        for k in scale[1:]:
            total[str(k)] = S.stats([run(bb, bk, k)["ops_per_s"] for _ in range(a.repeat)])
        scalar = max((v["max"] for k, v in vstats.items() if k.endswith("@scalar")), default=None)
        simd = max((v["max"] for k, v in vstats.items() if not k.endswith("@scalar")), default=None)
        compute[cls] = {"definition": CLASS_DOC[cls], "unit": "operations per second", "best_variant": best,
                        "variants": vstats, "total_ops_per_s": total,
                        "scalar_ops_per_s": scalar, "simd_ops_per_s": simd}
        print(f"optlevel: {cls:14s} peak {vstats[best]['max']:.3e} ops/s/core ({best}); "
              f"all {scale[-1]} cores {total[str(scale[-1])]['max']:.3e}", file=sys.stderr)

    # memory: triad arrays and the chase set each exceed the last-level cache
    llc = max(host["caches_bytes"].values(), default=32 << 20)
    avail = re.search(r"MemAvailable:\s*(\d+)", read("/proc/meminfo", ""))
    avail = int(avail.group(1)) * 1024 if avail else 4 << 30
    arr = max(64 << 20, int(1.5 * llc))
    cap = int(0.4 * avail / (3 * scale[-1]))
    if a.triad_bytes:
        arr = a.triad_bytes
    elif arr > cap:
        arr = cap
    if arr < llc:
        flags.append("TRIAD_ARRAY_NOT_ABOVE_LLC")
    n_el = arr // 8
    bw, bw_variants = {}, {}
    streams = (("triad", n_el), ("xor_u64", n_el), ("popcnt_u64", n_el))   # 32, 24 and 16 bytes per element
    for k in scale:
        per = {}
        for _ in range(a.repeat):
            for kern, n in streams:
                r = run("native", kern, k, ["--n", str(n)])
                per.setdefault(kern, []).append(r["bytes_declared"] / r["ops"] * r["ops_per_s"])
        st = {kern: S.stats(v) for kern, v in per.items()}
        bestk = max(st, key=lambda x: st[x]["max"])
        bw[str(k)] = dict(st[bestk], kernel=bestk)
        bw_variants[str(k)] = st
        print(f"optlevel: bandwidth {k} core(s): {bw[str(k)]['max'] / 1e9:.2f} GB/s ({bestk}; "
              + ", ".join(f"{x} {st[x]['max'] / 1e9:.2f}" for x in st) + ")", file=sys.stderr)
    lat = S.stats([1e9 / run("native", "chase", 1, ["--n", str(arr // 64)])["ops_per_s"] for _ in range(a.repeat)])
    cond = conditions_end(begin, own)
    if cond["other_cpu_cores"] > 0.25:
        flags.append("CONTENDED")
    if host["hypervisor"]:
        flags.append("VIRTUALIZED")
    cl = S.stats(clock)
    if (cl["max"] - cl["min"]) / cl["median"] > 0.03:
        flags.append("CLOCK_UNSTABLE")
    doc = {"schema": S.CALIBRATION_SCHEMA, "tool_version": S.TOOL_VERSION, "created_utc": utcnow(), "device": "cpu",
           "host": host, "build": build, "settings": {"seconds_per_kernel": a.seconds, "repeats": a.repeat, "scale_points": scale},
           "ceiling_rule": "max over repeats and over builds of the summed rate of k pinned copies; the median is kept as the low end",
           "clock_hz_effective": cl, "compute": compute,
           "memory": {"bandwidth_bytes_per_s": bw, "bandwidth_variants": bw_variants,
                      "bytes_convention": "bytes moved over arrays above the LLC: loads + stores + write-allocate reads "
                                          "(triad 32 B/element, xor 24, popcount 16); the ceiling is the best of the three",
                      "triad_array_bytes": arr, "llc_bytes": llc, "latency_ns": lat,
                      "latency_definition": "ns per dependent load over a random cycle of 64-byte nodes the size of one triad array"},
           "conditions": cond, "flags": flags}
    write_json(a.out, doc)
    print(f"optlevel: wrote {a.out} (host {host['fingerprint']}, flags {flags or 'none'})", file=sys.stderr)


# ---- measure ---------------------------------------------------------------
def parse_scale(spec):
    """'copies:1,4' or 'env:VAR:1,2,4' -> (mode, var, [k...])."""
    if not spec:
        return "copies", None, [1]
    parts = spec.split(":")
    if parts[0] == "copies" and len(parts) == 2:
        return "copies", None, sorted({int(x) for x in parts[1].split(",")})
    if parts[0] == "env" and len(parts) == 3:
        return "env", parts[1], sorted({int(x) for x in parts[2].split(",")})
    die("--scale wants copies:1,4 or env:VAR:1,2,4")


def summarise_pmu(runs_by_group, single_ops):
    """Per-operation PMU components from the single-thread runs."""
    out, mux = {}, set()

    def med(group, fn):
        return (S.stats([fn(r) for r in runs_by_group.get(group, []) if fn(r) is not None]) or {}).get("median")

    def per_op(ev):
        return lambda r: (r["counters"][ev] / r["ops"]) if ev in r.get("counters", {}) and r["ops"] else None

    for g in runs_by_group.values():
        for r in g:
            mux |= set(r.get("multiplexed", []))
    any_group = "core" if runs_by_group.get("core") else "mem"
    out["ipc"] = med(any_group, lambda r: r["counters"]["instructions"] / r["counters"]["cycles"]
                     if r.get("counters", {}).get("cycles") else None)
    out["cycles_per_op"] = med(any_group, per_op("cycles"))
    out["instructions_per_op"] = med(any_group, per_op("instructions"))
    out["branch_miss_rate"] = med("core", lambda r: r["counters"]["branch-misses"] / r["counters"]["branches"]
                                  if r.get("counters", {}).get("branches") else None)
    out["llc_misses_per_op"] = med("mem", per_op("cache-misses"))
    out["l1d_misses_per_op"] = med("mem", per_op("L1-dcache-load-misses"))
    out["dtlb_misses_per_op"] = med("mem", per_op("dTLB-load-misses"))
    return out, sorted(mux)


def cmd_measure(a):
    if not a.cmd:
        die("no command (put it after --)")
    mode, var, ks = parse_scale(a.scale)
    if 1 not in ks:
        ks = [1] + ks
    host = host_info()
    perf, why = (None, "disabled by --timing-only") if a.timing_only else pmu_probe()
    tier = "T-pmu" if perf else "T-timing"
    flags = []
    if host["hypervisor"]:
        flags.append("VIRTUALIZED")
    if host["smt_active"] == "1":
        flags.append("SMT_ACTIVE")
    if (host["numa_nodes"] or 1) > 1:
        flags.append("NUMA_MULTI_NODE")

    def one(k, events=None):
        env = dict(os.environ)
        if mode == "env":
            env[var] = str(k)
            return run_once(a.cmd, env=env, cwd=a.cwd, perf=perf if events else None, events=events,
                            ops=a.ops, ops_regex=a.ops_regex, cpus=None, timeout=a.timeout)
        return run_once(a.cmd, copies=k, cpus=None if a.no_pin else pick_cpus(k), env=env, cwd=a.cwd,
                        perf=perf if events else None, events=events, ops=a.ops, ops_regex=a.ops_regex, timeout=a.timeout)

    runs = {"A": [], "B": []}
    scaled = {k: [] for k in ks if k > 1}
    if a.warmup:
        one(1)
    for _ in range(a.repeat):   # interleaved: A(1), each k > 1, B(1)
        runs["A"].append(one(1, PMU_GROUPS["core"] if perf else None))
        for k in scaled:
            scaled[k].append(one(k))
        runs["B"].append(one(1, PMU_GROUPS["mem"] if perf else None))
    allruns = runs["A"] + runs["B"] + [r for v in scaled.values() for r in v]
    if any(r.get("ops") is None for r in allruns):
        flags.append("OPS_MISSING")
        doc = {"schema": S.MEASUREMENT_SCHEMA, "tier": tier, "flags": flags, "host": host, "command": a.cmd,
               "label": a.label, "created_utc": utcnow(), "points": {}, "op_class": a.op_class,
               "error": "no operation count: print an optlevel.ops/v1 line, or pass --ops or --ops-regex",
               "stderr_tail": next((r.get("stderr_tail") for r in allruns if r.get("ops") is None), "")}
        write_json(a.out, doc)
        die(doc["error"] + "\n" + (doc["stderr_tail"] or ""), 4)
    if any(any(r["exit_status"]) for r in allruns):
        flags.append("COMMAND_FAILED")
    single = runs["A"] + runs["B"]
    if len({r["ops"] for r in single}) > 1:
        flags.append("OPS_VARY")          # legal for time-bounded kernels; recorded
    ksecs = [r["kernel_seconds"] if r["kernel_seconds"] is not None else r["wall_s"] for r in single]
    if min(ksecs) < S.MIN_KERNEL_SECONDS:
        flags.append("TOO_SHORT")
    if any(r["kernel_seconds"] is None for r in single):
        flags.append("WALL_INCLUDES_STARTUP")
    startup = S.stats([1 - r["kernel_seconds"] / r["wall_s"] for r in single if r["kernel_seconds"] is not None])
    other = S.stats([r["conditions"]["other_cpu_cores"] for r in allruns])
    if other["median"] > 0.25:
        flags.append("CONTENDED")
    if any((r["conditions"]["thermal_throttle_events"] or 0) > 0 for r in allruns):
        flags.append("THERMAL_THROTTLE")
    op_class = a.op_class or next((r["op_class"] for r in single if r.get("op_class")), None)
    if not op_class:
        die("no operation class: pass --op-class (one of the calibrated classes)")

    def point(rs, k):
        return {"threads": k, "mode": mode if k > 1 else "single", "ops_per_s": S.stats([r["ops_per_s"] for r in rs]),
                "ops": S.stats([r["ops"] for r in rs]), "wall_s": S.stats([r["wall_s"] for r in rs]),
                "cpu_s": S.stats([r["cpu_s"] for r in rs]),
                "cpu_utilisation": S.stats([r["cpu_s"] / (r["wall_s"] * k) for r in rs]),
                "other_cpu_cores": S.stats([r["conditions"]["other_cpu_cores"] for r in rs]),
                "loadavg_1m_before": S.stats([r["conditions"]["loadavg_1m_before"] for r in rs])}

    points = {"1": point(single, 1)}
    for k, rs in scaled.items():
        points[str(k)] = point(rs, k)
    declared = a.bytes_per_op
    if declared is None and all(r.get("bytes_declared") is not None for r in single):
        declared = S.stats([r["bytes_declared"] / r["ops"] for r in single])["median"]
    doc = {"schema": S.MEASUREMENT_SCHEMA, "tool_version": S.TOOL_VERSION, "created_utc": utcnow(), "label": a.label,
           "tier": tier, "pmu_unavailable_reason": why, "host": host, "command": a.cmd, "cwd": a.cwd,
           "op_class": op_class, "ops_source": single[0]["ops_source"],
           "scale": {"mode": mode, "env_var": var, "points": ks, "pinned": mode == "copies" and not a.no_pin},
           "repeats": a.repeat, "points": points,
           "noise_floor": S.noise_floor([r["ops_per_s"] for r in runs["A"]], [r["ops_per_s"] for r in runs["B"]]),
           "declared_bytes_per_op": declared, "startup_share": startup["median"] if startup else None,
           "working_set_bytes": a.working_set_bytes if a.working_set_bytes is not None else single[0].get("working_set_bytes"),
           "conditions": {"other_cpu_cores": other, "loadavg_1m": S.stats([r["conditions"]["loadavg_1m_before"] for r in allruns]),
                          "steal_cores": S.stats([r["conditions"]["steal_cores"] for r in allruns]),
                          "psi_cpu_some_avg10": S.stats([r["conditions"]["psi_cpu_some_avg10_before"] for r in allruns])},
           "flags": flags, "runs": [{k: v for k, v in r.items() if k != "stderr_tail"} for r in allruns]}
    if perf:
        pmu, mux = summarise_pmu({"core": runs["A"], "mem": runs["B"]}, None)
        doc["pmu"] = pmu
        if mux:
            doc["flags"].append("MULTIPLEXED")
            doc["pmu_multiplexed"] = mux
        if pmu.get("llc_misses_per_op") is not None and "cache-misses" not in mux:
            doc["memory_traffic"] = {"source": "llc-miss", "bytes_per_op": pmu["llc_misses_per_op"] * CACHE_LINE,
                                     "note": "user-space last-level cache misses x 64 B; a lower bound when the "
                                             "last-level prefetcher hides traffic"}
    write_json(a.out, doc)
    s = points["1"]["ops_per_s"]
    print(f"optlevel: {tier} {op_class} single {s['median']:.3e} ops/s [{s['min']:.3e}, {s['max']:.3e}] "
          f"noise {doc['noise_floor']['value']:.3f} flags {flags or 'none'} -> {a.out}", file=sys.stderr)


# ---- score -----------------------------------------------------------------
def normalise_topdown(doc):
    """profile/v1 topdown.json (tools/profile.sh topdown) -> fractions."""
    out = {}
    for m in doc.get("metrics", []):
        name = re.sub(r"[^a-z0-9]+", "_", str(m.get("metric", "")).lower()).strip("_")
        name = re.sub(r"^(tma_|td_|topdown_)", "", name)
        try:
            v = float(m.get("value"))
        except (TypeError, ValueError):
            continue
        for key in ("frontend_bound", "bad_speculation", "backend_bound", "retiring", "memory_bound", "core_bound"):
            if name == key or name.endswith("_" + key) or name.startswith(key):
                out[key] = v
    if out and max(out.values()) > 1.5:      # perf prints percentages
        out = {k: v / 100.0 for k, v in out.items()}
    out["multiplexed"] = doc.get("multiplexed", [])
    out["source"] = "tools/profile.sh topdown (profile/v1)"
    return out


def gpu_from_profile(ncu_path, nsys_path):
    g = {}
    if ncu_path:
        d = json.loads(Path(ncu_path).read_text())
        ls = d.get("launches", [])
        if not ls:
            die(f"{ncu_path} holds no ncu launches")
        top = max(ls, key=lambda x: x.get("duration_ns") or 0)
        g["ncu"] = {k: top.get(k) for k in ("kernel", "sm_throughput_pct", "memory_throughput_pct", "dram_throughput_pct",
                                             "achieved_occupancy_pct", "theoretical_occupancy_pct", "issue_active_pct",
                                             "registers_per_thread", "stalls_per_issue", "sm_clock_hz")}
        g["ncu"]["source"] = "tools/profile.sh ncu (profile/v1 kernels.json), longest launch"
    if nsys_path:
        d = json.loads(Path(nsys_path).read_text())
        cov = (d.get("timeline") or {}).get("kernel_coverage")
        g["host_idle_share"] = None if cov is None else 1.0 - cov
        g["nsys_source"] = "tools/profile.sh nsys (summary.json timeline.kernel_coverage)"
    return g


def cmd_score(a):
    calib, meas = load_json(a.calibration), load_json(a.measurement)
    if calib.get("schema") != S.CALIBRATION_SCHEMA or meas.get("schema") != S.MEASUREMENT_SCHEMA:
        die("score needs an optlevel.calibration/v1 and an optlevel.measurement/v1 document")
    extra = {}
    if a.topdown:
        if meas["tier"] != "T-pmu":
            die("a top-down profile can only be attached to a T-pmu measurement")
        extra["topdown"] = normalise_topdown(json.loads(Path(a.topdown).read_text()))
    if a.vector_share is not None:
        extra["vector_share_static"] = a.vector_share
    if a.ncu or a.nsys:
        if not meas["tier"].startswith("T-gpu"):
            die("--ncu/--nsys belong to a GPU measurement")
        g = dict(meas.get("gpu") or {})
        g.update(gpu_from_profile(a.ncu, a.nsys))
        meas["gpu"] = g
        if a.ncu:
            meas["tier"] = "T-gpu-ncu"
    alg = None
    if a.best_known_ops is not None:
        counted = a.counted_ops if a.counted_ops is not None else meas["points"]["1"]["ops"]["median"]
        alg = {"counted_ops": counted, "best_known_ops": a.best_known_ops, "ratio": counted / a.best_known_ops,
               "source": a.best_known_source or "unstated", "note": "separate axis; never folded into the score"}
    kernel = {k: v for k, v in (("repository", a.repository), ("path", a.path), ("registry_id", a.registry_id)) if v}
    rec = S.build_record(calib, meas, algorithmic=alg, extra_diag=extra, kernel=kernel)
    errs = S.validate(rec, json.loads((HERE / "schema" / "optlevel.record.v1.schema.json").read_text()))
    if errs:
        die("record violates the schema: " + "; ".join(errs[:5]))
    write_json(a.out, rec)
    print(summary_text(rec))


def summary_text(rec):
    g, att = rec["grade"], rec["attainment"]
    lines = [f"{rec['label'] or rec['kernel']['op_class']}  [{rec['tier']}]  class {rec['kernel']['op_class']}"]
    for name in ("single", "full"):
        if name in att:
            r = rec["roofline"][name]
            lines.append(f"  attainment {name:6s} {att[name]['value']:.3f} [{att[name]['low']:.3f}, {att[name]['high']:.3f}]"
                         f"  of {r['bound_ops_per_s']:.3e} ops/s ({r['binding']} roof, {r['threads']} thread(s))")
    if rec["parallel"]:
        p = rec["parallel"]
        lines.append(f"  parallel   raw {p['raw_efficiency']:.3f}, roofline-normalised {p['roofline_normalised_efficiency']:.3f} at {p['threads']} threads")
    nf = rec["noise_floor"]
    lines.append(f"  grade      {g['letter_range']} (score {g['score']:.3f}, scope {g['scope']})" if g["score"] is not None
                 else f"  grade      none ({'; '.join(g['invalid'])})")
    lines.append(f"  limiting   {g['limiting_factor']}  [{g['limiting_basis']}]")
    lines.append(f"  noise      A/A {nf['value']:.3f} over {nf['pairs']} pairs" if nf else "  noise      not measured")
    lines.append(f"  flags      {', '.join(rec['flags']) or 'none'}")
    lines.append("  This is not a speed claim or a benchmark row (docs/OPTIMIZATION_LEVEL.md).")
    return "\n".join(lines)


# ---- static vector share ---------------------------------------------------
def vector_share(binary, symbols):
    """Share of instructions in the named symbols that use packed-vector
    registers (ymm/zmm, or xmm with a packed mnemonic).  Static: it counts
    instructions in the text, not instructions executed."""
    od = shutil.which("objdump")
    if not od:
        die("vector-share needs objdump", 3)
    text = subprocess.run([od, "-d", "--no-show-raw-insn", binary], capture_output=True, text=True).stdout
    total = vec = 0
    found = set()
    cur = None
    for line in text.splitlines():
        m = re.match(r"^[0-9a-f]+ <(.+)>:$", line)
        if m:
            cur = m.group(1) if any(m.group(1) == s or m.group(1).startswith(s + ".") for s in symbols) else None
            if cur:
                found.add(cur)
            continue
        if cur and re.match(r"^\s+[0-9a-f]+:\s+\S", line):
            ins = line.split(":", 1)[1].strip()
            mnem = ins.split()[0]
            if mnem in ("nop", "nopw", "nopl", "xchg", "data16", "cs"):
                continue
            total += 1
            if re.search(r"%[yz]mm\d", ins) or (re.search(r"%xmm\d", ins) and re.match(r"v?p[a-z]+|v?[a-z]+p[sd]$", mnem)):
                vec += 1
    if not total:
        die(f"none of {symbols} found in {binary}")
    return {"binary": binary, "symbols": sorted(found), "instructions": total, "vector_instructions": vec,
            "vector_share_static": vec / total, "kind": "static (instructions in the text, not executed)"}


# ---- controls --------------------------------------------------------------
def cmd_controls(a):
    """Run the synthetic controls end to end in whatever tier the host allows."""
    out = Path(a.out)
    out.mkdir(parents=True, exist_ok=True)
    build = build_kernels()
    b = {n: v["path"] for n, v in build["builds"].items()}
    cal = out / "calibration.json"
    if not cal.exists() or a.recalibrate:
        if a.rescore:
            die("--rescore needs the calibration.json of the original run")
        cmd_calibrate(argparse.Namespace(out=str(cal), seconds=a.cal_seconds, repeat=a.cal_repeat, triad_bytes=None))
    calib = load_json(cal)
    big = calib["memory"]["triad_array_bytes"]
    ncpu = len(os.sched_getaffinity(0))
    best = calib["compute"]["modmuladd_p31"]["best_variant"].split("@")[1]
    sec = ["--seconds", str(a.seconds)]
    controls = [
        ("positive-simd-incache", "modular multiply-add row operation, vectorized, in cache (expected: near the compute roof)",
         [b[best], "modmuladd_p31", *sec], ["pass_modmuladd_p31"], b[best]),
        ("scalar-incache", "the same loop compiled with vectorization disabled (expected: clearly lower, core bound / not vectorized)",
         [b["scalar"], "modmuladd_p31", *sec], ["pass_modmuladd_p31"], b["scalar"]),
        ("serial-dependency", "the same arithmetic as a Horner chain (expected: clearly lower, core bound)",
         [b[best], "horner_p31", *sec], ["pass_horner_p31"], b[best]),
        ("stream-daxpy", "a <- a*s + b in binary64 over arrays above the last-level cache (expected: on the bandwidth slope, high attainment)",
         [b["native"], "fma_f64", "--n", str(big // 8), *sec], None, None),
        ("pointer-chase", "dependent loads over a random cycle above the last-level cache (expected: low, memory latency bound)",
         [b["native"], "chase", "--n", str(big // 64), *sec], None, None),
        ("aa-repeat", "the positive control again, an independent A/A measurement (expected: equal to the first within the noise floor)",
         [b[best], "modmuladd_p31", *sec], ["pass_modmuladd_p31"], b[best]),
    ]
    recs = []
    for name, expect, cmd, syms, binary in controls:
        m = out / f"{name}.measurement.json"
        if not (a.rescore and m.exists()):
            cmd_measure(argparse.Namespace(cmd=cmd, scale=f"copies:1,{ncpu}" if ncpu > 1 else None, timing_only=a.timing_only,
                                           cwd=None, ops=None, ops_regex=None, no_pin=False, timeout=None, warmup=False,
                                           repeat=a.repeat, op_class=None, bytes_per_op=None, working_set_bytes=None, out=str(m), label=name))
        meas = load_json(m)
        extra = {}
        if syms:
            vs = vector_share(binary, syms)
            extra["vector_share_static"] = vs["vector_share_static"]
        rec = S.build_record(calib, meas, extra_diag=extra,
                             kernel={"repository": "isogeny-labs/crypto", "path": "tools/optlevel/kernels/optk.c", "expectation": expect})
        write_json(out / f"{name}.record.json", rec)
        recs.append(rec)
        print(summary_text(rec), file=sys.stderr)
    P.write_roofline(calib, recs, out / "controls-roofline.svg", title="optlevel controls")
    rows = ["| control | tier | class | binding roof | A single [low, high] | A full | raw par. eff. | norm. par. eff. | score | grade | A/A noise | other CPU (cores) | limiting factor |",
            "|:--|:--|:--|:--|:--|--:|--:|--:|--:|:--|--:|--:|:--|"]
    for r in recs:
        s, f, p, g = r["attainment"]["single"], r["attainment"].get("full"), r["parallel"], r["grade"]
        rows.append(f"| {r['label']} | {r['tier']} | {r['kernel']['op_class']} | {r['roofline']['single']['binding']} | "
                    f"{s['value']:.3f} [{s['low']:.3f}, {s['high']:.3f}] | {f['value']:.3f} | {p['raw_efficiency']:.3f} | "
                    f"{p['roofline_normalised_efficiency']:.3f} | {g['score'] if g['score'] is None else round(g['score'], 3)} | "
                    f"{g['letter_range']} | {r['noise_floor']['value']:.3f} | {r['conditions']['other_cpu_cores']['median']:.2f} | "
                    f"{g['limiting_factor']} |")
    (out / "controls.md").write_text("\n".join(rows) + "\n")
    print("\n".join(rows))


# ---- GPU path --------------------------------------------------------------
def gpu_list():
    smi = shutil.which("nvidia-smi")
    if not smi:
        return None, "nvidia-smi is not installed: no NVIDIA GPU is visible to this host"
    r = subprocess.run([smi, "--query-gpu=name,memory.total,driver_version,compute_cap", "--format=csv,noheader"],
                       capture_output=True, text=True)
    gpus = [x.strip() for x in r.stdout.splitlines() if x.strip()]
    if r.returncode != 0 or not gpus:
        return None, "nvidia-smi lists no GPU"
    return gpus, None


def parse_smi_samples(text):
    """Rows of `--query-gpu=utilization.gpu,utilization.memory,power.draw,temperature.gpu,clocks_throttle_reasons.active
    --format=csv,noheader,nounits`."""
    util, mem, power, temp, thr = [], [], [], [], 0
    for line in text.splitlines():
        f = [x.strip() for x in line.split(",")]
        if len(f) < 5:
            continue
        try:
            util.append(float(f[0])); mem.append(float(f[1])); power.append(float(f[2])); temp.append(float(f[3]))
        except ValueError:
            continue
        if f[4] not in ("0x0000000000000000", "0x0000000000000001", "[N/A]", "N/A", ""):   # 0x1 = GPU idle
            thr += 1
    n = len(util)
    return {"samples": n, "utilization_gpu_pct": S.stats(util), "utilization_memory_pct": S.stats(mem),
            "power_w": S.stats(power), "temperature_c": S.stats(temp),
            "throttled_sample_share": thr / n if n else None,
            "note": "nvidia-smi utilisation is the share of samples with a kernel resident; it is not SM activity"}


def cmd_gpu(a):
    gpus, why = gpu_list()
    if gpus is None:
        doc = {"schema": "optlevel.unsupported/v1", "status": "unsupported", "reason": why, "host": host_info()}
        if getattr(a, "out", None):
            write_json(a.out, doc)
        print(json.dumps(doc["reason"]))
        die(f"GPU path unsupported on this host: {why}", 3)
    host = host_info()
    host["gpus"] = gpus
    host["fingerprint"] = hashlib.sha256((host["fingerprint"] + "|".join(gpus)).encode()).hexdigest()[:16]
    if a.gcmd == "probe":
        print(json.dumps({"gpus": gpus, "nvcc": shutil.which("nvcc"), "ncu": shutil.which("ncu"), "nsys": shutil.which("nsys")}, indent=1))
        return
    if a.gcmd == "calibrate":
        nvcc = shutil.which("nvcc")
        if not nvcc:
            die("gpu calibrate needs nvcc to build kernels/gpu_calib.cu", 3)
        exe = cache_dir() / "gpu_calib"
        flags = ["-O3", "-std=c++17"]
        r = subprocess.run([nvcc, *flags, "-o", str(exe), str(HERE / "kernels" / "gpu_calib.cu")], capture_output=True, text=True)
        if r.returncode != 0:
            die(f"nvcc failed: {r.stderr}", 4)
        per = {}
        bw = []
        for _ in range(a.repeat):
            out = subprocess.run([str(exe)], capture_output=True, text=True)
            if out.returncode != 0:
                die(f"gpu_calib failed (its host re-check of results is part of the run): {out.stdout}{out.stderr}", 4)
            for line in out.stdout.splitlines():
                if S.OPS_SCHEMA not in line:
                    continue
                d = json.loads(line)
                rate = d["ops"] / d["kernel_seconds"]
                if d["op_class"] == "triad":
                    bw.append(d["bytes_declared"] / d["kernel_seconds"])
                else:
                    per.setdefault(d["op_class"], []).append(rate)
        compute = {c: {"definition": CLASS_DOC.get(c, c), "unit": "operations per second", "best_variant": "gpu_calib.cu",
                       "variants": {}, "total_ops_per_s": {"1": S.stats(v)}, "scalar_ops_per_s": None, "simd_ops_per_s": None}
                   for c, v in per.items()}
        doc = {"schema": S.CALIBRATION_SCHEMA, "tool_version": S.TOOL_VERSION, "created_utc": utcnow(), "device": "gpu",
               "host": host, "build": {"compiler": nvcc, "flags": flags, "source_sha256": sha256_file(HERE / "kernels" / "gpu_calib.cu")},
               "ceiling_rule": "max over repeats of the device rate with every SM occupied; threads '1' means one whole device",
               "compute": compute, "memory": {"bandwidth_bytes_per_s": {"1": S.stats(bw)}, "latency_ns": None,
                                               "bytes_convention": "device triad, 24 bytes per element"},
               "conditions": None, "flags": ["GPU_CEILING_UNVALIDATED_BY_AUTHOR"]}
        write_json(a.out, doc)
        return
    if a.gcmd == "measure":
        if not a.cmd:
            die("no command (put it after --)")
        smi = subprocess.Popen(["nvidia-smi", "--query-gpu=utilization.gpu,utilization.memory,power.draw,temperature.gpu,clocks_throttle_reasons.active",
                                "--format=csv,noheader,nounits", "-lms", "200"], stdout=subprocess.PIPE, text=True)
        runs = {"A": [], "B": []}
        try:
            for _ in range(a.repeat):
                for slot in ("A", "B"):
                    runs[slot].append(run_once(a.cmd, ops=a.ops, ops_regex=a.ops_regex, cwd=a.cwd))
        finally:
            smi.terminate()
            samples = parse_smi_samples(smi.communicate()[0])
        allr = runs["A"] + runs["B"]
        flags = []
        if any(r.get("ops") is None for r in allr):
            die("the command reported no operation count (optlevel.ops/v1 line, --ops or --ops-regex)", 4)
        if samples["throttled_sample_share"]:
            flags.append("GPU_THROTTLED")
        if min(r["kernel_seconds"] or r["wall_s"] for r in allr) < S.MIN_KERNEL_SECONDS:
            flags.append("TOO_SHORT")
        if any(r["kernel_seconds"] is None for r in allr):
            flags.append("WALL_INCLUDES_STARTUP")
        declared = a.bytes_per_op
        doc = {"schema": S.MEASUREMENT_SCHEMA, "tool_version": S.TOOL_VERSION, "created_utc": utcnow(), "label": a.label,
               "tier": "T-gpu-timing", "host": host, "command": a.cmd, "op_class": a.op_class or allr[0].get("op_class"),
               "ops_source": allr[0]["ops_source"], "repeats": a.repeat,
               "points": {"1": {"threads": 1, "mode": "device", "ops_per_s": S.stats([r["ops_per_s"] for r in allr]),
                                "ops": S.stats([r["ops"] for r in allr]), "wall_s": S.stats([r["wall_s"] for r in allr])}},
               "noise_floor": S.noise_floor([r["ops_per_s"] for r in runs["A"]], [r["ops_per_s"] for r in runs["B"]]),
               "declared_bytes_per_op": declared, "startup_share": None,
               "gpu": {"nvidia_smi": samples, "host_cpu_utilisation": S.stats([r["cpu_s"] / r["wall_s"] for r in allr])},
               "conditions": {"other_cpu_cores": S.stats([r["conditions"]["other_cpu_cores"] for r in allr])},
               "flags": flags}
        write_json(a.out, doc)


# ---- small commands --------------------------------------------------------
def cmd_probe(a):
    host = host_info()
    perf, why = pmu_probe()
    gpus, gwhy = gpu_list()
    print(json.dumps({"host": host, "cpu_tier": "T-pmu" if perf else "T-timing", "perf": perf, "pmu_unavailable_reason": why,
                      "gpu": gpus, "gpu_unavailable_reason": gwhy, "ncu": shutil.which("ncu"),
                      "uncore_imc": [s for s in host["event_sources"] if s.startswith("uncore_imc")]}, indent=1))


def cmd_validate(a):
    schema = json.loads((HERE / "schema" / "optlevel.record.v1.schema.json").read_text())
    bad = 0
    for p in a.records:
        rec = json.loads(Path(p).read_text())
        errs = S.validate(rec, schema)
        try:
            S.enforce_tier(rec)
        except ValueError as e:
            errs.append(str(e))
        bad += bool(errs)
        print(f"{p}: {'ok' if not errs else '; '.join(errs)}")
    sys.exit(1 if bad else 0)


def cmd_benchrec(a):
    rec = json.loads(Path(a.record).read_text())
    for m in S.benchrec_metrics(rec):
        args = ["--name", m["name"], "--value", repr(m["value"]), "--unit", m["unit"], "--better", m["better"],
                "--reference-kind", "none", "--note", f"optlevel {rec['tier']}; not a speed claim"]
        for k, f in (("n", "--n"), ("spread_low", "--spread-low"), ("spread_high", "--spread-high")):
            if m.get(k) is not None:
                args += [f, repr(m[k])]
        print("python3 tools/benchrec.py metric " + " ".join(shlex.quote(x) for x in args) + " <record>")


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="sub", required=True)
    sub.add_parser("probe").set_defaults(fn=cmd_probe)
    c = sub.add_parser("calibrate")
    c.add_argument("--out", required=True)
    c.add_argument("--seconds", type=float, default=0.5, help="timed seconds per kernel run")
    c.add_argument("--repeat", type=int, default=3)
    c.add_argument("--triad-bytes", type=int, default=None, help="bytes per triad array (default 1.5 x LLC)")
    c.set_defaults(fn=cmd_calibrate)

    def measure_args(m):
        m.add_argument("--out", required=True)
        m.add_argument("--label", default="")
        m.add_argument("--repeat", type=int, default=5, help="A/B pairs of the single-thread point (>= 5 for a full noise floor)")
        m.add_argument("--ops", type=int, default=None, help="operation count of one run, when the command prints none")
        m.add_argument("--ops-regex", default=None, help="regex with one group; matches in stdout+stderr are summed")
        m.add_argument("--op-class", default=None)
        m.add_argument("--bytes-per-op", type=float, default=None, help="declared memory traffic per operation")
        m.add_argument("--working-set-bytes", type=int, default=None, help="bytes the kernel touches repeatedly; at or below the LLC, declared traffic is not charged to DRAM")
        m.add_argument("--cwd", default=None)
        m.add_argument("cmd", nargs=argparse.REMAINDER)
    m = sub.add_parser("measure")
    measure_args(m)
    m.add_argument("--scale", default=None, help="copies:1,4 (pinned concurrent copies) or env:VAR:1,2,4 (thread-count variable)")
    m.add_argument("--timing-only", action="store_true", help="do not use perf even when a PMU exists")
    m.add_argument("--no-pin", action="store_true")
    m.add_argument("--warmup", action="store_true", help="one unrecorded run first")
    m.add_argument("--timeout", type=float, default=None)
    m.set_defaults(fn=cmd_measure)
    s = sub.add_parser("score")
    s.add_argument("--calibration", required=True)
    s.add_argument("--measurement", required=True)
    s.add_argument("--out", required=True)
    s.add_argument("--topdown", help="topdown.json from tools/profile.sh topdown (T-pmu only)")
    s.add_argument("--vector-share", type=float, help="static vector share from `optlevel.py vector-share`")
    s.add_argument("--ncu", help="kernels.json from tools/profile.sh ncu")
    s.add_argument("--nsys", help="summary.json from tools/profile.sh nsys")
    s.add_argument("--counted-ops", type=float)
    s.add_argument("--best-known-ops", type=float, help="best known operation count for the task (algorithmic axis)")
    s.add_argument("--best-known-source")
    s.add_argument("--repository")
    s.add_argument("--path")
    s.add_argument("--registry-id")
    s.set_defaults(fn=cmd_score)
    p = sub.add_parser("plot")
    p.add_argument("--calibration", required=True)
    p.add_argument("--out", required=True)
    p.add_argument("--title", default="Roofline")
    p.add_argument("records", nargs="+")
    p.set_defaults(fn=lambda a: P.write_roofline(load_json(a.calibration), [json.loads(Path(r).read_text()) for r in a.records], a.out, title=a.title))
    k = sub.add_parser("controls")
    k.add_argument("--out", required=True)
    k.add_argument("--repeat", type=int, default=5)
    k.add_argument("--seconds", type=float, default=1.0)
    k.add_argument("--cal-seconds", type=float, default=0.5)
    k.add_argument("--cal-repeat", type=int, default=3)
    k.add_argument("--recalibrate", action="store_true")
    k.add_argument("--rescore", action="store_true", help="reuse the measurement files already in --out; only score and plot again")
    k.add_argument("--timing-only", action="store_true")
    k.set_defaults(fn=cmd_controls)
    v = sub.add_parser("vector-share")
    v.add_argument("binary")
    v.add_argument("symbols", nargs="+")
    v.set_defaults(fn=lambda a: print(json.dumps(vector_share(a.binary, a.symbols), indent=1)))
    g = sub.add_parser("gpu")
    gs = g.add_subparsers(dest="gcmd", required=True)
    gs.add_parser("probe")
    gc = gs.add_parser("calibrate")
    gc.add_argument("--out", required=True)
    gc.add_argument("--repeat", type=int, default=3)
    gm = gs.add_parser("measure")
    measure_args(gm)
    g.set_defaults(fn=cmd_gpu)
    va = sub.add_parser("validate")
    va.add_argument("records", nargs="+")
    va.set_defaults(fn=cmd_validate)
    b = sub.add_parser("benchrec")
    b.add_argument("record")
    b.set_defaults(fn=cmd_benchrec)
    a = ap.parse_args()
    if getattr(a, "cmd", None) and a.cmd[0] == "--":
        a.cmd = a.cmd[1:]
    a.fn(a)


if __name__ == "__main__":
    main()
