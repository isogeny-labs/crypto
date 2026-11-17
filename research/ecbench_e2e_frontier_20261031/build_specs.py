#!/usr/bin/env python3
"""Generate the exact-build draft ECBench specs from the reviewed cells."""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
SPECS = ROOT / "specs"
SHARDS = SPECS / "shards"
SOURCE_COMMIT = "ff7ad61cfed11908fb79de9dd485b696d0ada779"
BINARY_SHA256 = "924d090c1d83a20ce954848c04b116cee4d5a7cb04187bb618dff60958bfd64d"
IMAGE_URI = "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:b8c20f9c151360b78f505360e8d47068c4068f00a8f6faa7ebd3086fa0a791c6"

TARGETS_PER_SHARD = 4
TARGET_BLOCKS = 2
MEASURED_ROUNDS = 6
WARMUPS = 1
# The portable EKS request has a 3,600-second deadline including provisioning.
# Leave 1,200 seconds outside child timeouts for Karpenter, setup, audit and upload.
PROCESS_TIMEOUT_BUDGET_SECONDS = 2_400


def arm(name: str, role: str, method_id: str, **params: str) -> dict:
    method = {"id": method_id}
    if params:
        method["params"] = params
    return {"name": name, "role": role, "method": method}


def measurement(seed: int, timeout: int = 1800) -> dict:
    return {
        "rounds": 6,
        "warmup": 1,
        "order": "alternate",
        "seed": seed,
        "isolation_required": "L2",
        "timeout_seconds": timeout,
    }


def write(name: str, spec: dict) -> None:
    SPECS.mkdir(parents=True, exist_ok=True)
    (SPECS / name).write_text(json.dumps(spec, indent=2) + "\n")


def write_shard(name: str, spec: dict) -> None:
    SHARDS.mkdir(parents=True, exist_ok=True)
    (SHARDS / name).write_text(json.dumps(spec, indent=2) + "\n")


def shard_measurement(seed: int, arm_count: int) -> tuple[dict, int]:
    executions = TARGETS_PER_SHARD * (MEASURED_ROUNDS + WARMUPS) * arm_count
    timeout = PROCESS_TIMEOUT_BUDGET_SECONDS // executions
    if timeout < 5:
        raise ValueError(f"{arm_count} arms leave only {timeout}s per child")
    return (
        {
            "rounds": MEASURED_ROUNDS,
            "warmup": WARMUPS,
            "order": "alternate",
            "seed": seed,
            "isolation_required": "L2",
            "timeout_seconds": timeout,
        },
        executions * timeout,
    )


def add_sharded_group(
    manifest: list[dict],
    *,
    slug: str,
    label: str,
    description: str,
    cell: str,
    size: int | str,
    curves: list[dict],
    references: list[dict],
    candidates: list[dict],
    target_seed: int,
    measurement_seed: int,
    target_kind: str = "public",
) -> None:
    arms = references + candidates
    measurement_template, process_budget = shard_measurement(measurement_seed, len(arms))
    for block in range(TARGET_BLOCKS):
        filename = f"{slug}-t{block}.json"
        spec = {
            "schema": "ecbench.spec/v1",
            "label": f"{label}, target block {block + 1}/{TARGET_BLOCKS}",
            "description": (
                f"{description} This prospectively selected target block has four public targets, "
                "one warmup and six balanced measured rounds. Per-child censoring is fixed so the "
                f"sum of child timeout ceilings is at most {PROCESS_TIMEOUT_BUDGET_SECONDS}s inside "
                "the 3,600-second EKS deadline; timeouts are retained and never retried."
            ),
            "workloads": {
                "curves": curves,
                "targets_per_curve": TARGETS_PER_SHARD,
                "target_seed": target_seed + block,
                "target_kind": target_kind,
            },
            "arms": arms,
            "measurement": {
                **measurement_template,
                "seed": measurement_seed + block,
            },
        }
        write_shard(filename, spec)
        manifest.append(
            {
                "spec": f"specs/shards/{filename}",
                "cell": cell,
                "size": size,
                "group": slug,
                "target_block": block,
                "targets": TARGETS_PER_SHARD,
                "arms": len(arms),
                "planned_executions": TARGETS_PER_SHARD
                * (MEASURED_ROUNDS + WARMUPS)
                * len(arms),
                "process_timeout_seconds": measurement_template["timeout_seconds"],
                "whole_job_process_timeout_ceiling_seconds": process_budget,
            }
        )


