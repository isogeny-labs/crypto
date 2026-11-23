#!/usr/bin/env python3
"""Metadata-only preparation/provenance; all identity evaluations run in Rust."""
from pathlib import Path
from datetime import datetime, timezone
import hashlib
import json
import os
import platform
import shutil
import signal
import subprocess
import tarfile
import tomllib

HERE = Path(__file__).resolve().parent
WORK = HERE.parents[1]
SNAPSHOT = WORK / "vitse-current-root"
SCRATCH = WORK.parent
BORROWED_CARGO_HOME = SCRATCH / "tmp/vitse-root-gate/cargo-home"
TOOLCHAIN = SCRATCH / "tmp/vitse-rust-toolchain/installed/bin"
REVIEW = SCRATCH / "tmp/vitse_math_review.md"
BASE = "7d61e3b45de6e93d235345a0061dae08db1f8b44"
IDENTITY_BLOB = "114f1f4a3f2a1be027a9b046ef166259df49dd45"
SHA256_BLOB = "9566e3de448e21ff98811cfc4c6992ff3448962e"
PKM_BLOB = "9a801f27ae779cd35a6ee21a80252922324a9089"
REVIEW_SHA = "30bbfda953e0ec1cecf044a96f849ef823f6e8831f963b4b79b717b884d5c159"
REGISTRY = "index.crates.io-1949cf8c6b5b557f"


