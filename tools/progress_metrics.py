#!/usr/bin/env python3
"""Progress-metrics exporter for the `crypto` repo (Rust ECDLP / index-calculus lab).

Sources (all committed, read at --ref unless noted):
  docs/ic/leaderboard.json        IC leaders per regime (S, ratio_rho, largest log2 r); history via git log
  docs/ic/progress-timeline.json  dated headline series (ec-cold etc.); points carry their own dates
  docs/perf/history.json          on branch origin/perf-history: chained Ir performance index per push
  benchmarks/README.md            append-only benchmark entries (one row per study)
  docs/ic/ci/runs/*.json          ic e2e history records
  research/nist_skipped_curve_dissection_20261021/factorization/progress.json
                                certified factorization status and recorded raw relation counts
  research/nist_skip_criterion_20261025/factor_followup_20261026/results/factor-summary.json
                                proved complete integer records in the all-21 selection census
Run: python3 tools/progress_metrics.py [--ref origin/main] [--since YYYY-MM-DD]
"""
# ---- progress-metrics exporter boilerplate (copied verbatim into each repo's tools/progress_metrics.py) ----
# Contract: print one JSON object per line to stdout. Fields: metric, value, date (YYYY-MM-DD),
# problem, unit, better (higher|lower|neutral), kind (gauge|event), label, source.
# Reads committed state at --ref via `git show`; never the working tree. stdlib only.
import argparse
import atexit
import datetime as dt
import json
import os
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor
from functools import lru_cache
from pathlib import Path

REPO = Path(os.environ.get("PM_REPO_PATH") or Path(__file__).resolve().parents[1])
TODAY = dt.date.today().isoformat()
_BLOB_READER = None


def _close_blob_reader():
    global _BLOB_READER
    if _BLOB_READER is not None:
        reader, _BLOB_READER = _BLOB_READER, None
        reader.stdin.close()
        reader.stdout.close()
        try:
            reader.wait(timeout=5)
        except subprocess.TimeoutExpired:
            # All requested blobs have been read; reap only our idle reader.
            reader.terminate()
            try:
                reader.wait(timeout=5)
            except subprocess.TimeoutExpired:
                reader.kill()
                reader.wait(timeout=5)


atexit.register(_close_blob_reader)


def _git(*args, check=True):
    p = subprocess.run(["git", "-C", str(REPO), *args], capture_output=True, text=True)
    if check and p.returncode != 0:
        raise RuntimeError(f"git {' '.join(args)}: {p.stderr.strip()[:200]}")
    return p.stdout


