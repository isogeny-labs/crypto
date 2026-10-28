#!/usr/bin/env python3
"""Generate independent PARI/GP point-equation replay from raw screen JSON."""
import json
import sys
from pathlib import Path


def main():
    runs = [json.loads(Path(arg).read_text()) for arg in sys.argv[1:]]
    assert len(runs) == 2 and {r["seed"] for r in runs} == {131, 132}
    v = {k: int(value, 16) for k, value in runs[0]["parameters"].items()}
    assert runs[0]["parameters"] == runs[1]["parameters"]
    for key in ("p", "a", "b", "gx", "gy", "qx", "qy", "n"):
        print(f"{key}={v[key]};")
    print("E=ellinit([a,b],p);")
    print("if(!isprime(p)||!isprime(n),error(\"nonprime parameter\"));")
    print("if(ellcard(E)!=n,error(\"wrong curve order\"));")
    print("r=155003179783434785924857641481313994241;")
    print("if(!isprime(r)||factor(p-1)!=[2,1;5,1;r,1],error(\"factorization changed\"));")
    print("G=[gx,gy];Q=[qx,qy];")
    print("if(!ellisoncurve(E,G)||!ellisoncurve(E,Q)||ellmul(E,G,n)!=[0]||ellmul(E,Q,n)!=[0],error(\"published point invalid\"));")
    count = 0
    for run in runs:
        for arm in run["arms"]:
            if arm["status"] != "complete":
                continue
            base = [[int(x, 16), int(y, 16)] for x, y in arm["base"]]
            for x, y in base:
                print(f"if(!ellisoncurve(E,[{x},{y}]),error(\"base point invalid\"));")
            for witness in arm["planted_controls"] + arm["setup_relations"] + arm["target_relations"]:
                count += 1
                target = [int(z, 16) for z in witness["point"]]
                print("R=[0];")
                for j, sign in witness["terms"]:
                    x, y = base[j]
                    print(f"R=elladd(E,R,{f'[{x},{y}]' if sign == 1 else f'ellneg(E,[{x},{y}])'});")
                print(f"if(R!=[{target[0]},{target[1]}],error(\"point relation {count} invalid\"));")
                if "scalar" in witness:
                    scalar = int(witness["scalar"], 16)
                    point = "elladd(E,Q,ellmul(E,G,%d))" % scalar if witness in arm["target_relations"] else "ellmul(E,G,%d)" % scalar
                    print(f"if(R!={point},error(\"candidate relation {count} invalid\"));")
    print(f'print("PASS_GP_REPLAY relations={count} curve_order=verified factorization=verified");')


if __name__ == "__main__":
    main()
