#!/usr/bin/env python3
"""Write the frozen inputs of the P4 confirmatory study, deterministically.

research/prime_medium_rho_p4_confirm_20261108 (PROTOCOL.md sections 3-6).
Standard library only. Run from the repository root:

    python3 -I research/prime_medium_rho_p4_confirm_20261108/build_study.py \
        [--ecbench /path/to/frozen/ecbench]

Writes curves.json, seeds.json, specs/M1..M9.json, specs/PC.json and
requests/*.request.json; with --ecbench it also runs `ecbench plan` on every
spec and writes specs/spec-ids.json (spec id and execution count per job,
with the planning binary's sha256). Rerunning writes the same bytes.
"""
import argparse
import hashlib
import json
import os
import subprocess

D = "research/prime_medium_rho_p4_confirm_20261108"
OLD = "research/prime_medium_tier_rho_20261107"
PREFIX = "prime_medium_rho_p4_confirm_20261108/v1/"
IMAGE = ("104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:"
         "292bdc3d91ef540bc7719f20902c5acc614dab458676225623275bb3a838cae5")
SOURCE = "dbd587a01aefba0634acc459ea30cfb9bbf49a09"
ECBENCH = "a18af09f9fb875dd09d0399259b5481b6952d9a8f1f52f5fccc7d9ab1a0cd3b9"