def show(ref, path):
    # Reuse Git's object/tree cache across historical snapshots on remote filesystems.
    global _BLOB_READER
    if "\n" in ref or "\n" in path:
        raise ValueError("Git blob specifications must be single-line")
    if _BLOB_READER is None:
        _BLOB_READER = subprocess.Popen(
            ["git", "-C", str(REPO), "cat-file", "--batch"],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
    _BLOB_READER.stdin.write(f"{ref}:{path}\n".encode())
    _BLOB_READER.stdin.flush()
    header = _BLOB_READER.stdout.readline().split()
    if header and header[-1] == b"missing":
        return None
    if len(header) != 3 or header[1] != b"blob":
        raise RuntimeError(f"Expected committed blob: {ref}:{path}")
    size = int(header[2])
    data = _BLOB_READER.stdout.read(size)
    if len(data) != size or _BLOB_READER.stdout.read(1) != b"\n":
        raise RuntimeError(f"Incomplete committed blob: {ref}:{path}")
    return data.decode()


def show_many(ref, paths):
    """Read committed blobs in one Git process, retaining input order."""
    if not paths:
        return []
    request = "".join(f"{ref}:{path}\n" for path in paths).encode()
    proc = subprocess.run(["git", "-C", str(REPO), "cat-file", "--batch"],
                          input=request, capture_output=True, check=True)
    data, offset, result = proc.stdout, 0, []
    for path in paths:
        end = data.index(b"\n", offset)
        header = data[offset:end].split()
        offset = end + 1
        if header[-1] == b"missing":
            result.append(None)
            continue
        if len(header) != 3 or header[1] != b"blob":
            raise RuntimeError(f"Expected committed blob: {ref}:{path}")
        size = int(header[2])
        result.append(data[offset:offset + size].decode())
        offset += size + 1
    return result


def ls(ref, path=""):
    return [l for l in _git("ls-tree", "-r", "--name-only", ref, "--", path, check=False).splitlines() if l]


@lru_cache(maxsize=None)
def _history_log(ref, path, since):
    return _git("log", "--first-parent", ref, f"--since={since}",
                "--format=%H %cs", "--", path, check=False)


def _prefetch_histories(ref, since):
    # Independent read-only history walks; output is still emitted in source order.
    paths = [
        "docs/ic/leaderboard.json", "benchmarks/README.md",
        "research/nist_skip_criterion_20261025/factor_followup_20261026/results/factor-summary.json",
        "research/nist_skip_criterion_20261025/exact_embedding_20261026/results/verification.json",
        "research/n83_fixed_fall_catalog_20261028/evidence/summary.json",
        "research/n83_top_quotient_20261028/evidence/summary.json",
        "research/n83_fixed_factor_base_20261028/data/index.json",
    ] + ["research/n83_candidate_pdp_20261028/" + path for path in (
        "data/index.json", "evidence/pdp/paired-summary.json",
        "evidence/degree3/summary.json", "evidence/top/verified-summary.json",
        "evidence/hilbert/verified-summary.json", "evidence/binary-pair/verified-summary.json", "evidence/chain/verified-summary.json", "evidence/degree3-m3/summary.json", "evidence/degree3-continuation/summary.json", "evidence/chain-exact/verified-summary.json", "evidence/degree4-witness/summary.json", "evidence/degree4-propagation/summary.json", "evidence/degree4-quartic/summary.json")]
    with ThreadPoolExecutor(max_workers=4) as pool:
        list(pool.map(lambda path: _history_log(ref, path, since), paths))


def file_history(ref, path, since):
    """Yield (date, commit, text) for the newest commit per day touching `path`, newest first."""
    out = _history_log(ref, path, since)
    seen = set()
    for line in out.splitlines():
        h, d = line.split()
        if d in seen:
            continue
        seen.add(d)
        txt = show(h, path)
        if txt is not None:
            yield d, h[:12], txt


def emit(metric, value, *, date=None, problem="misc", unit="", better="neutral", kind="gauge", label="", source=""):
    if value is None:
        return
    try:
        v = float(value)
    except (TypeError, ValueError):
        return
    if v != v:
        return
    print(json.dumps({"metric": metric, "value": v, "date": date or TODAY, "problem": problem, "unit": unit,
                      "better": better, "kind": kind, "label": label or metric, "source": source}))


def args():
    ap = argparse.ArgumentParser()
    ap.add_argument("--ref", default=os.environ.get("PM_REF", "origin/main"))
    ap.add_argument("--since", default=os.environ.get("PM_SINCE") or (dt.date.today() - dt.timedelta(days=90)).isoformat())
    return ap.parse_args()
# ---- end boilerplate ----

P_IC = "ecdlp-index-calculus"
P_PERF = "crypto-performance"


def leaderboard(ref, since):
    for date, commit, txt in file_history(ref, "docs/ic/leaderboard.json", since):
        try:
            d = json.loads(txt)
        except json.JSONDecodeError:
            continue
        src = f"docs/ic/leaderboard.json@{commit}"
        for regime, L in (d.get("leaders") or {}).items():
            if not isinstance(L, dict):
                continue
            emit(f"ic.leader.{regime}.ratio_rho", L.get("ratio_rho"), date=date, problem=P_IC, unit="IC/rho",
                 better="lower", label=f"IC leader {regime}: cost ratio to rho (whole pipeline)", source=src)
            emit(f"ic.leader.{regime}.s", L.get("s"), date=date, problem=P_IC, unit="S",
                 better="lower", label=f"IC leader {regime}: S = ops/sqrt(n)", source=src)
            emit(f"ic.leader.{regime}.log2_r", L.get("log2_r"), date=date, problem=P_IC, unit="bits",
                 better="higher", label=f"IC leader {regime}: curve size of best ratio", source=src)
            big = L.get("largest") or {}
            emit(f"ic.leader.{regime}.largest_log2_r", big.get("log2_r"), date=date, problem=P_IC, unit="bits",
                 better="higher", label=f"IC {regime}: largest curve priced end to end", source=src)
            emit(f"ic.leader.{regime}.largest_ratio_rho", big.get("ratio_rho"), date=date, problem=P_IC, unit="IC/rho",
                 better="lower", label=f"IC {regime}: ratio to rho on the largest curve", source=src)
            if L.get("online_speedup") is not None:
                emit(f"ic.leader.{regime}.online_speedup", L.get("online_speedup"), date=date, problem=P_IC, unit="x",
                     better="higher", label=f"IC leader {regime}: online speedup vs rho", source=src)
        emit("ic.leaderboard.board_rows", len(d.get("board") or []), date=date, problem=P_IC, unit="count",
             better="higher", label="IC leaderboard: priced board rows", source=src)
        emit("ic.leaderboard.roster", len(d.get("roster") or []), date=date, problem=P_IC, unit="count",
             better="higher", label="IC leaderboard: curves in roster", source=src)


def timeline(ref):
    txt = show(ref, "docs/ic/progress-timeline.json")
    if not txt:
        return
    d = json.loads(txt)
    for s in d.get("series", []):
        sid = s.get("id", "").replace("_", "-")
        if not sid:
            continue
        last_per_day = {}
        for p in s.get("points", []):
            if p.get("date") and p.get("value") is not None:
                last_per_day[p["date"]] = p
        for date, p in last_per_day.items():
            emit(f"ic.timeline.{sid}", p["value"], date=date, problem=P_IC, unit=p.get("unit") or "IC/rho",
                 better="lower", label=f"timeline {s.get('label', sid)} ({s.get('role','')})",
                 source=f"docs/ic/progress-timeline.json point {p.get('label','')[:40]} [{p.get('class','')}]")


def perf_history(since):
    txt = show("origin/perf-history", "docs/perf/history.json")
    if not txt:
        return
    d = json.loads(txt)
    last = {}
    for snap in d.get("snapshots", []):
        day = (snap.get("committed_at") or "")[:10]
        if day >= since:
            last[day] = snap
    for day, snap in last.items():
        src = f"perf-history docs/perf/history.json rev {str(snap.get('rev',''))[:12]}"
        emit("perf.index.overall", snap.get("overall"), date=day, problem=P_PERF, unit="x",
             better="higher", label="performance index, overall (chained callgrind Ir; <5% is noise)", source=src)
        for area, v in (snap.get("areas") or {}).items():
            emit(f"perf.index.{area}", v, date=day, problem=P_PERF, unit="x", better="higher",
                 label=f"performance index: {area}", source=src)


def benchmarks(ref, since):
    for date, commit, txt in file_history(ref, "benchmarks/README.md", since):
        n = sum(1 for l in txt.splitlines() if l.startswith("| 20"))
        emit("benchmarks.entries", n, date=date, problem=P_IC, unit="count", better="higher",
             label="benchmark studies recorded (benchmarks/README.md rows)", source=f"benchmarks/README.md@{commit}")


def e2e_runs(ref):
    n = len([p for p in ls(ref, "docs/ic/ci/runs") if p.endswith(".json")])
    emit("ic.e2e.history_runs", n, problem=P_IC, unit="count", better="higher",
         label="ic e2e gate history records", source="docs/ic/ci/runs/")



def photo_volcano(ref):
    path = 'research/photo_volcano_restart_20261017/reports/completion_20261025/completion.json'
    txt = show(ref, path)
    if not txt:
        return
    data = json.loads(txt)
    for metric, keys, unit in [
        ('graph.curves', ('graph', 'curves'), 'curves'),
        ('rho.verified', ('native_verified', 'rho'), 'cases'),
        ('bsgs.verified', ('native_verified', 'bsgs'), 'cases'),
        ('rho.counted_replays', ('field_replay_verified', 'rho'), 'cases'),
        ('bsgs.counted_replays', ('field_replay_verified', 'bsgs'), 'cases'),
        ('regularity.certified', ('regularity_certified_systems',), 'systems'),
        ('ic.random_recoveries', ('IC_random_corpus_recoveries',), 'cases'),
    ]:
        value = data
        for key in keys:
            value = value.get(key) if isinstance(value, dict) else None
        emit('photo_volcano.' + metric, value, date=data.get('snapshot_date'),
             problem=P_IC, unit=unit, better='higher',
             label='Photographed selected 2-volcano: ' + metric,
             source=path + '@' + ref)

def nist_factorization(ref, since):
    path = "research/nist_skipped_curve_dissection_20261021/factorization/progress.json"
    txt = show(ref, path)
    if not txt:
        return
    data = json.loads(txt)
    for row in data.get("observations", []):
        date = row.get("date")
        if not date or date < since:
            continue
        source = f"{path}: {row.get('source', '')}"
        for field, better, unit, label in (
            ("complete_targets", "higher", "count", "NIST timeout integers completely prime-certified"),
            ("resolved_timeout_fields", "higher", "count", "NIST timeout fields resolved"),
            ("largest_residual_bits", "lower", "bits", "Remaining skipped-P256 cofactor size"),
            ("main_raw_relation_lines", "neutral", "lines", "Skipped-P256 raw relation lines; no dedup/rank claim"),
            ("filter_unique_relations", "neutral", "relations", "Native unique relations, including generated free relations"),
            ("filter_singleton_survivors", "neutral", "relations", "Native relations surviving singleton removal; no rank claim"),
        ):
            emit(f"nist.factorization.{field}", row.get(field), date=date,
                 problem="nist-skipped-curve-factorization", unit=unit, better=better,
                 label=label, source=source)

def nist_selection_factor_census(ref, since):
    path = "research/nist_skip_criterion_20261025/factor_followup_20261026/results/factor-summary.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        source = f"{path}@{commit}"
        emit("nist.selection_factor_census.complete_targets", data.get("complete_and_proven"),
             date=date, problem="nist-candidate-selection", unit="integers", better="higher",
             label="NIST all-21 census: proved complete factorization records", source=source)
        for family, count in (data.get("complete_by_family") or {}).items():
            emit(f"nist.selection_factor_census.{family}.complete_targets", count,
                 date=date, problem="nist-candidate-selection", unit="integers", better="higher",
                 label=f"NIST all-21 census: complete {family} records", source=source)


def nist_selection_exact_embedding(ref, since):
    path = "research/nist_skip_criterion_20261025/exact_embedding_20261026/results/verification.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        emit("nist.selection_exact_embedding.certified_rows", data.get("passed"),
             date=date, problem="nist-candidate-selection", unit="curves", better="higher",
             label="NIST all-21 census: exact embedding degrees with direct-power certificates",
             source=f"{path}@{commit}")



