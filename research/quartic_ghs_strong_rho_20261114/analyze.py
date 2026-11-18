#!/usr/bin/env python3
"""Reprice historical GHS IC and validate the matched Round-A strong rho."""

from __future__ import annotations

import argparse
import collections
import dataclasses
import hashlib
import json
import math
import random
import re
import statistics
import sys
import tarfile
from pathlib import Path
from typing import Any, Iterable

GRID = (4051, 8191, 16363, 32719, 65479, 131071, 262111, 524287, 1048507, 2097091, 4194271)
SEEDS = (1, 2, 3)
WEIGHTS = (0.0, 0.1, 1.0)
PHASES = ("fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify")
IC_ARMS = ("B0", "S", "L0", "L1", "L", "D", "DL1", "DL")
SD_ARMS = ("S", "L0", "L1", "L", "D", "DL1", "DL")
DIAGNOSTIC_Q = (4051, 65479, 262111)
ARCHIVE_SHA256 = "aac9a25ce50ac5d23895f0dba8f907fb223aaac32a2a41391bc590e3c6bcca42"
ARCHIVE_BYTES = 382105
BOOTSTRAPS = 10_000
PRIMARY_ARM = "strong-b8"
MAIN_SHA256 = "b2244ffece12855622a88f9a2af4926b1fac6113b9519fdf11e2bc7449fa27c8"
STRONG_SHA256 = "73abd342613dee852add1e3459c94bc7f949801c94eb875a309cdfc4f6b0f5de"
POD_RUN_SHA256 = "a7a7795c0013485855805c87c14bb67fee2864273a934e72b85ebb3bef7e0a6e"
PROTOCOL_SHA256 = "784e8674119cd42d726f16f52258d64df7777317c7b6bc38665fae84b71c9345"
SOURCE_RE = re.compile(r"^[a-f0-9]{40}$")
HEX64_RE = re.compile(r"^[a-f0-9]{64}$")


class Invalid(ValueError):
    pass


def need(condition: bool, message: str) -> None:
    if not condition:
        raise Invalid(message)


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as handle:
        for block in iter(lambda: handle.read(1024 * 1024), b""):
            h.update(block)
    return h.hexdigest()


def integer(value: Any, label: str) -> int:
    need(isinstance(value, int) and not isinstance(value, bool) and value >= 0, f"{label} is not a nonnegative integer")
    return value


def number(value: Any, label: str) -> float:
    need(isinstance(value, (int, float)) and not isinstance(value, bool), f"{label} is not numeric")
    result = float(value)
    need(math.isfinite(result) and result >= 0, f"{label} is negative or non-finite")
    return result


def median3(values: Iterable[float]) -> float:
    values = list(values)
    need(len(values) == 3, "calibration median requires exactly three seeds")
    return statistics.median(values)


@dataclasses.dataclass
class Components:
    mul: int = 0
    inv: int = 0
    lmul: int = 0
    band: int = 0

    def add(self, other: "Components") -> None:
        self.mul += other.mul
        self.inv += other.inv
        self.lmul += other.lmul
        self.band += other.band

    def price(self, r_inv: float, r_lmul: float, weight: float) -> float:
        return self.mul + r_inv * self.inv + r_lmul * self.lmul + weight * (56.0 + r_inv) * self.band


@dataclasses.dataclass
class IcUnit:
    q: int
    seed: int
    arm: str
    target: int
    ell: int
    attempts: int
    successful: bool
    components: Components


@dataclasses.dataclass
class RhoRun:
    q: int
    seed: int
    index: int
    arm: str
    target: int
    ell: int
    steps: int
    batch: int | None
    components: Components
    setup: Components
    walk: Components
    verify: Components
    record: dict[str, Any]


def json_lines(payload: bytes, label: str) -> list[dict[str, Any]]:
    records = []
    for line_number, raw in enumerate(payload.decode("utf-8").splitlines(), 1):
        if not raw.startswith("{"):
            continue
        try:
            value = json.loads(raw)
        except json.JSONDecodeError as exc:
            raise Invalid(f"{label}:{line_number}: invalid JSON") from exc
        need(isinstance(value, dict), f"{label}:{line_number}: record is not an object")
        records.append(value)
    return records


