#!/usr/bin/env python3
# Legacy, kept as provenance (AGENTS.md "Implementation language: no Python"):
# examples/bitop_ladder_analyse.rs replaces this script and reproduces its
# committed outputs byte for byte. Run that instead.
"""Write this study's ecbench specs from curves.jsonl (and params.json).

    python3 make_specs.py train            # specs/train-P<b>.json, Phase A
    python3 make_specs.py holdout          # specs/holdout-P<b>.json, Phase B (needs params.json)
    python3 make_specs.py ext <b> <arm> <size>[,<size>...] <n>
                                           # specs/train-P<b>-<arm>-ext<n>.json, one selection extension

Every value comes from PROTOCOL.md; nothing here is a choice.
"""
import json
import math
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
SPECS = HERE / "specs"
MAX_TRIALS = "1000000000000"
PAIR = "mitm:negation_folded=1"
ROOTFIND = "rootfind-subtract:m=3"
RF_GRID = [8, 16, 32, 64, 128, 256]
RULE_MAX = 40
SMALL_MAX = 28


def curves():
    out = {}
    for line in (HERE / "curves.jsonl").read_text().splitlines():
        c = json.loads(line)
        out[c["p"].bit_length()] = c
    return out


def pair_grid(b):
    g = 32 * 2 ** (0.33 * (b - 20))
    sizes = sorted({round(g * 2 ** (k / 2)) for k in range(-4, 3)})
    return [s for s in sizes if s >= 8]


def ic(name, role, oracle, size):
    return {
        "name": name,
        "role": role,
        "method": {
            "id": "ic.pipeline",
            "params": {
                "factor_base": f"prime-abscissa:size={size}",
                "oracle": oracle,
                "max_trials": MAX_TRIALS,
            },
        },
    }


def generic(name, role, method_id):
    return {"name": name, "role": role, "method": {"id": method_id}}


def spec(label, curve, arms, targets, target_seed, seed, timeout):
    return {
        "schema": "ecbench.spec/v1",
        "label": label,
        "description": "research/bitop_ladder_20261019/PROTOCOL.md, frozen 2026-10-19. "
        "Counts only (bitop/v1); public synthetic curve; no security claim.",
        "workloads": {
            "curves": [curve],
            "targets_per_curve": targets,
            "target_seed": target_seed,
            "target_kind": "public",
        },
        "arms": arms,
        "measurement": {
            "rounds": 1,
            "warmup": 0,
            "order": "alternate",
            "seed": seed,
            "isolation_required": "L0",
            "timeout_seconds": timeout,
        },
    }


def train_arms(b, pair_sizes, rf_sizes):
    arms = [generic("rho-neg", "reference", "rho.negation")]
    arms += [ic(f"pair-N{s}", "candidate", PAIR, s) for s in pair_sizes]
    arms += [ic(f"rf3-N{s}", "candidate", ROOTFIND, s) for s in rf_sizes]
    return arms


def train_targets(b):
    return 8 if b <= 34 else 4


def write(path, obj):
    SPECS.mkdir(exist_ok=True)
    if path.exists():
        sys.exit(f"{path} exists; specs are write-once")
    path.write_text(json.dumps(obj, indent=1) + "\n")
    print(path.relative_to(HERE))


def main():
    cmd = sys.argv[1]
    cs = curves()
    if cmd == "train":
        for b in range(16, RULE_MAX + 1):
            rf = RF_GRID if b <= SMALL_MAX else []
            write(
                SPECS / f"train-P{b}.json",
                spec(
                    f"bitop ladder train panel: {b}-bit a=-3 prime-order curve",
                    cs[b],
                    train_arms(b, pair_grid(b), rf),
                    train_targets(b),
                    20261019100,
                    20261019101,
                    1800,
                ),
            )
    elif cmd == "ext":
        b, arm, sizes, n = int(sys.argv[2]), sys.argv[3], sys.argv[4], int(sys.argv[5])
        sizes = [int(s) for s in sizes.split(",")]
        pair = sizes if arm == "pair" else []
        rf = sizes if arm == "rf3" else []
        write(
            SPECS / f"train-P{b}-{arm}-ext{n}.json",
            spec(
                f"bitop ladder train panel extension {n}: {b}-bit a=-3 prime-order curve",
                cs[b],
                train_arms(b, pair, rf),
                train_targets(b),
                20261019100,
                20261019101,
                1800,
            ),
        )
    elif cmd == "holdout":
        params = json.loads((HERE / "params.json").read_text())
        pair = {int(k): v for k, v in params["pair"].items()}
        rf = {int(k): v for k, v in params["rf3"].items()}
        for b in range(16, RULE_MAX + 1):
            arms = [
                generic("rho-neg", "reference", "rho.negation"),
                generic("rho-neg-aa", "control", "rho.negation"),
                generic("bsgs-neg", "baseline", "bsgs.negation"),
                generic("kangaroo", "baseline", "kangaroo.vow"),
            ]
            if pair.get(b):
                arms.append(ic("ic-pair-rule", "candidate", PAIR, pair[b]))
            if b <= SMALL_MAX:
                arms.append(ic("ic-pair-b32", "candidate", PAIR, 32))
                if rf.get(b):
                    arms.append(ic("ic-rootfind-m3-rule", "candidate", ROOTFIND, rf[b]))
            write(
                SPECS / f"holdout-P{b}.json",
                spec(
                    f"bitop ladder hold-out: {b}-bit a=-3 prime-order curve",
                    cs[b],
                    arms,
                    80,
                    20261019200,
                    20261019201,
                    3600,
                ),
            )
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main()