def ghs_ipsec_archive(ref, since):
    """Export recorded archive counters; no cryptographic computation or estimates."""
    path = "research/ghs_ipsec_isogeny_class_20261023/historical_python/search_snapshot.json"
    txt = show(ref, path)
    if not txt:
        return
    data = json.loads(txt)
    date = data.get("snapshot_date_utc")
    if not date or date < since:
        return
    for key in ("discovered", "screened", "expanded"):
        emit("ghs.ipsec155.archive." + key + "_frobenius_orbits",
             data.get("counters", {}).get(key), date=date,
             problem="ghs-ipsec155-isogeny-class", unit="Frobenius orbits",
             better="higher", label="Historical IPsec GHS archive: " + key + " orbits",
             source=path + "@" + ref)
def fall_degree(ref):
    """HKY last fall degree d_F of Weil-descended chained Semaev systems (unsatisfiable draws,
    research/fall_degree_bounds_20261008, E7 onward). Counts of recorded draws only; a draw not
    refuted at its cap is counted as censored, never as a value."""
    base = "research/fall_degree_bounds_20261008/"
    files = [f for f in ls(ref, base) if f.endswith(".jsonl") and (
        f.startswith(base + "e7/E7_") or f.startswith(base + "e8/E8_")
        or f == base + "e12/completed-20261028/E_3-24-7.jsonl"
        or ("/out/E_" in f and (f.startswith(base + "e9/") or f.startswith(base + "e10/") or f.startswith(base + "e12/"))))]
    draws = censored = 0
    max_n = min_censored_n = None
    for text in show_many(ref, files):
        for line in (text or "").splitlines():
            if not line.startswith("{"):
                continue
            r = json.loads(line)
            if r.get("solutions") != 0 or "n_vars" not in r:
                continue
            draws += 1
            n = r["n_vars"]
            max_n = n if max_n is None else max(max_n, n)
            if r.get("d_f") is None:
                censored += 1
                min_censored_n = n if min_censored_n is None else min(min_censored_n, n)
    if not draws:
        return
    src = base + "{e7,e8,e9,e10,e12}"
    emit("fall_degree.draws_measured", draws, problem="fall-degree", unit="draws", label="Unsatisfiable chained Semaev draws with a recorded d_F (E7 onward)", source=src)
    emit("fall_degree.draws_censored", censored, problem="fall-degree", unit="draws", label="Draws whose closure did not reach 1 at its cap (d_F above the cap)", source=src)
    emit("fall_degree.max_n_vars_measured", max_n, problem="fall-degree", unit="variables", better="higher", label="Largest N with a recorded d_F draw", source=src)
    emit("fall_degree.min_n_vars_censored", min_censored_n, problem="fall-degree", unit="variables", label="Smallest N with a draw censored above its cap", source=src)


