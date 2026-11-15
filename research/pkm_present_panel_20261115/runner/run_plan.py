#!/usr/bin/env python3
"""Resumable, one-process-at-a-time runner of the panel P plan (PROTOCOL.md v1, sections 3 and 4).

Usage:  run_plan.py --plan plan.json --out results/<stamp> [--only-cell C] [--dry]
                    [--instrument BIN] [--msolve BIN] [--parse-log SCRIPT] [--cpu 0] [--allow-hash-change]

Per call: `taskset -c 0 /usr/bin/time -v -o <call>.time_v.txt timeout -k 10 <cap> prlimit --as=<bytes> <cmd>`, with
every F4_* variable removed and RAYON_NUM_THREADS=1.  Native: `<instrument> solve --system X.system.json
--max-degree 64 --deadline D --out OUT --extract`; msolve: `msolve -t 1 -v 2 -c 0 -g 2 -f X.ms -o OUT`, its stdout
trace parsed by parse_msolve_log.py.  One JSON object per call is appended to <out>/rows.jsonl; the key
(cell, curve, instance, arm, engine) makes the run resumable.  A call interrupted by a loss is tagged
`rerun: true` when redone (inflight.json marker, lost.jsonl); a second loss of the same call is recorded as a
crash row, never re-run (section 3: "re-run once").  Stop rule (section 3): chain-tower (native) censored on more
than half of the instances of a (cell, curve) block -> the later cells of that m get `not_run` rows.
The runner computes no statistic: w, w^+, w^solve, ratios and verdicts belong to analysis/build_results.py.
A missing or mistyped instrument field aborts the run loudly (exit 3); nothing is defaulted.
Everything produced is [pcc:unverified].
"""
import argparse, hashlib, json, os, platform, re, shlex, shutil, signal, subprocess, sys, time
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path

ROW_SCHEMA = "pkm.present-panel.row/v1"
CENSOR_CLASSES = ("timeout", "as_rlimit", "crash")
NATIVE_REQUIRED = ["solve_field_multiplications", "extract_field_multiplications", "solving_degree", "degree_reached",
                   "basis_size", "threads_used", "cpu_s", "censored", "verified_real_decompositions",
                   "unlifted_solutions", "fp_rational_solutions"]
SYSTEM_REQUIRED = ["construction_field_multiplications", "nterms_total", "max_total_degree", "solution_semantics",
                   "nvars", "nequations"]
MSOLVE_TOTALS_REQUIRED = ["sum_rows_x_cols_x_density", "max_round_degree", "rounds", "passes", "completed"]
ALLOC_PAT = re.compile(r"memory allocation of \d+ bytes failed|cannot allocate memory|bad_alloc|out of memory|"
                       r"capacity overflow|failed to allocate|malloc", re.I)

FIELD_SOURCES = {
    "arm,cell,curve,instance,label,algebraic_count,real_count,real_count_distinct": "plan.json row (from targets.json / cell.json)",
    "nvars,nequations,max_total_degree,nterms_total,construction_field_multiplications,solution_semantics": "<arm>.system.json (exporter)",
    "solve_field_multiplications,extract_field_multiplications,solving_degree,degree_reached,basis_size,threads_used,cpu_s": "native solve JSON (--out)",
    "verified_real_decompositions,unlifted_solutions,fp_rational_solutions": "native solve JSON (--out)",
    "wall_s,max_rss_kb,user_s,sys_s": "GNU time -v file <call>.time_v.txt (wall_s_runner = runner's monotonic clock)",
    "censored": "classify(): rc 124 / 137 at the cap -> timeout; allocation-failure text or crash with large RSS -> as_rlimit; other nonzero rc -> crash; instrument censored flag -> timeout",
    "progress_last,w_lo": "<out>.progress.json (last record: ops, degree, step, elapsed_s); w_lo = ops (censored native calls only)",
    "work_proxy,max_round_degree,rounds,passes,msolve_completed": "parse_msolve_log.py JSON totals of the msolve stdout log (work_proxy_lo = partial sum on a censored call)",
    "consistent_with_classifier,w,w_plus,w_solve,ratios,verdicts": "NOT computed here: analysis/build_results.py",
}


class ContractError(Exception):
    """The instrument (or a tool) broke its stated output contract; the run stops."""


