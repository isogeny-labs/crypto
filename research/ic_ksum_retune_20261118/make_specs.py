#!/usr/bin/env python3
"""Spec generator (cwd = this directory).
pilot:        python3 make_specs.py pilot         -> pilot/specs/<row>.json   (8 specs; grid around the rule)
confirmatory: python3 make_specs.py confirm ROWS  -> specs/<row>-r<rep>.json  (3 replicates per row)
Curves, max_trials, timeout and order are copied from the split study's spec of the same name; seeds are fresh."""
import json, sys, os, math
import rule

ROWS = ["P20", "P22", "P24", "B15", "B18", "B21", "B24", "B27"]
SPLIT = "../ic_ksum_split_20261118/specs"
# The split study's cheaper -psr arm at w = 0 (its README "best after" column), producer N.
INCUMBENT_K = {"P20": 5, "P22": 5, "P24": 5, "B15": 4, "B18": 4, "B21": 5, "B24": 5, "B27": 5}


def base(row):
    s = json.load(open(f"{SPLIT}/{row}-r1.json"))
    field = "binary" if s["workloads"]["curves"][0]["kind"].startswith("binary") else "prime"
    return s, field


def arm(name, k, N, field, max_trials):
    return {"name": name, "role": "candidate", "method": {"id": "ic.pipeline", "params": {
        "factor_base": rule.factor_base(field, N), "oracle": f"ksum:m={k},prune=1,split=1",
        "max_trials": max_trials, "targets": "stride-renew"}}}


def producer_N(s, k):
    fb = next(a for a in s["arms"] if a["name"] == f"ksum{k}-psr")["method"]["params"]["factor_base"]
    return int(fb.split("size=")[1].split(",")[0])


def best_N(k, r):
    Ns = [N for N in range(k + 1, 40) if rule.table(N, k) <= rule.MAX_T]
    return min(Ns, key=lambda N: rule.c0(N, k, r))


def pilot():
    os.makedirs("pilot/specs", exist_ok=True)
    for i, row in enumerate(ROWS):
        s, field = base(row)
        r = s["workloads"]["curves"][0]["r"]
        mt = s["arms"][2]["method"]["params"]["max_trials"]
        arms = [{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}}]
        seen = set()
        for k in (4, 5, 6, 7):
            b = best_N(k, r)
            for N in sorted({b - 1, b, b + 1, producer_N(s, k) if k in (4, 5) else b}):
                if N <= k or (k, N) in seen:
                    continue
                seen.add((k, N)); arms.append(arm(f"k{k}n{N}", k, N, field, mt))
        spec = {"schema": "ecbench.spec/v1", "label": f"ic_ksum_retune_20261118 pilot {row}",
                "description": "Sizing pilot (not data) for the k-sum (k, N) rule: prune=1, split=1, stride-renew. Generic decomposition track (bounded by sqrt(r); cannot beat rho). Public synthetic curve and targets; no security claim.",
                "workloads": {**s["workloads"], "targets_per_curve": 64, "target_seed": 20261118200 + i},
                "arms": arms,
                "measurement": {**s["measurement"], "rounds": 1, "warmup": 0, "seed": 20261118300 + i}}
        json.dump(spec, open(f"pilot/specs/{row}.json", "w"), indent=1); open(f"pilot/specs/{row}.json", "a").write("\n")
        print(row, len(arms) - 1, "k-sum arms")


def confirm(rows):
    os.makedirs("specs", exist_ok=True)
    for row in rows:
        i = ROWS.index(row)
        for rep in (1, 2, 3):
            s, field = base(f"{row}")
            s = json.load(open(f"{SPLIT}/{row}-r{rep}.json"))
            r = s["workloads"]["curves"][0]["r"]
            mt = s["arms"][2]["method"]["params"]["max_trials"]
            ki = INCUMBENT_K[row]; Ni = producer_N(s, ki)
            kc, Nc = rule.pick(r)
            idx = 10 * i + rep
            arms = [{"name": "rho-neg", "role": "reference", "method": {"id": "rho.negation"}},
                    {"name": "rho-neg-aa", "role": "control", "method": {"id": "rho.negation"}},
                    arm("incumbent", ki, Ni, field, mt), arm("retuned", kc, Nc, field, mt)]
            spec = {"schema": "ecbench.spec/v1", "label": f"ic_ksum_retune_20261118 {row}-r{rep}",
                    "description": f"Confirmatory: rule-sized k-sum (k={kc}, N={Nc}) against the producer-sized -psr incumbent (k={ki}, N={Ni}); prune=1, split=1, stride-renew. Generic decomposition track (bounded by sqrt(r); cannot beat rho). PROTOCOL.md. Public synthetic curve and targets; no security claim.",
                    "workloads": {**s["workloads"], "target_seed": 20261118600 + idx},
                    "arms": arms,
                    "measurement": {**s["measurement"], "seed": 20261118700 + idx}}
            with open(f"specs/{row}-r{rep}.json", "w") as f:
                json.dump(spec, f, indent=1); f.write("\n")
            print(row, rep, "incumbent", ki, Ni, "retuned", kc, Nc)


if __name__ == "__main__":
    pilot() if sys.argv[1] == "pilot" else confirm(sys.argv[2].split(","))
