#!/usr/bin/env python3
"""Generate the exact-build draft ECBench specs from the reviewed cells."""

from __future__ import annotations

import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent
SPECS = ROOT / "specs"
SOURCE_COMMIT = "ff7ad61cfed11908fb79de9dd485b696d0ada779"
BINARY_SHA256 = "924d090c1d83a20ce954848c04b116cee4d5a7cb04187bb618dff60958bfd64d"
IMAGE_URI = "104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:b8c20f9c151360b78f505360e8d47068c4068f00a8f6faa7ebd3086fa0a791c6"


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


def main() -> None:
    manifest = prime_specs() + koblitz_specs()
    write("solver-backends-k17.json", solver_spec())
    manifest.append({"spec": "specs/solver-backends-k17.json", "cell": "solver-backends", "size": 17})
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


if __name__ == "__main__":
    main()