def utcnow():
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def key_of(r):
    return (r["cell"], r["curve"], r["instance"], r["arm"], r["engine"])


def sha256_file(p):
    h = hashlib.sha256()
    with open(p, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def read_rows(path):
    rows = {}
    if Path(path).is_file():
        for ln in Path(path).read_text().splitlines():
            if ln.strip():
                r = json.loads(ln)
                rows.setdefault(key_of(r), r)
    return rows


def append_row(path, row):
    with open(path, "a") as f:
        f.write(json.dumps(row) + "\n")
        f.flush()
        os.fsync(f.fileno())


# ---------------------------------------------------------------- commands
def clean_env():
    env = {k: v for k, v in os.environ.items() if not k.startswith("F4_")}
    env["RAYON_NUM_THREADS"] = "1"
    return env


def inner_cmd(row, root, results, a):
    system = (root / row["system"]) if row.get("system") else None
    out = results / row["out"]
    if row["engine"] == "native":
        return [str(a.instrument), "solve", "--system", str(system), "--max-degree", str(row["max_degree"]),
                "--deadline", str(row["deadline_s"]), "--out", str(out), "--extract"]
    return [str(a.msolve), "-t", "1", "-v", "2", "-c", "0", "-g", "2", "-f", str(system), "-o", str(out)]


def wrap_cmd(inner, row, time_v, a):
    as_bytes = int(row["as_gb"]) * 1024 ** 3
    return ["taskset", "-c", str(a.cpu), str(a.time_bin), "-v", "-o", str(time_v), "timeout", "-k", "10",
            str(row["cap_wall_s"]), "prlimit", f"--as={as_bytes}"] + inner


def stem_of(out):
    return str(out.with_suffix(""))


def parse_time_v(p):
    d = {}
    try:
        txt = Path(p).read_text()
    except OSError:
        return d
    m = re.search(r"Elapsed \(wall clock\) time.*?:\s*([\d:.]+)\s*$", txt, re.M)
    if m:
        w = 0.0
        for part in m.group(1).split(":"):
            w = w * 60 + float(part)
        d["wall_s"] = w
    for key, rx, cast in (("max_rss_kb", r"Maximum resident set size \(kbytes\):\s*(\d+)", int),
                          ("user_s", r"User time \(seconds\):\s*([\d.]+)", float),
                          ("sys_s", r"System time \(seconds\):\s*([\d.]+)", float),
                          ("exit_status", r"Exit status:\s*(-?\d+)", int)):
        m = re.search(rx, txt)
        if m:
            d[key] = cast(m.group(1))
    return d


def classify(rc, wall_s, cap, text, rss_kb, as_bytes):
    """-> (censored, basis). rc is the exit status seen by the wrapper chain."""
    if rc == 124:
        return "timeout", "timeout(1) status 124"
    if rc == 137 and wall_s is not None and wall_s >= cap - 1.5:
        return "timeout", "timeout(1) -k status 137 at the cap"
    if rc == 0:
        return "no", "rc 0"
    if ALLOC_PAT.search(text or ""):
        return "as_rlimit", "allocation-failure text in stderr/stdout"
    if rc in (134, 139, 137, -6, -11, -9) and rss_kb and rss_kb * 1024 >= 0.5 * as_bytes:
        return "as_rlimit", f"rc {rc} with max RSS {rss_kb} kB >= 50% of the address-space limit (heuristic)"
    return "crash", f"rc {rc}"


def load_progress(path):
    """Last record of <out>.progress.json (a JSON object, or JSON lines).  -> dict or None."""
    try:
        txt = Path(path).read_text()
    except OSError:
        return None
    try:
        d = json.loads(txt)
        return d[-1] if isinstance(d, list) and d else d
    except json.JSONDecodeError:
        for ln in reversed(txt.splitlines()):
            try:
                return json.loads(ln)
            except json.JSONDecodeError:
                continue
    return None


def need(d, keys, what):
    miss = [k for k in keys if k not in d or d[k] is None]
    if miss:
        raise ContractError(f"{what}: missing field(s) {miss}")


# ---------------------------------------------------------------- one call
def run_call(row, root, results, a, lost_count):
    out = results / row["out"]
    out.parent.mkdir(parents=True, exist_ok=True)
    stem = stem_of(out)
    time_v = stem + ".time_v.txt"
    system = root / row["system"]
    if not system.is_file():
        raise ContractError(f"system file missing: {system}")
    for stale in (out, Path(str(out) + ".progress.json"), Path(time_v)):
        if stale.exists():
            stale.unlink()  # a lost call's partial files
    inner = inner_cmd(row, root, results, a)
    cmd = wrap_cmd(inner, row, time_v, a)
    so, se = (stem + ".stdout", stem + ".stderr") if row["engine"] == "native" else (stem + ".log", stem + ".err")
    base = {k: row[k] for k in ("cell", "m", "t", "curve", "p", "instance", "label", "planted", "draw_index",
                                "algebraic_count", "real_count", "real_count_distinct", "B", "B_real",
                                "sym_applicable", "arm", "engine", "system", "out", "cap_wall_s", "as_gb") if k in row}
    base.update(schema=ROW_SCHEMA, seq=row["seq"], rerun=lost_count == 1, lost_count=lost_count,
                started_utc=utcnow(), cmd=" ".join(shlex.quote(c) for c in cmd))
    t0 = time.monotonic()
    with open(so, "wb") as fo, open(se, "wb") as fe:
        proc = subprocess.Popen(cmd, stdout=fo, stderr=fe, env=clean_env(), cwd=str(root), start_new_session=True)
        try:
            rc = proc.wait(timeout=row["cap_wall_s"] + 60)
            safety = False
        except subprocess.TimeoutExpired:
            os.killpg(proc.pid, signal.SIGKILL)
            rc = proc.wait()
            safety = True
        except BaseException:
            os.killpg(proc.pid, signal.SIGKILL)
            raise
    wall_runner = time.monotonic() - t0
    tv = parse_time_v(time_v)
    wall = tv.get("wall_s", wall_runner)
    txt = ""
    for p in (se, so):
        try:
            txt += Path(p).read_text(errors="replace")[-20000:]
        except OSError:
            pass
    as_bytes = int(row["as_gb"]) * 1024 ** 3
    if safety:
        cens, basis = "timeout", "runner safety kill (cap + 60 s)"
    else:
        cens, basis = classify(rc, wall, row["cap_wall_s"], txt, tv.get("max_rss_kb"), as_bytes)
    base.update(rc=rc, wall_s=wall, wall_s_runner=wall_runner, user_s=tv.get("user_s"), sys_s=tv.get("sys_s"),
                max_rss_kb=tv.get("max_rss_kb"), censored=cens, censor_basis=basis, warnings=[])
    if row["engine"] == "native":
        native_fields(base, row, root, out, cens)
    else:
        msolve_fields(base, row, out, Path(so), cens, a)
    base["status"] = "ok" if base["censored"] == "no" else "censored"
    return base


def native_fields(base, row, root, out, cens):
    sj = root / row["system"]
    sysdoc = json.loads(sj.read_text())
    need(sysdoc, SYSTEM_REQUIRED, str(sj))
    for k in SYSTEM_REQUIRED:
        base[k] = sysdoc[k]
    prog = load_progress(str(out) + ".progress.json")
    if prog is not None:
        base["progress_last"] = {k: prog.get(k) for k in ("ops", "degree", "step", "elapsed_s")}
    else:
        base["progress_last"] = None
    if cens == "no":
        if not out.is_file():
            raise ContractError(f"{out}: rc 0 but no solve JSON")
        doc = json.loads(out.read_text())
        need(doc, NATIVE_REQUIRED, str(out))
        if not isinstance(doc["verified_real_decompositions"], list):
            raise ContractError(f"{out}: verified_real_decompositions must be a list, got "
                                f"{type(doc['verified_real_decompositions']).__name__}")
        for k in NATIVE_REQUIRED:
            if k != "censored":  # the row's `censored` is the runner's classification; the flag goes to instrument_censored
                base[k] = doc[k]
        for k in ("nvars", "nequations"):
            if k in doc and doc[k] != sysdoc[k]:
                raise ContractError(f"{out}: {k} {doc[k]} != system.json {sysdoc[k]}")
        base["instrument_censored"] = doc["censored"]
        if doc["censored"]:  # the instrument stopped itself (its --deadline)
            base["censored"], base["censor_basis"] = "timeout", "instrument censored flag (deadline)"
            base["w_lo"] = max([x for x in (doc["solve_field_multiplications"],
                                            (base["progress_last"] or {}).get("ops")) if x is not None] or [None])
        else:
            base["w_lo"] = None
    else:
        base["w_lo"] = (base["progress_last"] or {}).get("ops")
        if base["w_lo"] is None:
            base["warnings"].append("censored native call without a progress record: w_lo unknown")
        if out.is_file():  # a partial JSON would be a contract breach only if rc 0; keep nothing from it
            base["warnings"].append("solve JSON present on a censored call (ignored)")


def msolve_fields(base, row, out, log, cens, a):
    pj = str(out.with_suffix("")) + ".log.json"
    r = subprocess.run([sys.executable, str(a.parse_log), str(log), "--json", pj], capture_output=True, text=True)
    if r.returncode != 0 or not Path(pj).is_file():
        raise ContractError(f"parse_msolve_log failed on {log}: {r.stderr.strip()[-300:]}")
    doc = json.loads(Path(pj).read_text())
    tot = doc.get("totals", {})
    need(tot, MSOLVE_TOTALS_REQUIRED, f"{pj} totals")
    if cens == "no":
        base["work_proxy"] = tot["sum_rows_x_cols_x_density"]
        base["work_proxy_lo"] = None
    else:
        base["work_proxy"] = None
        base["work_proxy_lo"] = tot["sum_rows_x_cols_x_density"]
    base.update(max_round_degree=tot["max_round_degree"], rounds=tot["rounds"], passes=tot["passes"],
                msolve_completed=bool(tot["completed"]), incomplete_last_row=tot.get("incomplete_last_row"),
                msolve_overall_cpu_s=(doc.get("meta") or {}).get("overall_cpu_s"))
    base["cpu_s"] = (base.get("user_s") or 0.0) + (base.get("sys_s") or 0.0) if base.get("user_s") is not None else None
    if cens == "no" and not tot["completed"]:
        base["warnings"].append("msolve rc 0 but parse_msolve_log reports completed=false")
    if cens == "no" and not out.is_file():
        base["warnings"].append("msolve rc 0 but no output file")


# ---------------------------------------------------------------- stop rule, env, bookkeeping
def cell_order(plan_rows):
    seen = []
    for r in plan_rows:
        if r["cell"] not in seen:
            seen.append(r["cell"])
    return seen


def stop_reason(row, plan_rows, rows, order):
    """Section 3 stop rule, evaluated from rows.jsonl only (stateless, so a resume agrees with the original run)."""
    i = order.index(row["cell"])
    blocks = {}
    for r in plan_rows:
        if r["arm"] == "chain-tower" and r["engine"] == "native" and r["m"] == row["m"] and order.index(r["cell"]) < i:
            blocks.setdefault((r["cell"], r["curve"]), []).append(key_of(r))
    for (cell, curve), keys in blocks.items():
        got = [rows.get(k) for k in keys]
        if any(g is None for g in got):
            continue
        n = len(keys)
        c = sum(1 for g in got if g["censored"] in CENSOR_CLASSES)
        if 2 * c > n:
            return (f"stop rule: chain-tower native censored on {c} of {n} instances of cell {cell} curve {curve}; "
                    f"later cells of m={row['m']} are not run")
    return None


def env_dump():
    keep = ("PATH", "LANG", "LC_ALL", "RAYON_NUM_THREADS", "OMP_NUM_THREADS", "TZ", "PWD")
    return {"values": {k: os.environ[k] for k in keep if k in os.environ},
            "names_only": sorted(k for k in os.environ if k not in keep and not k.startswith("F4_")),
            "F4_vars_present_in_parent_and_removed": sorted(k for k in os.environ if k.startswith("F4_")),
            "child_env_overrides": {"RAYON_NUM_THREADS": "1", "F4_*": "unset"},
            "note": "values of non-allowlisted variables are not recorded (secrets)"}


def tool_ver(cmd):
    try:
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=10)
        return ((r.stdout or r.stderr).strip().splitlines() or [""])[0]
    except Exception as e:
        return f"unavailable: {e}"