def prime_specs() -> list[dict]:
    # N_current is the pricing-band incumbent after the 2026-10-29 holdout.
    # N_alt retains the smaller-base candidate even where w=1 rejected it.
    cells = {
        20: {"n_current": 45, "n_alt": 32, "max_trials": 1_410_236},
        24: {"n_current": 162, "n_alt": 115, "max_trials": 4_243_777},
        28: {"n_current": 407, "n_alt": 288, "max_trials": 26_479_085},
        32: {"n_current": 1024, "n_alt": 1024, "max_trials": 167_871_873},
    }
    manifest = []
    for bits, cfg in cells.items():
        n = cfg["n_current"]
        n_alt = cfg["n_alt"]
        trials = str(cfg["max_trials"])
        arms = [
            arm("rho-neg-v2", "reference", "rho.negation_v2"),
            arm(
                f"ic-mitm-n{n}",
                "candidate",
                "ic.pipeline",
                factor_base=f"prime-abscissa:size={n}",
                oracle="mitm:negation_folded=1",
                max_trials=trials,
            ),
        ]
        if n_alt != n:
            arms.append(
                arm(
                    f"ic-alt-n{n_alt}",
                    "baseline",
                    "ic.pipeline",
                    factor_base=f"prime-abscissa:size={n_alt}",
                    oracle="mitm:negation_folded=1",
                    max_trials=trials,
                )
            )
        arms.extend(
            [
                arm(
                    f"ic-rootfind-m2-n{n}",
                    "candidate",
                    "ic.pipeline",
                    factor_base=f"prime-abscissa:size={n}",
                    oracle="rootfind-subtract:m=2",
                    max_trials=trials,
                ),
                arm(
                    f"ic-rootfind-m3-n{n}",
                    "candidate",
                    "ic.pipeline",
                    factor_base=f"prime-abscissa:size={n}",
                    oracle="rootfind-subtract:m=3",
                    max_trials=trials,
                ),
                arm("bsgs-neg-v2", "baseline", "bsgs.negation_v2"),
                arm("kangaroo-v2", "baseline", "kangaroo.vow_v2"),
                arm("rho-neg-v2-aa", "control", "rho.negation_v2"),
            ]
        )
        spec = {
            "schema": "ecbench.spec/v1",
            "label": f"E2E frontier refresh: prime {bits}-bit cell",
            "description": (
                "Cold one-target comparison of the current pricing-band factor base, the prior/alternative base, "
                "generic root-finding variants, accounting-v2 generic algorithms, and an identical rho A/A arm. "
                "Root finding is the generic track; this spec makes no algebraic sub-square-root claim."
            ),
            "workloads": {
                "curves": [{"kind": "prime_search", "bits": bits, "seed": 202610310200 + bits}],
                "targets_per_curve": 8,
                "target_seed": 202610311000 + bits,
                "target_kind": "public",
            },
            "arms": arms,
            "measurement": measurement(202610312000 + bits),
        }
        filename = f"prime-p{bits}.json"
        write(filename, spec)
        manifest.append({"spec": f"specs/{filename}", "cell": "prime", "size": bits})
    return manifest


