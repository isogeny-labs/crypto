#!/usr/bin/env python3
"""Progress-metrics exporter for the `crypto` repo (Rust ECDLP / index-calculus lab).

Sources (all committed, read at --ref unless noted):
  docs/ic/leaderboard.json        IC leaders per regime (S, ratio_rho, largest log2 r); history via git log
  docs/ic/progress-timeline.json  dated headline series (ec-cold etc.); points carry their own dates
  docs/perf/history.json          on branch origin/perf-history: chained Ir performance index per push
  benchmarks/README.md            append-only benchmark entries (one row per study)
  docs/ic/ci/runs/*.json          ic e2e history records
  docs/curves/jacobian-transfers.json
                                exact catalog coverage counts, not attack-performance claims
  research/nist_skipped_curve_dissection_20261021/factorization/progress.json
                                certified factorization status and recorded raw relation counts
  research/nist_skip_criterion_20261025/factor_followup_20261026/results/factor-summary.json
                                proved complete integer records in the all-21 selection census
  research/nist_skip_criterion_20261025/period_ecpp_20261026/results/summary.json
                                retained original orders in the period-software trial
  research/nist_skip_criterion_20261025/freelip_replay_20261027/results/summary.json
                                distinct original orders in the recovered FreeLIP replay
  research/nist_skip_criterion_20261025/period_work_20261027/results/verification.json
                                independently checked saved work-count cells, not security
  research/nist_skip_criterion_20261025/generator_b_execution_20261029/results/verification.json
                                verified point-trait cells on 21 choices; repeated controls, not security
  research/nist_skip_criterion_20261025/expanded_cutoff_reanalysis_20261030/results/verification.json
                                verified fixed-cohort cutoff coverage, not random sampling or security
  research/nist_skip_criterion_20261025/broader_census_reanalysis_20261030/results/verification.json
                                verified broader recorded-census coverage, not historical use or security
  research/ic_aimed_gather_20261023/aa2_20261029/summary-corrected.json
                                IC collect-phase component: paired native ratio and its A/A floor
  research/explicit_relations_e2e_20261030/results-v*/
                                exact recovery counts and bounded two-target frontier; no L0 timing rank
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
from pathlib import Path

REPO = Path(os.environ.get("PM_REPO_PATH") or Path(__file__).resolve().parents[1])
TODAY = dt.date.today().isoformat()


def _git(*args, check=True):
    p = subprocess.run(["git", "-C", str(REPO), *args], capture_output=True, text=True)
    if check and p.returncode != 0:
        raise RuntimeError(f"git {' '.join(args)}: {p.stderr.strip()[:200]}")
    return p.stdout


_object_reader = None


def _close_object_reader():
    global _object_reader
    if _object_reader is not None:
        _object_reader.stdin.close()
        _object_reader.stdout.close()
        _object_reader.wait()
        _object_reader = None


atexit.register(_close_object_reader)


def show(ref, path):
    """Read committed blobs through one Git process, preserving missing=None."""
    global _object_reader
    if _object_reader is None:
        _object_reader = subprocess.Popen(
            ["git", "-C", str(REPO), "cat-file", "--batch"],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL)
    reader = _object_reader
    reader.stdin.write(f"{ref}:{path}\n".encode())
    reader.stdin.flush()
    header = reader.stdout.readline().split()
    if header and header[-1] == b"missing":
        return None
    if len(header) != 3 or header[1] != b"blob":
        raise RuntimeError(f"invalid committed metrics source: {path}")
    size = int(header[2])
    data = reader.stdout.read(size)
    if len(data) != size or reader.stdout.read(1) != b"\n":
        raise RuntimeError(f"truncated committed metrics source: {path}")
    return data.decode()


def ls(ref, path=""):
    return [l for l in _git("ls-tree", "-r", "--name-only", ref, "--", path, check=False).splitlines() if l]


def file_history(ref, path, since):
    """Yield (date, commit, text) for the newest commit per day touching `path`, newest first."""
    out = _git("log", "--first-parent", ref, f"--since={since}", "--format=%H %cs", "--", path, check=False)
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


def jacobian_catalog(ref, since):
    path = "docs/curves/jacobian-transfers.json"
    for date, commit, text in file_history(ref, path, since):
        summary = json.loads(text).get("summary") or {}
        source = f"{path}@{commit}"
        for field, label in (("curves", "screened registry entries"),
                             ("routes", "enumerated model routes")):
            emit(f"jacobian.catalog.{field}", summary.get(field), date=date,
                 problem="jacobian-transfer-catalog", unit="count", better="neutral",
                 label=f"Jacobian catalog: {label}", source=source)
        emit("jacobian.catalog.boundary_entries", (summary.get("by_class") or {}).get("boundary"),
             date=date, problem="jacobian-transfer-catalog", unit="count", better="neutral",
             label="Jacobian catalog: boundary classifications", source=source)


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


def nist_selection_period_ecpp(ref, since):
    path = "research/nist_skip_criterion_20261025/period_ecpp_20261026/results/summary.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        emit("nist.selection_period_ecpp.retained_rows", data.get("rows"),
             date=date, problem="nist-candidate-selection", unit="orders", better="neutral",
             label="Period ECPP trial: original cohort rows retained, including indeterminate",
             source=f"{path}@{commit}")


def nist_selection_exact_embedding(ref, since):
    path = "research/nist_skip_criterion_20261025/exact_embedding_20261026/results/verification.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        emit("nist.selection_exact_embedding.certified_rows", data.get("passed"),
             date=date, problem="nist-candidate-selection", unit="curves", better="higher",
             label="NIST all-21 census: exact embedding degrees with direct-power certificates",
             source=f"{path}@{commit}")


def nist_selection_freelip(ref, since):
    path = "research/nist_skip_criterion_20261025/freelip_replay_20261027/results/summary.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        emit("nist.selection_freelip.distinct_orders", data.get("distinct_orders"),
             date=date, problem="nist-candidate-selection", unit="orders", better="neutral",
             label="Recovered FreeLIP replay: distinct original orders, not repeated-call count",
             source=f"{path}@{commit}")


def nist_selection_period_work(ref, since):
    path = "research/nist_skip_criterion_20261025/period_work_20261027/results/verification.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        emit("nist.selection_period_work.verified_cells", data.get("cells_checked"),
             date=date, problem="nist-candidate-selection", unit="cells", better="neutral",
             label="Saved ECPP work: independently checked count cells; no historical-rule claim",
             source=f"{path}@{commit}")




def nist_selection_generator_traits(ref, since):
    """Export recorded coverage only; do not compute curve traits or estimates."""
    path = "research/nist_skip_criterion_20261025/generator_b_execution_20261029/results/verification.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        emit("nist.selection_generator_traits.verified_cells", data.get("trait_cells_checked"),
             date=date, problem="nist-candidate-selection", unit="cells", better="neutral",
             label="21 curve choices: verified point-trait cells under repeated controls; not security",
             source=f"{path}@{commit}")


def nist_selection_strict_elkies(ref, since):
    """Export recorded exact-character coverage, without recomputing a result."""
    path = "research/nist_skip_criterion_20261025/strict_elkies_20261029/results/verification.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        emit("nist.selection_strict_elkies.verified_character_cells",
             data.get("odd_prime_character_cells_checked"), date=date,
             problem="nist-candidate-selection", unit="cells", better="neutral",
             label="21 curve choices: strict-character cells verified; no historical-use or security claim",
             source=f"{path}@{commit}")


def nist_selection_expanded_cutoff(ref, since):
    """Export independently verified coverage from committed records only."""
    path = "research/nist_skip_criterion_20261025/expanded_cutoff_reanalysis_20261030/results/verification.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        if data.get("all_five_files_equal") is not True:
            continue
        for field, unit in (("character_cells", "cells"), ("integer_cutoffs", "cutoffs")):
            emit(f"nist.selection_expanded_cutoff.verified_{field}", data.get(field),
                 date=date, problem="nist-candidate-selection", unit=unit, better="neutral",
                 label="Fixed 21-curve cohort: independently verified coverage; not security or sampling",
                 source=f"{path}@{commit}")


def nist_selection_broader_census(ref, since):
    """Export recorded exact coverage only, without computing a new curve trait."""
    path = "research/nist_skip_criterion_20261025/broader_census_reanalysis_20261030/results/verification.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        if data.get("all_six_files_equal") is not True:
            continue
        for field, unit in (("character_cells", "cells"), ("seed_positions", "positions")):
            emit(f"nist.selection_broader_census.verified_{field}", data.get(field),
                 date=date, problem="nist-candidate-selection", unit=unit, better="neutral",
                 label="Broader recorded census: independently verified coverage; not security or sampling",
                 source=f"{path}@{commit}")


def koblitz_modular_augmentation(ref, since):
    """Export the recorded scalar for the fixed n=5 augmentation presentation."""
    path = "research/koblitz_modular_ic_20261018/rounds/22/ledger.json"
    for date, commit, text in file_history(ref, path, since):
        record = json.loads(text)
        value = record.get("native", {}).get("recovery", {}).get("augmentation_target_least_scalar")
        emit("koblitz_modular.n5_augmentation_recovery_scalar", value, date=date,
             problem=P_IC, unit="integer scalar", better="lower",
             label="Exact equivariant recovery scalar on the n=5 augmentation targets; no cost claim",
             source=f"{path}@{commit}")


def koblitz_modular_geometry(ref, since):
    """Export exact recorded certificate counts, never cost or subgroup rank."""
    base = "research/koblitz_modular_ic_20261018/rounds/"
    for round_id in (24, 25, 26):
        path = f"{base}{round_id}/results.json"
        for date, commit, text in file_history(ref, path, since):
            record = json.loads(text)
            if round_id == 24:
                geometry = record.get("written_geometric_consequences", {})
                if not geometry.get("proper_infinity_fiber_complete"):
                    continue
                value = len(record.get("points", []))
                metric = "koblitz_modular.infinity_source_points"
                label = "Original infinity fiber points under the written normalization bridge; no cost claim"
                unit = "closed points"
            elif round_id == 25:
                quotient = record.get("registered_quotient", {})
                if not (quotient.get("accepted") and quotient.get("all_pullback_residuals_zero")):
                    continue
                value = quotient.get("pullback_certificates")
                metric = "koblitz_modular.registered_quotient_pullbacks"
                label = "Exact original-source pullback identities on the registered b-unit quotient; no rank claim"
                unit = "identities"
            else:
                transport = record.get("normalization_transport", {})
                if not (transport.get("accepted") and transport.get("source_fifth_iterate_residuals_zero")):
                    continue
                value = len(transport.get("rational_base_cusps", []))
                metric = "koblitz_modular.rational_cusp_fibers_with_charts"
                label = "Rational base cusp fibers with transported charts under the written normalization bridge"
                unit = "base cusps"
            emit(metric, value, date=date, problem=P_IC, unit=unit,
                 better="neutral", label=label, source=f"{path}@{commit}")


    # Exact construction counters in the later Koblitz batches. These are
    # recorded certificate sizes, not attack rank or comparative cost.
    for round_id in (27, 28, 29):
        path = f"{base}{round_id}/results.json"
        for date, commit, text in file_history(ref, path, since):
            record = json.loads(text)
            counters = []
            if round_id == 27:
                rr = record.get("complete_RR", {})
                models = record.get("projective_models", {})
                if rr.get("accepted") and rr.get("complete_serialized_exports"):
                    counters.append(("koblitz_modular.complete_rr_sections", rr.get("sections"),
                                     "sections", "Complete prescribed source RR basis; written geometry bridge"))
                if models.get("accepted") and models.get("all_kernel_vectors_and_pivots_recombined"):
                    counters.append(("koblitz_modular.proper_source_quadrics", models.get("source_quadrics"),
                                     "homogeneous equations", "Complete source I2 basis; no IC relation-rank claim"))
                    counters.append(("koblitz_modular.proper_quotient_quadrics", models.get("quotient_quadrics"),
                                     "homogeneous equations", "Complete invariant quotient I2 basis; old coupled saturation remains separate"))
            else:
                images = record.get("ordinary_images", {})
                if images.get("accepted"):
                    count = images.get("selected_supports_cumulative", images.get("selected_supports"))
                    counters.append(("koblitz_modular.ordinary_supports_with_selected_hecke_images", count,
                                     "closed first-image supports", "Selected ordinary Hecke images under written norm bridge; C5 lifts and IC rank separate"))
            for metric, value, unit, label in counters:
                emit(metric, value, date=date, problem=P_IC, unit=unit,
                     better="neutral", label=label, source=f"{path}@{commit}")


    path = f"{base}30/results.json"
    for date, commit, text in file_history(ref, path, since):
        record = json.loads(text)
        if record.get("accepted_native") != 5 or record.get("native_failures") != 0:
            continue
        if not record.get("finite_middle_enumeration", {}).get("complete"):
            continue
        scale = record.get("scale", {}).get("records", [])
        descent = record.get("constant_extension", {}).get("records", [])
        if scale:
            value = max((r["n"] for r in scale if r.get("d") == 1), default=None)
            emit("koblitz_modular.formal_recovery_max_cyclic_length", value,
                 date=date, problem=P_IC, unit="formal cyclic length", better="neutral",
                 label="Certified generalized recovery family; larger-n geometric transfers not constructed",
                 source=f"{path}@{commit}")
        if descent:
            value = max(r["absolute_algebra_degree"] for r in descent)
            emit("koblitz_modular.ordinary_kernel_max_absolute_algebra_degree", value,
                 date=date, problem=P_IC, unit="dimension over F2", better="neutral",
                 label="Actual ordinary finite kernel algebra; no Jacobian image or IC-rank claim",
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
    for f in files:
        for line in (show(ref, f) or "").splitlines():
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


def pencil_gate_observations(ref, since):
    """Copy committed native gate counts; do not derive costs or clock admission."""
    path = "research/pencil_large_prime_20261029/validation/actual_gate_results_20261030/analysis-receipt.json"
    for date, commit, text in file_history(ref, path, since):
        record = json.loads(text)
        if record.get("schema") != "pencil-actual-gate-analysis-receipt/v1":
            continue
        for field, unit in (
            ("observed_executions", "execution records including warmups"),
            ("observed_warmup_executions", "warmup execution records"),
        ):
            emit("pencil.p739_gate." + field, record.get(field), date=date,
                 problem=P_IC, unit=unit, better="neutral",
                 label="Recorded synthetic p739 gate; no wall-time qualification",
                 source=f"{path}@{commit}")
        if record.get("all_expected_records_present") is True:
            emit("pencil.p739_gate.observed_workloads", record.get("expected_workloads"),
                 date=date, problem=P_IC, unit="distinct prescribed workloads",
                 better="neutral", label="Complete 60-target gate inventory; not the 1260-target cohort",
                 source=f"{path}@{commit}")
def ic_aimed_gather(ref, since):
    """The aimed-subset gather component (a stage measurement, not a whole-pipeline row):
    the 42-round paired collect-phase ratio main / candidate and the lower end of its
    corrected 95% interval, with the same-session A/A's upper end beside it."""
    path = "research/ic_aimed_gather_20261023/aa2_20261029/summary-corrected.json"
    for date, commit, text in file_history(ref, path, since):
        s = json.loads(text).get("sessions", {}).get("2026-10-29", {})
        ab, aa = s.get("ab_collect", {}), s.get("aa_collect", {})
        src = f"{path}@{commit}"
        emit("ic_component.aimed_gather.collect_ratio", ab.get("median"), date=date, problem=P_IC,
             unit="ratio", better="higher", label="IC collect phase, main / aimed-gather candidate, median of 42 paired rounds (stage)",
             source=src)
        emit("ic_component.aimed_gather.collect_ratio_ci95_lo", (ab.get("ci95") or {}).get("lo"), date=date, problem=P_IC,
             unit="ratio", better="higher", label="IC collect phase, main / candidate: lower end of the corrected 95% interval (stage)",
             source=src)
        emit("ic_component.aimed_gather.aa_ci95_hi", (aa.get("ci95") or {}).get("hi"), date=date, problem=P_IC,
             unit="ratio", better="lower", label="IC collect phase, same-session A/A upper end of the corrected 95% interval (stage)",
             source=src)


