"""Portable file/hash/process custody only; mathematics remains frozen Rust."""
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import platform
import resource
import shutil
import signal
import subprocess
import sys
import traceback

STUDY = "research/chained_cubic_coordinate_20261127"
SOURCES = {"C": "91db340ea313aaefe12e9c39a458a7b81929eeb7",
           "E": "e30f6512a0d13075ef177c38693f34f7027d317e"}
REFERENCE = "7b52e831e2df3b6c69e23493c889738cacc742da"
ROOT = Path(__file__).resolve().parents[2]
OUT = Path(sys.argv[1]).resolve()
OUT.mkdir(parents=True, exist_ok=False)
TOOLS, INPUTS, BUILDS, PHASES = {}, {}, {}, []
ENV = os.environ.copy()
ENV.update({"RAYON_NUM_THREADS": "1", "CARGO_BUILD_JOBS": "1",
            "CHECKPOINT_SKIP": "Distinct portable replay of frozen public correctness/source-operation certificates C/E; no new cohort, timing, scaling or performance admission."})
for key in ["CLIPPY_ARGS", "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER",
            "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS"]:
    ENV.pop(key, None)


def now():
    return datetime.now(timezone.utc).isoformat()


def write(path, value):
    if path.exists():
        raise RuntimeError("write-once record exists: " + str(path))
    path.write_text(json.dumps(value, indent=2) + "\n")


def digest(path):
    raw = path.read_bytes()
    return {"bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest(),
            "mode": path.stat().st_mode & 0o777}