def koblitz_specs() -> list[dict]:
    cells = {
        13: {"dimension": 5, "neg_dimension": 4},
        19: {"dimension": 7, "neg_dimension": 6},
        23: {"dimension": 8, "neg_dimension": 8},
        31: {"dimension": 11, "neg_dimension": 11},
    }
    manifest = []
    for n, cfg in cells.items():
        d = cfg["dimension"]
        dn = cfg["neg_dimension"]
        arms = [
            arm("rho-strong-v2", "reference", "rho.signed_frobenius_strong_v2"),
            arm(
                "rho-strong-escape",
                "baseline",
                "rho.signed_frobenius_strong_escape",
            ),
            arm("rho-frob-v2", "baseline", "rho.signed_frobenius_v2"),
            arm("rho-neg-v2", "baseline", "rho.negation_v2"),
            arm(
                "ic-rootfind-negfold-m2",
                "candidate",
                "ic.pipeline",
                factor_base=f"koblitz-standard-subspace:dimension={dn}",
                oracle="pdp3-koblitz:m=2,engine=rootfind-negfold",
                max_trials="200000",
            ),
            arm(
                "ic-rootfind-m3",
                "candidate",
                "ic.pipeline",
                factor_base=f"koblitz-standard-subspace:dimension={d}",
                oracle="pdp3-koblitz:m=3,engine=rootfind",
                max_trials="200000",
            ),
            arm(
                "ic-inherited-f4-m3",
                "baseline",
                "ic.pipeline",
                factor_base=f"koblitz-standard-subspace:dimension={d}",
                oracle="pdp3-koblitz:m=3,engine=inherited-f4,degree=3,node_budget=8192",
                max_trials="200000",
            ),
            arm(
                "ic-f6-m3",
                "candidate",
                "ic.pipeline",
                factor_base=f"koblitz-standard-subspace:dimension={d}",
                oracle="pdp3-koblitz:m=3,engine=f6-ic,degree=3,node_budget=8192",
                max_trials="200000",
            ),
            arm("claw-v2", "baseline", "claw.pair_table_v2", base_scale="4.0", table_scale="1.0"),
            arm("rho-strong-v2-aa", "control", "rho.signed_frobenius_strong_v2"),
        ]
        spec = {
            "schema": "ecbench.spec/v1",
            "label": f"E2E frontier refresh: Koblitz E0 n={n}",
            "description": (
                "Cold one-target comparison of accounting-v2 strong rho, fruitless-cycle-escape rho, tuned signed-Frobenius rho, "
                "root finding, inherited F4, F6-IC, the pair claw, and a strong-rho A/A arm. "
                "Timeouts and budget exhaustion are retained."
            ),
            "workloads": {
                "curves": [{"kind": "koblitz", "a": 0, "n": n}],
                "targets_per_curve": 8,
                "target_seed": 202610313000 + n,
                "target_kind": "public",
            },
            "arms": arms,
            "measurement": measurement(202610314000 + n, 3600),
        }
        filename = f"koblitz-n{n}.json"
        write(filename, spec)
        manifest.append({"spec": f"specs/{filename}", "cell": "koblitz", "size": n})
    return manifest


def solver_spec() -> dict:
    solver_names = [
        "f4-f2",
        "buchberger-f2",
        "matrix-f4",
        "matrix-f5",
        "inherited-f4",
        "crossbred-f2",
        "xl-f2",
        "sat-cdcl:sat_conflict_budget=20000,sat_xor_encoding=cnf",
        "fes-f2",
        "fes-f2-wide",
        "exhaustive",
    ]
    arms = [arm("rho-strong-v2", "reference", "rho.signed_frobenius_strong_v2")]
    for solver in solver_names:
        name = solver.split(":", 1)[0]
        arms.append(
            arm(
                f"solver-{name}",
                "candidate",
                "ic.pipeline",
                factor_base="koblitz-orbit:divisor=0;1",
                oracle="descent-algebraic:m=2",
                solver=solver,
                max_trials="20000",
            )
        )
    arms.append(arm("rho-strong-v2-aa", "control", "rho.signed_frobenius_strong_v2"))
    return {
        "schema": "ecbench.spec/v1",
        "label": "E2E frontier refresh: all algebraic solver backends on Koblitz n=17",
        "description": (
            "Same factor base, target stream, linear algebra, and relation cap across every live solver backend. "
            "Native units remain backend-local; verified L2 cold E2E wall and complete common accounting are the only cross-backend comparisons."
        ),
        "workloads": {
            "curves": [{"kind": "koblitz", "a": 1, "n": 17}],
            "targets_per_curve": 8,
            "target_seed": 202610315017,
            "target_kind": "public",
        },
        "arms": arms,
        "measurement": measurement(202610316017, 3600),
    }