def explicit_gpu_frontier(ref, since, study="research/explicit_relations_e2e_20261030",
                          metric_prefix="gpu.explicit_relations"):
    """Exact completed-panel counts; no timing or global-maximum claims."""
    revision = _git("rev-parse", ref).strip()[:12]
    paths = [p for p in ls(ref, study) if p.endswith("/coverage.json")]
    requested = [f"{p.rsplit('/', 1)[0]}/{name}" for p in paths
                 for name in ("coverage.json", "export.json", "frontier.txt")]
    # One Git process avoids per-file process and remote-filesystem overhead.
    batch = subprocess.run(["git", "-C", str(REPO), "cat-file", "--batch"],
                           input="".join(f"{ref}:{p}\n" for p in requested).encode(),
                           capture_output=True, check=True).stdout
    blobs, offset = {}, 0
    for path in requested:
        end = batch.index(b"\n", offset)
        header = batch[offset:end].split()
        offset = end + 1
        if header[-1] == b"missing":
            continue
        if len(header) != 3 or header[1] != b"blob":
            raise ValueError(f"invalid committed GPU source: {path}")
        size = int(header[2])
        blobs[path] = batch[offset:offset + size].decode()
        if batch[offset + size:offset + size + 1] != b"\n":
            raise ValueError(f"truncated committed GPU source: {path}")
        offset += size + 1
    for path in paths:
        directory = path.rsplit("/", 1)[0]
        name = directory.rsplit("/", 1)[-1]
        if not name.startswith("results-v") or not name[9:].isdigit():
            continue
        coverage_text = blobs.get(path)
        export_text = blobs.get(f"{directory}/export.json")
        if not coverage_text or not export_text:
            continue
        coverage, export = json.loads(coverage_text), json.loads(export_text)
        date = export.get("created", "")[:10]
        if len(date) != 10 or date < since:
            continue
        runs = export.get("runs", [])
        verified = sum(r.get("status") == "succeeded" and
                       r.get("verification", {}).get("status") == "verified" for r in runs)
        if len(runs) != coverage.get("attempted") or verified != coverage.get("verified_recoveries"):
            raise ValueError(f"inconsistent completed GPU panel: {path}")
        prefix = f"{metric_prefix}.{name[8:]}"
        source = f"{path}@{revision}"
        for suffix, value, label in (
            ("attempted", coverage["attempted"], "recorded executions"),
            ("verified_recoveries", verified, "fresh CPU-replayed recoveries"),
            ("timeouts", sum(r.get("status") == "timeout" for r in runs), "retained solve timeouts"),
        ):
            emit(f"{prefix}.{suffix}", value, date=date, problem="gpu-generic-decomposition",
                 unit="count", better="neutral", label=f"GPU {name}: {label}; fixed bounded panel, L0",
                 source=source)
        frontier = blobs.get(f"{directory}/frontier.txt")
        if frontier:
            key, _, value = frontier.strip().partition("=")
            if key == "largest_complete_bits" and value.isdigit():
                emit(f"{prefix}.complete_field_bits", int(value), date=date,
                     problem="gpu-generic-decomposition", unit="field bits", better="neutral",
                     label=f"GPU {name}: complete two-target rung under fixed limits; not a global maximum",
                     source=f"{directory}/frontier.txt@{revision}")