def phase_components(record: dict[str, Any]) -> Components:
    total = Components()
    for phase in PHASES:
        total.add(Components(
            integer(record.get(f"{phase}.mul", 0), f"{phase}.mul"),
            integer(record.get(f"{phase}.inv", 0), f"{phase}.inv"),
            integer(record.get(f"{phase}.lmul", 0), f"{phase}.lmul"),
            integer(record.get(f"{phase}.hash", 0), f"{phase}.hash")
            + integer(record.get(f"{phase}.cmp", 0), f"{phase}.cmp"),
        ))
    return total


def audit_historical(path: Path) -> tuple[dict[tuple[int, int, str], IcUnit], dict[tuple[int, int], int], dict[str, Any]]:
    need(path.stat().st_size == ARCHIVE_BYTES, "historical archive byte count mismatch")
    need(sha256_file(path) == ARCHIVE_SHA256, "historical archive SHA-256 mismatch")
    gathered: dict[tuple[int, int, str], list[dict[str, Any]]] = collections.defaultdict(list)
    member_count = 0
    with tarfile.open(path, "r:gz") as archive:
        members = {member.name: member for member in archive.getmembers() if member.isfile()}
        for q in GRID:
            for seed in SEEDS:
                names = [f"units/ic_{q}_{seed}_SD.jsonl"]
                if q != 4194271:
                    names.append(f"units/ic_{q}_{seed}_B0.jsonl")
                for name in names:
                    need(name in members, f"historical member missing: {name}")
                    handle = archive.extractfile(members[name])
                    need(handle is not None, f"cannot read historical member: {name}")
                    member_count += 1
                    for record in json_lines(handle.read(), name):
                        if record.get("rec") != "ic":
                            continue
                        arm = record.get("arm")
                        need(arm in IC_ARMS, f"unexpected historical IC arm: {arm}")
                        need(record.get("q") == q and record.get("seed") == seed, f"historical identity mismatch: {name}")
                        gathered[(q, seed, arm)].append(record)

    units: dict[tuple[int, int, str], IcUnit] = {}
    targets: dict[tuple[int, int], int] = {}
    for q in GRID:
        for seed in SEEDS:
            required = set(SD_ARMS) | ({"B0"} if q != 4194271 else set())
            need(required <= {arm for qq, ss, arm in gathered if qq == q and ss == seed}, f"historical arm census incomplete at q={q}, seed={seed}")
            seen_targets: set[int] = set()
            seen_ell: set[int] = set()
            for arm in required:
                records = gathered[(q, seed, arm)]
                need(records, f"empty historical unit q={q}, seed={seed}, arm={arm}")
                components = Components()
                for record in records:
                    components.add(phase_components(record))
                    target = integer(record.get("k"), "historical k")
                    seen_targets.add(target)
                    seen_ell.add(integer(record.get("ell"), "historical ell"))
                final = records[-1]
                successful = final.get("ok") is True and final.get("k_found") == final.get("k")
                units[(q, seed, arm)] = IcUnit(
                    q=q,
                    seed=seed,
                    arm=arm,
                    target=integer(final.get("k"), "historical final k"),
                    ell=integer(final.get("ell"), "historical final ell"),
                    attempts=len(records),
                    successful=successful,
                    components=components,
                )
            need(len(seen_targets) == 1, f"historical targets disagree at q={q}, seed={seed}")
            need(len(seen_ell) == 1, f"historical subgroup orders disagree at q={q}, seed={seed}")
            targets[(q, seed)] = next(iter(seen_targets))
    summary = {
        "sha256": ARCHIVE_SHA256,
        "bytes": ARCHIVE_BYTES,
        "members_read": member_count,
        "units": len(units),
        "successful_units": sum(unit.successful for unit in units.values()),
        "targets": len(targets),
        "q22_b0_status": "not_recorded",
    }
    return units, targets, summary


