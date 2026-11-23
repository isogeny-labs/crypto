"""JSONL adapters for external solver binaries: msolve, CryptoMiniSat CLI, Magma.

Run from the repository root: python -m groebner_compare.external_worker NAME.
Each solver runs as a measured child process; its own wall seconds, CPU
seconds and peak RSS (sampled VmHWM of the exec'd solver; see run_measured) are
reported in `metrics` as solver_wall_seconds, solver_cpu_seconds and
solver_peak_rss_bytes.
The runner's own peak_worker_process_rss_bytes covers only this Python worker.

msolve: only `-g 2` (F4 reduced DRL basis) is used. Over GF(2) its default
solving path (sparse FGLM / rational parametrisation) returned degenerate or
wrong output on toy systems with more than one root (scout, 0.6.5 and 0.11.0),
so roots are extracted and certified by the runner instead. Degree and matrix
telemetry is parsed from the `-v 2` F4 table: highest_degree is the largest
`deg` column of an F4 round (the step degree msolve reached, i.e. the measured
solving degree of this run), and the largest matrix is the round (or final
reduction) with the most entries rows*cols.

cryptominisat5: CNF with one Tseitin AND gate per nonlinear monomial plus
native XOR rows (`x` lines), enumerating up to --max-solutions models. It
exposes no degree or matrix data; those stay null. An UNSAT answer is the
solver's claim only and is returned as `unknown`.

magma: reported unavailable unless a licensed binary is reachable; even then
this adapter has not been validated against a real Magma and refuses to run.
"""

import contextlib
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import time

from .certificate import terms
from .semaev import to_dimacs_xor, to_msolve


def vmhwm(pid):
    try:
        for line in Path(f"/proc/{pid}/status").read_text().splitlines():
            if line.startswith("VmHWM:"):
                return int(line.split()[1]) * 1024
    except (OSError, ValueError, IndexError):
        pass
    return None


def run_measured(argv, stdout_path, stderr_path, stdin_path=None, poll=0.002):
    """Run argv; return (exit code, wall s, cpu s, rss dict) for that child.

    Peak RSS: `sampled_vmhwm_bytes` is the largest /proc VmHWM read while the
    exec'd solver ran (its own address space; may miss growth in the final
    poll interval, and is None for runs shorter than one poll). `ru_maxrss` from
    wait4 is only an UPPER bound: on Linux the child inherits the forking
    worker's RSS high-water mark across exec, so it is floored at this Python
    worker's RSS. Wall time includes up to one poll interval of reaping delay.
    """
    begin = time.perf_counter()
    peak = None
    with open(stdout_path, "wb") as out, open(stderr_path, "wb") as err, \
            (open(stdin_path, "rb") if stdin_path else open(os.devnull, "rb")) as inp:
        process = subprocess.Popen(argv, stdin=inp, stdout=out, stderr=err)
        while True:
            pid, status, usage = os.wait4(process.pid, os.WNOHANG)
            if pid:
                break
            sample = vmhwm(process.pid)
            if sample is not None:
                peak = max(peak or 0, sample)
            time.sleep(poll)
        process.returncode = os.waitstatus_to_exitcode(status)
    wall = time.perf_counter() - begin
    rss = {"solver_peak_rss_bytes": peak,
           "solver_peak_rss_method": "sampled /proc/PID/status VmHWM of the exec'd solver",
           "solver_ru_maxrss_upper_bound_bytes": usage.ru_maxrss * 1024}  # KiB on Linux
    return process.returncode, wall, usage.ru_utime + usage.ru_stime, rss


def binary_identity(path):
    return {"path": str(path),
            "sha256": hashlib.sha256(Path(path).read_bytes()).hexdigest()}


def locate(variable, name):
    path = os.environ.get(variable) or shutil.which(name)
    return path if path and Path(path).is_file() else None


ROUND = re.compile(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+)\s*x\s*(\d+)\s+([\d.]+)%")
FINAL = re.compile(r"reduce final basis\s+(\d+)\s*x\s*(\d+)")
QUOTIENT = re.compile(r"Dimension of quotient:\s*(\d+)")


