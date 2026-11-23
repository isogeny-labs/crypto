#!/usr/bin/env python3
"""Register native values and hashes; leaves records open for raw publication."""
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
STUDY = Path(__file__).resolve().parent
REL = str(STUDY.relative_to(ROOT))


def bench(*args):
    subprocess.run(["python3", "tools/benchrec.py", *map(str, args)], cwd=ROOT, check=True)


def main():
    analysis = json.loads((STUDY / "both-pilots-analysis.json").read_text())
    cases = [
        ("20261119T1749Z-conductor-local-pilot-20261119", "pilot-l0", 202),
        ("20261119T1814Z-conductor-local-pilot96-20261119", "pilot96-l0", 96),
        ("20261119T1802Z-conductor-local-scalars-20261119", "scalar-l0", None),
        ("20261119T1752Z-conductor-target80-local-20261119", "target80-bounded", None),
    ]
    for name, folder, field in cases:
        record = f"benchmarks/records/{name}.json"
        raw = sorted(p for p in (STUDY / "results" / folder).rglob("*") if p.is_file())
        bench("add-raw", record, *(str(p.relative_to(ROOT)) for p in raw),
              f"{REL}/local-results.tar.xz")
        if field:
            for group in analysis["groups"]:
                if group["field_bits"] != field:
                    continue
                bench("metric", record, "--name", f"{group['curve_slug']}.{group['arm']}.mean_solve",
                      "--value", group["mean_solve_seconds"], "--unit", "wall_s", "--n", group["attempts"],
                      "--better", "neutral", "--note",
                      "Native descriptive L0 mean, one shared target/two rounds; setup included; unpriced work; no wall or ratio admission.")
            summary = "12 verified solves and exact native replays; independent PARI checks pass. L0 capacity pilot, one target and two rounds."
            if field == 202:
                summary += " Record initialized after initial call; recorded commands launch the replay only."
        elif folder == "scalar-l0":
            manifest = json.loads((STUDY / "results/scalar-l0/manifest.json").read_text())
            for row in manifest["descriptive_by_curve"]:
                for kind in ("mean", "median"):
                    bench("metric", record, "--name", f"{row['curve_slug']}.{kind}_scalar",
                          "--value", row[f"{kind}_ns_per_mul"] / 1e9, "--unit", "wall_s", "--n", row["samples"],
                          "--better", "neutral", "--note",
                          "Native nanoseconds converted to seconds per multiplication; L0 batched throughput; batch64; warmups excluded; setup/verification excluded; no wall admission.")
            summary = "1344 independently PARI-checked batches: six models,32 points,6 measured rounds plus warmup; not ECDLP solves."
        else:
            summary = "Censored at60second watchdog (exit124); original80bit native constructor produces no quotient/images or durable partial coverage."
        path = ROOT / record
        value = json.loads(path.read_text())
        assert value["status"] == "open"
        value["summary"] = summary + " Record remains open pending authenticated raw-data publication."
        value["reports"] = {
            "pdf": f"{REL}/reports/report.pdf",
            "svg": [f"{REL}/reports/ecdLP.svg", f"{REL}/reports/scalar.svg"],
            "png": [f"{REL}/reports/ecdLP.png", f"{REL}/reports/scalar.png"],
        }
        value["validation"] = f"{REL}/summary.json"
        value["publication_pending"] = f"{REL}/publication-pending.json"
        path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n")


if __name__ == "__main__":
    main()