def explicit_gpu_profile(ref, since):
    """Committed diagnostic coverage only; no recovery or timing promotion."""
    study = "research/explicit_relations_profile_20261031"
    revision = _git("rev-parse", ref).strip()[:12]
    for version in (1, 2):
        path = f"{study}/results-v{version}/export.json"
        text = show(ref, path)
        if text is None:
            continue
        export = json.loads(text)
        date = export.get("created", "")[:10]
        if len(date) != 10 or date < since:
            continue
        runs = export.get("runs", [])
        if len(runs) != export.get("flow", {}).get("attempted"):
            raise ValueError(f"inconsistent GPU profile coverage: {path}")
        verified = [r for r in runs if r.get("status") == "succeeded" and
                    r.get("verification", {}).get("status") == "verified"]
        if version == 1:
            samples = (
                ("verified_cells", len(verified), "byte-exact query replay cells"),
                ("verified_queries", sum(r["measurements"]["counted"]["verified_queries"]
                                         for r in verified), "byte-exact query replies"),
            )
        else:
            samples = (
                ("timeline_profiles", sum(r["arm"] == "t" for r in verified),
                 "successful Nsight Systems traces"),
                ("counter_unavailable", sum(r["measurements"]["counters"].get("native_status")
                                             == "unsupported" for r in runs),
                 "retained unavailable hardware-counter profiles"),
            )
        for suffix, value, label in samples:
            emit(f"gpu.explicit_relations.profile.v{version}.{suffix}", value,
                 date=date, problem="gpu-query-diagnostics", unit="count",
                 better="neutral", label=f"GPU profile v{version}: {label}; fixed prefixes",
                 source=f"{path}@{revision}")


