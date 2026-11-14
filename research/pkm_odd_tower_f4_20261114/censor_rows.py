"""Amendment 3: write censored placeholder rows for a cell whose instrument was killed
(host memory), so the cell is recorded rather than silently missing. No counts are
written or imputed; `censored: true` excludes the rows from every verdict.

    python3 censor_rows.py PART FILE CELL ROWS REASON -- <instrument args>
PART is A (pkm_odd_tower_f4 rows) or B (pkm_algebraic_oracle rows).
"""
import json
import math
import sys


def main():
    part, path, cell, nrows, reason = sys.argv[1:6]
    args = sys.argv[sys.argv.index("--") + 1:]
    o = {args[i].lstrip("-"): args[i + 1] for i in range(0, len(args) - 1, 2)}
    degrees = [int(x) for x in o["degrees"].split(",")]
    s = math.prod(degrees)
    try:
        have = sum(1 for line in open(path) if line.strip())
    except FileNotFoundError:
        have = 0
    with open(path, "a") as f:
        for k in range(have, int(nrows)):
            base = {"cell": cell, "p": int(o["p"]), "m": int(o["m"]), "degrees": degrees, "S": s,
                    "log2S": math.log2(s), "censored": True, "censor_reason": reason, "draw": k}
            if part == "A":
                base.update({"schema": "pkm-odd-tower-f4.row/v1", "arm": "".join(map(str, degrees)),
                             "control": o.get("control", "tower"), "form": o.get("form", "kummer"),
                             "kind": o.get("kind", "refute"), "agree": False, "field_ops": None,
                             "inconsistent": None, "planted_ok": None, "timed_out": False})
            else:
                base.update({"schema": "pkm-algebraic-oracle.row/v1", "form": o.get("form", "kummer"),
                             "kind": "planted" if k < 2 else "random", "index": k,
                             "alg_undetermined": True, "agree": False, "alg_field_ops": None,
                             "enum_field_ops": None, "enum_relations": None,
                             "ratio_alg_over_enum": float("nan")})
            f.write(json.dumps(base) + "\n")


if __name__ == "__main__":
    main()