def git_blob(ref, path):
    proc = subprocess.run(["git", "cat-file", "blob", ref + ":" + path],
                          cwd=ROOT, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    if proc.returncode:
        index = len(list(OUT.glob("failed-git-*.json")))
        write(OUT / ("failed-git-" + str(index) + ".json"),
              {"ref": ref, "path": path, "exit": proc.returncode,
               "stdout_hex": proc.stdout.hex(),
               "stderr_utf8": proc.stderr.decode(errors="replace")})
        raise RuntimeError("complete published input unavailable: " + path)
    return proc.stdout


def install_input(path, raw, expected, mode):
    if len(raw) != expected["bytes"] or hashlib.sha256(raw).hexdigest() != expected["sha256"]:
        raise RuntimeError("published input digest mismatch: " + str(path))
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(raw)
    path.chmod(mode)
    INPUTS[str(path)] = digest(path)


def snapshot():
    return {"inputs": {p: digest(Path(p)) for p in INPUTS},
            "tools_and_runtime": {p: digest(Path(p)) for p in TOOLS},
            "executables": {p: digest(Path(p)) for p in BUILDS}}


def guard():
    snap = snapshot()
    if snap["inputs"] != INPUTS or snap["tools_and_runtime"] != TOOLS or snap["executables"] != BUILDS:
        raise RuntimeError("source/tool/runtime/executable custody mismatch")
    return snap


def limits():
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    resource.setrlimit(resource.RLIMIT_AS, (2147483648, 2147483648))


def phase(name, argv):
    guard()
    write(OUT / (name + ".argv.json"), argv)
    timed_out = False
    with (OUT / (name + ".stdout")).open("wb") as stdout, (OUT / (name + ".stderr")).open("wb") as stderr:
        proc = subprocess.Popen(argv, cwd=ROOT, env=ENV, stdout=stdout, stderr=stderr,
                                start_new_session=True, preexec_fn=limits)
        try:
            status = proc.wait(timeout=120)
        except subprocess.TimeoutExpired:
            timed_out = True
            os.killpg(proc.pid, signal.SIGTERM)
            try:
                status = proc.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(proc.pid, signal.SIGKILL)
                status = proc.wait()
    record = {"name": name, "recorded": now(), "exit": status,
              "expected_exit": 0, "timeout": timed_out,
              "stdout": digest(OUT / (name + ".stdout")),
              "stderr": digest(OUT / (name + ".stderr"))}
    write(OUT / (name + ".phase.json"), record)
    PHASES.append(record)
    guard()
    if status != 0 or timed_out:
        raise RuntimeError("first portable phase failed: " + name)


def initialize():
    head = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT).decode().strip()
    lock_path = STUDY + "/CI_REPLAY_LOCK_G.json"
    raw_lock = git_blob(head, lock_path)
    own_lock = json.loads(raw_lock)
    for path, expected in own_lock["files"].items():
        raw = git_blob(head, path)
        if (ROOT / path).read_bytes() != raw:
            raise RuntimeError("portable source differs from published bytes: " + path)
        install_input(OUT / "portable-inputs" / path, raw, expected, expected["mode"])
        INPUTS[str(ROOT / path)] = digest(ROOT / path)
    install_input(OUT / "portable-inputs" / lock_path, raw_lock,
                  {"bytes": len(raw_lock), "sha256": hashlib.sha256(raw_lock).hexdigest()}, 0o600)
    if (ROOT / lock_path).read_bytes() != raw_lock:
        raise RuntimeError("portable lock differs from published bytes")
    INPUTS[str(ROOT / lock_path)] = digest(ROOT / lock_path)

    toolchain = Path(os.environ.get("RUSTUP_HOME", str(Path.home() / ".rustup"))) / "toolchains/1.99.1-x86_64-unknown-linux-gnu"
    tools = {name: toolchain / "bin" / name for name in ["rustc", "clippy-driver", "rustfmt"]}
    for tag, ref in SOURCES.items():
        frozen_path = STUDY + "/SOURCE_LOCK_" + tag + ".json"
        raw = git_blob(ref, frozen_path)
        frozen = json.loads(raw)
        for path, expected in frozen["files"].items():
            blob = git_blob(ref, path)
            for destination in ["original-" + tag, "source-" + tag]:
                install_input(OUT / destination / path, blob, expected, expected["mode"])
        for destination in ["original-" + tag, "source-" + tag]:
            install_input(OUT / destination / frozen_path, raw,
                          {"bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()}, 0o600)
        for name, path in tools.items():
            if digest(path)["sha256"] != frozen["tool_binary_sha256"][name]:
                raise RuntimeError("official compiler bytes differ from source " + tag + ": " + name)
            TOOLS[str(path)] = digest(path)
        for old_path, expected_hash in frozen["runtime_file_sha256"].items():
            if "/.rustup/toolchains/" not in old_path:
                # Cloud rustup is not executed here. CI invokes official tools directly.
                continue
            relative = old_path.split("/.rustup/toolchains/", 1)[1].split("/", 1)[1]
            path = toolchain / relative
            if digest(path)["sha256"] != expected_hash:
                raise RuntimeError("official Rust runtime differs: " + relative)
            TOOLS[str(path)] = digest(path)
    dispatcher = shutil.which("rustup")
    if dispatcher:
        TOOLS[dispatcher] = digest(Path(dispatcher))

    # Whole reference originals are frozen and retained before strict/native calls too.
    references = {}
    for tag, names in {
        "C": ["actual-cubic-coordinate.stdout", "data/class_pushes.csv",
              "data/cold_operation_controls.csv", "data/target_controls.csv"],
        "E": ["actual-counter-analysis.stdout", "data/counter_summary.csv",
              "data/paired_map_deltas.csv", "data/counter_table.tex"],
    }.items():
        for name in names:
            path = STUDY + "/first-" + tag + "-native/" + name
            raw = git_blob(REFERENCE, path)
            dest = OUT / "reference-results" / path
            install_input(dest, raw, {"bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()}, 0o600)
            references[(tag, name)] = dest

    write(OUT / "CONTEXT.json",
          {"schema": "chained-portable-replay/v1", "recorded": now(), "head": head,
           "sources": SOURCES, "reference_results": REFERENCE,
           "host": platform.uname()._asdict(),
           "limits": {"phase_seconds": 120, "address_space_bytes": 2147483648, "workers": 1},
           "classification": ENV["CHECKPOINT_SKIP"],
           "boundary": "Distinct CI runtime. Official compiler and named Rust runtime pinned; OS libraries ambient. No timing or independent-signature claim.",
           "environment": {k: ENV[k] for k in ["RAYON_NUM_THREADS", "CARGO_BUILD_JOBS", "CHECKPOINT_SKIP"]}})
    write(OUT / "BUILD_MANIFEST.json", guard())
    return tools, references


def replay(tag, tools, references):
    source = OUT / ("source-" + tag) / STUDY
    name = "actual-cubic-coordinate" if tag == "C" else "actual-counter-analysis"
    entry = "main.rs" if tag == "C" else "analysis_e.rs"
    crate_name = "chained_cubic_coordinate_a" if tag == "C" else "chained_cubic_counter_analysis_e"
    prefix = tag + "-"
    for tool in ["rustc", "clippy-driver", "rustfmt"]:
        phase(prefix + tool + "-version", [str(tools[tool]), "--version"])
    formats = ["analysis_e.rs"] if tag == "E" else [
        "main.rs", "accounting.rs", "field_d.rs", "rational.rs", "cubic.rs", "rho.rs",
        "inputs/cryptanalysis1964/field49_a.rs", "inputs/cryptanalysis1964/elliptic_a.rs",
        "inputs/cryptanalysis1964/divisor_a.rs", "inputs/crypto5114/field_a.rs",
        "inputs/crypto5114/poly_d.rs"]
    phase(prefix + "format", [str(tools["rustfmt"]), "--edition", "2021",
                              "--config", "skip_children=true", "--check",
                              *[str(source / p) for p in formats]])
    built, qualifier = OUT / (tag + "-native-built"), OUT / (tag + "-qualifier")
    phase(prefix + "compile", [str(tools["rustc"]), "--edition=2021", "--crate-name", crate_name,
                               "-Dwarnings", "-Copt-level=3", "-Coverflow-checks=yes",
                               "-Cdebug-assertions=yes", "-Cdebuginfo=0", "-Ccodegen-units=1",
                               str(source / entry), "-o", str(built)])
    BUILDS[str(built)] = digest(built)
    phase(prefix + "clippy", [str(tools["clippy-driver"]), "--edition=2021", "--crate-name", crate_name,
                              "-Dwarnings", "--emit=metadata", str(source / entry),
                              "-o", str(OUT / (tag + "-clippy.rmeta"))])
    shutil.copyfile(built, qualifier)
    qualifier.chmod(0o555)
    if digest(built)["sha256"] != digest(qualifier)["sha256"]:
        raise RuntimeError("whole separate qualifier differs")
    BUILDS[str(qualifier)] = digest(qualifier)
    write(OUT / (tag + "-PRE_CALL_MANIFEST.json"), guard())
    phase(prefix + name, [str(qualifier), str(OUT / (tag + "-data"))])
    pairs = [(OUT / (prefix + name + ".stdout"), name + ".stdout")]
    names = ["class_pushes.csv", "cold_operation_controls.csv", "target_controls.csv"] if tag == "C" else [
        "counter_summary.csv", "paired_map_deltas.csv", "counter_table.tex"]
    pairs += [(OUT / (tag + "-data") / n, "data/" + n) for n in names]
    compared = []
    for path, reference_name in pairs:
        expected_path = references[(tag, reference_name)]
        if path.read_bytes() != expected_path.read_bytes():
            raise RuntimeError("whole mathematical output differs: " + reference_name)
        compared.append({"output": str(path), "reference": str(expected_path), "digest": digest(path)})
    write(OUT / (tag + "-REPLAY_MATCH.json"), {"whole_byte_matches": compared})
    write(OUT / (tag + "-POST_CALL_MANIFEST.json"), guard())


status = 1
try:
    installed_tools, expected_outputs = initialize()
    temp = OUT / "compiler-temp"
    temp.mkdir()
    ENV["TMPDIR"] = str(temp)
    replay("C", installed_tools, expected_outputs)
    replay("E", installed_tools, expected_outputs)
    status = 0
except BaseException:
    with (OUT / "driver-failure.stderr").open("w") as failure:
        traceback.print_exc(file=failure)
    raise
finally:
    failures = []
    try:
        guard()
    except Exception as error:
        failures.append(str(error))
    write(OUT / "FINAL.json",
          {"recorded": now(), "outer_exit": status, "guard_failures": failures,
           "completed_phase_records": PHASES,
           "originals": {str(p.relative_to(OUT)): digest(p) for p in OUT.rglob("*") if p.is_file()},
           "qualification": "Producer/CI replay only; never replaces Cloud originals or independently verifies either PCC claim"})
    if failures:
        raise RuntimeError("final portable custody guard failed: " + repr(failures))