def verify_sha_manifests(root: Path) -> dict[str, Any]:
    manifests = sorted(root.rglob("SHA256SUMS"))
    need(len(manifests) == 4, f"expected four split SHA256SUMS files, found {len(manifests)}")
    covered: set[Path] = set()
    splits: set[str] = set()
    files_checked = 0
    for manifest in manifests:
        directory = manifest.parent.resolve()
        lines = manifest.read_text(encoding="utf-8").splitlines()
        need(lines, f"empty manifest: {manifest}")
        listed: set[Path] = set()
        for line in lines:
            match = re.fullmatch(r"([a-f0-9]{64})  (.+)", line)
            need(match is not None, f"malformed SHA256SUMS line: {manifest}")
            relative = Path(match.group(2))
            need(not relative.is_absolute() and ".." not in relative.parts, f"unsafe manifest path: {relative}")
            target = (directory / relative).resolve()
            need(target.is_relative_to(directory), f"manifest path escapes split: {relative}")
            need(target.is_file(), f"manifest target missing: {target}")
            need(target not in listed, f"duplicate manifest target: {target}")
            need(sha256_file(target) == match.group(1), f"manifest hash mismatch: {target}")
            listed.add(target)
            covered.add(target)
            files_checked += 1
        actual = {
            path.resolve()
            for path in directory.rglob("*")
            if path.is_file() and path.name != "SHA256SUMS"
        }
        need(actual == listed, f"split files and SHA256SUMS coverage differ: {directory}")
        host = directory / "host.txt"
        source_revision = directory / "source_revision.txt"
        source_hash = directory / "source.sha256"
        binary_hash = directory / "binary.sha256"
        selftest = directory / "selftest.out"
        exits = directory / "exit_codes.txt"
        for required in (host, source_revision, source_hash, binary_hash, selftest, exits):
            need(required.resolve() in covered, f"required split artifact not covered: {required}")
        host_text = host.read_text(encoding="utf-8", errors="replace")
        split_lines = [line.split("=", 1)[1] for line in host_text.splitlines() if line.startswith("split=")]
        need(len(split_lines) == 1 and split_lines[0] in {"low", "mid", "high", "top"}, f"invalid split host receipt: {host}")
        splits.add(split_lines[0])
        need(selftest.read_text(encoding="utf-8").strip() == "SELFTEST OK", f"self-test did not pass: {selftest}")
        expected_sources = {
            "PROTOCOL.md": PROTOCOL_SHA256,
            "main.rs": MAIN_SHA256,
            "strong_rho.rs": STRONG_SHA256,
            "pod_run.sh": POD_RUN_SHA256,
        }
        source_lines = source_hash.read_text(encoding="utf-8").splitlines()
        need(len(source_lines) == len(expected_sources), f"wrong source hash count: {source_hash}")
        recorded_sources = {}
        for line in source_lines:
            match = re.fullmatch(r"([a-f0-9]{64})  (/.+)", line)
            need(match is not None, f"invalid source hash line: {source_hash}")
            source_path = Path(match.group(2))
            name = source_path.name
            need(name in expected_sources and name not in recorded_sources, f"unexpected source hash path: {source_path}")
            need(
                source_path.as_posix().endswith(f"/research/quartic_ghs_strong_rho_20261114/{name}"),
                f"source hash path is outside the study: {source_path}",
            )
            recorded_sources[name] = match.group(1)
        need(recorded_sources == expected_sources, f"split source hashes do not match: {source_hash}")

        binary_lines = binary_hash.read_text(encoding="utf-8").splitlines()
        need(len(binary_lines) == 1, f"wrong binary hash count: {binary_hash}")
        binary_match = re.fullmatch(r"([a-f0-9]{64})  (/.+/ghs-strong-rho)", binary_lines[0])
        need(binary_match is not None, f"invalid binary hash: {binary_hash}")
        binary = (directory / "ghs-strong-rho").resolve()
        need(binary in covered, f"worker binary is not manifest-covered: {binary}")
        need(binary_match.group(1) == sha256_file(binary), f"recorded binary hash mismatch: {binary}")

        exit_lines = exits.read_text(encoding="utf-8", errors="replace").splitlines()
        for seed in SEEDS:
            seed_lines = [line for line in exit_lines if line.startswith(f"seed={seed} ")]
            need(seed_lines == [f"seed={seed} rc=0"], f"seed {seed} did not finish exactly once in {directory}")
        finish_lines = [line for line in exit_lines if line.startswith("finish=")]
        need(len(finish_lines) == 1, f"split has no unique finish record: {directory}")
        need(re.fullmatch(r"finish=\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z rc=0", finish_lines[0]) is not None,
             f"split did not finish successfully: {directory}")
        replay_lines = [line for line in exit_lines if line.startswith("replay=")]
        if split_lines[0] == "low":
            need(replay_lines == ["replay=match"], f"low split replay exit record failed: {directory}")
        else:
            need(not replay_lines, f"unexpected replay exit record: {directory}")
        expected_exit_lines = [f"seed={seed} rc=0" for seed in SEEDS]
        if split_lines[0] == "low":
            expected_exit_lines.append("replay=match")
        expected_exit_lines.append(finish_lines[0])
        need(exit_lines == expected_exit_lines, f"unexpected or reordered split exit record: {directory}")
    need(splits == {"low", "mid", "high", "top"}, "split manifests are not low/mid/high/top")
    return {"manifests": len(manifests), "files_checked": files_checked, "covered": covered}


