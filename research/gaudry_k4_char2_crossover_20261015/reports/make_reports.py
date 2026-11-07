"""Layout only: one descriptive run report (Typst source + SVG graph) per
ecbench session of this study, from the session's own records. Nothing is
computed here beyond unit conversions of recorded fields (ns to hours)."""
import datetime
import json
import os
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = os.path.dirname(os.path.abspath(__file__))
STUDY = os.path.dirname(HERE)
BLUE, ORANGE, GREEN, PURPLE, GREY = "#2a78d6", "#e8673a", "#1fa37a", "#8b5cf6", "#555555"
ARM_COLOUR = {"rho-neg": ORANGE, "rho-neg-aa": ORANGE, "ic-wied": BLUE, "ic-dense": GREY,
              "lp1-700": PURPLE, "lp2-700": GREEN, "lp2-500": "#c9a227", "ic-frobenius": BLUE}

# directory, kind, note, spec file
SESSIONS = [
    ("sessions/e-m15", "complete", "Part E, `m = 15`", "specs/SPEC-E-m15.json"),
    ("sessions/e-m17-rho", "complete", "Part E, `m = 17`, the rho half", "specs/SPEC-E-m17-rho.json"),
    ("sessions/d-ladder", "complete", "Part D, sparse against dense, `m = 8, 10, 12`", "specs/SPEC-D-ladder.json"),
    ("sessions/f-lp", "complete", "Part F, large primes, `m = 10, 12, 14`", "specs/SPEC-F-lp.json"),
    ("sessions/e-m16.attempt3-killed", "interrupted", "Part E, `m = 16` (killed after 3 of 4 runs)", "specs/SPEC-E-m16.json"),
    ("sessions/e-m17-ic.attempt1-killed", "interrupted", "Part E, `m = 17`, the IC half (killed after 1 of 2 runs)", "specs/SPEC-E-m17-ic.json"),
    ("sessions/d-m14.attempt3-killed", "interrupted", "Part D, `m = 14` (killed after 2 of 3 runs)", "specs/SPEC-D-m14.json"),
    ("pilot/session-m10", "pilot", "Pilot P2, `m = 10`: dense, `wiedemann`, large-prime settings", "pilot/SPEC-pilot-m10.json"),
    ("pilot/session-rates", "pilot", "Pilot P3: capped rates at `m = 16, 17, 18` (exhausted by design)", "pilot/SPEC-pilot-rates.json"),
]


def esc(s):
    return str(s).replace("_", "\\_").replace("#", "\\#")


def main():
    for d, kind, note, spec in SESSIONS:
        sd = os.path.join(STUDY, d)
        if not os.path.exists(os.path.join(sd, "records.jsonl")):
            continue
        sess = json.load(open(os.path.join(sd, "session.json")))
        recs = [json.loads(l) for l in open(os.path.join(sd, "records.jsonl")) if l.strip()]
        start = datetime.datetime.fromtimestamp(sess["started_unix_ms"] / 1000, datetime.timezone.utc)
        name = f"run-{start:%Y%m%dT%H%MZ}-{os.path.basename(d)}"
        out = os.path.join(HERE, name)
        os.makedirs(out, exist_ok=True)
        rows = []
        for r in recs:
            c = r["counters"]
            wall = r["time"]["solve_wall_ns"] / 3.6e12
            rows.append((r["arm"], r["workload"]["workload_id"], r["workload"]["curve_spec"]["m"],
                         r["outcome"]["status"], r["cost"]["s"], r["cost"]["total_gae"],
                         c.get("targets_tried"), c.get("relations_found"), wall, r["isolation"].get("level", "") if isinstance(r.get("isolation"), dict) else ""))
        # graph: S per run (log scale unless the pilot rates)
        fig, ax = plt.subplots(figsize=(7.4, 3.6))
        labels = [f"{a}\n{w[:7]}" for a, w, *_ in rows]
        if "rates" in d:
            ys = [x[8] * 3600e3 for x in rows]
            ax.set_ylabel("solve wall time (ms)")
        else:
            ys = [x[4] for x in rows]
            ax.set_ylabel("S (group-addition equivalents ÷ √r)")
            ax.set_yscale("log")
        ax.bar(range(len(rows)), ys, color=[ARM_COLOUR.get(x[0], GREY) for x in rows])
        ax.set_xticks(range(len(rows)))
        ax.set_xticklabels(labels, fontsize=6, rotation=60, ha="right")
        ax.set_title(f"{os.path.basename(d)}: {len(rows)} recorded runs ({sess['session_id']})", fontsize=9)
        ax.grid(alpha=0.3)
        fig.tight_layout()
        fig.savefig(os.path.join(out, "runs.svg"))
        fig.savefig(os.path.join(out, "runs.png"), dpi=150)
        plt.close(fig)
        table = ["  [arm], [workload], [$m$], [status], [$S$], [GAE], [trials], [relations], [wall (h)],"]
        for a, w, m, st, s, g, t, rel, wall, _ in rows:
            table.append(f"  [{esc(a)}], [`{w}`], [{m}], [{st}], [{s:.4g}], [{g:.4g}], [{t if t is not None else '–'}], [{rel if rel is not None else '–'}], [{wall:.3f}],")
        status_line = {
            "complete": "The session completed: every planned run was recorded.",
            "interrupted": f"The session was killed by a container restart: {len(rows)} of {sess['executions_planned']} planned runs were recorded and the rest are missing. Its status is `running`, `ecbench verify` fails exactly its `session_complete` and `record_count` checks, and its records were not replayed.",
            "pilot": "A pilot, run before PROTOCOL.md; no claim rests on it.",
        }[kind]
        typ = f"""#set page(paper: "a4", margin: 1.7cm)
#set text(size: 9.3pt)
#set table(stroke: 0.4pt)
#show table: set text(size: 8pt)
= Run report: ecbench session `{sess['session_id']}` ({esc(os.path.basename(d))}), {start:%Y-%m-%dT%H:%MZ}

{esc(note)}. Spec `{spec}`. Binary sha256 `{(sess.get('binary_sha256') or 'unrecorded')[:8]}…`, commit `{(sess.get('git_commit') or 'unrecorded')[:9]}`{' (clean)' if sess.get('git_commit') and not sess.get('git_dirty') else ''}. Host: Xeon \\@ 2.10 GHz, 4 vCPU, one core per session (isolation level L1 or L0, recorded per run: wall time is descriptive only). {esc(status_line)}

#table(columns: 9, align: (left, left, right, left, right, right, right, right, right),
{chr(10).join(table)}
)

All values are recorded fields (`S` = `cost.s`, GAE = `cost.total_gae`, trials and relations from `counters`, wall = `time.solve_wall_ns` ÷ 3.6·10¹²); no run here is a replay. Records: `{d}/records.jsonl`. Audit receipts: `audits/`.

#figure(image("runs.svg", width: 92%), caption: [Recorded `S` per run, session `{sess['session_id']}`.])
"""
        open(os.path.join(out, "report.typ"), "w").write(typ)
        print(name)


main()
