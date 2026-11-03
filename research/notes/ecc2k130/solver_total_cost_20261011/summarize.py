"""Recompute results/summary.json, results/tables.md and RESULTS.md from the receipts.

    python3 summarize.py            # reads results/w1/receipts.jsonl (+ results/w2/receipts.jsonl if present)
"""
import json
from pathlib import Path

import analysis

HERE = Path(__file__).resolve().parent


def main():
    summary = analysis.build_summary(HERE / "results" / "w1" / "receipts.jsonl",
                                     HERE / "results" / "w2" / "receipts.jsonl")
    (HERE / "results" / "summary.json").write_text(json.dumps(summary, indent=1, sort_keys=True) + "\n")
    tabs = analysis.tables(summary)
    (HERE / "results" / "tables.md").write_text("\n\n".join(f"<!-- {k} -->\n{v}" for k, v in tabs.items()) + "\n")
    template = HERE / "RESULTS.template.md"
    if template.exists():
        (HERE / "RESULTS.md").write_text(analysis.render(template.read_text(), summary))
    print(json.dumps({"summary": "results/summary.json", "tables": sorted(tabs),
                      "results_md": template.exists()}))


if __name__ == "__main__":
    main()