def one_file(root: Path, pattern: str) -> Path:
    matches = sorted(root.rglob(pattern))
    need(len(matches) == 1, f"expected one {pattern}, found {len(matches)}")
    return matches[0].resolve()


def strong_components(record: dict[str, Any]) -> tuple[Components, Components, Components, Components]:
    phases = []
    for prefix in ("setup", "walk", "verify"):
        phases.append(Components(
            integer(record.get(f"{prefix}.fq_mul"), f"{prefix}.fq_mul"),
            integer(record.get(f"{prefix}.fq_inv"), f"{prefix}.fq_inv"),
            0,
            0,
        ))
    for prefix in ("setup_native", "walk_native"):
        fields = ("canon", "lookahead_test", "dp", "table", "cycle_cmp")
        calculated = sum(integer(record.get(f"{prefix}.{field}"), f"{prefix}.{field}") for field in fields)
        need(calculated == integer(record.get(f"{prefix}.band_ops"), f"{prefix}.band_ops"), f"{prefix} band identity failed")
    phases[0].lmul = integer(record.get("setup_native.scalar_mul"), "setup scalar_mul")
    phases[0].band = integer(record.get("setup_native.band_ops"), "setup band_ops")
    phases[1].lmul = integer(record.get("walk_native.scalar_mul"), "walk scalar_mul")
    phases[1].band = integer(record.get("walk_native.band_ops"), "walk band_ops")
    total = Components()
    for phase in phases:
        total.add(phase)
    return total, phases[0], phases[1], phases[2]


def old_components(record: dict[str, Any]) -> Components:
    return Components(
        integer(record.get("c.mul"), "c.mul"),
        integer(record.get("c.inv"), "c.inv"),
        integer(record.get("c.lmul"), "c.lmul"),
        integer(record.get("c.hash"), "c.hash") + integer(record.get("c.cmp"), "c.cmp"),
    )


def parse_rho_record(record: dict[str, Any], q: int, seed: int, target: int, ell: int,
                     arm: str, expected_index: int) -> RhoRun:
    need(record.get("count_authoritative") is not False,
         "non-authoritative rho field counts cannot enter modeled cost")
    need(record.get("rec") == "rho" and record.get("arm") == arm, "wrong rho record arm")
    need(record.get("q") == q and record.get("seed") == seed and record.get("i") == expected_index, "rho identity mismatch")
    need(record.get("target_kind") == "ic" and record.get("k") == target, "rho target mismatch")
    need(record.get("ell") == ell, "rho subgroup-order mismatch")
    need(record.get("ok") is True, "rho correctness flag failed")
    if arm.startswith("strong-b"):
        need(record.get("k_found") == target, "rho recovered scalar mismatch")
        batch = integer(record.get("batch"), "rho batch")
        total, setup, walk, verify = strong_components(record)
    else:
        batch = None
        total = old_components(record)
        setup, walk, verify = Components(), total, Components()
    return RhoRun(
        q=q,
        seed=seed,
        index=expected_index,
        arm=arm,
        target=target,
        ell=ell,
        steps=integer(record.get("steps"), "rho steps"),
        batch=batch,
        components=total,
        setup=setup,
        walk=walk,
        verify=verify,
        record=record,
    )


