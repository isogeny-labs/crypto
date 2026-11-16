"""Bounded full-D18 cadence-32 native XOR positive/natural supervisor."""

import argparse
import copy
import importlib.util
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import time

HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location(
    "n83_witness_preflight", HERE.parent / "native-xcnf-witness-20261025/run.py")
witness_run = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(witness_run)
PRIOR_SPEC = importlib.util.spec_from_file_location(
    "n83_prior_group_replay", HERE.parent / "local-extended-natural-20261025/audit.py")
prior = importlib.util.module_from_spec(PRIOR_SPEC)
PRIOR_SPEC.loader.exec_module(prior)


def save(path, record):
    with path.open("x") as stream:
        json.dump(record, stream, sort_keys=True, indent=2)
        stream.write("\n")


def run_arm(source, binary, base, output, identity, mode):
    arm = output / mode
    arm.mkdir(exist_ok=False)
    positive = mode == "positive"
    outer_cap = 310 if positive else 2_410
    result_path = arm / "result.json"
    argv = ["/usr/bin/time", "-l", str(binary),
            "primary-full-d18-m4-l0-ordered-xor-cadence32", mode, str(base),
            str(source / witness_run.FIXTURE), str(result_path)]
    save(arm / "command.json", argv)
    started_utc = witness_run.utc()
    started = time.monotonic()
    load_before = os.getloadavg()
    timed_out = False
    with (arm / "stdout.txt").open("wb") as stdout, (arm / "stderr-time.txt").open("wb") as stderr:
        process = subprocess.Popen(argv, cwd=source, stdout=stdout, stderr=stderr,
                                   start_new_session=True)
        try:
            code = process.wait(timeout=outer_cap)
        except subprocess.TimeoutExpired:
            timed_out = True
            os.killpg(process.pid, signal.SIGKILL)
            code = process.wait()
    usage = (arm / "stderr-time.txt").read_text(errors="replace")
    match = re.search(r"(\d+)\s+maximum resident set size", usage)
    supervisor = {"schema": "n83.d18-xor-cadence32-supervisor/v1",
                  "mode": mode, "source_commit": identity["source_commit"],
                  "binary_sha256": identity["binary_sha256"],
                  "object_sha256": witness_run.OBJECT_SHA,
                  "started_utc": started_utc, "ended_utc": witness_run.utc(),
                  "elapsed_wall_seconds": time.monotonic() - started,
                  "outer_cap_seconds": outer_cap, "outer_cap_reached": timed_out,
                  "exit_code": code,
                  "maximum_resident_set_bytes": int(match.group(1)) if match else None,
                  "loadavg_before": load_before, "loadavg_after": os.getloadavg(),
                  "isolation": "L0_shared_host_no_hard_cgroup",
                  "selected_best_total_runtime": None}
    save(arm / "supervisor.json", supervisor)
    result = json.loads(result_path.read_text()) if result_path.exists() else None
    if result is not None and (result["source_commit"] != identity["source_commit"]
                               or result["binary_sha256"] != identity["binary_sha256"]
                               or result["object"] != "objects/" + witness_run.OBJECT
                               or result["search_policy"] != "phase_zero_first_ordered_first_pair_xor_cadence32"
                               or result["shape"]["ordered_first_pair"] is not True
                               or result["shape"]["xor_decision_cadence"] != 32
                               or result["shape"]["variables"] != 329_808
                               or result["shape"]["clauses"] != (2_807_999 if positive else 2_807_501)
                               or result["shape"]["xor_rows"] != 3_901
                               or result["solver_stats"]["conflicts"] != result["solver_conflicts"]
                               or result["solver_stats"]["xor_passes"] < 1
                               or result["selected_best_total_runtime"] is not None):
        raise ValueError(f"{mode} worker result identity or policy differs")
    if timed_out:
        status = "OUTER_WATCHDOG_CENSORED"
    elif code == 124 and result is None and Path(str(result_path) + ".cap.json").exists():
        cap = json.loads(Path(str(result_path) + ".cap.json").read_text())
        if (cap["schema"] != "n83.full-d18-m4-cap/v1"
                or cap["status"] != "UNKNOWN_wall_cap"
                or cap["mode"] != mode
                or cap["budget_seconds"] != (300 if positive else 2_400)
                or cap["root_encoding"] != "shared_root_bits"
                or cap["root_choice_pinned"] is not False
                or cap["choice_pin_policy"] != "none"
                or cap["search_policy"] != "phase_zero_first_ordered_first_pair_xor_cadence32"
                or cap["isolation"] != "L0_shared_host_no_hard_cgroup"
                or cap["elapsed_process_wall_ms"] < cap["budget_seconds"] * 1_000
                or cap["selected_best_total_runtime"] is not None):
            raise ValueError(f"{mode} internal wall-cap receipt differs")
        status = "UNKNOWN_wall_cap"
    elif positive:
        status = ("PASS_CONSTRUCTED_CONTROL" if code == 0
                  and result is not None
                  and result["status"] == "PASS_CONSTRUCTED_CONTROL"
                  and result["relation"]["rank_after_insert"] == 1
                  and result["relation"]["tamper_rejected"] else "POSITIVE_GATE_FAILED")
    elif code == 0 and result is not None:
        status = result["status"]
    else:
        status = "PRODUCER_FAILURE"
    return {"mode": mode, "status": status, "supervisor": supervisor,
            "result_status": None if result is None else result["status"]}


