"""Independent Python GF(2^83) and custody replay of the planted control.

Usage: PYTHONPATH=<blake3==1.0.8 directory> python3 verify.py OBJECT BINARY
"""

import base64
import gzip
import hashlib
import json
import pathlib
import re
import sys

import blake3


ROOT = pathlib.Path(__file__).resolve().parent
PARENT = ROOT.parent
SOURCE = "9e3edd9786a30c954abad96ab2fc858ab6d0fe65"
OBJECT_SHA = "eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b"
POLY = (1 << 83) | (1 << 45) | (1 << 2) | (1 << 1) | 1
MASK = (1 << 83) - 1
ORDER = 2417851639230796216685689


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def multiply(left, right):
    out = 0
    while right:
        if right & 1:
            out ^= left
        right >>= 1
        left <<= 1
        if left & (1 << 83):
            left ^= POLY
    return out


def inverse(value):
    if not value:
        raise ZeroDivisionError
    left, right = value, POLY
    left_coeff, right_coeff = 1, 0
    while left != 1:
        shift = left.bit_length() - right.bit_length()
        if shift < 0:
            left, right = right, left
            left_coeff, right_coeff = right_coeff, left_coeff
            shift = -shift
        left ^= right << shift
        left_coeff ^= right_coeff << shift
    return left_coeff & MASK


def add(left, right):
    if left is None:
        return right
    if right is None:
        return left
    x1, y1 = left
    x2, y2 = right
    if x1 == x2:
        if y1 ^ y2:
            return None
        if not x1:
            return None
        slope = x1 ^ multiply(y1, inverse(x1))
        x3 = multiply(slope, slope) ^ slope
        y3 = multiply(x1, x1) ^ multiply(slope ^ 1, x3)
    else:
        slope = multiply(y1 ^ y2, inverse(x1 ^ x2))
        x3 = multiply(slope, slope) ^ slope ^ x1 ^ x2
        y3 = multiply(slope, x1 ^ x3) ^ x3 ^ y1
    return x3, y3


def scalar_mul(point, scalar):
    out = None
    while scalar:
        if scalar & 1:
            out = add(out, point)
        point = add(point, point)
        scalar >>= 1
    return out


def on_curve(point):
    x, y = point
    return multiply(y, y) ^ multiply(x, y) == multiply(multiply(x, x), x) ^ 1


def replay(object_path, binary_path):
    result = json.loads((ROOT / "result.json").read_text())
    host = json.loads((ROOT / "host.json").read_text())
    wrapper = json.loads((PARENT / "shared-root-20261025/s3-manifest.json").read_text())
    get = json.loads((PARENT / "shared-root-20261025/object-get-metadata.json").read_text())
    binary_sha = (ROOT / "binary-sha256.txt").read_text().split()[0]
    assert sha256(binary_path) == binary_sha == result["binary_sha256"]
    assert result["source_commit"] == SOURCE and result["source_attestation_mode"] == "clean_checkout"
    assert object_path.stat().st_size == get["ContentLength"] == 160_747_529
    assert sha256(object_path) == OBJECT_SHA == wrapper["compressed_sha256"]
    assert get["ChecksumSHA256"] == base64.b64encode(bytes.fromhex(OBJECT_SHA)).decode()
    assert wrapper["producer_manifest_sha256"] == sha256(PARENT / "manifest.json")
    assert result["status"] == "PASS_CONSTRUCTED_CONTROL"
    assert result["mode"] == "positive" and result["root_encoding"] == "shared_root_bits"
    assert result["root_choice_pinned"] is True and result["solver_answer"] == "Sat"
    assert result["solver_conflicts"] == 0 and result["selected_best_total_runtime"] is None
    assert host["source_commit"] == SOURCE and host["binary_sha256"] == binary_sha
    assert host["supervisor_exit_code"] == 0
    assert host["internal_process_wall_cap_seconds"] == 300
    assert host["independent_outer_cap_seconds"] == 310
    assert host["selected_best_total_runtime"] is None
    assert result["shape"]["clauses"] == 2_807_784
    assert result["shape"]["variables"] == 329_726 and result["shape"]["xor_rows"] == 3_901
    assert (ROOT / "exit-code.txt").read_text().strip() == "0"
    phases = [json.loads(line) for line in (ROOT / "result.json.phases.jsonl").read_text().splitlines()]
    assert [row["phase"] for row in phases] == ["started", "import_checked", "model_constructed", "solver_returned"]
    assert phases[0]["detail"]["source_commit"] == SOURCE
    assert phases[2]["detail"] == result["shape"]
    assert phases[3]["detail"]["answer"] == "Sat"

    indices = result["relation"]["indices"]
    assert indices == [0, 166, 332, 498]
    rows = {}
    with gzip.open(object_path, "rt") as stream:
        header = json.loads(next(stream))
        for index, line in enumerate(stream):
            if index in indices:
                rows[index] = json.loads(line)
            if index >= indices[-1]:
                break
    assert header["subgroup_order"] == str(ORDER)
    assert header["curve_a"] == 0 and header["curve_b"] == "1"
    assert header["generator"] == ["5402229379304697578719232", "3600185575789834070382019"]
    generator = tuple(map(int, header["generator"]))
    target = tuple(map(int, result["target"]))
    assert on_curve(generator) and on_curve(target) and scalar_mul(generator, ORDER) is None
    total = None
    points = []
    for column, index in enumerate(indices):
        row = rows[index]
        assert row["column"] == column and row["phase"] == 0
        assert row["negative"] is False and row["coefficient"] == "1"
        point = tuple(map(int, row["point"]))
        assert on_curve(point) and scalar_mul(point, ORDER) is None
        total = add(total, point)
        points.append(point)
    assert total == target and add(target, generator) != total
    coefficients = ["0"] * 32_643
    for column in range(4):
        coefficients[column] = "1"
    row_digest = blake3.blake3(json.dumps(coefficients, separators=(",", ":")).encode()).hexdigest()
    relation = result["relation"]
    assert relation["row_decimal_blake3"] == row_digest
    assert relation["matches_planted_row"] is True
    assert relation["tamper_rejected"] is True and relation["rank_after_insert"] == 1
    time_record = (ROOT / "stderr-time.txt").read_text()
    max_rss = int(re.search(r"(\d+)\s+maximum resident set size", time_record).group(1))
    assert max_rss == 3_887_513_600
    return {
        "schema": "n83.full-d18-root-pinned-independent-replay/v1",
        "status": "PASS_INDEPENDENT_GROUP_AND_ROW_REPLAY",
        "source_commit": SOURCE,
        "binary_sha256": binary_sha,
        "object_sha256": OBJECT_SHA,
        "point_indices": indices,
        "point_coordinates": [list(map(str, point)) for point in points],
        "target": list(map(str, target)),
        "row_decimal_blake3": row_digest,
        "rank_after_insert": 1,
        "search_ms": result["search_ms"],
        "process_wall_ms": result["process_wall_ms"],
        "supervisor_max_rss_bytes": max_rss,
        "natural_search_executed": False,
        "selected_best_total_runtime": None,
    }


if __name__ == "__main__":
    if len(sys.argv) != 3:
        raise SystemExit("usage: verify.py COMPRESSED_S3_OBJECT SOURCE_BINARY")
    print(json.dumps(replay(pathlib.Path(sys.argv[1]), pathlib.Path(sys.argv[2])), indent=2, sort_keys=True))