def load_results(root: Path, revision: str, targets: dict[tuple[int, int], int],
                 historical: dict[tuple[int, int, str], IcUnit]) -> tuple[dict[int, dict[str, Any]], dict[str, Any]]:
    manifest_audit = verify_sha_manifests(root)
    covered: set[Path] = manifest_audit.pop("covered")
    by_q: dict[int, dict[str, Any]] = {}
    for q in GRID:
        expected_repetitions = 5 if q <= 262111 else 3
        seed_rows = {}
        for seed in SEEDS:
            path = one_file(root, f"q{q}_s{seed}_primary.jsonl")
            need(path in covered, f"primary file is not manifest-covered: {path}")
            records = json_lines(path.read_bytes(), str(path))
            instances = [row for row in records if row.get("rec") == "instance"]
            calibrations = [row for row in records if row.get("rec") == "calibration"]
            rhos = [row for row in records if row.get("rec") == "rho"]
            need(len(instances) == 1 and len(calibrations) == 1, f"instance/calibration census failed: {path}")
            instance = instances[0]
            calibration = calibrations[0]
            target = targets[(q, seed)]
            ell = historical[(q, seed, SD_ARMS[0])].ell
            need(instance.get("q") == q and instance.get("seed") == seed, "instance identity mismatch")
            need(instance.get("ell") == ell and instance.get("order") == 4 * ell, "instance order mismatch")
            need(instance.get("ic_target") == target and instance.get("target_policy") == "ic-repeat", "instance target mismatch")
            need(instance.get("source_revision") == revision, "instance source revision mismatch")
            need(calibration.get("q") == q and calibration.get("seed") == seed, "calibration identity mismatch")
            need(calibration.get("triples") == 600 and calibration.get("isolation") == "isolab-eks-standard", "calibration protocol mismatch")
            r_inv = number(calibration.get("r_inv"), "r_inv")
            r_lmul = number(calibration.get("r_lmul"), "r_lmul")
            need(r_inv > 0 and r_lmul > 0, "zero calibration ratio")
            need(len(rhos) == expected_repetitions, f"wrong rho repetition count q={q}, seed={seed}")
            runs = [parse_rho_record(row, q, seed, target, ell, PRIMARY_ARM, index) for index, row in enumerate(rhos)]
            need(all(run.batch == 8 for run in runs), f"primary batch is not eight q={q}, seed={seed}")
            seed_rows[seed] = {"calibration": calibration, "runs": runs}
        by_q[q] = {"seeds": seed_rows}

    diagnostics: dict[str, Any] = {}
    for q in DIAGNOSTIC_Q:
        diagnostics[str(q)] = {}
        ell = historical[(q, 1, SD_ARMS[0])].ell
        for arm, tag, batch in (("strong-b4", "width-low-control", 4), ("strong-b16", "width-high-control", 16), ("old-neg", "frozen-control", None)):
            arm_runs = []
            for seed in SEEDS:
                path = one_file(root, f"q{q}_s{seed}_{tag}.jsonl")
                need(path in covered, f"diagnostic file is not manifest-covered: {path}")
                rows = [row for row in json_lines(path.read_bytes(), str(path)) if row.get("rec") == "rho"]
                need(len(rows) == 1, f"wrong diagnostic repetition count: {path}")
                run = parse_rho_record(rows[0], q, seed, targets[(q, seed)], ell, arm, 0)
                need(run.batch == batch, f"diagnostic batch mismatch: {path}")
                arm_runs.append(run)
            diagnostics[str(q)][arm] = arm_runs

    replay_path = one_file(root, "q4051_s1_replay.jsonl")
    need(replay_path in covered, "replay file is not manifest-covered")
    replay_rows = [row for row in json_lines(replay_path.read_bytes(), str(replay_path)) if row.get("rec") == "rho"]
    need(len(replay_rows) == 1, "replay rho census failed")
    primary = by_q[4051]["seeds"][1]["runs"][0].record.copy()
    replay = replay_rows[0].copy()
    primary.pop("wall_s", None)
    replay.pop("wall_s", None)
    need(primary == replay, "q4051 seed1 duplicate replay mismatch")

    for manifest in root.rglob("source_revision.txt"):
        need(manifest.read_text(encoding="utf-8").strip() == revision, f"split source revision mismatch: {manifest}")
    return by_q, {"sha_manifests": manifest_audit, "diagnostics": diagnostics, "replay": "match"}


def quantile(values: list[float], probability: float) -> float:
    ordered = sorted(values)
    return ordered[round((len(ordered) - 1) * probability)]


def ratio_interval(ic: dict[int, float], rho: dict[int, float], q: int, weight: float) -> tuple[float, float, float]:
    seeds = list(SEEDS)
    point = statistics.mean(ic.values()) / statistics.mean(rho.values())
    seed_material = hashlib.sha256(f"{q}:{weight}:ghs-strong-rho-r1".encode()).digest()
    rng = random.Random(int.from_bytes(seed_material[:8], "big"))
    samples = []
    for _ in range(BOOTSTRAPS):
        draw = [seeds[rng.randrange(len(seeds))] for _ in seeds]
        samples.append(statistics.mean(ic[seed] for seed in draw) / statistics.mean(rho[seed] for seed in draw))
    return point, quantile(samples, 0.025), quantile(samples, 0.975)