def write_env(results, plan_path, a, study):
    env = results / "env"
    env.mkdir(parents=True, exist_ok=True)
    inst = Path(a.instrument)
    msol = Path(shutil.which(str(a.msolve)) or a.msolve)
    cur = {"instrument_sha256": sha256_file(inst) if inst.is_file() else None,
           "msolve_sha256": sha256_file(msol) if msol.is_file() else None,
           "plan_sha256": sha256_file(plan_path), "runner_sha256": sha256_file(__file__)}
    stored = env / "hashes.json"
    if stored.is_file():
        old = json.loads(stored.read_text())
        diff = {k: (old.get(k), v) for k, v in cur.items() if old.get(k) != v}
        if diff and not a.allow_hash_change:
            raise ContractError(f"hash(es) changed since this results dir was started: {diff} "
                                f"(pass --allow-hash-change to record a resume with a different artefact)")
    else:
        stored.write_text(json.dumps(cur, indent=1))
        (env / "lscpu.txt").write_text(subprocess.run(["lscpu"], capture_output=True, text=True).stdout)
        (env / "env_dump.json").write_text(json.dumps(env_dump(), indent=1))
        (env / "field_sources.json").write_text(json.dumps(FIELD_SOURCES, indent=1))
        g = lambda *x: tool_ver(["git", "-C", str(study)] + list(x))
        (env / "versions.json").write_text(json.dumps({
            "git_commit": g("rev-parse", "HEAD"), "git_status_porcelain_first_line": g("status", "--porcelain"),
            "python": sys.version, "platform": platform.platform(), "uname": " ".join(platform.uname()),
            "msolve_path": str(msol), "msolve_version": tool_ver([str(msol), "-V"]) if msol.is_file() else None,
            "taskset": tool_ver(["taskset", "--version"]), "prlimit": tool_ver(["prlimit", "--version"]),
            "timeout": tool_ver(["timeout", "--version"]), "time": tool_ver([str(a.time_bin), "-V"]),
            "hashes": cur}, indent=1))
    with open(env / "resumes.jsonl", "a") as f:
        f.write(json.dumps({"utc": utcnow(), "pid": os.getpid(), "argv": sys.argv, "hashes": cur}) + "\n")