def prime_shards(manifest: list[dict]) -> None:
    cells = {
        20: {"n_current": 45, "n_alt": 32, "max_trials": 1_410_236},
        24: {"n_current": 162, "n_alt": 115, "max_trials": 4_243_777},
        28: {"n_current": 407, "n_alt": 288, "max_trials": 26_479_085},
        32: {"n_current": 1024, "n_alt": 1024, "max_trials": 167_871_873},
    }
    for bits, cfg in cells.items():
        curve = [{"kind": "prime_search", "bits": bits, "seed": 202610310200 + bits}]
        refs = [
            arm("rho-neg-v2", "reference", "rho.negation_v2"),
            arm("rho-neg-v2-aa", "control", "rho.negation_v2"),
        ]
        n = cfg["n_current"]
        trials = str(cfg["max_trials"])
        ic_candidates = [
            arm(
                f"ic-mitm-n{n}",
                "candidate",
                "ic.pipeline",
                factor_base=f"prime-abscissa:size={n}",
                oracle="mitm:negation_folded=1",
                max_trials=trials,
            ),
            arm(
                f"ic-rootfind-m2-n{n}",
                "candidate",
                "ic.pipeline",
                factor_base=f"prime-abscissa:size={n}",
                oracle="rootfind-subtract:m=2",
                max_trials=trials,
            ),
            arm(
                f"ic-rootfind-m3-n{n}",
                "candidate",
                "ic.pipeline",
                factor_base=f"prime-abscissa:size={n}",
                oracle="rootfind-subtract:m=3",
                max_trials=trials,
            ),
        ]
        if cfg["n_alt"] != n:
            ic_candidates.insert(
                1,
                arm(
                    f"ic-alt-n{cfg['n_alt']}",
                    "baseline",
                    "ic.pipeline",
                    factor_base=f"prime-abscissa:size={cfg['n_alt']}",
                    oracle="mitm:negation_folded=1",
                    max_trials=trials,
                ),
            )
        add_sharded_group(
            manifest,
            slug=f"prime-p{bits}-ic",
            label=f"Prime {bits}-bit factor-base and root-finding frontier",
            description="Current pricing-band factor base, retained alternative, and both generic root-finding arities against matched negation rho.",
            cell="prime",
            size=bits,
            curves=curve,
            references=refs,
            candidates=ic_candidates,
            target_seed=202610320000 + bits * 10,
            measurement_seed=202610330000 + bits * 10,
        )
        add_sharded_group(
            manifest,
            slug=f"prime-p{bits}-generic",
            label=f"Prime {bits}-bit generic-algorithm frontier",
            description="All accounting-v2 BSGS variants and van Oorschot-Wiener kangaroo against matched negation rho.",
            cell="prime",
            size=bits,
            curves=curve,
            references=refs,
            candidates=[
                arm("bsgs-textbook-v2", "baseline", "bsgs.textbook_v2"),
                arm("bsgs-interleaved-v2", "candidate", "bsgs.interleaved_v2"),
                arm("bsgs-neg-v2", "candidate", "bsgs.negation_v2"),
                arm("kangaroo-v2", "candidate", "kangaroo.vow_v2"),
            ],
            target_seed=202610320000 + bits * 10,
            measurement_seed=202610331000 + bits * 10,
        )


def koblitz_references() -> list[dict]:
    return [
        arm("rho-strong-v2", "reference", "rho.signed_frobenius_strong_v2"),
        arm("rho-strong-escape", "reference", "rho.signed_frobenius_strong_escape"),
        arm("rho-strong-v2-aa", "control", "rho.signed_frobenius_strong_v2"),
        arm("rho-strong-escape-aa", "control", "rho.signed_frobenius_strong_escape"),
    ]