def independent_positive_gate(output, base, identity):
    result = json.loads((output / "positive/result.json").read_text())
    obj = base / "objects" / witness_run.OBJECT
    group = prior.group_module()
    relation = prior.replay_relation(result, obj, group)
    negative = {}
    for name in ("changed_target", "changed_row_digest"):
        altered = copy.deepcopy(result)
        if name == "changed_target":
            altered["target"][0] = str(int(altered["target"][0]) ^ 1)
        else:
            altered["relation"]["row_decimal_blake3"] = "0" * 64
        try:
            prior.replay_relation(altered, obj, group)
        except AssertionError:
            negative[name] = "REJECTED"
        else:
            raise ValueError(f"constructed {name} control was accepted")
    gate = {"schema": "n83.d18-xor-cadence32-positive-gate/v1",
            "source_commit": identity["source_commit"],
            "binary_sha256": identity["binary_sha256"],
            "relation": relation, "negative_controls": negative}
    save(output / "positive-independent-gate.json", gate)
    return gate


def run(args):
    source, binary, base, output = map(lambda value: Path(value).resolve(),
                                       (args.source, args.binary, args.base, args.output))
    identity = witness_run.preflight(source, binary, base)
    identity.pop("expected_xcnf_sha256")
    identity["schema"] = "n83.d18-xor-cadence32-identity/v1"
    identity["protocol"] = "positive-300-310_then_natural-2400-2410_xor-cadence32"
    output.mkdir(exist_ok=False)
    save(output / "identity.json", identity)
    positive = run_arm(source, binary, base, output, identity, "positive")
    if positive["status"] == "PASS_CONSTRUCTED_CONTROL":
        independent_positive_gate(output, base, identity)
    natural = (run_arm(source, binary, base, output, identity, "natural")
               if positive["status"] == "PASS_CONSTRUCTED_CONTROL" else None)
    summary = {"schema": "n83.d18-xor-cadence32-run/v1",
               "source_commit": identity["source_commit"],
               "binary_sha256": identity["binary_sha256"],
               "positive_status": positive["status"],
               "natural_status": None if natural is None else natural["status"],
               "natural_search_executed": natural is not None,
               "selected_best_total_runtime": None}
    save(output / "summary.json", summary)
    return 0 if positive["status"] == "PASS_CONSTRUCTED_CONTROL" else 2


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    for name in ("source", "binary", "base", "output"):
        parser.add_argument("--" + name, required=True)
    sys.exit(run(parser.parse_args()))
