#!/usr/bin/env bash
# One command from finished measurement to run report, graph and PDFs.
#
#   research/dreg_closure_slope_20261018/make_report.sh run-20261018T0618Z
#
# Scores runs/ with the native scorer (dreg_score slope), plots the recorded
# degrees, fills report.typ.template and results.typ.template from
# score-output.txt (layout only: nothing here computes a result), and renders
# reports/$RUN_ID/report.pdf and results.pdf with the Typst python package.
# Optional env: PY (default /tmp/report-venv/bin/python), SCORE_BIN.
set -eu
RUN_ID=${1:?usage: make_report.sh RUN_ID (e.g. run-20261018T0618Z)}
HERE=$(cd "$(dirname "$0")" && pwd)
REPO=$(cd "$HERE/../.." && pwd)
PY=${PY:-/tmp/report-venv/bin/python}
OUT=$HERE/reports/$RUN_ID
mkdir -p "$OUT"
cd "$REPO"
STUDY=research/dreg_closure_slope_20261018

# 1. score
if [ -n "${SCORE_BIN:-}" ]; then
  "$SCORE_BIN" slope $STUDY/runs/*.D*.jsonl > "$HERE/score-output.txt"
else
  cargo run --release --example dreg_score -- slope $STUDY/runs/*.D*.jsonl > "$HERE/score-output.txt"
fi
SLOPE=$(sed -n 's/^Slope s = \(-\{0,1\}[0-9.]*\).*/\1/p' "$HERE/score-output.txt" | head -1)
[ -n "$SLOPE" ] || { echo "no 'Slope s =' line in score-output.txt" >&2; exit 1; }

# 2. graph (recorded values only; slope read from the scorer)
cp "$HERE/plot.py" "$OUT/plot.py"
"$PY" "$HERE/plot.py" "$HERE/runs" "$OUT" "$SLOPE" "$RUN_ID"

# 3. fill templates and render
COMMIT=$(git rev-parse --short HEAD)
"$PY" - "$HERE" "$OUT" "$RUN_ID" "$COMMIT" "$SLOPE" <<'PYEOF'
import glob, os, re, sys
here, out, run_id, commit, slope = sys.argv[1:6]
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
    return "#table(columns: %d, inset: 4pt, stroke: 0.5pt + gray,\n  %s)" % (n, ",\n  ".join(body))

# Split the scorer output: first table, then the line block, then the derived table.
tables, cur = [], []
for ln in score + [""]:
    if ln.startswith("|"):
        cur.append(ln)
    elif cur:
        tables.append(cur); cur = []
score_table = typst_table(tables[0]) if tables else "_No table in score-output.txt._"
derived_table = typst_table(tables[1]) if len(tables) > 1 else "_No derived table in score-output.txt._"
lines = [l for l in score if l and not l.startswith("|") and not l.startswith("Derived")]
verdict = next((l for l in lines if l.startswith("Registered verdict")), "")
slope_line = next((l for l in lines if l.startswith("Slope s")), "")
plines = [l for l in lines if re.match(r"^P[1-5]\b", l)]
derived_head = next((l for l in score if l.startswith("Derived")), "Derived:")

# Mechanical facts about the records.
recs = [p for p in glob.glob(os.path.join(here, "runs", "*.D*.jsonl"))]
def cnt(prefix): return [p for p in recs if os.path.basename(p).startswith(prefix)]
nonempty = lambda ps: [p for p in ps if os.path.getsize(p) > 0]
meas = [p for p in recs if not os.path.basename(p).startswith(("check-", "control-"))]
def flag(ps, key): return sum(1 for p in ps if os.path.getsize(p) and ('"%s":true' % key) in open(p).read())
qlog = os.path.join(here, "runs", "queue.log")
q = open(qlog).read().splitlines() if os.path.exists(qlog) else []
stamps = [l.split()[0] for l in q if l[:2] == "20"]
start, end = (stamps[0], stamps[-1]) if stamps else ("unrecorded", "unrecorded")
m = re.match(r"run-(\d{4})(\d\d)(\d\d)T(\d\d)(\d\d)Z", run_id)
run_date = "%s-%s-%s %s:%s UTC" % m.groups() if m else run_id

subs = {
 "RUN_ID": run_id, "COMMIT": commit, "SLOPE": slope, "RUN_DATE": run_date,
 "QUEUE_START": start, "QUEUE_END": end,
 "SCORE_TABLE": score_table, "DERIVED_TABLE": derived_table,
 "DERIVED_HEAD": esc(derived_head),
 "SLOPE_LINE": slope_line, "VERDICT_LINE": verdict,
 "P_LINES": "\n".join(plines),
 "N_MEAS_FILES": str(len(meas)), "N_MEAS_NONEMPTY": str(len(nonempty(meas))),
 "N_CAPPED": str(flag(meas, "capped")), "N_REFUTED": str(flag(meas, "closure_refuted")),
 "N_CHECK_FILES": str(len(cnt("check-"))), "N_CHECK_NONEMPTY": str(len(nonempty(cnt("check-")))),
 "N_CONTROL_FILES": str(len(cnt("control-"))), "N_CONTROL_NONEMPTY": str(len(nonempty(cnt("control-")))),
}
def fill(tpl, dst):
    t = open(os.path.join(here, tpl)).read()
    for k, v in subs.items():
        t = t.replace("{{%s}}" % k, v)
    left = re.findall(r"\{\{[A-Z_0-9]+\}\}", t)
    if left: sys.exit("unfilled placeholders in %s: %s" % (tpl, sorted(set(left))))
    open(dst, "w").write(t)
fill("report.typ.template", os.path.join(out, "report.typ"))
fill("results.typ.template", os.path.join(here, "results.typ"))
import typst
typst.compile(os.path.join(out, "report.typ"), output=os.path.join(out, "report.pdf"))
typst.compile(os.path.join(here, "results.typ"), output=os.path.join(here, "results.pdf"))
print("wrote", out, "and", here + "/results.pdf")
PYEOF
if grep -q "TODO-INTERPRETATION" "$HERE/results.typ"; then
  echo "NOTE: results.typ still has a TODO-INTERPRETATION block; write RESULTS.md and fill it (edit results.typ.template, rerun)." >&2
fi