def n83_fixed_fall(ref, since):
    """Forward recorded fixed-base certificates, without inferring regularity."""
    path = "research/n83_fixed_fall_catalog_20261028/evidence/summary.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        for row in data.get("rows", []):
            status = row.get("closure_status", {})
            if status.get("closure_exit_code") != 0 or status.get("replay_exit_code") != 0:
                continue
            for field in ("last_fall_degree", "closure_rank", "linear_rank"):
                emit(f"n83.fixed_ell27.{row['id']}.{field}", row.get(field),
                     date=date, problem="n83-fixed-factor-base-fall-degree",
                     unit="degree" if field == "last_fall_degree" else "rank",
                     label=f"Fixed ell27 {row['id']}: {field}; producer replay, review pending",
                     source=f"{path}@{commit}")


def n83_top_quotient(ref, since):
    """Forward certified top-form bounds; never turn an interval into a value."""
    path = "research/n83_top_quotient_20261028/evidence/summary.json"
    for date, commit, text in file_history(ref, path, since):
        for row in json.loads(text).get("rows", []):
            if row.get("status") not in ("finite_interval", "positive_dimensional_top_ideal"):
                continue
            for field in ("dreg_lower_bound", "dreg_upper_bound", "top_quotient_dimension_lower_bound"):
                emit(f"n83.fixed_top_ell27.{row['id']}.{field}", row.get(field),
                     date=date, problem="n83-fixed-factor-base-top-regularity",
                     unit="dimension" if field == "top_quotient_dimension_lower_bound" else "degree",
                     label=f"Fixed ell27 {row['id']}: {field}; producer certificate, review pending",
                     source=f"{path}@{commit}")