def classify_h1(sizes: list[dict[str, Any]]) -> dict[str, Any]:
    admitted = [size for size in sizes if size["admitted"]]
    unadmitted = [size for size in sizes if not size["admitted"]]
    admitted_weights = [row for size in admitted for row in size["weights"]]
    any_rejected = any(row["ci95"][0] >= 1.0 for row in admitted_weights)
    any_unresolved = any(row["ci95"][1] >= 1.0 for row in admitted_weights)
    all_below = all(row["ci95"][1] < 1.0 for row in admitted_weights)
    if any_rejected:
        verdict = "rejected"
    elif any_unresolved:
        verdict = "unresolved_interval_crosses_one"
    elif unadmitted:
        verdict = "limited_unadmitted_sizes"
    else:
        verdict = "confirmed_on_measured_grid"
    return {
        "verdict": verdict,
        "all_sizes_admitted": not unadmitted,
        "admitted_sizes": [size["q"] for size in admitted],
        "unadmitted_sizes": [size["q"] for size in unadmitted],
        "all_admitted_upper_bounds_below_one": all_below,
    }


def analyze(units: dict[tuple[int, int, str], IcUnit], results: dict[int, dict[str, Any]],
            audit: dict[str, Any], revision: str, archive_summary: dict[str, Any]) -> dict[str, Any]:
    sizes = []
    for q in GRID:
        seed_rows = results[q]["seeds"]
        r_inv = median3(seed_rows[seed]["calibration"]["r_inv"] for seed in SEEDS)
        r_lmul = median3(seed_rows[seed]["calibration"]["r_lmul"] for seed in SEEDS)
        ell = seed_rows[1]["runs"][0].ell
        all_runs = [run for seed in SEEDS for run in seed_rows[seed]["runs"]]
        steps_over_sqrt = statistics.mean(run.steps / math.sqrt(ell) for run in all_runs)
        admitted = 0.75 <= steps_over_sqrt <= 1.15
        weight_rows = []
        selected_arms = {}
        for weight in WEIGHTS:
            arm_means = {}
            arm_seed_costs = {}
            for arm in IC_ARMS:
                candidates = [units.get((q, seed, arm)) for seed in SEEDS]
                if not all(unit is not None and unit.successful for unit in candidates):
                    continue
                costs = {unit.seed: unit.components.price(r_inv, r_lmul, weight) for unit in candidates if unit is not None}
                arm_seed_costs[arm] = costs
                arm_means[arm] = statistics.mean(costs.values())
            need(arm_means, f"no complete historical IC arm at q={q}")
            selected = min(arm_means, key=lambda arm: (arm_means[arm], arm))
            selected_arms[str(weight)] = selected
            rho_seed_costs = {
                seed: statistics.mean(run.components.price(r_inv, r_lmul, weight) for run in seed_rows[seed]["runs"])
                for seed in SEEDS
            }
            point, lower, upper = ratio_interval(arm_seed_costs[selected], rho_seed_costs, q, weight)
            weight_rows.append({
                "weight": weight,
                "ic_arm": selected,
                "ic_mean_meq": arm_means[selected],
                "rho_mean_meq": statistics.mean(rho_seed_costs.values()),
                "ic_over_rho": point,
                "ci95": [lower, upper],
                "ic_S": arm_means[selected] / math.sqrt(ell),
                "rho_S": statistics.mean(rho_seed_costs.values()) / math.sqrt(ell),
            })
        phase_means = {}
        for phase in ("setup", "walk", "verify"):
            phase_means[phase] = statistics.mean(
                getattr(run, phase).price(r_inv, r_lmul, 0.0) for run in all_runs
            )
        total_phase = sum(phase_means.values())
        need(total_phase > 0.0, f"zero strong-rho phase cost at q={q}")
        sizes.append({
            "q": q,
            "ell": ell,
            "runs": len(all_runs),
            "r_inv": r_inv,
            "r_lmul": r_lmul,
            "steps_over_sqrt_ell": steps_over_sqrt,
            "admitted": admitted,
            "selected_arms": selected_arms,
            "weights": weight_rows,
            "rho_phase_share_w0": {key: value / total_phase for key, value in phase_means.items()},
        })
    h1 = classify_h1(sizes)

    h2_cells = []
    h2_pass = True
    for q in DIAGNOSTIC_Q:
        r_inv = next(row["r_inv"] for row in sizes if row["q"] == q)
        r_lmul = next(row["r_lmul"] for row in sizes if row["q"] == q)
        strong = [run for seed in SEEDS for run in results[q]["seeds"][seed]["runs"]]
        old = audit["diagnostics"][str(q)]["old-neg"]
        for weight in WEIGHTS:
            strong_mean = statistics.mean(run.components.price(r_inv, r_lmul, weight) for run in strong)
            old_mean = statistics.mean(run.components.price(r_inv, r_lmul, weight) for run in old)
            ratio = strong_mean / old_mean
            h2_pass &= ratio < 1.0
            h2_cells.append({"q": q, "weight": weight, "strong_over_old_neg": ratio})

    return {
        "schema": "ghs.strong-rho.analysis/v1",
        "source_revision": revision,
        "round_id": "ghs-strong-rho-r1-duration-a",
        "historical_archive": archive_summary,
        "artifact_audit": {key: value for key, value in audit.items() if key != "diagnostics"},
        "bootstrap": {"clusters": "instance_seed", "resamples": BOOTSTRAPS},
        "sizes": sizes,
        "H1": h1,
        "H2": {"confirmed": h2_pass, "cells": h2_cells},
    }