def koblitz_shards(manifest: list[dict]) -> None:
    cells = {
        13: {"dimension": 5, "neg_dimension": 4},
        19: {"dimension": 7, "neg_dimension": 6},
        23: {"dimension": 8, "neg_dimension": 8},
        31: {"dimension": 11, "neg_dimension": 11},
    }
    for n, cfg in cells.items():
        curve = [{"kind": "koblitz", "a": 0, "n": n}]
        refs = koblitz_references()
        d = cfg["dimension"]
        dn = cfg["neg_dimension"]
        common = {
            "cell": "koblitz",
            "size": n,
            "curves": curve,
            "references": refs,
            "target_seed": 202610340000 + n * 10,
        }
        add_sharded_group(
            manifest,
            slug=f"koblitz-n{n}-rho",
            label=f"Koblitz E0 n={n} rho frontier",
            description="The two preregistered strong-rho references, signed-Frobenius rho, and generic negation rho. Candidate ranking uses the paired minimum of the two common-accounting strong references, fixed before data.",
            candidates=[
                arm("rho-frob-v2", "baseline", "rho.signed_frobenius_v2"),
                arm("rho-neg-v2", "baseline", "rho.negation_v2"),
            ],
            measurement_seed=202610350000 + n * 10,
            **common,
        )
        add_sharded_group(
            manifest,
            slug=f"koblitz-n{n}-rootfind",
            label=f"Koblitz E0 n={n} root-finding frontier",
            description="Negation-folded m=2 and ordinary m=3 root-finding pipelines against both strong-rho references.",
            candidates=[
                arm(
                    "ic-rootfind-negfold-m2",
                    "candidate",
                    "ic.pipeline",
                    factor_base=f"koblitz-standard-subspace:dimension={dn}",
                    oracle="pdp3-koblitz:m=2,engine=rootfind-negfold",
                    max_trials="200000",
                ),
                arm(
                    "ic-rootfind-m3",
                    "candidate",
                    "ic.pipeline",
                    factor_base=f"koblitz-standard-subspace:dimension={d}",
                    oracle="pdp3-koblitz:m=3,engine=rootfind",
                    max_trials="200000",
                ),
            ],
            measurement_seed=202610351000 + n * 10,
            **common,
        )
        add_sharded_group(
            manifest,
            slug=f"koblitz-n{n}-algebraic",
            label=f"Koblitz E0 n={n} inherited-F4/F6 frontier",
            description="Inherited F4 and F6-IC with the same factor base, target stream, node budget and relation cap against both strong-rho references.",
            candidates=[
                arm(
                    "ic-inherited-f4-m3",
                    "baseline",
                    "ic.pipeline",
                    factor_base=f"koblitz-standard-subspace:dimension={d}",
                    oracle="pdp3-koblitz:m=3,engine=inherited-f4,degree=3,node_budget=8192",
                    max_trials="200000",
                ),
                arm(
                    "ic-f6-m3",
                    "candidate",
                    "ic.pipeline",
                    factor_base=f"koblitz-standard-subspace:dimension={d}",
                    oracle="pdp3-koblitz:m=3,engine=f6-ic,degree=3,node_budget=8192",
                    max_trials="200000",
                ),
            ],
            measurement_seed=202610352000 + n * 10,
            **common,
        )
        add_sharded_group(
            manifest,
            slug=f"koblitz-n{n}-claw",
            label=f"Koblitz E0 n={n} pair-claw frontier",
            description="The accounting-v2 pair-table claw against both strong-rho references.",
            candidates=[
                arm("claw-v2", "candidate", "claw.pair_table_v2", base_scale="4.0", table_scale="1.0")
            ],
            measurement_seed=202610353000 + n * 10,
            **common,
        )


def random_binary_shards(manifest: list[dict]) -> None:
    for n, dimension in [(13, 5), (19, 7), (23, 8), (31, 11)]:
        curve = [{"kind": "binary_random", "n": n, "seed": 59297, "max_cofactor": 8}]
        refs = [
            arm("rho-neg-v2", "reference", "rho.negation_v2"),
            arm("rho-neg-v2-aa", "control", "rho.negation_v2"),
        ]
        common = {
            "cell": "random-binary",
            "size": n,
            "curves": curve,
            "references": refs,
            "target_seed": 202610360000 + n * 10,
        }
        add_sharded_group(
            manifest,
            slug=f"random-binary-n{n}-matrix",
            label=f"Random-binary n={n} matrix F4/F5 frontier",
            description="Matrix F4 and matrix F5 on the same binary-subspace system against matched negation rho.",
            candidates=[
                arm(
                    "matrix-f4",
                    "baseline",
                    "ic.pipeline",
                    factor_base=f"binary-standard-subspace:dimension={dimension}",
                    oracle="pdp3-binary:m=3,engine=matrix-f4,degree=3,node_budget=8192",
                    max_trials="200000",
                ),
                arm(
                    "matrix-f5",
                    "candidate",
                    "ic.pipeline",
                    factor_base=f"binary-standard-subspace:dimension={dimension}",
                    oracle="pdp3-binary:m=3,engine=matrix-f5,degree=3,node_budget=8192",
                    max_trials="200000",
                ),
            ],
            measurement_seed=202610370000 + n * 10,
            **common,
        )
        add_sharded_group(
            manifest,
            slug=f"random-binary-n{n}-inherited",
            label=f"Random-binary n={n} inherited-F4/F6 frontier",
            description="Inherited F4 and F6-IC on the same binary-subspace system against matched negation rho.",
            candidates=[
                arm(
                    "inherited-f4",
                    "baseline",
                    "ic.pipeline",
                    factor_base=f"binary-standard-subspace:dimension={dimension}",
                    oracle="pdp3-binary:m=3,engine=inherited-f4,degree=3,node_budget=8192",
                    max_trials="200000",
                ),
                arm(
                    "f6-ic",
                    "candidate",
                    "ic.pipeline",
                    factor_base=f"binary-standard-subspace:dimension={dimension}",
                    oracle="pdp3-binary:m=3,engine=f6-ic,degree=3,node_budget=8192",
                    max_trials="200000",
                ),
            ],
            measurement_seed=202610371000 + n * 10,
            **common,
        )


