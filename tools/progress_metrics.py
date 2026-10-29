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


def algebraic_stage(ref, since):
    """Coverage of finite native panels, kept separate from pipeline ratios."""
    path = "research/ic_algebraic_crossover_20261023/report-data.json"
    for date, commit, txt in file_history(ref, path, since):
        d = json.loads(txt)
        if d.get("schema") != "ic-campaign-report-data/v1" or d.get("status") != "recomputed_from_raw":
            continue
        for panel in d.get("panels", []):
            degree = panel.get("degree")
            for field, label in (("paired_queries", "completed direct/symmetric pairs"),
                                 ("symmetric_lower_words", "pairs with lower symmetric solver words")):
                emit(f"ic.algebraic.closure-d{degree}.{field}", panel.get(field), date=date,
                     problem=P_IC, unit="count", better="neutral",
                     label=f"finite closure cap {degree}: {label}; stage only",
                     source=f"{path}@{commit}")


def algebraic_audits(ref, since):
    """Recorded finite correctness/relevance counts, never a performance index."""
    study = "research/ic_algebraic_crossover_20261023/pipeline-receipts/"
    sources = [
        ("fresh-target-relevance-20261025/receipt.json", "ic-fresh-target-relevance/v1",
         "status", "all_target_roots_and_trace_criteria_agree", [
             (("totals", field), f"ic.algebraic.fresh-target.{field}",
              f"fresh Boolean fixtures: {label} on E0 reference; this audit checks lifts only")
             for field, label in [
                 ("sat_liftable", "SAT, target liftable"),
                 ("sat_nonliftable", "SAT, target nonliftable"),
                 ("unsat_liftable", "UNSAT, target liftable"),
                 ("unsat_nonliftable", "UNSAT, target nonliftable"),
             ]]),
        ("quartic-smoke-20261025-02.json", "ecbench.audit/v1", "ok", True, [
            (("verified_records",), "ic.algebraic.quartic-smoke.verified_records",
             "quartic m6/8/10/12 smoke: verified records; counted L0 only")]),
        ("external-cas-strengthened-20261025/replay.json", "ic-cas-correctness-replay/v1",
         "status", "all_external_zero_sets_equal", [
             (("cases",), "ic.algebraic.external-cas.correctness_cases",
              "finite systems with equal Singular/native Boolean zero sets; no speed comparison")]),
        ("fixture-subgroups-enumerated-20261025/receipt.json", "ic-fixture-subgroups/v2",
         "status", "all_lifts_trace_orders_and_membership_checks_agree", [
             (("totals", field), f"ic.algebraic.fixture-subgroups.{field}", label)
             for field, label in [
                 ("in_generated_subgroup", "original/fresh targets inside declared reference/twist G"),
                 ("outside_generated_subgroup", "original/fresh targets outside declared reference/twist G"),
                 ("generated_subgroup_unknown", "original/fresh targets with unresolved declared subgroup membership"),
             ]]),
        ("qualified-queries-20261025/summary.json", "ic-qualified-query-summary/v1",
         "status", "all_2048_finite_chain_counts_and_paired_targets_verified", [
             ((field,), f"ic.algebraic.qualified-queries.{field}", label)
             for field, label in [
                 ("rows", "additional qualified a0/full-degree-b query draws"),
                 ("boolean_sat", "additional qualified queries with finite-chain roots"),
                 ("point_sat", "additional qualified queries with actual subgroup point decompositions"),
                 ("independent_point_arithmetic_rows_verified", "qualified point counts checked by independent arithmetic"),
                 ("paired_target_count", "underlying public targets paired across qualified basis families"),
             ]]),
        *[(f"wide-core-audit-20261025/m{m}/output.json", "ic-quartic-wide-core-audit/v1",
           "status", "verified", [
               ((field,), f"ic.algebraic.wide-core.m{m}.{field}", f"m{m} wide-core {label}; full IC unmeasured")
               for field, label in [
                   ("independent_field_products", "independent field-product controls"),
                   ("independent_scalar_controls", "independent full-width scalar controls"),
               ]]) for m in (16, 17, 18)],
    ]
    for relative, schema, key, expected, fields in sources:
        path = study + relative
        for date, commit, txt in file_history(ref, path, since):
            data = json.loads(txt)
            if data.get("schema") != schema or data.get(key) != expected:
                continue
            for keys, metric, label in fields:
                value = data
                for part in keys:
                    value = value.get(part) if isinstance(value, dict) else None
                emit(metric, value, date=date, problem=P_IC, unit="count", better="neutral",
                     label=label, source=f"{path}@{commit}")
    path = study + "qualified-closure-replay-20261025/summary.json"
    for date, commit, txt in file_history(ref, path, since):
        data = json.loads(txt)
        if data.get("schema") != "ic-qualified-closure-summary/v1":
            continue
        for group in data.get("groups", []):
            degree = group.get("degree")
            kind = "positive" if group.get("sat_control") else "negative"
            for field in ("nonrefuting", "independent_hky_verified"):
                emit(f"ic.algebraic.qualified-closure.d{degree}.{kind}.{field}", group.get(field),
                     date=date, problem=P_IC, unit="count", better="neutral",
                     label=f"qualified {kind} D{degree} {field}; finite closure only",
                     source=f"{path}@{commit}")