def markdown(analysis: dict[str, Any]) -> str:
    lines = [
        "# GHS strong-rho Round-A result",
        "",
        f"Source: `{analysis['source_revision']}`. Round: `{analysis['round_id']}`.",
        "",
        f"Frozen H1 verdict: **{analysis['H1']['verdict']}** `[pcc:unverified]`.",
        "",
        "| q | admitted | steps/sqrt(ell) | w | IC arm | IC/rho | 95% interval |",
        "|--:|:--:|--:|--:|:--|--:|:--|",
    ]
    for size in analysis["sizes"]:
        for row in size["weights"]:
            lines.append(
                f"| {size['q']:,} | {'yes' if size['admitted'] else 'no'} | {size['steps_over_sqrt_ell']:.4f} "
                f"| {row['weight']:g} | {row['ic_arm']} | {row['ic_over_rho']:.6g} "
                f"| [{row['ci95'][0]:.6g}, {row['ci95'][1]:.6g}] |"
            )
    lines += [
        "",
        "This table is producer output. It is not independently verified and does not update the canonical frontier by itself.",
        "",
    ]
    return "\n".join(lines)


def write_new(path: Path, payload: str) -> None:
    need(not path.exists(), f"refusing to overwrite output: {path}")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(payload, encoding="utf-8")


def command_audit_historical(args: argparse.Namespace) -> None:
    units, targets, summary = audit_historical(Path(args.archive))
    output = {"schema": "ghs.historical-archive-audit/v1", "archive": summary, "arms": sorted({unit.arm for unit in units.values()}), "targets": len(targets)}
    print(json.dumps(output, indent=2, sort_keys=True))


def command_analyze(args: argparse.Namespace) -> None:
    need(SOURCE_RE.fullmatch(args.source_revision) is not None, "source revision must be a full SHA-1")
    units, targets, archive_summary = audit_historical(Path(args.archive))
    results, audit = load_results(Path(args.results), args.source_revision, targets, units)
    value = analyze(units, results, audit, args.source_revision, archive_summary)
    write_new(Path(args.output_json), json.dumps(value, indent=2, sort_keys=True) + "\n")
    write_new(Path(args.output_markdown), markdown(value))


def parser() -> argparse.ArgumentParser:
    root = argparse.ArgumentParser(description=__doc__)
    sub = root.add_subparsers(dest="command", required=True)
    historical = sub.add_parser("audit-historical")
    historical.add_argument("--archive", required=True)
    historical.set_defaults(func=command_audit_historical)
    run = sub.add_parser("analyze")
    run.add_argument("--archive", required=True)
    run.add_argument("--results", required=True)
    run.add_argument("--source-revision", required=True)
    run.add_argument("--output-json", required=True)
    run.add_argument("--output-markdown", required=True)
    run.set_defaults(func=command_analyze)
    return root


def main() -> int:
    try:
        args = parser().parse_args()
        args.func(args)
    except (Invalid, OSError, json.JSONDecodeError, tarfile.TarError) as exc:
        print(f"analyze: refused: {exc}", file=sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
