#!/usr/bin/env python3
"""Metadata-only test-closure runner; tests run in the unchanged Rust modules."""
from pathlib import Path
import json
import os
import runpy
import subprocess

HERE = Path(__file__).resolve().parent
STUDY = HERE.parents[1]
IDENTITY = STUDY / "identity"
ROOT = STUDY.parent / "vitse-current-root"
BASE = "7d61e3b45de6e93d235345a0061dae08db1f8b44"
FIXTURE_REL = "docs/identity-certificates/schoenhage-10mult.json"
FIXTURE_BLOB = "8a17c8270bbb6c2ffee6c3cddf6ccd5c2093586e"
FIXTURE_SHA = "84a61d6ccc237b724695e8f1416e9ad48f24c899ae316512bf0b952e46edb293"


def main():
    helpers = runpy.run_path(str(IDENTITY / "qualify_identity.py"))
    pin = helpers["pin"]
    sha = helpers["sha"]
    utc = helpers["utc"]
    write_new = helpers["write_json_new"]
    write_bytes = helpers["write_bytes_new"]
    call = helpers["call"]
    tree_pin = helpers["tree_pin"]
    old_evidence = STUDY / "evidence/identity-01"
    old_manifest_path = old_evidence / "source-build.json"
    old_manifest = json.loads(old_manifest_path.read_text())
    dependencies = json.loads((old_evidence / "dependency-source-manifest.json").read_text())
    fixture = STUDY / FIXTURE_REL
    # Read the actual baseline object; do not trust a mutable checkout copy.
    object_id = subprocess.check_output(
        ["git", "rev-parse", BASE + ":" + FIXTURE_REL], cwd=ROOT, text=True, timeout=10).strip()
    if object_id != FIXTURE_BLOB:
        raise RuntimeError("Unexpected baseline fixture object")
    raw = subprocess.check_output(["git", "cat-file", "blob", object_id], cwd=ROOT, timeout=10)
    if helpers["blob"](raw) != FIXTURE_BLOB or sha(raw) != FIXTURE_SHA:
        raise RuntimeError("Fixture object bytes do not match the frozen protocol")
    write_bytes(fixture, raw)
    certificate = IDENTITY / "identity.certificate.json"
    certificate_pin = pin(certificate)
    new_source_files = [HERE / "PROTOCOL.md", HERE / "qualify_tests.py", fixture,
                        old_manifest_path, old_evidence / "summary.json"]
    new_source_pins = [pin(p) for p in new_source_files]

    def closure_unchanged():
        if [pin(p) for p in new_source_files] != new_source_pins:
            return False
        if pin(certificate) != certificate_pin:
            return False
        if helpers["cargo_configs"]() != old_manifest["cargo_configs"]:
            return False
        for item in old_manifest["sources"]:
            if pin(Path(item["path"])) != item:
                return False
        for item in dependencies["index_metadata"]:
            if pin(Path(item["path"])) != item:
                return False
        for item in dependencies["packages"]:
            if pin(Path(item["archive"]["path"])) != item["archive"]:
                return False
            if tree_pin(Path(item["source"]["root"])) != item["source"]:
                return False
        for tool in ("compiler", "cargo"):
            if pin(Path(old_manifest[tool]["file"]["path"])) != old_manifest[tool]["file"]:
                return False
        return True

    forbidden = [k for k in os.environ if k.startswith("CARGO_SOURCE_") or k in {
        "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER", "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS",
        "CARGO_BUILD_RUSTC", "CARGO_BUILD_RUSTC_WRAPPER", "CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER"}]
    if forbidden or not closure_unchanged():
        raise RuntimeError("Inherited overrides or changed frozen source/dependency closure")
    env = os.environ.copy()
    env.update(old_manifest["explicit_process_environment"])
    rustc = old_manifest["compiler"]["file"]["path"]
    cargo = old_manifest["cargo"]["file"]["path"]
    source_build = HERE / "source-build.json"
    write_new(source_build, {
        "schema": "vitse-prime31.identity-test-source-build/v1", "at": utc(),
        "source_base": BASE, "fixture": pin(fixture), "fixture_git_blob": object_id,
        "fixture_origin": BASE + ":" + FIXTURE_REL,
        "prior_source_build": pin(old_manifest_path), "new_sources": new_source_pins,
        "prior_sources": old_manifest["sources"], "dependency_manifest": old_manifest["dependency_manifest"],
        "compiler": {"file": pin(Path(rustc)), "version": subprocess.check_output([rustc, "-Vv"], env=env, timeout=10, text=True)},
        "cargo": {"file": pin(Path(cargo)), "version": subprocess.check_output([cargo, "-V"], env=env, timeout=10, text=True)},
        "explicit_process_environment": old_manifest["explicit_process_environment"],
        "cargo_configs": helpers["cargo_configs"](), "source_closure_unchanged": True,
        "source_override": False, "performance_measurement": False,
        "scope": "New test-closure qualification after adding the exact missing baseline fixture; unchanged production tests.",
    })
    common = [cargo, "test", "--release", "--manifest-path", str(IDENTITY / "Cargo.toml"),
              "--frozen", "--offline", "--locked"]
    command = common + ["--no-run", "--message-format=json"]
    write_new(HERE / "build-request.json", {"at": utc(), "command": command, "cwd": str(IDENTITY),
              "timeout_seconds": 180, "source_build": pin(source_build), "source_closure_unchanged": closure_unchanged()})
    result = call(command, HERE / "build.stdout", HERE / "build.stderr", 180, env)
    unchanged = closure_unchanged()
    write_new(HERE / "build-result.json", {"at": utc(), **result, "source_closure_unchanged": unchanged,
              "stdout": pin(HERE / "build.stdout"), "stderr": pin(HERE / "build.stderr")})
    if result["exit_code"] != 0 or not unchanged:
        raise SystemExit(result["exit_code"] or 1)
    artifacts = []
    for line in (HERE / "build.stdout").read_text().splitlines():
        item = json.loads(line)
        if item.get("reason") == "compiler-artifact" and item.get("profile", {}).get("test") and item.get("executable"):
            artifacts.append(item)
    if not artifacts:
        raise RuntimeError("Cargo reported no test executables")
    binaries = []
    for index, item in enumerate(sorted(artifacts, key=lambda x: x["target"]["name"])):
        executable = Path(item["executable"])
        frozen = HERE / "test-binaries" / (str(index) + "-" + item["target"]["name"])
        write_bytes(frozen, executable.read_bytes())
        frozen.chmod(0o555)
        binaries.append({"target": item["target"], "actual": pin(executable), "frozen": pin(frozen)})
    command = common + ["--", "--test-threads=1"]
    precall = {"at": utc(), "command": command, "cwd": str(IDENTITY), "timeout_seconds": 60,
               "source_build": pin(source_build), "test_binaries": binaries,
               "source_closure_unchanged": closure_unchanged(), "certificate_before": certificate_pin}
    write_new(HERE / "precall.json", precall)
    if not precall["source_closure_unchanged"]:
        raise RuntimeError("Source closure changed before the test call")
    result = call(command, HERE / "stdout.txt", HERE / "stderr.txt", 60, env)
    binaries_after = [{"target": x["target"], "actual": pin(Path(x["actual"]["path"])),
                       "frozen": pin(Path(x["frozen"]["path"]))} for x in binaries]
    postcall = {"at": utc(), **result, "precall": pin(HERE / "precall.json"),
                "source_build": pin(source_build), "source_closure_unchanged": closure_unchanged(),
                "test_binaries": binaries_after, "test_binaries_unchanged": binaries_after == binaries,
                "stdout": pin(HERE / "stdout.txt"), "stderr": pin(HERE / "stderr.txt"),
                "certificate_after": pin(certificate), "certificate_unchanged": pin(certificate) == certificate_pin}
    write_new(HERE / "postcall.json", postcall)
    success = result["exit_code"] == 0 and not result["timed_out"] and all(postcall[k] for k in
              ("source_closure_unchanged", "test_binaries_unchanged", "certificate_unchanged"))
    summary = {"schema": "vitse-prime31.identity-test-qualification/v1", "status": "pass" if success else "fail",
               "fixture": pin(fixture), "fixture_git_blob": object_id, "source_build": pin(source_build),
               "postcall": pin(HERE / "postcall.json"), "test_executable_count": len(binaries),
               "scope": "New standalone production-module test-closure qualification; no prior pass implied."}
    write_new(HERE / "summary.json", summary)
    print(json.dumps(summary, sort_keys=True))
    raise SystemExit(0 if success else result["exit_code"] or 1)


if __name__ == "__main__":
    main()