MAIN_ARMS = [
    {"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
    {"name": "rho-neg-aa", "role": "control", "method": {"id": "rho.negation"}},
    {"name": "rho-plain", "role": "baseline", "method": {"id": "rho.plain"}},
    {"name": "rho-neg-v2", "role": "candidate", "method": {"id": "rho.negation_v2"}},
]
PC_ARMS = [
    {"name": "bsgs-neg", "role": "reference", "method": {"id": "bsgs.negation"}},
    {"name": "bsgs-neg-aa", "role": "control", "method": {"id": "bsgs.negation"}},
    {"name": "bsgs-tb", "role": "baseline", "method": {"id": "bsgs.textbook"}},
]
# job, curve indices into curves.json, targets per curve, target block, arms, label
JOBS = [
    ("M1", [0, 1, 2, 3, 4, 5], 36, "K01", "main", "33-, 37- and 41-bit curves"),
    ("M2", [6, 7], 18, "K02", "main", "45-bit curves, target block K02"),
    ("M3", [6, 7], 18, "K03", "main", "45-bit curves, target block K03"),
    ("M4", [8], 12, "K04", "main", "49-bit curve 1, target block K04"),
    ("M5", [8], 12, "K05", "main", "49-bit curve 1, target block K05"),
    ("M6", [8], 12, "K06", "main", "49-bit curve 1, target block K06"),
    ("M7", [9], 12, "K07", "main", "49-bit curve 2, target block K07"),
    ("M8", [9], 12, "K08", "main", "49-bit curve 2, target block K08"),
    ("M9", [9], 12, "K09", "main", "49-bit curve 2, target block K09"),
    ("PC", [0, 1, 2, 3, 4, 5], 16, "K10", "pc", "positive control, bsgs.textbook / bsgs.negation on the 33-41-bit curves"),
]
ROUNDS, WARMUP, TIMEOUT = 6, 1, 300


def seed(tag):
    return int(hashlib.sha256((PREFIX + tag).encode()).hexdigest()[:12], 16)


def dump(path, obj):
    with open(path, "w") as fh:
        json.dump(obj, fh, indent=2)
        fh.write("\n")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--ecbench")
    a = ap.parse_args()
    os.makedirs(os.path.join(D, "specs"), exist_ok=True)
    os.makedirs(os.path.join(D, "requests"), exist_ok=True)

    old_curves_path = os.path.join(OLD, "curves.json")
    raw = open(old_curves_path, "rb").read()
    old = json.loads(raw)
    curves = []
    for i, c in enumerate(old["curves"]):
        curves.append({"slug": c["slug"], "field_bits": c["field_bits"], "index_in_size": c["index"],
                       "r": c["subgroup_order"], "cofactor": c["cofactor"],
                       "generator_call": c["generator_call"], "curve_spec": c["curve_spec"],
                       "jobs": [j[0] for j in JOBS if i in j[1]]})
    dump(os.path.join(D, "curves.json"), {
        "schema": "prime_medium_rho_p4_confirm.curves/v1",
        "what_this_is": "The same ten generic prime-order curves as the 2026-11-07 study (no superset), copied from its curves.json; curve facts were resolved there by ecbench plan and are registered in docs/curves/registry.json.",
        "source": {"path": old_curves_path, "sha256": hashlib.sha256(raw).hexdigest()},
        "curves": curves})

    tags = (["target_seed/block=K%02d" % k for k in range(1, 11)]
            + ["measurement_seed/job=%s" % j[0] for j in JOBS]
            + ["analysis/bootstrap", "analysis/bootstrap-large", "analysis/bootstrap-positive-control"])
    seeds = [{"tag": t, "seed": seed(t)} for t in tags]
    old_seeds = json.load(open(os.path.join(OLD, "seeds.json")))
    old_vals = {s["seed"] for s in old_seeds["seeds"]}
    new_vals = [s["seed"] for s in seeds]
    assert len(set(new_vals)) == len(new_vals), "seed collision inside the new list"
    assert not (set(new_vals) & old_vals), "a new seed equals a 2026-11-07 seed"
    dump(os.path.join(D, "seeds.json"), {
        "schema": "prime_medium_rho_p4_confirm.seeds/v1",
        "rule": "seed(tag) = int(SHA-256(UTF-8(prefix + tag)).hexdigest()[0:12], 16), a 48-bit integer",
        "prefix": PREFIX,
        "written": "2026-11-08, before any data of this study",
        "disjoint_from": {"path": os.path.join(OLD, "seeds.json"),
                          "checked": "no value here equals any value there, and the values here are distinct"},
        "seeds": seeds})

    specs = {}
    for job, idx, targets, block, kind, what in JOBS:
        arms = MAIN_ARMS if kind == "main" else PC_ARMS
        spec = {
            "schema": "ecbench.spec/v1",
            "label": "prime medium rho P4 confirm %s: %s" % (job, what),
            "description": ("research/prime_medium_rho_p4_confirm_20261108/PROTOCOL.md. Confirmatory study of the paired "
                            "rho.plain / rho.negation operation-count ratio on the ten medium-tier curves of "
                            "research/prime_medium_tier_rho_20261107 (curves.json); seeds from seeds.json (new, disjoint). "
                            "Operation counts are the result; wall time only at L2 against the A/A arm."),
            "workloads": {"curves": [curves[i]["curve_spec"] for i in idx],
                          "targets_per_curve": targets,
                          "target_seed": seed("target_seed/block=%s" % block),
                          "target_kind": "planted"},
            "arms": arms,
            "measurement": {"rounds": ROUNDS, "warmup": WARMUP, "order": "alternate",
                            "seed": seed("measurement_seed/job=%s" % job),
                            "isolation_required": "L2", "timeout_seconds": TIMEOUT},
        }
        path = os.path.join(D, "specs", "%s.json" % job)
        dump(path, spec)
        specs[job] = path
        req = {"schema": "isolab.eks.request/v1",
               "job": {"job_name": "prime-medium-rho-p4-20261108-%s" % job.lower(),
                       "instance_type": "c8a.2xlarge", "seconds": 3600, "memory": "4Gi",
                       "spec": spec,
                       "build": {"image_uri": IMAGE, "source_revision": SOURCE, "ecbench_sha256": ECBENCH}},
               "provenance": {"harness": "claude-code: research/prime_medium_rho_p4_confirm_20261108 (PROTOCOL.md)",
                              "source_revision": SOURCE}}
        dump(os.path.join(D, "requests", "%s.request.json" % job), req)

    if a.ecbench:
        binsha = hashlib.sha256(open(a.ecbench, "rb").read()).hexdigest()
        ids = {}
        execs = {}
        for job, path in specs.items():
            outp = subprocess.run([a.ecbench, "plan", "--spec", path], capture_output=True, text=True, check=True)
            first = outp.stdout.splitlines()[0]
            assert first.startswith("spec ECS1h"), first
            ids[job] = first.split()[1]
            execs[job] = int(first.rsplit(":", 1)[1].split()[0])
            if "warn" in (outp.stdout + outp.stderr).lower():
                raise SystemExit("plan warned for %s:\n%s%s" % (job, outp.stdout, outp.stderr))
        dump(os.path.join(D, "specs", "spec-ids.json"), {
            "schema": "prime_medium_rho_p4_confirm.spec-ids/v1",
            "planned_with": {"ecbench_sha256": binsha, "command": "ecbench plan --spec specs/<job>.json"},
            "spec_ids": ids, "executions": execs})


if __name__ == "__main__":
    main()
