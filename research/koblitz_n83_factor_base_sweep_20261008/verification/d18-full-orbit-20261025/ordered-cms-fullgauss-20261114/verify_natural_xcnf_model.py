#!/usr/bin/env python3
"""Independently replay a complete CMS model against original XCNF rows."""

import argparse
import hashlib
import json
from pathlib import Path


def require(ok, message):
    if not ok:
        raise ValueError(message)


def sha256(path):
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def parse_model(stdout_path, variables):
    values = bytearray(variables + 1)
    statuses = []
    with stdout_path.open(errors="replace") as stream:
        for line in stream:
            if line.startswith("s "):
                statuses.append(line.strip())
            elif line.startswith("v "):
                for token in line.split()[1:]:
                    literal = int(token)
                    if literal == 0:
                        continue
                    index = abs(literal)
                    require(1 <= index <= variables, "model variable out of range")
                    require(values[index] == 0, "model variable duplicated")
                    values[index] = 1 if literal > 0 else 2
    require(statuses == ["s SATISFIABLE"], "solver SAT status differs")
    require(all(values[1:]), "model assignment incomplete")
    return values


def verify_formula(path, values, expected_constraints):
    cnf_count = 0
    xor_count = 0
    with path.open("r", encoding="ascii") as stream:
        header = stream.readline().strip()
        require(header == f"p cnf {len(values) - 1} {expected_constraints}", "XCNF header differs")
        for line_number, line in enumerate(stream, start=2):
            if line.startswith("c "):
                continue
            is_xor = line.startswith("x ")
            tokens = line[2:].split() if is_xor else line.split()
            require(tokens and tokens[-1] == "0", f"unterminated constraint at line {line_number}")
            literals = [int(token) for token in tokens[:-1]]
            require(all(1 <= abs(value) < len(values) for value in literals), f"literal range differs at line {line_number}")
            if is_xor:
                xor_count += 1
                parity = 0
                for literal in literals:
                    truth = values[abs(literal)] == 1
                    parity ^= truth if literal > 0 else not truth
                require(parity == 1, f"XOR row fails at line {line_number}")
            else:
                cnf_count += 1
                satisfied = any((values[abs(lit)] == 1) if lit > 0 else (values[abs(lit)] == 2) for lit in literals)
                require(satisfied, f"CNF clause fails at line {line_number}")
    require(cnf_count + xor_count == expected_constraints, "constraint count differs")
    return cnf_count, xor_count


def selftest():
    from tempfile import TemporaryDirectory
    with TemporaryDirectory() as folder:
        path = Path(folder) / "small.xcnf"
        path.write_text("p cnf 3 3\n1 -2 0\n3 0\nx -1 2 3 0\n")
        good = bytearray((0, 1, 2, 1))
        assert verify_formula(path, good, 3) == (2, 1)
        for bad in (bytearray((0, 2, 2, 1)), bytearray((0, 1, 1, 1))):
            try:
                verify_formula(path, bad, 3)
            except ValueError:
                pass
            else:
                raise AssertionError("invalid small model passed")
    print("XCNF replay selftest PASS")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("mode", nargs="?", choices=("natural",))
    parser.add_argument("root", nargs="?", type=Path)
    parser.add_argument("--selftest", action="store_true")
    args = parser.parse_args()
    if args.selftest:
        selftest()
        return
    require(args.mode is not None and args.root is not None, "mode and root required")
    lock = json.loads(Path(__file__).with_name("NATURAL_SOLVER_LOCK.json").read_text())
    entry = lock["exports"][args.mode]
    out = args.root / f"solver-{args.mode}"
    receipt = json.loads((out / "receipt.json").read_text())
    require(receipt["status"] == "SAT_PENDING_XCNF_REPLAY" and receipt["exit_code"] == 10, "solver receipt is not terminal SAT")
    stdout = out / "stdout.txt"
    xcnf = args.root / f"export-{args.mode}" / "result.xcnf"
    require(sha256(stdout) == receipt["stdout_sha256"], "solver stdout differs")
    require(sha256(xcnf) == entry["xcnf_sha256"], "original XCNF differs")
    require(receipt["xcnf_sha256"] == entry["xcnf_sha256"], "solver formula binding differs")
    values = parse_model(stdout, entry["variables"])
    cnf_count, xor_count = verify_formula(xcnf, values, entry["dimacs_header_clauses"])
    require((cnf_count, xor_count) == (entry["cnf_clauses"], entry["xor_rows"]), "formula type counts differ")
    model_digest = hashlib.sha256(bytes(values)).hexdigest()
    report = {
        "schema": "n83.d18-fullgauss-natural-xcnf-model-replay/v1",
        "status": "PASS_ORIGINAL_XCNF_REPLAY",
        "mode": args.mode,
        "xcnf_sha256": entry["xcnf_sha256"],
        "solver_stdout_sha256": receipt["stdout_sha256"],
        "complete_variables": entry["variables"],
        "checked_cnf_clauses": cnf_count,
        "checked_xor_rows": xor_count,
        "model_values_sha256": model_digest,
        "group_replay_status": "PENDING_GROUP_REPLAY",
        "selected_best_total_runtime": None,
    }
    result_path = out / "verification.json"
    with result_path.open("x") as stream:
        json.dump(report, stream, sort_keys=True, indent=2)
        stream.write("\n")
    print(json.dumps({"status": report["status"], "mode": args.mode, "cnf": cnf_count, "xor": xor_count}, sort_keys=True))


if __name__ == "__main__":
    main()
