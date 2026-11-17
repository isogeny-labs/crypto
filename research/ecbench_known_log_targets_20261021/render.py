"""Layout only: plot native summaries and render their descriptive reports.

No curve arithmetic, measurement, replay, aggregation, or result calculation
happens here. All plotted values are emitted by the Rust recorded-value tool.
"""
import json
from pathlib import Path
import sys

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import typst


def quoted(text):
    return json.dumps(str(text), ensure_ascii=False)


def render(report):
    summary = json.loads((report / "summary.json").read_text())
    spec = json.loads((Path(summary["session"]) / "spec.json").read_text())
    session = json.loads((Path(summary["session"]) / "session.json").read_text())
    rows = summary["band"]["rows"]
    fig, axes = plt.subplots(1, 3, figsize=(12, 3.9), constrained_layout=True)
    colors = {"rho-neg": "#52606d", "abscissa": "#d66b34", "known-log": "#156b9a",
              "transported": "#7b4cb0", "projected": "#368762"}
    for index, (axis, weight) in enumerate(zip(axes, summary["weights"])):
        for arm in colors:
            selected = sorted((r for r in rows if r["arm"] == arm), key=lambda r: r["log2_r"])
            selected = [r for r in selected if r["mean_s"][index] is not None]
            if selected:
                axis.plot([r["log2_r"] for r in selected],
                          [r["mean_s"][index] for r in selected], marker="o", ms=4,
                          label=arm, color=colors[arm])
        axis.set_yscale("log")
        axis.set_xlabel("log₂ subgroup order r")
        axis.set_ylabel("Cold S = priced GAE / √r")
        axis.set_title(f"Native work: {weight:g} GAE / operation", fontsize=10)
        axis.grid(alpha=0.2, which="both")
        axis.legend(fontsize=8)
    fig.suptitle(report.name + f" · {spec['workloads']['targets_per_curve']} public targets/curve × "
                 f"{spec['measurement']['rounds']} measured rounds", fontsize=10)
    fig.savefig(report / "cost.svg")
    fig.savefig(report / "cost.png", dpi=150)
    plt.close(fig)
    cells = ["Arm", "log₂ r", "Runs", "Verified", "S(0)", "S(0.1)", "S(1)"]
    for row in rows:
        cells += [row["arm"], f'{row["log2_r"]:.3f}', str(row["runs"]), str(row["verified"])]
        cells += ["missing" if v is None else f"{v:.5g}" for v in row["mean_s"]]
    table = ",\n".join("[" + cell.replace("_", r"\_") + "]" for cell in cells)
    text = f'''#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 8pt)
#set heading(numbering: none)
= Arbitrary-target known-log study
#text(size: 10pt)[{report.name}]

This report displays the native recorded-value summary. Each workload is one
public target with no supplied logarithm; cold setup is charged on every run.
The session recorded #text({quoted(session['status_counts'])}).
Native work is priced equally on all arms at the three displayed weights.
Uncounted work or failed recoveries block rankings. A/A controls are retained in
the table. Raw wall times are descriptive unless the harness admits them at L2.
This report makes no standard-curve, asymptotic, or fastest-tool claim.

#image("cost.svg", width: 100%)

The figure displays native per-curve means, not a fitted growth exponent.
Read paired uncertainty and admission from the comparison JSON beside this
report. The plotted values are copied directly from summary.json.

#table(columns: (2fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr), inset: 3pt,
{table})

== Evidence
Session: #text({quoted(summary["session"])})

Records SHA-256: #text({quoted(summary["records_sha256"])})

Source files: summary.json, audit.json and its SHA-256 file, resources.json,
comparison JSON, COMMANDS.txt, and the sealed session records. Replays here
are on the measuring host; they are not independent hardware validation.
'''
    (report / "REPORT.typ").write_text(text)
    typst.compile(str(report / "REPORT.typ"), output=str(report / "REPORT.pdf"), root=str(report))
    (report / "REPORT.md").write_text(
        f"# {report.name}\n\n"
        "Native recorded-value analysis; one public target per workload, cold setup charged.\n\n"
        "![Native pricing band](cost.svg)\n\n"
        "[PDF](REPORT.pdf) · [native summary](summary.json) · [all-run replay](audit.json) · "
        "[commands](COMMANDS.txt) · [resources](resources.json)\n\n"
        f"Records SHA-256: `{summary['records_sha256']}`. "
        "Paired uncertainty and wall-time admission are in the comparison JSON files. "
        "This same-host replay is not independent hardware validation.\n")


if __name__ == "__main__":
    for argument in sys.argv[1:]:
        render(Path(argument).resolve())