def n83_candidate_datasets(ref, since):
    """Forward native point counts and replayed fall fields; unknowns stay absent."""
    study = "research/n83_candidate_pdp_20261028"
    for name, path in (
        ("baseline", "research/n83_fixed_factor_base_20261028/data/index.json"),
        ("spike40", f"{study}/data/index.json"),
    ):
        for date, commit, text in file_history(ref, path, since):
            index = json.loads(text)
            if index.get("complete") is not True:
                continue
            for family in index.get("families", []):
                for field, unit in (("point_count", "points"), ("liftable_x", "x-values"),
                                    ("compressed_bytes", "bytes")):
                    emit(f"n83.base_ell27.{name}.{family['family']}.{field}", family.get(field),
                         date=date, problem="n83-factor-base-datasets", unit=unit,
                         label=f"{name} {family['family']} exhaustive base: {field}; no degree claim",
                         source=f"{path}@{commit}")
    path = f"{study}/evidence/pdp/paired-summary.json"
    for date, commit, text in file_history(ref, path, since):
        for pair in json.loads(text).get("rows", []):
            if pair.get("targets_identical") is not True:
                continue
            for arm in ("baseline", "spike40"):
                row = pair.get(arm, {})
                status = row.get("closure_status", {})
                if status.get("closure_exit_code") != 0 or status.get("replay_exit_code") != 0:
                    continue
                for field in ("last_fall_degree", "closure_rank", "linear_rank"):
                    emit(f"n83.paired_v2.{arm}.{pair['id']}.{field}", row.get(field),
                         date=date, problem="n83-paired-fall-degree",
                         unit="degree" if field == "last_fall_degree" else "rank",
                         label=f"Paired V2 {arm} {pair['id']}: {field}; producer replay, review pending",
                         source=f"{path}@{commit}")
    path = f"{study}/evidence/degree3/summary.json"
    for date, commit, text in file_history(ref, path, since):
        for row in json.loads(text).get("rows", []):
            status = row.get("status", {})
            if (row.get("verified") is not True or status.get("run_exit_code") != 0
                    or status.get("replay_exit_code") != 0):
                continue
            for field in ("last_fall_degree", "last_fall_lower_bound", "last_fall_upper_bound",
                          "closure_rank", "linear_rank"):
                emit(f"n83.paired_v3.{row['key']}.{field}", row.get(field),
                     date=date, problem="n83-paired-fall-degree",
                     unit="rank" if field.endswith("rank") else "degree",
                     label=f"Paired V3 {row['key']}: {field}; producer replay, review pending",
                     source=f"{path}@{commit}")
    path = f"{study}/evidence/top/verified-summary.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        for row in data.get("rows", []):
            if row.get("verified") is not True:
                continue
            for field in ("dreg_lower_bound", "dreg_upper_bound", "linear_top_rank",
                          "top_quotient_dimension_lower_bound"):
                emit(f"n83.paired_top.{row['key']}.{field}", row.get(field),
                     date=date, problem="n83-paired-top-regularity",
                     unit="degree" if field.startswith("dreg_") else "dimension",
                     label=f"Original top ideal {row['key']}: {field}; certified bound, review pending",
                     source=f"{path}@{commit}")
        for proof in data.get("prime_isomorphisms", []):
            if proof.get("verified") is True:
                emit(f"n83.paired_top.{proof['id']}.graded_quotient_isomorphic", 1,
                     date=date, problem="n83-paired-top-regularity", unit="certificate",
                     label=f"Prime paired top quotients {proof['id']}: diagonal isomorphism; review pending",
                     source=f"{path}@{commit}")


    path = f"{study}/evidence/hilbert/verified-summary.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        status = data.get("status", {})
        if (status.get("producer_exit_code") != 0 or status.get("replay_exit_code") != 0
                or data.get("replay_passed") is not True):
            continue
        for row in data.get("rows", []):
            if row.get("verified") is not True:
                continue
            for field in ("dreg_exact", "last_nonzero_degree", "total_dimension"):
                emit(f"n83.prime_m2_hilbert.{row['key']}.{field}", row.get(field),
                     date=date, problem="n83-prime-m2-top-regularity",
                     unit="dimension" if field == "total_dimension" else "degree",
                     label=f"Original top ideal {row['key']}: exact {field}; producer replay, review pending",
                     source=f"{path}@{commit}")


    path = f"{study}/evidence/binary-pair/verified-summary.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        status = data.get("status", {})
        if (status.get("producer_exit_code") != 0 or status.get("replay_exit_code") != 0
                or data.get("replay_passed") is not True):
            continue
        for row in data.get("rows", []):
            if row.get("verified") is not True:
                continue
            for field in ("dreg_exact", "dreg_lower_bound", "dreg_upper_bound", "q_rank", "product_rank"):
                emit(f"n83.binary_m2_pair.{row['key']}.{field}", row.get(field),
                     date=date, problem="n83-binary-m2-top-regularity",
                     unit="rank" if field.endswith("rank") else "degree",
                     label=f"Original top ideal {row['key']}: {field}; producer replay, review pending",
                     source=f"{path}@{commit}")


    path = f"{study}/evidence/chain/verified-summary.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        status = data.get("status", {})
        if (status.get("producer_exit_code") != 0 or status.get("replay_exit_code") != 0
                or data.get("replay_passed") is not True):
            continue
        for row in data.get("rows", []):
            if row.get("verified") is not True:
                continue
            for field in ("dreg_lower_bound", "dreg_upper_bound", "selected_blocks"):
                emit(f"n83.binary_chain.{row['key']}.{field}", row.get(field),
                     date=date, problem="n83-binary-chain-top-regularity",
                     unit="blocks" if field == "selected_blocks" else "degree",
                     label=f"Original top ideal {row['key']}: {field}; certified bound, review pending",
                     source=f"{path}@{commit}")

    path = f"{study}/evidence/chain-exact/verified-summary.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        status = data.get("status", {})
        if (status.get("producer_exit_code") != 0 or status.get("replay_exit_code") != 0
                or data.get("replay_passed") is not True):
            continue
        for row in data.get("rows", []):
            if row.get("verified") is not True:
                continue
            for field in ("dreg_exact", "dreg_lower_bound", "dreg_upper_bound", "defect_socle_upper"):
                emit(f"n83.binary_chain_exact.{row['key']}.{field}", row.get(field),
                     date=date, problem="n83-binary-chain-top-regularity",
                     unit="degree",
                     label=f"Original top ideal {row['key']}: {field}; written overlap proof, review pending",
                     source=f"{path}@{commit}")

    path = f"{study}/evidence/degree3-m3/summary.json"
    for date, commit, text in file_history(ref, path, since):
        for row in json.loads(text).get("rows", []):
            status = row.get("status", {})
            if (row.get("verified") is not True or status.get("run_exit_code") != 0
                    or status.get("replay_exit_code") != 0):
                continue
            for field in ("last_fall_degree", "last_fall_lower_bound", "last_fall_upper_bound",
                          "closure_rank", "linear_rank"):
                emit(f"n83.m3_v3.{row['key']}.{field}", row.get(field),
                     date=date, problem="n83-m3-fall-degree",
                     unit="rank" if field.endswith("rank") else "degree",
                     label=f"Retained m3 V3 {row['key']}: {field}; admitted subspace; complete={row.get('closure_complete')}; review pending",
                     source=f"{path}@{commit}")


    path = f"{study}/evidence/degree3-continuation/summary.json"
    for date, commit, text in file_history(ref, path, since):
        for row in json.loads(text).get("rows", []):
            status = row.get("status", {})
            if (row.get("verified") is not True or status.get("run_exit_code") != 0
                    or status.get("replay_exit_code") != 0):
                continue
            for field in ("last_fall_degree", "last_fall_lower_bound", "last_fall_upper_bound",
                          "closure_rank", "linear_rank", "added_rows", "added_operations", "closure_complete"):
                unit = ("rank" if field.endswith("rank") else "rows" if field == "added_rows"
                        else "operations" if field == "added_operations" else "flag" if field == "closure_complete" else "degree")
                emit(f"n83.m3_continue.{row['key']}.{field}", row.get(field),
                     date=date, problem="n83-m3-fall-degree", unit=unit,
                     label=f"Retained m3 continuation {row['key']}: {field}; complete={row.get('closure_complete')}; witness={row.get('witness_origin')}; review pending",
                     source=f"{path}@{commit}")


    path = f"{study}/evidence/degree4-witness/summary.json"
    for date, commit, text in file_history(ref, path, since):
        for row in json.loads(text).get("rows", []):
            if (row.get("verified") is not True or row.get("run_exit_code") != 0
                    or row.get("replay_exit_code") != 0):
                continue
            for field in ("candidates", "strict_candidates", "v3_rank",
                          "last_fall_lower_bound", "strict_fall_found"):
                unit = ("rank" if field == "v3_rank" else "degree" if field == "last_fall_lower_bound"
                        else "flag" if field == "strict_fall_found" else "candidates")
                emit(f"n83.m3_degree4.{row['key']}.{field}", row.get(field),
                     date=date, problem="n83-m3-fall-degree", unit=unit,
                     label=f"Retained m3 degree4 witness family {row['key']}: {field}; finite family only; review pending",
                     source=f"{path}@{commit}")


    path = f"{study}/evidence/degree4-propagation/summary.json"
    for date, commit, text in file_history(ref, path, since):
        for row in json.loads(text).get("rows", []):
            if (row.get("verified") is not True or row.get("run_exit_code") != 0
                    or row.get("replay_exit_code") != 0):
                continue
            values = {field: row.get(field) for field in (
                "linear_rank", "last_fall_lower_bound", "last_fall_upper_bound", "last_fall_degree")}
            values.update({field: row["state"][field] for field in (
                "added_rows", "independent_seed_rows", "processed_operations", "pending_candidates")})
            for field, value in values.items():
                unit = ("degree" if field.startswith("last_fall") else "rank" if field == "linear_rank"
                        else "operations" if field == "processed_operations" else "rows" if field.endswith("rows") else "candidates")
                emit(f"n83.m3_propagation.{row['key']}.{field}", value,
                     date=date, problem="n83-m3-fall-degree", unit=unit,
                     label=f"Retained m3 V4 consequences {row['key']}: {field}; stop={row['state']['stop_reason']}; lower inherited; no unrestricted V4; review pending",
                     source=f"{path}@{commit}")


    path = f"{study}/evidence/degree4-quartic/summary.json"
    for date, commit, text in file_history(ref, path, since):
        for row in json.loads(text).get("rows", []):
            if (row.get("verified") is not True or row.get("run_exit_code") != 0
                    or row.get("replay_exit_code") != 0):
                continue
            values = {field: row.get(field) for field in (
                "linear_rank", "last_fall_lower_bound", "last_fall_upper_bound", "last_fall_degree",
                "complete_v4_normal_forms")}
            values.update({field: row["state"][field] for field in (
                "added_rows", "attempted_products", "completed_products", "pending_products",
                "new_basis_payload_bytes")})
            values.update({f"added_degree_{degree}_rows": count
                           for degree, count in enumerate(row["state"]["added_rows_by_degree"])})
            for field, value in values.items():
                unit = ("degree" if field.startswith("last_fall") else "rank" if field == "linear_rank"
                        else "flag" if field == "complete_v4_normal_forms"
                        else "bytes" if field.endswith("bytes")
                        else "rows" if field.endswith("rows") else "products")
                emit(f"n83.m3_quartic.{row['key']}.{field}", value,
                     date=date, problem="n83-m3-fall-degree", unit=unit,
                     label=f"Retained m3 quartic closure {row['key']}: {field}; stop={row['state']['stop_reason']}; global upper requires separate certificate; no base-quality ranking; review pending",
                     source=f"{path}@{commit}")


