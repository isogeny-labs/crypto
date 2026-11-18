#!/usr/bin/env bash
# One command from finished measurement to run report, graph and PDF.
#
#   research/target_free_relations_20261018/make_report.sh run-20261018T1645Z
#
# Scores runs/ with the native scorer (dreg_score target_free), plots the
# recorded solving degrees, fills report.typ.template from score-output.txt
# (layout only: nothing here computes a result) and renders
# reports/$RUN_ID/report.pdf with the Typst python package.
# Optional env: PY (default /tmp/report-venv/bin/python), SCORE_BIN (absolute path).
set -eu
RUN_ID=${1:?usage: make_report.sh RUN_ID}
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
PY=${PY:-/tmp/report-venv/bin/python}
OUT=$HERE/reports/$RUN_ID
mkdir -p "$OUT"
cd "$REPO"
STUDY=research/target_free_relations_20261018

if [ -n "${SCORE_BIN:-}" ]; then
  "$SCORE_BIN" target_free $STUDY/runs/cell-*.jsonl > "$HERE/score-output.txt"
else
  cargo run --release --example dreg_score -- target_free $STUDY/runs/cell-*.jsonl > "$HERE/score-output.txt"
fi
# Deviation (raised cap, RESULTS.md): scored separately, never pooled with runs/.
if ls $STUDY/runs_highcap/cell-*.jsonl >/dev/null 2>&1; then
  HC=$(for f in $STUDY/runs_highcap/cell-*.jsonl; do [ -s "$f" ] && echo "$f"; done)
  if [ -n "${SCORE_BIN:-}" ]; then "$SCORE_BIN" target_free $HC; else cargo run --release --example dreg_score -- target_free $HC; fi > "$HERE/score-output-highcap.txt"
fi
cp "$HERE/plot.py" "$OUT/plot.py"
"$PY" "$HERE/plot.py" "$HERE/runs" "$OUT" "$RUN_ID" "$HERE/runs_highcap"

COMMIT=$(git rev-parse --short HEAD)
"$PY" - "$HERE" "$OUT" "$RUN_ID" "$COMMIT" <<'PYEOF'
import glob, json, os, re, sys
here, out, run_id, commit = sys.argv[1:5]
score = open(os.path.join(here, "score-output.txt")).read().splitlines()

def esc(s):
    return re.sub(r'([#$@*_`<\\\[\]])', r'\\\1', s)

def typst_table(rows):
    cells = [[c.strip() for c in r.strip().strip("|").split("|")] for r in rows
             if not re.match(r"^\|[-: |]+\|$", r.strip())]
    n = len(cells[0])
    body = []
    for i, r in enumerate(cells):
        for c in r:
            body.append("[*%s*]" % esc(c) if i == 0 else "[%s]" % esc(c))
    return "#table(columns: %d, inset: 3pt, stroke: 0.5pt + gray,\n  %s)" % (n, ",\n  ".join(body))

tables, cur = [], []
for ln in score + [""]:
    if ln.startswith("|"):
        cur.append(ln)
    elif cur:
        tables.append(cur); cur = []
score_table = typst_table(tables[0]) if tables else "_No table in score-output.txt._"
verdict = next((l for l in score if l.startswith("Registered verdict")), "")
hcp = os.path.join(here, "score-output-highcap.txt")
hc = open(hcp).read().splitlines() if os.path.exists(hcp) else []
hc_rows = [l for l in hc if l.startswith("|")]
hc_table = typst_table(hc_rows) if hc_rows else "_No raised-cap records._"
hq = os.path.join(here, "runs_highcap", "queue.log")
hc_queue = open(hq).read().strip().replace("/home/user/crypto/", "") if os.path.exists(hq) else "none"
plines = [l for l in score if re.match(r"^P[1-4]\b", l)]

recs = []
for p in sorted(glob.glob(os.path.join(here, "runs", "cell-*.jsonl"))):
    for line in open(p):
        if line.strip():
            recs.append(json.loads(line))
files = sorted(glob.glob(os.path.join(here, "runs", "cell-*.jsonl")))
nonempty = [p for p in files if os.path.getsize(p) > 0]
tf = [r for r in recs if r["arm"] == "target_free"]
tg = [r for r in recs if r["arm"] == "target_m3"]
def fin(rs): return sum(1 for r in rs if not (r["timed_out"] or r["oversize"]))
def over(rs): return sum(1 for r in rs if r["oversize"])
def tout(rs): return sum(1 for r in rs if r["timed_out"])
secs = [r["secs"] for r in recs]
peak = max((r.get("peak_matrix_bytes", 0) for r in recs), default=0)
qlog = os.path.join(here, "runs", "queue.log")
q = open(qlog).read().splitlines() if os.path.exists(qlog) else []
stamps = [l.split()[0] for l in q if l[:2] == "20"]
start, end = (stamps[0], stamps[-1]) if stamps else ("unrecorded", "unrecorded")
exits = [l for l in q if " exit " in l]
nonzero = [l for l in exits if not re.search(r" exit 0 ", l)]  # unreliable for run-20261018T1645Z: see the hand note in queue.log
m = re.match(r"run-(\d{4})(\d\d)(\d\d)T(\d\d)(\d\d)Z", run_id)
run_date = "%s-%s-%s %s:%s UTC" % m.groups() if m else run_id
subs = {
 "RUN_ID": run_id, "COMMIT": commit, "RUN_DATE": run_date,
 "QUEUE_START": start, "QUEUE_END": end,
 "SCORE_TABLE": score_table, "HC_TABLE": hc_table, "HC_QUEUE": hc_queue, "VERDICT_LINE": verdict, "P_LINES": "\n".join(plines),
 "N_FILES": str(len(files)), "N_NONEMPTY": str(len(nonempty)),
 "N_TF": str(len(tf)), "N_TF_FIN": str(fin(tf)), "N_TF_OVER": str(over(tf)), "N_TF_TOUT": str(tout(tf)),
 "N_TG": str(len(tg)), "N_TG_FIN": str(fin(tg)), "N_TG_OVER": str(over(tg)), "N_TG_TOUT": str(tout(tg)),
 "N_EXITS": str(len(exits)), "N_NONZERO": str(len(nonzero)),
 "NONZERO_LINES": esc("; ".join(l.split(" ", 1)[1] for l in nonzero) or "none"),
 "MAX_SECS": "%.0f" % max(secs) if secs else "—",
 "PEAK_MATRIX_GIB": "%.2f" % (peak / 2**30),
}
t = open(os.path.join(here, "report.typ.template")).read()
for k, v in subs.items():
    t = t.replace("{{%s}}" % k, v)
left = re.findall(r"\{\{[A-Z_0-9]+\}\}", t)
if left: sys.exit("unfilled placeholders: %s" % sorted(set(left)))
open(os.path.join(out, "report.typ"), "w").write(t)
import typst
typst.compile(os.path.join(out, "report.typ"), output=os.path.join(out, "report.pdf"))
print("wrote", out)
PYEOF