def write_progress(results, plan_rows, rows, current, extra=None):
    cnt = Counter(r.get("status") for r in rows.values())
    per = {}
    for r in plan_rows:
        d = per.setdefault(r["cell"], {"done": 0, "total": 0})
        d["total"] += 1
        d["done"] += key_of(r) in rows
    doc = {"updated_utc": utcnow(), "total": len(plan_rows), "done": len(rows), "by_status": dict(cnt),
           "current": current, "per_cell": per}
    doc.update(extra or {})
    tmp = results / "progress.json.tmp"
    tmp.write_text(json.dumps(doc, indent=1))
    os.replace(tmp, results / "progress.json")


def main(argv=None):
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    repo = Path(__file__).resolve().parents[3]
    ap.add_argument("--plan", type=Path, required=True)
    ap.add_argument("--out", type=Path, required=True)
    ap.add_argument("--only-cell")
    ap.add_argument("--dry", action="store_true")
    ap.add_argument("--instrument", type=Path, default=repo / "target/release/examples/pkm_present_panel")
    ap.add_argument("--msolve", type=Path, default=Path("/usr/bin/msolve"))
    ap.add_argument("--time-bin", type=Path, default=Path("/usr/bin/time"))
    ap.add_argument("--parse-log", type=Path,
                    default=repo / "research/pkm_m8_sparse_panel_20261114/pod/parse_msolve_log.py")
    ap.add_argument("--cpu", default="0")
    ap.add_argument("--allow-hash-change", action="store_true")
    ap.add_argument("--max-calls", type=int, help="stop after this many executed calls (testing)")
    a = ap.parse_args(argv)
    plan_path = a.plan.resolve()
    root = plan_path.parent
    results = a.out.resolve()
    plan = json.loads(plan_path.read_text())
    sha_file = root / "plan.sha256"
    if sha_file.is_file():
        want = sha_file.read_text().split()[0]
        if want != sha256_file(plan_path):
            print("run_plan: plan.json does not match plan.sha256", file=sys.stderr)
            return 3
    plan_rows = plan["rows"]
    order = cell_order(plan_rows)
    sel = [r for r in plan_rows if not a.only_cell or r["cell"] == a.only_cell]
    if a.only_cell and not sel:
        print(f"run_plan: no rows for cell {a.only_cell}", file=sys.stderr)
        return 2
    rows = read_rows(results / "rows.jsonl")

    if a.dry:
        n = 0
        for r in sel:
            if key_of(r) in rows:
                continue
            if r.get("status") == "not_applicable":
                print(f"# {r['cell']} {r['curve']} {r['instance']} {r['arm']} {r['engine']}: not_applicable")
                continue
            inner = inner_cmd(r, root, results, a)
            print(" ".join(shlex.quote(c) for c in wrap_cmd(inner, r, results / (stem_of(Path(r['out'])) + '.time_v.txt'), a)))
            n += 1
        print(f"# dry: {n} commands, {len(rows)} rows already done; env: F4_* unset, RAYON_NUM_THREADS=1", file=sys.stderr)
        return 0

    if not a.instrument.is_file():
        print(f"run_plan: instrument not found: {a.instrument}", file=sys.stderr)
        return 3
    missing = [r["system"] for r in sel if r.get("system") and key_of(r) not in rows and not (root / r["system"]).is_file()]
    if missing:
        print(f"run_plan: {len(missing)} system file(s) missing, e.g. {missing[:5]}", file=sys.stderr)
        return 3
    results.mkdir(parents=True, exist_ok=True)
    try:
        write_env(results, plan_path, a, root)
    except ContractError as e:
        print(f"run_plan: {e}", file=sys.stderr)
        return 3
    rows_path = results / "rows.jsonl"
    inflight = results / "inflight.json"
    lost = results / "lost.jsonl"
    if inflight.is_file():  # a call was interrupted by a loss
        try:
            k = tuple(json.loads(inflight.read_text())["key"])
        except Exception:
            k = None
        if k and k not in rows:
            with open(lost, "a") as f:
                f.write(json.dumps({"key": list(k), "noted_utc": utcnow()}) + "\n")
        inflight.unlink()
    lost_counts = Counter()
    if lost.is_file():
        for ln in lost.read_text().splitlines():
            if ln.strip():
                lost_counts[tuple(json.loads(ln)["key"])] += 1

    executed = 0
    for r in sel:
        k = key_of(r)
        if k in rows:
            continue
        if r.get("status") == "not_applicable":
            row = {kk: r[kk] for kk in r if kk not in ("max_degree", "deadline_s")}
            row.update(schema=ROW_SCHEMA, rerun=False, lost_count=0, written_utc=utcnow())
            append_row(rows_path, row)
            rows[k] = row
            continue
        why = stop_reason(r, plan_rows, rows, order)
        if why:
            row = {kk: r[kk] for kk in r if kk not in ("max_degree", "deadline_s", "system")}
            row.update(schema=ROW_SCHEMA, status="not_run", censored="not_run", not_run_reason=why, rerun=False,
                       lost_count=0, written_utc=utcnow())
            append_row(rows_path, row)
            rows[k] = row
            continue
        lc = lost_counts.get(k, 0)
        if lc >= 2:
            row = {kk: r[kk] for kk in r if kk not in ("max_degree", "deadline_s", "system")}
            row.update(schema=ROW_SCHEMA, status="censored", censored="crash",
                       censor_basis="call lost twice (container/session loss); not re-run again", rerun=True,
                       lost_count=lc, written_utc=utcnow())
            append_row(rows_path, row)
            rows[k] = row
            continue
        inflight.write_text(json.dumps({"key": list(k), "started_utc": utcnow(), "pid": os.getpid()}))
        write_progress(results, plan_rows, rows, {"key": list(k), "arm": r["arm"], "engine": r["engine"]})
        try:
            row = run_call(r, root, results, a, lc)
        except ContractError as e:
            inflight.unlink(missing_ok=True)
            print(f"run_plan: CONTRACT ERROR on {k}: {e}", file=sys.stderr)
            write_progress(results, plan_rows, rows, None, {"aborted": str(e)})
            return 3
        append_row(rows_path, row)
        rows[k] = row
        inflight.unlink(missing_ok=True)
        executed += 1
        write_progress(results, plan_rows, rows, None)
        if a.max_calls and executed >= a.max_calls:
            break
    write_progress(results, plan_rows, rows, None, {"finished_selection": executed == 0 or not a.max_calls})
    print(f"run_plan: executed {executed} calls; {len(rows)}/{len(plan_rows)} rows in {rows_path}", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