def utc():
    return datetime.now(timezone.utc).isoformat()


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def blob(raw):
    return hashlib.sha1(b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest()


def pin(path):
    raw = path.read_bytes()
    return {"path": str(path), "bytes": len(raw), "sha256": sha(raw)}


def write_bytes_new(path, raw):
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("xb") as stream:
        stream.write(raw)


def write_json_new(path, value):
    write_bytes_new(path, (json.dumps(value, indent=2, sort_keys=True) + "\n").encode())


def ensure_source_bytes(path, raw):
    """Resume a preparation-only failure without rewriting existing source bytes."""
    if path.exists():
        if path.read_bytes() != raw:
            raise RuntimeError("Refusing to replace a different source copy: " + str(path))
    else:
        write_bytes_new(path, raw)


def tree_pin(path):
    records = []
    for file in sorted(path.rglob("*")):
        if file.is_symlink():
            raise RuntimeError("Symlink in dependency source closure: " + str(file))
        if file.is_file():
            data = pin(file)
            data["path"] = str(file.relative_to(path))
            records.append(data)
    serialized = json.dumps(records, sort_keys=True, separators=(",", ":")).encode()
    return {"root": str(path), "tree_sha256": sha(serialized), "files": records}


def cargo_configs():
    candidates = [HERE, *HERE.parents]
    candidates = [p / ".cargo" / n for p in candidates for n in ("config", "config.toml")]
    candidates += [HERE / "cargo-home" / n for n in ("config", "config.toml")]
    records = []
    for p in candidates:
        if not p.is_file():
            continue
        data = tomllib.loads(p.read_text())
        if data.get("source") or data.get("patch") or data.get("replace"):
            raise RuntimeError("Cargo source override is forbidden: " + str(p))
        build = data.get("build", {})
        if any(k in build for k in ("rustc", "rustc-wrapper", "rustc-workspace-wrapper")):
            raise RuntimeError("Cargo compiler override is forbidden: " + str(p))
        records.append(pin(p))
    return records


def call(command, stdout, stderr, timeout, env):
    with stdout.open("xb") as out, stderr.open("xb") as err:
        process = subprocess.Popen(command, cwd=HERE, stdout=out, stderr=err,
                                   env=env, start_new_session=True)
        try:
            code = process.wait(timeout=timeout)
            return {"exit_code": code, "timed_out": False}
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            process.wait(timeout=10)
            return {"exit_code": 124, "timed_out": True}


def prepare_sources():
    source = HERE / "source"
    source.mkdir(exist_ok=True)
    originals = [
        (SNAPSHOT / "src/cryptanalysis/identity_certificate.rs", "identity_certificate.rs", IDENTITY_BLOB),
        (SNAPSHOT / "src/hash/sha256.rs", "sha256.rs", SHA256_BLOB),
    ]
    records = []
    for original, name, expected in originals:
        raw = original.read_bytes()
        if blob(raw) != expected:
            raise RuntimeError("Wrong production source Git blob: " + str(original))
        copy = source / name
        ensure_source_bytes(copy, raw)
        records.append({"original": str(original), "copy": pin(copy), "git_blob": expected,
                        "whole_file": True, "rewritten": False})
    review_raw = REVIEW.read_bytes()
    if sha(review_raw) != REVIEW_SHA:
        raise RuntimeError("Mathematical-review source witness changed")
    ensure_source_bytes(source / "MATHEMATICAL_REVIEW.input.md", review_raw)
    pkm = HERE.parent / "source/pkm_smooth_tower.rs"
    pkm_raw = pkm.read_bytes()
    if blob(pkm_raw) != PKM_BLOB:
        raise RuntimeError("Wrong S3 normalization source Git blob")
    start = pkm_raw.index(b"pub fn s3(e: &Curve, x1: u64, x2: u64, x3: u64) -> u64 {")
    end = pkm_raw.index(b"\n}", start) + 2
    ensure_source_bytes(source / "s3-normalization.rs.txt", pkm_raw[start:end])
    records.append({"source": pin(pkm), "git_blob": PKM_BLOB, "item": "s3",
                    "start_byte": start, "end_byte_exclusive": end,
                    "selected": pin(source / "s3-normalization.rs.txt"), "rewritten": False})
    baseline_lock = (SNAPSHOT / "Cargo.lock").read_bytes()
    ensure_source_bytes(source / "BASELINE.Cargo.lock", baseline_lock)
    return records, baseline_lock


def prepare_dependencies(baseline_lock):
    packages = tomllib.loads(baseline_lock.decode())["package"]
    keyed = {(p["name"], p["version"]): p for p in packages}
    wanted = {}
    pending = [("hex", "0.4.3"), ("serde", "1.0.229"), ("serde_json", "1.0.151")]
    while pending:
        key = pending.pop()
        if key in wanted:
            continue
        package = keyed[key]
        wanted[key] = package
        for item in package.get("dependencies", []):
            words = item.split()
            matches = [k for k in keyed if k[0] == words[0] and (len(words) == 1 or k[1] == words[1])]
            if len(matches) != 1:
                raise RuntimeError("Ambiguous or missing baseline dependency: " + item)
            pending.append(matches[0])
    parts = baseline_lock.decode().split("[[package]]")
    selected_blocks = []
    for text in parts[1:]:
        package = tomllib.loads("[[package]]" + text)["package"][0]
        if (package["name"], package["version"]) in wanted:
            selected_blocks.append("[[package]]" + text)
    assert len(selected_blocks) == len(wanted)
    project = '\n[[package]]\nname = "vitse-prime31-identity"\nversion = "0.1.0"\ndependencies = [\n "hex",\n "serde",\n "serde_json",\n]\n'
    ensure_source_bytes(HERE / "Cargo.lock", (parts[0] + "".join(selected_blocks) + project).encode())
    home = HERE / "cargo-home"
    home.mkdir(exist_ok=True)
    old_registry = BORROWED_CARGO_HOME / "registry"
    new_registry = home / "registry"
    old_index = old_registry / "index" / REGISTRY
    new_index = new_registry / "index" / REGISTRY
    index_pins = []
    ensure_source_bytes(new_index / "config.json", (old_index / "config.json").read_bytes())
    index_pins.append(pin(new_index / "config.json"))
    records = []
    for (name, version), package in sorted(wanted.items()):
        if package.get("source") != "registry+https://github.com/rust-lang/crates.io-index":
            raise RuntimeError("Non-registry dependency in pinned closure")
        stem = name + "-" + version
        archive = new_registry / "cache" / REGISTRY / (stem + ".crate")
        raw = (old_registry / "cache" / REGISTRY / (stem + ".crate")).read_bytes()
        if sha(raw) != package["checksum"]:
            raise RuntimeError("Registry archive checksum mismatch: " + stem)
        ensure_source_bytes(archive, raw)
        source = new_registry / "src" / REGISTRY / stem
        if not source.exists():
            shutil.copytree(old_registry / "src" / REGISTRY / stem, source)
        archived_files = set()
        with tarfile.open(archive, "r:gz") as tar:
            for member in tar.getmembers():
                if member.isdir():
                    continue
                parts = Path(member.name).parts
                if not member.isfile() or not parts or parts[0] != stem or ".." in parts:
                    raise RuntimeError("Unexpected registry archive entry: " + member.name)
                relative = Path(*parts[1:])
                archived_files.add(str(relative))
                if (source / relative).read_bytes() != tar.extractfile(member).read():
                    raise RuntimeError("Copied dependency differs from locked archive: " + member.name)
        extra = {str(p.relative_to(source)) for p in source.rglob("*") if p.is_file()} - archived_files
        if extra - {".cargo-checksum.json", ".cargo-ok"}:
            raise RuntimeError("Unexpected extra dependency source files: " + repr(extra))
        checksum_metadata = source / ".cargo-checksum.json"
        if checksum_metadata.exists():
            metadata = json.loads(checksum_metadata.read_text())
            if metadata["package"] != package["checksum"]:
                raise RuntimeError("Cargo source checksum metadata mismatch: " + stem)
        if len(name) == 1:
            index_name = "1/" + name
        elif len(name) == 2:
            index_name = "2/" + name
        elif len(name) == 3:
            index_name = "3/" + name[0] + "/" + name
        else:
            index_name = name[:2] + "/" + name[2:4] + "/" + name
        index_file = new_index / ".cache" / index_name
        ensure_source_bytes(index_file, (old_index / ".cache" / index_name).read_bytes())
        index_pins.append(pin(index_file))
        records.append({"baseline_package_record": package, "archive": pin(archive),
                        "source": tree_pin(source), "archive_source_equality": True,
                        "optional_cargo_checksum_metadata_present": checksum_metadata.exists()})
    return records, index_pins


def main():
    evidence = HERE.parent / "evidence/identity-01"
    if evidence.exists():
        if not (evidence / "preparation-01.failure.json").is_file():
            raise RuntimeError("Evidence exists without the recorded preparation-only failure")
        if (evidence / "source-build.json").exists() or (evidence / "dependency-source-manifest.json").exists():
            raise RuntimeError("Refusing to repeat a frozen qualification")
    else:
        evidence.mkdir(parents=True, exist_ok=False)
    source_records, baseline_lock = prepare_sources()
    dependencies, index_pins = prepare_dependencies(baseline_lock)
    dependency_manifest = evidence / "dependency-source-manifest.json"
    write_json_new(dependency_manifest, {"packages": dependencies, "index_metadata": index_pins})
    configs = cargo_configs()
    forbidden_env = [name for name in os.environ if name.startswith("CARGO_SOURCE_") or name in {
        "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS",
        "CARGO_BUILD_RUSTC", "CARGO_BUILD_RUSTC_WRAPPER", "CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER"}]
    if forbidden_env:
        raise RuntimeError("Inherited Cargo/compiler override variables: " + repr(forbidden_env))
    rustc = (TOOLCHAIN / "rustc").resolve()
    cargo = (TOOLCHAIN / "cargo").resolve()
    env = os.environ.copy()
    explicit_env = {
        "CARGO_HOME": str(HERE / "cargo-home"), "CARGO_TARGET_DIR": str(HERE / "target"),
        "RUSTC": str(rustc), "RUSTDOC": str(TOOLCHAIN / "rustdoc"),
        "PATH": str(TOOLCHAIN) + os.pathsep + env.get("PATH", ""), "LC_ALL": "C",
        "CARGO_NET_OFFLINE": "true", "CARGO_INCREMENTAL": "0",
    }
    env.update(explicit_env)
    source_files = [HERE / "IDENTITY_PROTOCOL.md", HERE / "Cargo.toml", HERE / "Cargo.lock",
                    HERE / "qualify_identity.py", dependency_manifest]
    source_files += sorted((HERE / "src").rglob("*.rs"))
    source_files += sorted(p for p in (HERE / "source").iterdir() if p.is_file())
    source_pins = [pin(p) for p in source_files]
    compiler_version = subprocess.check_output([str(rustc), "-Vv"], env=env, timeout=10, text=True)
    cargo_version = subprocess.check_output([str(cargo), "-V"], env=env, timeout=10, text=True)
    source_build = evidence / "source-build.json"
    write_json_new(source_build, {
        "schema": "vitse-prime31.identity-source-build/v1", "at": utc(), "source_base": BASE,
        "production_sources": source_records, "sources": source_pins,
        "preparation_failure_records": [pin(p) for p in sorted(evidence.glob("preparation-01.*"))],
        "dependency_manifest": pin(dependency_manifest), "dependency_count": len(dependencies),
        "cargo_configs": configs, "cargo_source_override": False, "network": "offline",
        "compiler": {"file": pin(rustc), "version": compiler_version},
        "cargo": {"file": pin(cargo), "version": cargo_version},
        "explicit_process_environment": explicit_env,
        "platform": platform.platform(), "machine": platform.machine(),
        "isolation": "shared development container; same-team correctness qualification only",
        "performance_measurement": False, "formal_proof": False,
    })

    def closure_unchanged():
        if [pin(p) for p in source_files] != source_pins or cargo_configs() != configs:
            return False
        if [pin(Path(p["path"])) for p in index_pins] != index_pins:
            return False
        for dep in dependencies:
            if pin(Path(dep["archive"]["path"])) != dep["archive"]:
                return False
            if tree_pin(Path(dep["source"]["root"])) != dep["source"]:
                return False
        return True

    command = [str(cargo), "build", "--release", "--frozen", "--offline", "--locked", "--bins"]
    write_json_new(evidence / "build-request.json", {
        "at": utc(), "command": command, "cwd": str(HERE), "timeout_seconds": 180,
        "source_build": pin(source_build), "source_closure_unchanged": closure_unchanged(),
    })
    result = call(command, evidence / "build.stdout", evidence / "build.stderr", 180, env)
    unchanged = closure_unchanged()
    write_json_new(evidence / "build-result.json", {"at": utc(), **result,
        "source_closure_unchanged": unchanged, "stdout": pin(evidence / "build.stdout"),
        "stderr": pin(evidence / "build.stderr")})
    if result["exit_code"] != 0 or not unchanged:
        raise SystemExit(result["exit_code"] or 1)

    certificate = HERE / "identity.certificate.json"
    for stage_name, binary_name in [("issuer", "issue_identity"), ("checker", "check_identity")]:
        stage = evidence / stage_name
        stage.mkdir(exist_ok=False)
        built = HERE / "target/release" / binary_name
        frozen = stage / "frozen-executable"
        write_bytes_new(frozen, built.read_bytes())
        frozen.chmod(0o555)
        certificate_before = pin(certificate) if certificate.exists() else None
        if (stage_name == "issuer") != (certificate_before is None):
            raise RuntimeError("Certificate presence violates write-once call order")
        command = [str(frozen), str(certificate)]
        precall = {
            "at": utc(), "command": command, "cwd": str(HERE), "timeout_seconds": 60,
            "source_build": pin(source_build), "executable": pin(frozen),
            "build_output": pin(built), "source_closure_unchanged": closure_unchanged(),
            "sources_before_call": [pin(p) for p in source_files], "certificate_before": certificate_before,
        }
        write_json_new(stage / "precall.json", precall)
        if not precall["source_closure_unchanged"]:
            raise RuntimeError("Source closure changed before native call")
        result = call(command, stage / "stdout.txt", stage / "stderr.txt", 60, env)
        certificate_after = pin(certificate) if certificate.exists() else None
        postcall = {
            "at": utc(), **result, "source_build": pin(source_build), "precall": pin(stage / "precall.json"),
            "stdout": pin(stage / "stdout.txt"), "stderr": pin(stage / "stderr.txt"),
            "executable": pin(frozen), "executable_unchanged": pin(frozen) == precall["executable"],
            "source_closure_unchanged": closure_unchanged(), "sources_after_call": [pin(p) for p in source_files],
            "certificate_after": certificate_after,
            "certificate_unchanged": certificate_before is None or certificate_before == certificate_after,
        }
        write_json_new(stage / "postcall.json", postcall)
        if result["exit_code"] != 0 or not all(postcall[k] for k in
                ("executable_unchanged", "source_closure_unchanged", "certificate_unchanged")):
            raise SystemExit(result["exit_code"] or 1)
        output = json.loads((stage / "stdout.txt").read_text())
        if output.get("status") != "pass":
            raise RuntimeError("Native call did not report passing controls")
    cert = json.loads(certificate.read_text())
    summary = {"schema": "vitse-prime31.identity-qualification/v1", "status": "pass",
               "statement_id": cert["id"], "certificate": pin(certificate), "source_build": pin(source_build),
               "issuer_postcall": pin(evidence / "issuer/postcall.json"),
               "checker_postcall": pin(evidence / "checker/postcall.json"),
               "proof": None, "independent_verification": False,
               "scope": "Same-team deterministic finite-evaluation record over F_(2^61-1)."}
    write_json_new(evidence / "summary.json", summary)
    print(json.dumps(summary, sort_keys=True))


if __name__ == "__main__":
    main()
