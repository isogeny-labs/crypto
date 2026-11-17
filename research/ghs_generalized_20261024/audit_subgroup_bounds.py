"""Apply the Weil bound before attempting a low-genus GHS subgroup map.

For a genus-g curve over GF(q), #Jac(GF(q)) <= (1 + sqrt(q))^(2g).
An injective image of a prime-order source subgroup of order r requires
r <= #Jac(GF(q)).  This is only a necessary condition: passing the bound
does not establish a map, a nonzero image, or an attack.
"""

import json
from math import isqrt
from pathlib import Path


HERE = Path(__file__).resolve().parent
RESULTS = HERE / "results"
CASES = (
    (6, "screen-n6-a0-b20.json", "exact-n6-a0-b32-direct.json"),
    (8, "screen-n8-a0-b20.json", "exact-n8-a0-b32-direct.json"),
    (10, "screen-n10-a0-b20.json", "exact-n10-a0-b32-direct.json"),
    (12, "leads-n12-a1-b200-g2.json", "exact-n12-a1-b512-direct.json"),
)


def weil_upper_bound(q: int, genus: int) -> int:
    """Exact floor of (1 + sqrt(q))^(2g), using integer arithmetic."""
    constant, radical = 1, 0
    for _ in range(2 * genus):
        constant, radical = constant + radical * q, constant + radical
    return constant + isqrt(radical * radical * q)


def audit() -> dict:
    cases = []
    for n, screen_file, receipt_file in CASES:
        screen = json.loads((RESULTS / screen_file).read_text())
        receipt = json.loads((RESULTS / receipt_file).read_text())
        source = receipt["source"]
        assert int(screen["input"]["absolute_degree"]) == n
        assert int(source["absolute_degree"]) == n
        assert int(screen["input"]["a"], 0) == int(source["a"], 0)
        assert int(screen["input"]["b"], 0) == int(source["b"], 0)
        assert screen["input"]["modulus"] == source["modulus"]
        r = int(source["subgroup_order"])
        assert r > 1 and all(r % d for d in range(2, isqrt(r) + 1))
        rows = []
        for base_row in screen["base_rows"]:
            s = base_row["base_degree"]
            q = 1 << s
            for genus_text, count in base_row["genus_histogram"].items():
                g = int(genus_text)
                if g > 2:
                    continue
                bound = weil_upper_bound(q, g)
                rows.append(
                    {
                        "base_degree": s,
                        "genus": g,
                        "profiles": count,
                        "jacobian_order_upper_bound": bound,
                        "subgroup_order": r,
                        "status": "excluded_by_weil_bound" if r > bound else "order_bound_allows_only",
                    }
                )
        rows.sort(key=lambda row: (row["base_degree"], row["genus"]))
        case = {
            "absolute_degree": n,
            "screen_file": screen_file,
            "verified_source_receipt": receipt_file,
            "source": {
                "modulus": source["modulus"],
                "a": source["a"],
                "b": source["b"],
                "subgroup_order": r,
            },
            "rows": rows,
            "excluded_profiles": sum(row["profiles"] for row in rows if row["status"] == "excluded_by_weil_bound"),
            "remaining_profiles": sum(row["profiles"] for row in rows if row["status"] == "order_bound_allows_only"),
        }
        if n == 12:
            # The saved n=12 lead file has each c; bind the exclusion to exact parameters.
            actual = {}
            for lead in screen["candidates"]:
                key = (lead["base_degree"], int(lead["genus"]))
                actual[key] = actual.get(key, 0) + 1
            assert actual == {(row["base_degree"], row["genus"]): row["profiles"] for row in rows}
            case["excluded_c"] = {
                str(row["base_degree"]): sorted((
                    lead["c"] for lead in screen["candidates"]
                    if lead["base_degree"] == row["base_degree"] and int(lead["genus"]) == row["genus"]
                ), key=lambda c: int(c, 0))
                for row in rows if row["status"] == "excluded_by_weil_bound"
            }
        cases.append(case)
    return {
        "schema": "ghs-subgroup-weil-audit/v1",
        "criterion": "If prime r > floor((1+sqrt(2^s))^(2g)), every map from the r-subgroup into Jac(C)(GF(2^s)) has zero image.",
        "cases": cases,
        "excluded_profiles": sum(case["excluded_profiles"] for case in cases),
        "remaining_profiles": sum(case["remaining_profiles"] for case in cases),
    }


if __name__ == "__main__":
    result = audit()
    assert (result["excluded_profiles"], result["remaining_profiles"]) == (21, 232)
    output = RESULTS / "subgroup-weil-audit.json"
    output.write_text(json.dumps(result, indent=2) + "\n")
    print(f"{output}: {result['excluded_profiles']} excluded; {result['remaining_profiles']} need a map/order check")