def solver_shards(manifest: list[dict]) -> None:
    solvers = [
        "f4-f2",
        "buchberger-f2",
        "matrix-f4",
        "matrix-f5",
        "inherited-f4",
        "crossbred-f2",
        "xl-f2",
        "sat-cdcl:sat_conflict_budget=20000,sat_xor_encoding=cnf",
        "fes-f2",
        "fes-f2-wide",
        "exhaustive",
    ]
    for group_index in range(0, len(solvers), 2):
        group = solvers[group_index : group_index + 2]
        candidates = []
        for solver in group:
            name = solver.split(":", 1)[0]
            candidates.append(
                arm(
                    f"solver-{name}",
                    "candidate",
                    "ic.pipeline",
                    factor_base="koblitz-orbit:divisor=0;1",
                    oracle="descent-algebraic:m=2",
                    solver=solver,
                    max_trials="20000",
                )
            )
        add_sharded_group(
            manifest,
            slug=f"solver-k17-g{group_index // 2}",
            label=f"Koblitz n=17 algebraic solver backends {group_index + 1}-{group_index + len(group)}",
            description="Backend-local counters remain unlike; only verified cold E2E L2 wall and complete common accounting are cross-ranked.",
            cell="solver-backends",
            size=17,
            curves=[{"kind": "koblitz", "a": 1, "n": 17}],
            references=koblitz_references(),
            candidates=candidates,
            target_seed=202610380170,
            measurement_seed=202610390170 + group_index * 10,
        )


def symmetry_shards(manifest: list[dict]) -> None:
    panels = [
        (
            "j0",
            {
                "kind": "prime_explicit",
                "p": 23736841,
                "a": 0,
                "b": 1,
                "group_order": 23727108,
                "r": 1977259,
                "gx": 19157303,
                "gy": 13563414,
            },
            "rho.cubic_v2",
            "rho.cubic",
            "cubic",
        ),
        (
            "j1728",
            {
                "kind": "prime_explicit",
                "p": 2463541,
                "a": 8,
                "b": 0,
                "group_order": 2460762,
                "r": 136709,
                "gx": 1664813,
                "gy": 581781,
            },
            "rho.quartic_v2",
            "rho.quartic",
            "quartic",
        ),
    ]
    for index, (slug, curve, current_rho, legacy_rho, symmetry) in enumerate(panels):
        add_sharded_group(
            manifest,
            slug=f"symmetry-{slug}",
            label=f"Prime {slug} symmetry frontier",
            description=f"Accounting-v2 {symmetry} rho and its A/A control, with legacy {symmetry} rho and current generic algorithms retained as matched baselines.",
            cell=f"symmetry-{slug}",
            size=curve["r"].bit_length(),
            curves=[curve],
            references=[
                arm(f"rho-{symmetry}-v2", "reference", current_rho),
                arm(f"rho-{symmetry}-v2-aa", "control", current_rho),
            ],
            candidates=[
                arm(f"rho-{symmetry}-legacy", "baseline", legacy_rho),
                arm("rho-neg-v2", "baseline", "rho.negation_v2"),
                arm("bsgs-neg-v2", "candidate", "bsgs.negation_v2"),
                arm("kangaroo-v2", "candidate", "kangaroo.vow_v2"),
            ],
            target_seed=202610400000 + index * 100,
            measurement_seed=202610410000 + index * 100,
        )


