#!/usr/bin/env python3
"""POST-HOC follow-up (not preregistered): k-sum size/arity sweep, specs/x24.json, x28.json.
Same curves/targets/seed as p24/p28 so `inc` pairs with the main session's inc."""
import json, pathlib
here = pathlib.Path(__file__).resolve().parent.parent
SZ = {"p24": {3: (12, 16, 20), 4: (12, 14, 16), 5: (9, 11, 13)},
      "p28": {3: (24, 32, 40), 4: (14, 16, 20), 5: (11, 13, 16)}}
FB = {"p24": 128, "p28": 256}
for c in ("p24", "p28"):
    s = json.load(open(here/"specs"/f"{c}.json"))
    arms = [s["arms"][0]]
    def mk(name, orc, n, role="candidate"):
        return {"name": name, "role": role, "method": {"id": "ic.pipeline", "params": {
            "factor_base": f"prime-abscissa:size={n}", "linalg": "incremental-gauss", "oracle": orc, "targets": "walk"}}}
    arms.append(mk("inc", "mitm-prime-blocked:m=2", FB[c], "baseline"))
    for m, ns in SZ[c].items():
        for n in ns:
            arms.append(mk(f"k{m}_{n}", f"ksum:m={m}", n))
    s["arms"] = arms
    s["label"] = f"POST-HOC ksum m=3,4,5 size sweep, {c}"
    s["description"] = "research/ic_prime_pair_constants_20261119 README, deviation D2 (not preregistered)"
    json.dump(s, open(here/"specs"/f"x{c[1:]}.json", "w"), indent=1)
