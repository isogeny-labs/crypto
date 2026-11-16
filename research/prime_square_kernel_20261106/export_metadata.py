"""Transcode native CSV values to experiment.export/v1; no metric analysis.

The original protocol cross-check is expected to expose its layout error.
The corrected draft remains explicitly retrospective and unfrozen.
"""
import csv
import gzip
import importlib.util
import io
import json
import pathlib

root = pathlib.Path(__file__).resolve().parent
workspace = root.parents[2]
tool = workspace / "crypto-autoresearcher/tools/experiment_protocol.py"
spec = importlib.util.spec_from_file_location("experiment_protocol", tool)
ep = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ep)


def build(protocol_name, output_name):
    protocol = json.loads((root / protocol_name).read_text())
    e = ep.export_skeleton(protocol, protocol_name, "local", "square-kernel-local-20261107-v" + str(protocol["version"]))
    e["producer"].update(runtime="codex_api", role="executor")
    e["source"].update(repo="crypto", commit=protocol["arms"][0]["instrument"]["commit"], dirty=None)
    e["host"]["path"] = "runs/host.json"
    e["host"]["sha256"] = ep.sha256_file(root / "runs/host.json")
    host = json.loads((root / "runs/host.json").read_text())
    e["host"]["summary"].update(cpu_model=host["cpu_model"], logical_cpus=host["logical_cpus"],
                                kernel=host["kernel"], isolation_tier_max="L0")
    for arm in e["arms"]:
        p = next(a for a in protocol["arms"] if a["id"] == arm["id"])
        arm.update(commit=p["instrument"]["commit"], binary_sha256=p["instrument"]["binary_sha256"])
    for replicate, name in enumerate(("local-20261107-main", "local-20261107-replay")):
        folder = root / "runs" / name
        assert (folder / "exit-code.txt").read_text().strip() == "0"
        text = gzip.decompress((folder / "panel.csv.gz").read_bytes()).decode()
        for index, row in enumerate(csv.DictReader(io.StringIO(text))):
            for arm, field in (("original", "original_tuples"), ("square", "candidate_tuples")):
                value = row[field]
                # Number/string conversion is representation only, not a recomputation.
                encoded = int(value) if len(value) < 16 else value
                r = ep.run_row(
                    run_id=f"{name}/{index}/{arm}", arm=arm, instance_id=str(index), seed=0,
                    replicate=replicate, status="succeeded", worker="local-cpu0",
                    backend_ref=name,
                    measurements={"counted": {"tuples": encoded}, "wall_valid": False,
                                  "counters": {k: row[k] for k in ("L", "t", "level", "zeta", "q", "method")}},
                    isolation={"tier": "L0", "contended": None, "blockers": ["No timing claim"]},
                    verification={"status": "no_claim", "verifier": "Unsigned native assertions and grouped recomputation are retained separately"},
                    started_at=(folder / "started.txt").read_text().strip(),
                    finished_at=(folder / "finished.txt").read_text().strip(),
                )
                e["runs"].append(r)
    e["flow"] = ep.compute_flow(e["runs"])
    for p in sorted((root / "runs").rglob("*")):
        if p.is_file():
            e["files"].append(ep.file_entry(root, str(p.relative_to(root)), "raw"))
    e["files"].append(ep.file_entry(root, "local-override-20261107.json", "receipt"))
    e["analysis"]["script"] = ep.file_entry(root, "check.cpp", "analysis")
    for name in ("summary.json", "per-level.csv", "comparisons.svgz", "comparisons.pdf", "raw-bindings.json"):
        e["analysis"]["outputs"].append(ep.file_entry(root, "reports/local-20261107/" + name, "analysis"))
    texts = [
        "Two OS panel invocations;10752 logical arm/factorization/cell/replicate observations.",
        "Fixed unit controls ran once before both panels. Positive Smith controls are embedded in every cell.",
        "No timings were measured. Version1 is unchanged; version2 is a post-data export-layout draft.",
        "All comparisons and classifications were fixed in version1. The validator refuses retrospective freezing.",
        "The native metric computation is frozen check.cpp. Post-run analyze.cpp independently recounts the histograms and implements the predeclared grouping.",
    ]
    e["notes"] = [{"kind": "observation", "text": text, "by": "producer"} for text in texts]
    path = root / output_name
    path.write_text(json.dumps(e, separators=(",", ":")) + "\n")
    errors = ep.schema_errors(e, "experiment.export/v1")
    (root / "reports/local-20261107" / (output_name + ".schema.json")).write_text(
        json.dumps({"schema_errors": errors, "logical_rows": len(e["runs"]), "flow": e["flow"]}, indent=2) + "\n")
    assert not errors, errors[:5]


build("protocol.json", "export-v1-attempt.json")
build("protocol-export-v2.json", "export-v2-draft.json")