def generator_orbit_shards(manifest: list[dict]) -> None:
    add_sharded_group(
        manifest,
        slug="generator-orbit-a1-n29",
        label="Koblitz a=1 n=29 one-column generator-orbit engineering frontier",
        description="Full-price Frobenius references, lazy/layout table baselines and the fixed-setup generator-orbit candidate. This panel is never pooled with uniform-target scaling IC.",
        cell="generator-orbit",
        size=29,
        curves=[{"kind": "koblitz", "a": 1, "n": 29}],
        references=[
            arm("rho-frob-full", "reference", "rho.signed_frobenius_full"),
            arm("rho-strong-full", "reference", "rho.signed_frobenius_strong_full"),
            arm("rho-frob-full-aa", "control", "rho.signed_frobenius_full"),
            arm("rho-strong-full-aa", "control", "rho.signed_frobenius_strong_full"),
        ],
        candidates=[
            arm(
                "ic-lazy-c1",
                "baseline",
                "ic.pipeline_full",
                factor_base="compact-orbit-scan:columns=1,raw_x_cap=100000000",
                oracle="mitm-frobenius-lazy-counted:m=2",
                max_trials="1000000000",
                targets="stride-renew",
            ),
            arm(
                "ic-layout-c1",
                "candidate",
                "ic.pipeline_full",
                factor_base="compact-orbit-scan:columns=1,raw_x_cap=100000000",
                oracle="mitm-frobenius-layout-counted:m=2",
                max_trials="1000000000",
                targets="stride-renew",
            ),
            arm(
                "ic-gen-layout-c1",
                "candidate",
                "ic.pipeline_full",
                factor_base="compact-orbit-scan:columns=1,raw_x_cap=100000000,generator_orbit=1",
                oracle="mitm-frobenius-layout-counted:m=2",
                max_trials="1000000000",
                targets="stride-generator",
            ),
        ],
        target_seed=202610420290,
        measurement_seed=202610430290,
    )


def f2m4_shards(manifest: list[dict]) -> None:
    oracle = "gaudry-d4-bin:macaulay=11,frobenius=1,sparsest=1,replay=1,charge_unsolved=1"
    for m, max_trials in [(10, "5000000"), (12, "5000000"), (15, "50000000")]:
        add_sharded_group(
            manifest,
            slug=f"f2m4-m{m}",
            label=f"F_((2^{m})^4) specialized IC/rho frontier",
            description="Two-word-field Gaudry-D4 pipeline with dense and Wiedemann linear algebra against the counted strong negation-map rho; legacy negation rho is retained for continuity. This harness path uses planted known-answer targets because public hash-to-subgroup exhausts its fixed 10,000-attempt bound on these torsion fixtures; the solver does not receive the scalar.",
            cell="f2m4-specialized",
            size=m,
            curves=[{"kind": "f2m4_torsion", "m": m, "seed": 1, "max_cofactor": 16 if m < 15 else 2}],
            references=[
                arm(
                    "rho-neg-strong",
                    "reference",
                    "rho.negation_strong",
                    lanes="32",
                    dp_bits="auto",
                ),
                arm(
                    "rho-neg-strong-aa",
                    "control",
                    "rho.negation_strong",
                    lanes="32",
                    dp_bits="auto",
                ),
            ],
            candidates=[
                arm("rho-neg-legacy", "baseline", "rho.negation"),
                arm(
                    "ic-dense",
                    "baseline",
                    "ic.pipeline",
                    factor_base="bin-t-line",
                    oracle=oracle,
                    max_trials=max_trials,
                ),
                arm(
                    "ic-wiedemann",
                    "candidate",
                    "ic.pipeline",
                    factor_base="bin-t-line",
                    oracle=oracle,
                    max_trials=max_trials,
                    linalg="wiedemann",
                ),
            ],
            target_seed=202610440000 + m * 10,
            measurement_seed=202610450000 + m * 10,
            target_kind="planted",
        )
    add_sharded_group(
        manifest,
        slug="f2m4-wide-m16",
        label="F_((2^16)^4) wide two-word implementation frontier",
        description="The current u128 Gaudry-D4 pipeline and its only implemented wide matched rho on planted known-answer targets because this curve's public hash-to-subgroup constructor exhausts its fixed 10,000-attempt bound. Historical multi-hour rows predict censoring; the fixed short cap is an explicit reachability/lower-bound panel, not a public-target frontier or a completion guarantee.",
        cell="f2m4-wide",
        size=16,
        curves=[{"kind": "f2m4_torsion", "m": 16, "seed": 1, "max_cofactor": 2}],
        references=[
            arm("rho-neg", "reference", "rho.negation"),
            arm("rho-neg-aa", "control", "rho.negation"),
        ],
        candidates=[
            arm(
                "ic-wiedemann",
                "candidate",
                "ic.pipeline",
                factor_base="bin-t-line",
                oracle=oracle,
                max_trials="50000000",
                linalg="wiedemann",
            )
        ],
        target_seed=202610440160,
        measurement_seed=202610450160,
        target_kind="planted",
    )


