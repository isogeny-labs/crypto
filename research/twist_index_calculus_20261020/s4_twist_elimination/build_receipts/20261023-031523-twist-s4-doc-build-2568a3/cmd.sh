set -euo pipefail
mkdir -p /work/report-source /work/report-tools
{ printf "%s" "$S4_BUNDLE_A"; printf "%s" "$S4_BUNDLE_B"; } | base64 -d | tar -xz -C /work/report-source
python3 -m venv /work/report-venv
/work/report-venv/bin/pip install matplotlib > "$EPHEMERAL_OUT/pip.log" 2>&1
curl --fail --location --retry 2 https://github.com/typst/typst/releases/download/v0.13.1/typst-x86_64-unknown-linux-musl.tar.xz -o /work/report-tools/typst.tar.xz > "$EPHEMERAL_OUT/typst-download.log" 2>&1
tar -xJf /work/report-tools/typst.tar.xz -C /work/report-tools
export TWIST_REPORT_TYPST=/work/report-tools/typst-x86_64-unknown-linux-musl/typst
export MPLCONFIGDIR=/work/report-mpl
report_root=/work/report-source/research/twist_index_calculus_20261020/s4_twist_elimination
/work/report-venv/bin/python "$report_root/render_reports.py" > "$EPHEMERAL_OUT/render.log" 2>&1
mkdir -p "$EPHEMERAL_OUT/rendered"
cp -a "$report_root/reports" "$report_root/report.pdf" "$report_root/report.typ" "$report_root/overview.svg" "$report_root/overview.png" "$report_root/overview-data.json" "$EPHEMERAL_OUT/rendered/"
printf "all ten run reports and the round report rendered\n" > "$EPHEMERAL_OUT/render-complete.txt"
