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
import tarfile
import urllib.request
from pathlib import PurePosixPath
import traceback

STUDY = "research/chained_cubic_coordinate_20261127"
SOURCE = "82852f5636edfbae2e9c4f1323450da75c0c0176"
REFERENCE = "be0916263876ff77ee364578e1c29176b443c262"
ROOT = Path(__file__).resolve().parents[2]
OUT = Path(sys.argv[1]).resolve()
OUT.mkdir(parents=True, exist_ok=False)
TOOLS, INPUTS, BUILDS, PHASES = {}, {}, {}, []
ENV = os.environ.copy()
ENV.update({"RAYON_NUM_THREADS": "1", "CARGO_BUILD_JOBS": "1",
            "CHECKPOINT_SKIP": "Distinct portable replay of fixed integer-lift canonical identity J; no new cohort, timing, scaling, signature or performance admission."})
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
    lock_path = STUDY + "/CI_IDENTITY_LOCK_K.json"
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

    frozen_path = STUDY + "/SOURCE_LOCK_J.json"
    raw = git_blob(SOURCE, frozen_path)
    frozen = json.loads(raw)
    for path, expected in frozen["files"].items():
        blob = git_blob(SOURCE, path)
        for destination in ["original-J", "source-J"]:
            install_input(OUT / destination / path, blob, expected, expected["mode"])
    for destination in ["original-J", "source-J"]:
        install_input(OUT / destination / frozen_path, raw,
                      {"bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()}, 0o600)

    toolchain = Path(os.environ.get("RUSTUP_HOME", str(Path.home() / ".rustup"))) / "toolchains/1.99.1-x86_64-unknown-linux-gnu"
    tools = {}
    for old_path, expected in frozen["tools"].items():
        if "/.rustup/toolchains/" not in old_path:
            continue
        path = toolchain / "bin" / Path(old_path).name
        if digest(path)["sha256"] != expected["sha256"]:
            raise RuntimeError("official compiler tool bytes differ: " + str(path))
        TOOLS[str(path)] = digest(path)
        tools[path.name] = path
    for old_path, expected in frozen["runtime"].items():
        if "/.rustup/toolchains/" not in old_path:
            continue
        relative = old_path.split("/.rustup/toolchains/", 1)[1].split("/", 1)[1]
        path = toolchain / relative
        if digest(path)["sha256"] != expected["sha256"]:
            raise RuntimeError("official Rust runtime differs: " + relative)
        TOOLS[str(path)] = digest(path)

    dependency_record = json.loads((OUT / "source-J" / STUDY / "DEPENDENCY_INPUTS_H.json").read_text())
    vendor = OUT / "vendor"
    archives = OUT / "official-archives"
    archives.mkdir()
    acquired = []
    for package in dependency_record["packages"]:
        name = package["package"] + "-" + package["version"]
        with urllib.request.urlopen(package["url"], timeout=60) as response:
            raw = response.read()
        expected = {"bytes": package["whole_bytes"], "sha256": package["sha256"]}
        archive_path = archives / (name + ".crate")
        install_input(archive_path, raw, expected, 0o600)
        checksum_files = {}
        with tarfile.open(archive_path, "r:gz") as archive:
            for member in archive.getmembers():
                parts = PurePosixPath(member.name).parts
                if not parts or parts[0] != name or ".." in parts or member.name.startswith("/"):
                    raise RuntimeError("invalid official archive path")
                if member.isdir():
                    continue
                if not member.isfile():
                    raise RuntimeError("unexpected non-file official archive member")
                relative = PurePosixPath(*parts[1:]).as_posix()
                path = vendor / name / relative
                body = archive.extractfile(member).read()
                install_input(path, body, {"bytes": len(body),
                              "sha256": hashlib.sha256(body).hexdigest()}, member.mode & 0o777)
                checksum_files[relative] = hashlib.sha256(body).hexdigest()
        checksum = vendor / name / ".cargo-checksum.json"
        checksum.write_text(json.dumps({"package": package["sha256"], "files": checksum_files},
                                       sort_keys=True) + "\n")
        INPUTS[str(checksum)] = digest(checksum)
        acquired.append({"name": name, "url": package["url"],
                         "whole_archive": digest(archive_path), "source_files": len(checksum_files)})
    write(OUT / "WHOLE_PUBLIC_DEPENDENCIES.json", acquired)
    cargo_home = OUT / "cargo-home"
    cargo_home.mkdir()
    config = cargo_home / "config.toml"
    config.write_text('[source.crates-io]\nreplace-with = "vendored-sources"\n'
                      '[source.vendored-sources]\ndirectory = ' + json.dumps(str(vendor)) + "\n")
    INPUTS[str(config)] = digest(config)
    temp = OUT / "compiler-temp"
    temp.mkdir()
    ENV.update({"RUSTUP_TOOLCHAIN": "1.99.1", "RUSTC": str(tools["rustc"]),
                "CARGO_HOME": str(cargo_home), "TMPDIR": str(temp),
                "PATH": str(toolchain / "bin") + ":" + ENV["PATH"],
                "RUSTFLAGS": "-Dwarnings -Copt-level=3 -Coverflow-checks=yes -Cdebug-assertions=yes -Cdebuginfo=0 -Ccodegen-units=1"})

    references = {}
    for name in ["actual-integer-lift-identity.stdout", "data/identity.certificate.json",
                 "data/integer_lifts.json", "data/statement.canonical.json"]:
        path = STUDY + "/first-J-native/" + name
        raw = git_blob(REFERENCE, path)
        dest = OUT / "reference-results" / path
        install_input(dest, raw, {"bytes": len(raw), "sha256": hashlib.sha256(raw).hexdigest()}, 0o600)
        references[name] = dest
    write(OUT / "CONTEXT.json",
          {"schema": "chained-portable-identity-replay/v1", "recorded": now(),
           "head": head, "source": SOURCE, "reference_results": REFERENCE,
           "host": platform.uname()._asdict(),
           "limits": {"phase_seconds": 120, "address_space_bytes": 2147483648, "workers": 1},
           "classification": ENV["CHECKPOINT_SKIP"],
           "boundary": "Distinct runtime with official compiler/Rust runtime and whole official crate sources pinned. Standard offline Cargo directory-source replacement; Cloud index/dispatcher not executed. OS ambient; no timing/independent signature.",
           "environment": {k: ENV[k] for k in ["RAYON_NUM_THREADS", "CARGO_BUILD_JOBS",
                           "CHECKPOINT_SKIP", "RUSTFLAGS", "RUSTC", "CARGO_HOME"]}})
    write(OUT / "BUILD_MANIFEST.json", guard())
    return tools, references


def replay(tools, references):
    source = OUT / "source-J" / STUDY / "identity_record_j"
    for tool in ["rustc", "cargo", "clippy-driver", "rustfmt"]:
        phase(tool + "-version", [str(tools[tool]), "--version"])
    phase("format", [str(tools["rustfmt"]), "--edition", "2021", "--config",
                     "skip_children=true", "--check", str(source / "src/main.rs"),
                     str(source / "inputs/canonical_sha256_rust199.rs")])
    target = OUT / "target"
    common = ["--manifest-path", str(source / "Cargo.toml"), "--release",
              "--locked", "--offline", "--verbose", "--jobs", "1", "--target-dir", str(target)]
    phase("compile", [str(tools["cargo"]), "build", *common])
    built = target / "release/chained-cubic-identity-record-j"
    BUILDS[str(built)] = digest(built)
    phase("clippy", [str(tools["cargo"]), "clippy", *common, "--", "-Dwarnings"])
    for path in target.rglob("*"):
        if path.is_file():
            BUILDS[str(path)] = digest(path)
    qualifier = OUT / "qualifier"
    shutil.copyfile(built, qualifier)
    qualifier.chmod(0o555)
    if digest(built)["sha256"] != digest(qualifier)["sha256"]:
        raise RuntimeError("separate whole qualifier differs")
    BUILDS[str(qualifier)] = digest(qualifier)
    write(OUT / "PRE_CALL_MANIFEST.json", guard())
    phase("actual-integer-lift-identity", [str(qualifier), str(OUT / "data")])
    pairs = [(OUT / "actual-integer-lift-identity.stdout", "actual-integer-lift-identity.stdout")]
    pairs += [(OUT / "data" / name, "data/" + name) for name in [
        "identity.certificate.json", "integer_lifts.json", "statement.canonical.json"]]
    compared = []
    for path, reference in pairs:
        if path.read_bytes() != references[reference].read_bytes():
            raise RuntimeError("whole canonical output differs: " + reference)
        compared.append({"output": str(path), "reference": str(references[reference]),
                         "digest": digest(path)})
    certificate = json.loads((OUT / "data/identity.certificate.json").read_text())
    statement = (OUT / "data/statement.canonical.json").read_bytes()
    if certificate["id"] != "IDC1h" + hashlib.sha256(statement).hexdigest()[:16]:
        raise RuntimeError("canonical statement-file hash differs from id")
    write(OUT / "REPLAY_MATCH.json", {"whole_byte_matches": compared,
                                    "statement_id_file_hash_match": True})
    write(OUT / "POST_CALL_MANIFEST.json", guard())


status = 1
try:
    installed_tools, expected_outputs = initialize()
    replay(installed_tools, expected_outputs)
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
           "qualification": "Producer/CI replay only; never replaces Cloud originals or independently verifies a PCC claim"})
    if failures:
        raise RuntimeError("final portable custody guard failed: " + repr(failures))