def large_prime_shards(manifest: list[dict]) -> None:
    add_sharded_group(
        manifest,
        slug="binary-large-prime-n11",
        label="Binary n=11 large-prime-v2 implementation frontier",
        description="Exact n-1, double-large-prime candidate against both current Koblitz strong-rho references on the same explicit public-target fixture.",
        cell="binary-large-prime",
        size=11,
        curves=[
            {
                "kind": "binary_explicit",
                "n": 11,
                "modulus": 2323,
                "a": 1,
                "b": 1,
                "group_order": 1982,
                "r": 991,
                "gx": 1193,
                "gy": 1007,
            }
        ],
        references=koblitz_references(),
        candidates=[
            arm(
                "ic-nminus1-dlp-v2",
                "candidate",
                "ic.large_prime_v2",
                small_dimension="3",
                envelope_dimension="4",
                summands="n-1",
                large_primes="2",
                max_trials="20000",
                max_states="100000",
            )
        ],
        target_seed=202610460110,
        measurement_seed=202610470110,
    )


def queue_safe_shards() -> list[dict]:
    manifest: list[dict] = []
    prime_shards(manifest)
    koblitz_shards(manifest)
    random_binary_shards(manifest)
    solver_shards(manifest)
    symmetry_shards(manifest)
    generator_orbit_shards(manifest)
    f2m4_shards(manifest)
    large_prime_shards(manifest)
    return manifest


def main() -> None:
    manifest = prime_specs() + koblitz_specs()
    write("solver-backends-k17.json", solver_spec())
    manifest.append({"spec": "specs/solver-backends-k17.json", "cell": "solver-backends", "size": 17})
    shards = queue_safe_shards()
    execution = {
        "schema": "ecbench.frontier-campaign/v1",
        "date": "2026-10-31",
        "source_commit": SOURCE_COMMIT,
        "binary_sha256": BINARY_SHA256,
        "image_uri": IMAGE_URI,
        "publication_receipt": "execution/publication.json",
        "backend": "isolab-eks-spot",
        "measurement_class": "measurement",
        "state": "blocked_checkpoint_and_protocol_revision",
        "protocol_status": "draft",
        "draft_warning": (
            "The listed nine jobs plan on the exact current binary but do not yet cover every "
            "current parameterized implementation or fit the 3600-second queue envelope; they "
            "must be expanded and sharded before freeze."
        ),
        "fixed_requirements": {
            "one_target_scope": True,
            "targets_per_cell": 8,
            "warmups": 1,
            "measured_rounds": 6,
            "isolation_required": "L2",
            "replay": "all",
            "retain_failures_timeouts": True,
            "pricing_weights_gae_per_native_op": [0, 0.1, 1],
        },
        "jobs": manifest,
    }
    (ROOT / "execution" / "manifest.json").write_text(json.dumps(execution, indent=2) + "\n")
    shard_execution = {
        "schema": "ecbench.frontier-shard-campaign/v1",
        "date": "2026-10-31",
        "source_commit": SOURCE_COMMIT,
        "binary_sha256": BINARY_SHA256,
        "image_uri": IMAGE_URI,
        "backend": "isolab-eks-spot",
        "measurement_class": "measurement",
        "state": "blocked_checkpoint_and_preflight_runtime",
        "protocol_status": "draft",
        "queue_deadline_seconds": 3600,
        "whole_job_process_timeout_budget_seconds": PROCESS_TIMEOUT_BUDGET_SECONDS,
        "deadline_reserve_seconds": 3600 - PROCESS_TIMEOUT_BUDGET_SECONDS,
        "target_blocks": TARGET_BLOCKS,
        "targets_per_block": TARGETS_PER_SHARD,
        "targets_per_cell": TARGET_BLOCKS * TARGETS_PER_SHARD,
        "warmups": WARMUPS,
        "measured_rounds": MEASURED_ROUNDS,
        "order": "alternate",
        "isolation_required": "L2",
        "replay": "all",
        "retain_failures_timeouts": True,
        "measurement_retries": 0,
        "koblitz_reference_rule": (
            "Use the paired minimum common-accounting S of signed_frobenius_strong_v2 "
            "and signed_frobenius_strong_escape; use the union of both A/A envelopes. "
            "This rule is fixed before data and never selected by wall time."
        ),
        "warning": (
            "These shards are queue-envelope-safe by timeout arithmetic and validate on the exact "
            "binary, but the protocol remains draft until exact-image runtime preflight, analysis-script "
            "hashing, checkpoint upload, prospective freeze and merge to main. Do not execute them yet."
        ),
        "jobs": shards,
    }
    (ROOT / "execution" / "shard-manifest.json").write_text(
        json.dumps(shard_execution, indent=2) + "\n"
    )


if __name__ == "__main__":
    main()
