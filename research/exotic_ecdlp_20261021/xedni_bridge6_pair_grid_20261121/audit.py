"""Independent SymPy replay for every 22-by-25 coefficient cell."""

import hashlib
import importlib.util
import json
import re
from collections import Counter
from concurrent.futures import ProcessPoolExecutor
from pathlib import Path

import sympy as s

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
HELPER = ROOT / "xedni_bridge6_pair_shifts_20261121/audit.py"

spec = importlib.util.spec_from_file_location("pair_shift_independent_audit", HELPER)
helper = importlib.util.module_from_spec(spec)
spec.loader.exec_module(helper)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def rederive(row):
    num, den = helper.independent_polynomials(row["points"],
                                                [row["k1"], row["k2"], row["base_k"][2]])
    return ([int(c) for c in reversed(num.all_coeffs())],
            [int(c) for c in reversed(den.all_coeffs())])


def event(stage, **data):
    with (HERE / "audit-events.jsonl").open("a") as f:
        f.write(json.dumps({"stage": stage, **data}, sort_keys=True) + "\n")


def main():
    protocol = json.loads((HERE / "audit-protocol.json").read_text())
    assert sha(HELPER.read_bytes()) == protocol["sympy_helper_sha256"]
    result = json.loads((HERE / "result.json").read_text())
    files = sorted((HERE / "rows").glob("[0-9][0-9][0-9][0-9][0-9].jsonl"))
    assert len(files) == result["cells"] == 550
    raw = b"".join(file.read_bytes() for file in files)
    assert sha(raw) == protocol["source_rows_sha256"] == result["rows_raw_sha256"]
    rows = [json.loads(file.read_text()) for file in files]
    source = [json.loads(line) for line in (ROOT / "xedni_bridge6_source_policy_20261121/outcomes/000.jsonl").read_text().splitlines()]
    source = [row for row in source if row["status"] == "local_bridge"]
    central = [json.loads(file.read_text()) for file in sorted((ROOT / "xedni_bridge6_pair_shifts_20261121/rows").glob("[0-9][0-9][0-9].jsonl"))]
    old25 = [json.loads(file.read_text()) for file in sorted((ROOT / "xedni_bridge6_k1k2_grid_20261120/rows").glob("[0-9][0-9][0-9].jsonl"))]
    assert len(source) == len(central) == 22 and len(old25) == 25
    for i, row in enumerate(rows):
        pair, offset = divmod(i, 25)
        u, v = divmod(offset, 5)
        u, v = u-2, v-2
        base = source[pair]
        assert (row["pair_index"], row["u"], row["v"]) == (base["pair_index"], u, v)
        assert row["points"] == base["points"] and row["base_k"] == base["crt_k"]
        assert (row["k1"], row["k2"]) == (base["crt_k"][0]+77*u,
                                                base["crt_k"][1]+77*v)
        assert row["status"] == "excluded_all_integer_t"
        assert row["center_local_7"] == [4, True] and row["center_local_11"] == [8, True]
        assert row["center_relation"] is False
        if u == v == 0:
            assert row["numerator_coefficients"] == central[pair]["numerator_coefficients"]
            assert row["denominator_coefficients"] == central[pair]["denominator_coefficients"]
        if pair == 0:
            assert row["numerator_coefficients"] == old25[offset]["numerator_coefficients"]
            assert row["denominator_coefficients"] == old25[offset]["denominator_coefficients"]
    (HERE / "audit-events.jsonl").write_text("")
    event("started", cells=len(rows))
    primes = list(s.primerange(13, 998))
    prime_counts = Counter()
    with ProcessPoolExecutor(max_workers=4) as pool:
        for i, (row, (num, den)) in enumerate(zip(rows, pool.map(rederive, rows, chunksize=4))):
            assert num == list(map(int, row["numerator_coefficients"]))
            assert den == list(map(int, row["denominator_coefficients"]))
            p = row["certificate_prime"]
            assert p in primes
            for q in primes[:primes.index(p)]:
                assert any(helper.mod_value(num, z, q) == 0 for z in range(q))
            assert all(helper.mod_value(num, z, p) != 0 for z in range(p))
            prime_counts[p] += 1
            event("verified", cell=i, pair_index=row["pair_index"], u=row["u"], v=row["v"])
    assert {str(k): v for k, v in sorted(prime_counts.items())} == result["certificate_prime_counts"]
    lean_raw = (HERE / "formal" / "PairGrid.lean").read_bytes()
    lean_receipt = json.loads((HERE / "formal" / "source-result.json").read_text())
    assert sha(lean_raw) == lean_receipt["lean_sha256"]
    lean_lists = re.findall(r"def coeffs(\d{3}) : List ℤ := \[(.*?)\]", lean_raw.decode(), re.S)
    assert len(lean_lists) == 550
    for i, (label, body) in enumerate(lean_lists):
        assert int(label) == i
        assert [int(x) for x in re.findall(r"-?\d+", body)] == [int(x) for x in rows[i]["numerator_coefficients"]]
    build = (HERE / "formal" / "build-output.txt").read_text().splitlines()
    assert len(build) == 1100
    assert all("depends on axioms: [propext, Classical.choice, Quot.sound]" in line for line in build)
    assert not (HERE / "formal" / "build-stderr.txt").read_text()
    event("finished", cells=len(rows))
    print(json.dumps({"schema": "xedni-bridge6-pair-grid-audit-v1",
                      "status": "PASS", "symbolic_replays": len(rows),
                      "first_prime_certificates": sum(prime_counts.values()),
                      "old_grid_overlaps": 25, "central_overlaps": 22,
                      "lean_polynomial_bindings": 550,
                      "rows_raw_sha256": sha(raw)}, sort_keys=True, indent=2))


if __name__ == "__main__":
    main()