def main():
    a = args()
    jacobian_catalog(a.ref, a.since)
    leaderboard(a.ref, a.since)
    timeline(a.ref)
    perf_history(a.since)
    benchmarks(a.ref, a.since)
    e2e_runs(a.ref)
    photo_volcano(a.ref)
    nist_factorization(a.ref, a.since)
    nist_selection_factor_census(a.ref, a.since)
    nist_selection_period_ecpp(a.ref, a.since)
    nist_selection_exact_embedding(a.ref, a.since)
    nist_selection_freelip(a.ref, a.since)
    nist_selection_period_work(a.ref, a.since)
    nist_selection_generator_traits(a.ref, a.since)
    nist_selection_strict_elkies(a.ref, a.since)
    nist_selection_expanded_cutoff(a.ref, a.since)
    nist_selection_broader_census(a.ref, a.since)


    ghs_ipsec_archive(a.ref, a.since)
    fall_degree(a.ref)
    curve_traits(a.ref)

    koblitz_modular_augmentation(a.ref, a.since)
    koblitz_modular_geometry(a.ref, a.since)
    pencil_gate_observations(a.ref, a.since)
    ic_aimed_gather(a.ref, a.since)
    explicit_gpu_frontier(a.ref, a.since)
    explicit_gpu_frontier(a.ref, a.since, "research/explicit_relations_affine_20261030",
                          "gpu.explicit_relations.affine")
    explicit_gpu_profile(a.ref, a.since)

if __name__ == "__main__":
    main()