def curve_traits(ref):
    """End(E) and Frobenius-discriminant status counts in the committed curve-trait catalogue
    (docs/curves/traits.json, built by curve_traits from the registry and the factor hints)."""
    path = "docs/curves/traits.json"
    txt = show(ref, path)
    if not txt:
        return
    curves = json.loads(txt).get("curves", [])
    end = {}
    for r in curves:
        st = (r.get("endomorphism") or {}).get("status")
        end[st] = end.get(st, 0) + 1
    unfactored = sum(1 for r in curves if (r.get("frobenius") or {}).get("status") == "bounded")
    hints = sum(1 for l in (show(ref, "docs/curves/factor-hints.txt") or "").splitlines() if l.strip().isdigit())
    emit("curve_traits.curves", len(curves), problem="curve-traits", unit="curves", label="Curves in the trait catalogue", source=path)
    for st, better, label in (
        ("proved", "neutral", "Curves with End(E) proved"),
        ("probable", "neutral", "Curves with End(E) exact if every probable prime under it is prime"),
        ("bounded", "lower", "Curves with End(E) only bounded (open volcano levels)"),
        ("unknown", "lower", "Curves with End(E) unknown (Frobenius discriminant not factored)"),
    ):
        emit(f"curve_traits.end.{st}", end.get(st, 0), problem="curve-traits", unit="curves", better=better, label=label, source=path)
    emit("curve_traits.disc_unfactored", unfactored, problem="curve-traits", unit="curves", better="lower",
         label="Curves whose Frobenius discriminant has an unsplit composite", source=path)
    emit("curve_traits.factor_hints", hints, problem="curve-traits", unit="primes", better="neutral",
         label="Proved primes in docs/curves/factor-hints.txt", source="docs/curves/factor-hints.txt")


def main():
    a = args()
    _prefetch_histories(a.ref, a.since)
    leaderboard(a.ref, a.since)
    timeline(a.ref)
    perf_history(a.since)
    benchmarks(a.ref, a.since)
    e2e_runs(a.ref)
    photo_volcano(a.ref)
    nist_factorization(a.ref, a.since)
    nist_selection_factor_census(a.ref, a.since)
    nist_selection_exact_embedding(a.ref, a.since)
    ghs_ipsec_archive(a.ref, a.since)
    fall_degree(a.ref)
    n83_fixed_fall(a.ref, a.since)
    n83_top_quotient(a.ref, a.since)
    n83_candidate_datasets(a.ref, a.since)

    curve_traits(a.ref)


if __name__ == "__main__":
    main()