def parse_msolve_log(text):
    rounds = []
    for line in text.splitlines():
        match = ROUND.match(line)
        if match:
            deg, sel, pairs, rows, cols, density = match.groups()
            rounds.append({"deg": int(deg), "selected": int(sel), "pairs": int(pairs),
                           "rows": int(rows), "cols": int(cols),
                           "density_percent": float(density)})
    final = FINAL.search(text)
    matrices = [(r["rows"], r["cols"]) for r in rounds]
    if final:
        matrices.append((int(final.group(1)), int(final.group(2))))
    largest = max(matrices, key=lambda rc: (rc[0] * rc[1], rc)) if matrices else (None, None)
    quotient = QUOTIENT.search(text)
    return {"highest_degree": max((r["deg"] for r in rounds), default=None),
            "largest_matrix_rows": largest[0], "largest_matrix_columns": largest[1],
            "f4_rounds": len(rounds),
            "f4_round_table": [[r["deg"], r["rows"], r["cols"]] for r in rounds],
            "final_reduction_matrix": ([int(final.group(1)), int(final.group(2))]
                                       if final else None),
            "msolve_claimed_quotient_dimension": int(quotient.group(1)) if quotient else None,
            "degree_semantics": "max deg column of msolve -v 2 F4 rounds"}


def parse_msolve_basis(text, nvars):
    body = text[text.index("["):text.rindex("]")]
    body = body[1:]
    rows = []
    for chunk in body.split(","):
        chunk = chunk.strip()
        if not chunk:
            continue
        row = []
        for term in chunk.replace(" ", "").split("+"):
            factors = term.split("*")
            coefficient = 1
            mask = 0
            for factor in factors:
                if re.fullmatch(r"-?\d+", factor):
                    coefficient *= int(factor)
                    continue
                name, _, power = factor.partition("^")
                if not re.fullmatch(r"x\d+", name):
                    raise ValueError(f"unexpected msolve variable {name!r}")
                if int(power or 1) > 0:
                    mask |= 1 << int(name[1:])
            if coefficient % 2:
                row.append(mask)
        rows.append(row)
    return terms(rows, nvars)


def msolve(request, threads):
    binary = locate("MSOLVE_BINARY", "msolve")
    if binary is None:
        return {"status": "unavailable", "reason": "msolve not on PATH and MSOLVE_BINARY unset"}
    n = request["ring"]["nvars"]
    equations = terms(request["instance"]["equations"], n)
    with tempfile.TemporaryDirectory(prefix="gc-msolve-") as directory:
        directory = Path(directory)
        source, output = directory / "in.ms", directory / "out.gb"
        source.write_text(to_msolve(equations, n))
        argv = [binary, "-g", "2", "-v", "2", "-t", str(threads),
                "-f", str(source), "-o", str(output)]
        code, wall, cpu, rss = run_measured(argv, directory / "log", directory / "err")
        log = (directory / "log").read_text(errors="replace")
        print(log, file=sys.stderr)
        print((directory / "err").read_text(errors="replace"), file=sys.stderr)
        metrics = {**parse_msolve_log(log), "solver_wall_seconds": wall,
                   "solver_cpu_seconds": cpu, **rss,
                   "solver_exit_code": code, "solver_argv_flags": argv[1:6]}
        version = {"solver": "msolve", **binary_identity(binary)}
        if code != 0 or not output.is_file():
            return {"status": "error", "reason": f"msolve exit {code}", "metrics": metrics,
                    "solver_version": json.dumps(version, sort_keys=True)}
        basis = parse_msolve_basis(output.read_text(), n)
    return {"status": "ok", "basis_terms": basis, "metrics": metrics,
            "solver_version": json.dumps(version, sort_keys=True)}


