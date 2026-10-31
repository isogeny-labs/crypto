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
Run: python3 tools/progress_metrics.py [--ref origin/main] [--since YYYY-MM-DD]
"""
# ---- progress-metrics exporter boilerplate (copied verbatim into each repo's tools/progress_metrics.py) ----
# Contract: print one JSON object per line to stdout. Fields: metric, value, date (YYYY-MM-DD),
# problem, unit, better (higher|lower|neutral), kind (gauge|event), label, source.
# Reads committed state at --ref via `git show`; never the working tree. stdlib only.
import argparse
from concurrent.futures import ThreadPoolExecutor
from functools import lru_cache
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


def show(ref, path):
    p = subprocess.run(["git", "-C", str(REPO), "show", f"{ref}:{path}"], capture_output=True, text=True)
    return p.stdout if p.returncode == 0 else None


def ls(ref, path=""):
    return [l for l in _git("ls-tree", "-r", "--name-only", ref, "--", path, check=False).splitlines() if l]


def _uncached_file_history(ref, path, since):
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


@lru_cache(maxsize=None)
def _history_rows(ref, path, since):
    return tuple(_uncached_file_history(ref, path, since))


def file_history(ref, path, since):
    return iter(_history_rows(ref, path, since))


def show_many(ref, paths):
    """Read committed text blobs in one size-framed Git batch."""
    if not paths:
        return []
    requests = "".join(f"{ref}:{path}\n" for path in paths).encode()
    proc = subprocess.run(["git", "-C", str(REPO), "cat-file", "--batch"],
                          input=requests, capture_output=True, check=True)
    data = proc.stdout
    pos = 0
    values = []
    for path in paths:
        end = data.index(b"\n", pos)
        header = data[pos:end].split()
        pos = end + 1
        if header[-1] == b"missing":
            values.append(None)
            continue
        if len(header) != 3 or header[1] != b"blob":
            raise ValueError(f"unexpected Git object for {path}")
        size = int(header[2])
        values.append(data[pos:pos + size].decode())
        pos += size
        if data[pos:pos + 1] != b"\n":
            raise ValueError("invalid Git batch framing")
        pos += 1
    if pos != len(data):
        raise ValueError("extra Git batch output")
    return values


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


def nist_selection_exact_embedding(ref, since):
    path = "research/nist_skip_criterion_20261025/exact_embedding_20261026/results/verification.json"
    for date, commit, text in file_history(ref, path, since):
        data = json.loads(text)
        emit("nist.selection_exact_embedding.certified_rows", data.get("passed"),
             date=date, problem="nist-candidate-selection", unit="curves", better="higher",
             label="NIST all-21 census: exact embedding degrees with direct-power certificates",
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


    path = f"{base}33/results.json"
    for date, commit, text in file_history(ref, path, since):
        record = json.loads(text)
        for key, metric, unit, label in (
            ("twist_projective_quadrics", "koblitz_modular.twist_projective_quadrics", "homogeneous equations",
             "Exact twist I2 equations; proper curve interpretation uses written bridge, no polarized factor embedding"),
            ("descended_sections", "koblitz_modular.twist_rr_sections", "sections",
             "Explicit descended complete RR sections; inherited written pole bridge"),
            ("evaluated_degree83_points", "koblitz_modular.twist_rr_evaluated_degree83_points", "closed points",
             "Complete section evaluation at actual degree83 twist points; not IC relation rank"),
            ("padding_closed_points", "koblitz_modular.twist_rational_padding_closed_points", "closed points",
             "Native closed degree5 twist points; proper local binding remains separate")):
            value = record.get("construction_counts", {}).get(key)
            emit(metric, value, date=date, problem=P_IC, unit=unit, better="neutral",
                 label=label, source=f"{path}@{commit}")


    path = f"{base}34/results.json"
    for date, commit, text in file_history(ref, path, since):
        record = json.loads(text)
        for key, metric, unit, label in (
            ("local_source_presentations", "koblitz_modular.ordinary_source_local_presentations", "closed supports",
             "Exact source-bound standard-etale witnesses; global quotient saturation remains separate"),
            ("complete_coupled_divisors", "koblitz_modular.complete_coupled_degree128_divisors", "divisor representations",
             "Complete ordinary P+9Q vanishing spaces; no projected factor image or IC-rank claim")):
            value = record.get("construction_counts", {}).get(key)
            emit(metric, value, date=date, problem=P_IC, unit=unit, better="neutral",
                 label=label, source=f"{path}@{commit}")


    path = f"{base}35/results.json"
    for date, commit, text in file_history(ref, path, since):
        record = json.loads(text)
        for key, metric, unit, label in (
            ("closed_point_algebra_dimension", "koblitz_modular.closed_point_algebra_dimension", "F2 dimensions",
             "Exact closed-point algebra and complete inverse basis; not IC relation rank"),
            ("source_jet_precision", "koblitz_modular.source_jet_precision", "local coefficients",
             "Complete source/descent jets at fixed degree5 point; no target factor action"),
            ("multiplication_space_dimension", "koblitz_modular.large_model_product_dimension", "F2 dimensions",
             "Exact full product space; RR interpretation uses retained proper and pole bridge"),
            ("large_model_vanishing_spaces", "koblitz_modular.large_model_vanishing_spaces", "divisor representations",
             "Actual L3D vanishing spaces; signed Picard and order11 images remain separate")):
            value = record.get("construction_counts", {}).get(key)
            emit(metric, value, date=date, problem=P_IC, unit=unit, better="neutral",
                 label=label, source=f"{path}@{commit}")


    path = f"{base}36/results.json"
    for date, commit, text in file_history(ref, path, since):
        record = json.loads(text)
        for key, metric, unit, label in (
            ("exact_subspace_divisions", "koblitz_modular.exact_subspace_divisions", "certified computations",
             "Exact all-generator quotient controls; no IC performance claim"),
            ("actual_source_classes_tested", "koblitz_modular.actual_source_classes_with_group_controls", "source classes",
             "Native class equality, negation and cancellation; target factor action remains separate")):
            value = record.get("construction_counts", {}).get(key)
            emit(metric, value, date=date, problem=P_IC, unit=unit, better="neutral",
                 label=label, source=f"{path}@{commit}")



def koblitz_larger_constructions(ref, since):
    """Exact R42/R43 construction counters; never attack or timing metrics."""
    base = "research/koblitz_modular_ic_20261018/rounds/"
    path = base + "42/results.json"
    for date, commit, text in file_history(ref, path, since):
        record = json.loads(text)
        if not isinstance(record.get("cases"), list):
            continue
        certified = [row for row in record["cases"]
                     if row.get("global_recovery_status") == "certified"]
        rows = (
            ("koblitz_modular.cm_block_inverse_cases", len(certified), "fixed cases",
             "Explicit CM-block polynomial inverses; no evaluated geometric transfer"),
            ("koblitz_modular.cm_block_recovery_max_n",
             max((row["n"] for row in certified), default=None), "cyclic length",
             "Largest actual CM-block inverse case; not a larger-n point transfer"),
            ("koblitz_modular.cm_denominator_lattices",
             len(record["integer_lattices"]) if isinstance(record.get("integer_lattices"), list) else None,
             "fixed integral lattices",
             "Exact least scalar for fixed denominator maps; not full modular Hom lattices"),
        )
        for metric, value, unit, label in rows:
            emit(metric, value, date=date, problem=P_IC, unit=unit, better="neutral",
                 label=label, source=f"{path}@{commit}")
    path = base + "43/results.json"
    for date, commit, text in file_history(ref, path, since):
        record = json.loads(text)
        emit("koblitz_modular.source_jet_precision", record.get("source_precision"),
             date=date, problem=P_IC, unit="local coefficients", better="neutral",
             label="Actual source jets with full R35 regression; no target factor map",
             source=f"{path}@{commit}")
        for stage in record.get("product_stages", []):
            if stage.get("stop") != "expected_rank_reached":
                continue
            emit("koblitz_modular.complete_rr_" + stage["label"] + "_dimension",
                 stage.get("output_rank"), date=date, problem=P_IC,
                 unit="F2 dimensions", better="neutral",
                 label="Exact product rank; completeness uses retained proper/pole/RR bridge, not IC rank",
                 source=f"{path}@{commit}")


def koblitz_ordinary_orientation(ref, since):
    """Fixed n5 source-algebra counters; not larger-n Koblitz or IC performance."""
    path = "research/koblitz_modular_ic_20261018/rounds/44/results.json"
    for date, commit, text in file_history(ref, path, since):
        counts = json.loads(text).get("construction_counts")
        if not isinstance(counts, dict):
            continue
        for key, metric, unit, label in (
            ("actual_source_blocks", "koblitz_modular.ordinary_source_algebra_blocks", "fixed source blocks",
             "Actual nonsplit-kernel source checks; proper/moduli interpretation retained, not IC rank"),
            ("exact_orientation_trace_cases", "koblitz_modular.orientation_trace_cases", "fixed middle cases",
             "Independent seven-kernel/Tate-twist trace decomposition; not a success distribution"),
            ("orientation_filter_control_blocks", "koblitz_modular.orientation_filter_control_blocks", "retained controls",
             "Trace screen against recorded83/284source algebras; not independent collector images"),
            ("max_actual_source_tuple_degree", "koblitz_modular.max_ordinary_source_tuple_degree", "absolute residue degree",
             "Largest certified source tuple on fixed n5carrier; not a larger-n Koblitz transfer"),
        ):
            emit(metric, counts.get(key), date=date, problem=P_IC, unit=unit,
                 better="neutral", label=label, source=f"{path}@{commit}")

def koblitz_source_collector(ref, since):
    """Actual fixed-n5 chart enumeration; neither Picard rank nor target recovery."""
    path = "research/koblitz_modular_ic_20261018/rounds/45/results.json"
    for date, commit, text in file_history(ref, path, since):
        for row in json.loads(text).get("collector_cases", []):
            if row.get("status") != "accepted" or row.get("role") == "control":
                continue
            degree = row["extension_degree"]
            for key, unit in (("rational_source_points", "rational source points"),
                              ("closed_points", "closed source points"),
                              ("deck_orbits", "deck orbits")):
                emit(f"koblitz_modular.collector_degree{degree}_{key}", row.get(key),
                     date=date, problem=P_IC, unit=unit, better="neutral",
                     label=f"Complete fixed chart over F2^{degree}; fixed n5 carrier, Picard images unevaluated",
                     source=f"{path}@{commit}")


def koblitz_root_collector(ref, since):
    """Exact candidate counts and equality controls, not measured execution cost."""
    path = "research/koblitz_modular_ic_20261018/rounds/46/results.json"
    for date, commit, text in file_history(ref, path, since):
        record = json.loads(text)
        for row in record.get("collector_cases", []):
            if row.get("status") == "accepted":
                degree = row["extension_degree"]
                emit(f"koblitz_modular.root_collector_degree{degree}_candidates",
                     row.get("lambda_candidates"), date=date, problem=P_IC,
                     unit="algebraic root candidates", better="neutral",
                     label="Exact root collector; total cost and Picard images unevaluated",
                     source=f"{path}@{commit}")
        comparisons = record.get("point_set_comparisons", [])
        emit("koblitz_modular.root_collector_complete_set_controls",
             sum(x.get("byte_identical") is True for x in comparisons),
             date=date, problem=P_IC, unit="complete point-set comparisons",
             better="neutral", label="Compared with exhaustive fixed-chart detector; not IC rank",
             source=f"{path}@{commit}")


def koblitz_degree13_divisors(ref, since):
    """Actual fixed-input divisor computations; retained geometric bridge, no IC rank."""
    path = "research/koblitz_modular_ic_20261018/rounds/47/results.json"
    for date, commit, text in file_history(ref, path, since):
        counts = json.loads(text).get("construction_counts", {})
        for key, unit, label in (
            ("full_jet_cases", "closed source points", "Degree13 source jets; fixed n5 carrier, not a larger-n transfer"),
            ("complete_divisor_representations", "effective divisors", "Complete 23Q+P kernels under retained proper/RR bridge"),
            ("signed_class_cases", "actual source classes", "Degree13 collected signed sums; chosen target images not evaluated"),
        ):
            emit("koblitz_modular.degree13_" + key, counts.get(key), date=date,
                 problem=P_IC, unit=unit, better="neutral", label=label,
                 source=f"{path}@{commit}")


def koblitz_degree13_characters(ref, since):
    """Fixed source character tests and finite old-line matches; no target rank."""
    path = "research/koblitz_modular_ic_20261018/rounds/48/results.json"
    for date, commit, text in file_history(ref, path, since):
        counts = json.loads(text).get("construction_counts", {})
        for key, unit, label in (
            ("character_components_tested", "source components", "Degree13 actual diamond characters under retained proper/Picard bridge"),
            ("old_line_scalar_comparisons", "complete scalar tests", "All eleven multiples in each tested old source line"),
            ("matched_old_coordinates", "matched source components", "Exact cyclic-line coordinates; independent target rank unevaluated"),
            ("crossed_source_operator_images", "source operator images", "Full retained crossed operators on a collected class; chosen-model map open"),
        ):
            emit("koblitz_modular.degree13_" + key, counts.get(key), date=date,
                 problem=P_IC, unit=unit, better="neutral", label=label,
                 source=f"{path}@{commit}")


def koblitz_affine_normalization(ref, since):
    """Neutral exact-work counters; output computation and verification stay separate."""
    path = "research/koblitz_modular_ic_20261018/rounds/49/results.json"
    for date, commit, text in file_history(ref, path, since):
        counts = json.loads(text).get("construction_counts", {})
        for key, unit, label in (
            ("normalization_control_cases", "control cases", "Smooth and cusp normalization API controls over F2"),
            ("source_preparation_cases", "prepared source ideals", "Exact twisted F2 source input; not a normalization"),
            ("generic_source_prime", "certified generic fields", "Exact generic primeness; affine contraction and normalization separate"),
            ("integral_lambda_plane_models", "integral plane models", "Accepted monic lambda-plane models; normalization separate"),
            ("primitive_presentations", "two-way field presentations", "Fixed primitive-element test; integral model and normalization separate"),
            ("normalization_outputs_computed", "normalizer outputs", "Computed affine presentations; independent exported-data checks counted separately"),
            ("verified_affine_normalizations", "checked affine presentations", "Affine normalization only; proper projective and modular-factor geometry remain open"),
            ("audited_conditional_declarations", "audited declarations", "Conditional field-degree lemmas; concrete native ideals not formalized"),
        ):
            emit("koblitz_modular.affine_" + key, counts.get(key), date=date,
                 problem=P_IC, unit=unit, better="neutral", label=label,
                 source=f"{path}@{commit}")


def koblitz_checked_source_geometry(ref, since):
    """Exact source geometry and RR algebra counters, with neutral direction."""
    path = "research/koblitz_modular_ic_20261018/rounds/50/results.json"
    for date, commit, text in file_history(ref, path, since):
        counts = json.loads(text).get("construction_counts", {})
        for key, unit, label in (
            ("proper_monic_charts", "checked source charts", "Monic rank32 charts with exact overlap products"),
            ("integral_v_charts", "checked source charts", "Integral quadratic-base refinement on both charts"),
            ("proper_sign_quotient_charts", "checked quotient charts", "Known sign quotient with explicit monic rank16 equations"),
            ("complete_forward_source_products", "complete polynomial products", "All four original source equations under the rational map"),
            ("boundary_cyclic_F2_dimension", "F2 dimensions", "Exact source coordinate-defect block at the degree2 boundary"),
            ("boundary_exact_saturation_exponent", "parameter powers", "Least power of u+1 annihilating the checked cyclic block"),
            ("global_coordinate_defect_F2_dimension", "F2 dimensions", "Complete plane-to-integral-v coordinate defect on the proper model"),
            ("RR_product_columns", "checked field columns", "All196 complete-RR products in the actual32-element field basis"),
            ("RR_PID_basis_rank", "module generators", "Checked complete-RR polynomial module with original-column lifts"),
            ("RR_PID_normalization_index", "F2 dimensions", "Exact source lattice index under the retained complete-RR bridge"),
            ("RR_algebra_product_pairs", "complete product pairs", "All528 source-bound multiplication identities of the RR algebra"),
            ("marked23_base_generic_degree", "field degrees", "Actual prime characteristic2 marked23 base field"),
            ("marked23_point_exact_order", "point order", "Exact order23 point verified in the actual larger base field"),
            ("normalization_presentations_computed", "normalizer outputs", "Accepted serialized presentations; independent ring checks separate"),
            ("verified_affine_normalizations", "checked presentations", "Independent full affine-presentation verification receipts"),
            ("audited_unique_theorems", "audited declarations", "Unique conditional algebra lemmas; concrete arrays remain native data"),
        ):
            emit("koblitz_modular.geometry50_" + key, counts.get(key), date=date,
                 problem=P_IC, unit=unit, better="neutral", label=label,
                 source=f"{path}@{commit}")



def koblitz_integral_chart_transfers(ref, since):
    """Neutral exact chart, packet and kernel-geometry counters from R51 receipts."""
    path = "research/koblitz_modular_ic_20261018/rounds/51/results.json"
    for date, commit, text in file_history(ref, path, since):
        counts = json.loads(text).get("construction_counts", {})
        for key, unit, label in (
            ("localized_generator_compatibilities", "complete generator identities", "Independent source-bound normal-generator compatibility checks"),
            ("localized_normal_chart_maps", "localized chart maps", "Actual first and inverse diamond maps on the complete normal basis"),
            ("universal_certificate_denominator_degree", "polynomial degrees", "Actual denominator of the portable QR polynomial certificate"),
            ("structured_marked23_algebra_dimension", "algebra dimensions", "Complete structured marked23 source algebra over F2(x)"),
            ("structured_marked23_source_equations", "coupled source equations", "All QR and marked-base equations in the structured presentation"),
            ("integral_nu_normal_chart_coordinates", "integral source coordinates", "Actual descended nu with a complete normal-module lift"),
            ("QR_square_determinant_actions", "finite matrix actions", "Exhaustive QR7 square-determinant action checks"),
            ("QR_largest_cycle", "cycle lengths", "Largest cycle in the complete QR7 action enumeration"),
            ("Hecke3_kernel_parameter_degree", "algebra dimensions", "Generic separable Tate3 kernel-parameter algebra"),
            ("Hecke3_isogeny_degree", "isogeny degrees", "Generic Tate quotient with full function and unit certificates"),
            ("ordinary_QR_point_algebra_dimension", "algebra dimensions", "Actual ordinary QR line-packet point algebra"),
            ("coprime_kernel_Bezout_Delta_exponent", "discriminant powers", "Full polynomial coprime3/7 kernel collision certificate"),
            ("nu_diamond_PID_entries", "complete module entries", "Actual nu diamond transport in the retained normal module"),
            ("nu_diamond_clearing_power", "principal-open powers", "Fixed source-bound clearing exponent for nu diamond transport"),
            ("audited_unique_theorems", "audited declarations", "Unique locally audited R51 algebra and packet lemmas"),
        ):
            emit("koblitz_modular.geometry51_" + key, counts.get(key), date=date,
                 problem=P_IC, unit=unit, better="neutral", label=label,
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
    # Independent histories are prefetched without emitting output; metric
    # ordering and the original first-parent/per-day semantics stay unchanged.
    base = "research/koblitz_modular_ic_20261018/rounds/"
    paths = [base + "22/ledger.json"] + [base + f"{n}/results.json"
             for n in (24, 25, 26, 27, 28, 29, 30, 33, 34, 35, 36, 42, 43, 44, 45, 46, 47, 48)]
    pool = ThreadPoolExecutor(max_workers=4)
    histories = [pool.submit(_history_rows, a.ref, path, a.since) for path in paths]
    jacobian_catalog(a.ref, a.since)
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
    curve_traits(a.ref)

    for history in histories:
        history.result()
    pool.shutdown()
    koblitz_modular_augmentation(a.ref, a.since)
    koblitz_modular_geometry(a.ref, a.since)
    koblitz_larger_constructions(a.ref, a.since)
    koblitz_ordinary_orientation(a.ref, a.since)
    koblitz_source_collector(a.ref, a.since)
    koblitz_root_collector(a.ref, a.since)
    koblitz_degree13_divisors(a.ref, a.since)
    koblitz_degree13_characters(a.ref, a.since)
    koblitz_affine_normalization(a.ref, a.since)
    koblitz_checked_source_geometry(a.ref, a.since)
    koblitz_integral_chart_transfers(a.ref, a.since)

if __name__ == "__main__":
    main()