def completed_guess(ref, since):
    """Frozen producer counts; separate from independent HKY and full IC price."""
    study = "research/ic_algebraic_crossover_20261023/pipeline-receipts/"
    path = study + "guess-completion-20261025-01/summary.json"
    for date, commit, txt in file_history(ref, path, since):
        data = json.loads(txt)
        if data.get("status") != "all_registered_attempts_terminated_native_raw_accounting_recomputed":
            continue
        for field in ("producer_refuted", "producer_saturated_nonrefuting", "producer_unknown"):
            emit(f"ic.algebraic.completed-guess.{field}", data.get(field), date=date,
                 problem=P_IC, unit="count", better="neutral",
                 label=f"registered original Boolean branches: {field}; producer classification",
                 source=f"{path}@{commit}")
        for row in data.get("draws", []):
            for field, unit in (("balanced_prefix_minimum", "count"),
                                ("passing_set_solver_words", "solver_word_ops"),
                                ("search_through_first_passing_set_solver_words", "solver_word_ops")):
                emit(f"ic.algebraic.completed-guess.draw{row['draw']}.{field}", row.get(field),
                     date=date, problem=P_IC, unit=unit, better="neutral",
                     label=f"draw {row['draw']} registered balanced prefix: {field}; no global minimum or full price",
                     source=f"{path}@{commit}")
    path = study + "cofactor-projection-20261025/receipt.json"
    for date, commit, txt in file_history(ref, path, since):
        data = json.loads(txt)
        if data.get("status") != "all_finite_projected_membership_and_relation_checks_agree":
            continue
        for field in ("selected_projected_witness_relations", "selected_all_summands_in_prime_subgroup_relations"):
            emit(f"ic.algebraic.cofactor-projection.{field}", data.get(field), date=date,
                 problem=P_IC, unit="count", better="neutral",
                 label=f"one canonical-sign witness per original SAT draw: {field}; no base logs or full IC",
                 source=f"{path}@{commit}")


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

def publication_followup(ref, since):
    """Native retained-profile/scalar diagnostics; no full IC performance index."""
    path = "research/ic_algebraic_crossover_20261023/pipeline-receipts/publication-followup-audit-v3-20261026/output.json"
    for date, commit, txt in file_history(ref, path, since):
        data = json.loads(txt)
        if data.get("status") != "native_profile_accounting_and_partial_scalar_checks_pass":
            continue
        for arm in data.get("constructor", {}).get("arms", []):
            emit(f"ic.algebraic.narrow-constructor.{arm['arm']}.ir", arm.get("whole_process_ir"),
                 date=date, problem=P_IC, unit="simulated_Ir", better="neutral",
                 label=f"narrow6 {arm['arm']}: simulated whole-process Ir including loader/Valgrind; no native speedup",
                 source=f"{path}@{commit}")
        for row in data.get("partial_quartics", []):
            for field in ("recorded", "missing_registered_executions", "independent_scalar_checks"):
                emit(f"ic.algebraic.partial-quartic.m{row['m']}.{field}", row.get(field),
                     date=date, problem=P_IC, unit="count", better="neutral",
                     label=f"interrupted m{row['m']}: {field}; no completed IC comparison",
                     source=f"{path}@{commit}")
            for arm, statuses in row.get("counts", {}).items():
                for status, value in statuses.items():
                    emit(f"ic.algebraic.partial-quartic.m{row['m']}.{arm}.{status}", value,
                         date=date, problem=P_IC, unit="count", better="neutral",
                         label=f"interrupted m{row['m']} {arm}: {status}; missing/censored costs unknown",
                         source=f"{path}@{commit}")
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


def main():
    a = args()
    leaderboard(a.ref, a.since)
    timeline(a.ref)
    perf_history(a.since)
    benchmarks(a.ref, a.since)
    e2e_runs(a.ref)
    algebraic_stage(a.ref, a.since)
    algebraic_audits(a.ref, a.since)
    completed_guess(a.ref, a.since)
    publication_followup(a.ref, a.since)
    photo_volcano(a.ref)
    nist_factorization(a.ref, a.since)
    nist_selection_factor_census(a.ref, a.since)
    nist_selection_exact_embedding(a.ref, a.since)
    ghs_ipsec_archive(a.ref, a.since)
    fall_degree(a.ref)
    curve_traits(a.ref)


if __name__ == "__main__":
    main()