def parse_cms(text, nvars):
    solutions, current, verdicts = [], None, []
    conflicts = None
    for line in text.splitlines():
        if line.startswith("s "):
            verdicts.append(line[2:].strip())
            current = [] if line.strip() == "s SATISFIABLE" else None
            if current is not None:
                solutions.append(current)
        elif line.startswith("v ") and current is not None:
            current.extend(int(x) for x in line[2:].split())
        elif line.startswith("c conflicts"):
            match = re.search(r":\s*(\d+)", line)
            if match:
                conflicts = int(match.group(1))
    masks = []
    for literals in solutions:
        positive = {x for x in literals if x > 0}
        mask = sum(1 << (v - 1) for v in range(1, nvars + 1) if v in positive)
        if mask not in masks:
            masks.append(mask)
    return masks, verdicts, conflicts


def cryptominisat(request, threads, max_solutions):
    binary = locate("CRYPTOMINISAT_BINARY", "cryptominisat5")
    if binary is None:
        return {"status": "unavailable",
                "reason": "cryptominisat5 not on PATH and CRYPTOMINISAT_BINARY unset"}
    n = request["ring"]["nvars"]
    equations = terms(request["instance"]["equations"], n)
    with tempfile.TemporaryDirectory(prefix="gc-cms-") as directory:
        directory = Path(directory)
        text, shape = to_dimacs_xor(equations, n)
        (directory / "in.cnf").write_text(text)
        argv = [binary, "--verb", "1", "--threads", str(threads),
                "--maxsol", str(max_solutions), str(directory / "in.cnf")]
        code, wall, cpu, rss = run_measured(argv, directory / "out", directory / "err")
        output = (directory / "out").read_text(errors="replace")
        print(output, file=sys.stderr)
        print((directory / "err").read_text(errors="replace"), file=sys.stderr)
    solutions, verdicts, conflicts = parse_cms(output, n)
    metrics = {"highest_degree": None, "largest_matrix_rows": None,
               "largest_matrix_columns": None, "solver_wall_seconds": wall,
               "solver_cpu_seconds": cpu, **rss,
               "solver_exit_code": code, "cms_conflicts": conflicts,
               "max_solutions": max_solutions, **shape,
               "solver_claimed_enumeration_complete": bool(
                   verdicts and verdicts[-1] == "UNSATISFIABLE"
                   and len(solutions) < max_solutions)}
    version = json.dumps({"solver": "cryptominisat5", **binary_identity(binary)},
                         sort_keys=True)
    if code not in (10, 20):
        return {"status": "error", "reason": f"cryptominisat5 exit {code}",
                "metrics": metrics, "solver_version": version}
    if not solutions:
        return {"status": "unknown", "reason": "CMS reported UNSAT; not independently certified",
                "metrics": metrics, "solver_version": version}
    return {"status": "ok", "solutions": solutions, "metrics": metrics,
            "solver_version": version}


def magma(request):
    binary = locate("MAGMA_BINARY", "magma")
    if binary is None:
        return {"status": "unavailable",
                "reason": "UNAVAILABLE: no licensed Magma binary reachable "
                          "(magma not on PATH, MAGMA_BINARY unset)"}
    return {"status": "unavailable",
            "reason": f"UNAVAILABLE: Magma found at {binary}, but this adapter has not been "
                      "validated against a licensed Magma; refusing to run it unvalidated"}


def solve(name, request, options):
    if request["operation"] != "solve":
        return {"status": "incompatible", "reason": "no trace support"}
    if name == "msolve":
        return msolve(request, options["threads"])
    if name == "cryptominisat5":
        return cryptominisat(request, options["threads"], options["max_solutions"])
    if name == "magma":
        return magma(request)
    return {"status": "unavailable", "reason": f"unknown external solver: {name}"}


def main():
    name = sys.argv[1]
    options = {"threads": int(os.environ.get("OMP_NUM_THREADS", "1") or 1),
               "max_solutions": 64}
    arguments = sys.argv[2:]
    if arguments[:1] == ["--max-solutions"]:
        options["max_solutions"] = int(arguments[1])
    for line in sys.stdin:
        request = json.loads(line)
        try:
            with contextlib.redirect_stdout(sys.stderr):
                result = solve(name, request, options)
        except (OSError, ValueError) as error:
            result = {"status": "error", "reason": str(error)}
        result["request_id"] = request["request_id"]
        print(json.dumps(result, allow_nan=False), flush=True)


if __name__ == "__main__":
    main()
