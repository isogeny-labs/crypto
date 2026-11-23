#!/usr/bin/env python3
"""Bundle existing Prime31 executables, exact sparse Git source and ELF metadata.

The only supplied-binary commands are source-manifest and source-check. This
program never builds code, executes cells, uploads, publishes images or admits a
launch. ELF inspection covers the current host's declared startup dependency
closure; it does not certify an OCI image, runtime dlopen or a future base image.
"""

import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import stat
import struct
import subprocess
import sys


class BundleError(RuntimeError):
    pass


def digest(data):
    return hashlib.sha256(data).hexdigest()


def unique_object(items):
    result = {}
    for key, value in items:
        if key in result:
            raise BundleError("duplicate JSON key")
        result[key] = value
    return result


def json_object(raw):
    def invalid_constant(_):
        raise BundleError("non-finite JSON value")
    try:
        result = json.loads(raw, object_pairs_hook=unique_object, parse_constant=invalid_constant)
    except (UnicodeError, ValueError) as error:
        raise BundleError("invalid JSON metadata") from error
    if not isinstance(result, dict):
        raise BundleError("JSON object required")
    return result


def metadata_environment():
    # Do not inherit alternate object stores, index files, config injections or
    # loader overrides. No environment values are copied into the artifact.
    result = {key: value for key, value in os.environ.items()
              if not key.startswith(("GIT_", "LD_")) and key != "GLIBC_TUNABLES"}
    result.update(LC_ALL="C", GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_SYSTEM="/dev/null",
                  GIT_CONFIG_NOSYSTEM="1", GIT_NO_REPLACE_OBJECTS="1", GIT_NO_LAZY_FETCH="1",
                  GIT_TERMINAL_PROMPT="0", GIT_OPTIONAL_LOCKS="0")
    return result


def command(arguments, *, input_bytes=None, timeout=60):
    try:
        result = subprocess.run([str(item) for item in arguments], input=input_bytes,
                                capture_output=True, env=metadata_environment(), timeout=timeout)
    except (OSError, subprocess.TimeoutExpired) as error:
        raise BundleError(f"metadata command unavailable or timed out: {Path(arguments[0]).name}") from error
    if result.returncode:
        # Avoid echoing arbitrary stderr or command arguments into durable metadata.
        raise BundleError(f"metadata command failed: {Path(arguments[0]).name}, exit {result.returncode}")
    return result.stdout


def tool(name):
    value = shutil.which(name)
    if value is None:
        raise BundleError(f"required tool unavailable: {name}")
    return Path(value).resolve(strict=True)


def git(repository, *arguments, input_bytes=None):
    return command([tool("git"), "-c", "core.hooksPath=/dev/null", "-c", "core.fsmonitor=false",
                    "-c", "core.autocrlf=false", "-c", "gc.auto=0", "-c", "maintenance.auto=false",
                    "-C", repository, *arguments], input_bytes=input_bytes)


def relative_path(value):
    if not isinstance(value, str) or not value:
        raise BundleError("repository-relative path required")
    path = PurePosixPath(value)
    if (not path.parts or path.is_absolute() or path.as_posix() != value
            or any(part in (".", "..", ".git") or not re.fullmatch(r"[A-Za-z0-9_.+-]+", part)
                   for part in path.parts)):
        raise BundleError("unsupported or unsafe repository-relative path")
    return value


def new_file(path, contents, mode=0o644):
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("xb") as destination:
        destination.write(contents)
    path.chmod(mode)


def new_json(path, value):
    new_file(path, (json.dumps(value, sort_keys=True, indent=2) + "\n").encode())


def fresh_directory(path):
    if not path.is_absolute() or path.exists() or path.is_symlink():
        raise BundleError("output must be a fresh absolute path, never an existing path or symlink")
    if not path.parent.is_dir():
        raise BundleError("output parent must already exist")
    path = path.parent.resolve(strict=True) / path.name
    path.mkdir(mode=0o700, exist_ok=False)
    return path


def revision_of(repository):
    revision = git(repository, "rev-parse", "HEAD").decode("ascii").strip()
    if not re.fullmatch(r"[0-9a-f]{40}", revision):
        raise BundleError("a full SHA-1 source revision is required by the native source-check interface")
    return revision


def head_blob(repository, revision, relative):
    relative_path(relative)
    tree = git(repository, "ls-tree", "-z", revision, "--", relative).split(b"\0")
    entries = [item for item in tree if item]
    if len(entries) != 1:
        raise BundleError("selected source path is not one exact tracked file")
    prefix, name = entries[0].split(b"\t", 1)
    mode, kind, object_id = prefix.decode("ascii").split()
    if name.decode("utf-8") != relative or kind != "blob" or mode not in ("100644", "100755"):
        raise BundleError("selected source must be a tracked regular file, not a symlink or submodule")
    content = git(repository, "cat-file", "blob", object_id)
    return content, object_id, mode


def clean_source(repository):
    if git(repository, "status", "--porcelain", "--untracked-files=all"):
        raise BundleError("source checkout must be clean; no source publication is inferred from this check")
    if git(repository, "rev-parse", "--is-shallow-repository").strip() != b"false":
        raise BundleError("shallow source cannot export the complete real ancestry; no automatic fetch is attempted")
    graft_path = Path(git(repository, "rev-parse", "--git-path", "info/grafts").decode().strip())
    if not graft_path.is_absolute():
        graft_path = repository / graft_path
    if graft_path.exists() and graft_path.stat().st_size:
        raise BundleError("source grafts are unsupported; actual ancestry must remain intact")


def inventory(manifest):
    if manifest.get("schema") != "prime31-aux-matched-source-manifest/v1":
        raise BundleError("unsupported native source-manifest schema")
    result = {}

    def add(entry, hash_field, size_field):
        if not isinstance(entry, dict):
            raise BundleError("source inventory entry must be an object")
        path = relative_path(entry.get("path"))
        value = (entry.get(hash_field), entry.get(size_field))
        if (not isinstance(value[0], str) or not re.fullmatch(r"[0-9a-f]{64}", value[0])
                or type(value[1]) is not int or value[1] < 0):
            raise BundleError("invalid source inventory hash or byte count")
        if path in result and result[path] != value:
            raise BundleError("conflicting native source inventory entries")
        result[path] = value

    def array(container, key):
        value = container.get(key)
        if not isinstance(value, list):
            raise BundleError("required source inventory array unavailable")
        return value

    for entry in array(manifest, "whole_files"):
        add(entry, "sha256", "bytes")
    for entry in array(manifest, "extracted_production_ranges"):
        if (entry.get("whole_source_sha256") != entry.get("pinned_whole_source_sha256")
                or entry.get("rewritten") is not False or entry.get("inserted_bytes") != 0):
            raise BundleError("source ranges are not pinned verbatim")
        add(entry, "whole_source_sha256", "whole_source_bytes")
    layout = manifest.get("layout_preserving_f4_copy")
    if not isinstance(layout, dict):
        raise BundleError("F4 source layout inventory unavailable")
    for entry in array(layout, "copied_files"):
        if entry.get("rewritten") is not False:
            raise BundleError("F4 source copy is not verbatim")
        add(entry, "source_and_copy_sha256", "bytes")
    if not result:
        raise BundleError("empty source inventory")
    return result


def export_source(repository, output, selected, revision):
    """Keep original commit/tree objects; include only selected HEAD file bodies."""
    clean_source(repository)
    if revision_of(repository) != revision:
        raise BundleError("source revision changed")
    blobs = {path: head_blob(repository, revision, path) for path in sorted(selected)}
    bundle = output / "source.bundle"
    git(repository, "bundle", "create", "--version=3", bundle, "--filter=blob:none", "HEAD")
    with bundle.open("rb") as stream:
        header = []
        for _ in range(32):
            line = stream.readline(4096)
            header.append(line)
            if line == b"\n":
                break
    if b"@filter=blob:none\n" not in header or header[0] != b"# v3 git bundle\n":
        raise BundleError("Git did not produce the required explicit blob-filtered bundle")
    source = output / "rootfs/opt/prime31/source"
    source.parent.mkdir(parents=True, exist_ok=True)
    command([tool("git"), "init", "--quiet", "--template=", "--object-format=sha1", source])
    git(source, "bundle", "verify", bundle)
    git(source, "bundle", "unbundle", bundle)
    git(source, "update-ref", "refs/heads/bundled", revision)
    git(source, "symbolic-ref", "HEAD", "refs/heads/bundled")
    git(source, "config", "core.sparseCheckout", "true")
    git(source, "config", "core.sparseCheckoutCone", "false")
    new_file(source / ".git/info/sparse-checkout",
             ("\n".join("/" + path for path in sorted(selected)) + "\n").encode())
    for path, (content, expected_object, _) in blobs.items():
        actual = git(source, "hash-object", "-w", "--stdin", input_bytes=content).decode().strip()
        if actual != expected_object:
            raise BundleError("selected blob object identity changed")
    git(source, "read-tree", "-m", "-u", "HEAD")
    if git(source, "status", "--porcelain", "--untracked-files=all"):
        raise BundleError("sanitized sparse checkout is not clean")
    if revision_of(source) != revision:
        raise BundleError("sanitized checkout lost the original commit identity")
    ancestry = git(repository, "rev-list", "--parents", revision)
    if git(source, "rev-list", "--parents", revision) != ancestry:
        raise BundleError("sanitized checkout did not preserve the original ancestry")
    materialized = set()
    for directory, directories, files in os.walk(source):
        if Path(directory) == source:
            directories[:] = [name for name in directories if name != ".git"]
        for name in files:
            materialized.add((Path(directory) / name).relative_to(source).as_posix())
    if materialized != set(selected):
        raise BundleError("sparse checkout materialized an undeclared file or omitted a selected path")
    for path, (content, _, _) in blobs.items():
        if (source / path).read_bytes() != content:
            raise BundleError("exported file bytes differ from HEAD")
    return source, {
        "source_revision": revision, "ancestry_sha256": digest(ancestry),
        "ancestor_commits_including_head": len(ancestry.splitlines()),
        "source_bundle_sha256": digest(bundle.read_bytes()),
        "git_object_coverage": "all_reachable_commits_and_trees_plus_selected_HEAD_blobs",
        "unselected_and_historical_blob_contents_included": False,
        "source_config_hooks_credentials_copied": False,
        "materialized_paths": sorted(selected),
    }


def elf_metadata(path):
    raw = path.read_bytes()
    if (len(raw) < 64 or raw[:4] != b"\x7fELF" or raw[4:6] != b"\x02\x01"
            or struct.unpack_from("<H", raw, 18)[0] != 62):
        raise BundleError("only existing ELF64 little-endian x86-64 executables/libraries are qualified")
    listing = command([tool("readelf"), "--wide", "--file-header", "--program-headers",
                       "--dynamic", "--version-info", path])
    text = listing.decode("utf-8")
    interpreters = re.findall(r"Requesting program interpreter: ([^\]\n]+)", text)
    if len(interpreters) > 1:
        raise BundleError("ambiguous ELF interpreter")
    needed = re.findall(r"\(NEEDED\).*Shared library: \[([^\]]+)\]", text)
    if any("/" in name for name in needed) or re.search(r"\((?:RPATH|RUNPATH)\)", text):
        raise BundleError("ELF RPATH/RUNPATH or slash-containing NEEDED requires separate relocation qualification")
    return {
        "sha256": digest(raw), "bytes": len(raw), "architecture": "linux/amd64",
        "interpreter": interpreters[0] if interpreters else None,
        "needed": needed,
        "required_glibc_versions": sorted(set(re.findall(r"Name: (GLIBC_[A-Za-z0-9_.]+)", text))),
        "readelf_sha256": digest(listing),
    }, listing


def inspect_runtime(executables, output):
    """Capture actual loader resolution without executing executable main/init."""
    host_git = tool("git")
    git_metadata, _ = elf_metadata(host_git)
    interpreter = git_metadata["interpreter"]
    if not interpreter:
        raise BundleError("trusted system Git does not identify a supported dynamic loader")
    trusted_loader = Path(interpreter).resolve(strict=True)
    loader_stat = trusted_loader.stat()
    # The container may map system-file ownership to uid 65534. Trust the same
    # interpreter used by the already selected host Git, not an assumed uid 0.
    if not stat.S_ISREG(loader_stat.st_mode) or loader_stat.st_mode & (stat.S_IWGRP | stat.S_IWOTH):
        raise BundleError("host Git interpreter is not a stable regular executable")
    all_executables = dict(executables)
    all_executables["git"] = host_git
    files = {}
    reports = {}

    def register(path, install_path):
        resolved = path.resolve(strict=True)
        if not resolved.is_file() or not install_path.is_absolute() or ".." in install_path.parts:
            raise BundleError("unsafe runtime file")
        content = resolved.read_bytes()
        key = install_path.as_posix()
        record = {"source_path": str(path), "resolved_source_path": str(resolved),
                  "sha256": digest(content), "bytes": len(content), "install_path": key}
        if key in files:
            if files[key]["sha256"] != record["sha256"]:
                raise BundleError("conflicting runtime install paths")
        else:
            destination = output / "rootfs" / key.lstrip("/")
            if not destination.exists():
                new_file(destination, content, 0o755)
            elif digest(destination.read_bytes()) != record["sha256"]:
                raise BundleError("preexisting runtime payload bytes differ")
            files[key] = record
        return resolved

    for name, executable in all_executables.items():
        metadata, listing = elf_metadata(executable)
        new_file(output / "metadata" / f"elf-{name}.txt", listing)
        install = Path("/usr/bin/git") if name == "git" else Path("/usr/local/bin") / name
        register(executable, install)
        resolved_dependencies = {}
        loader_listing = b""
        if metadata["interpreter"]:
            if Path(metadata["interpreter"]).resolve(strict=True) != trusted_loader:
                raise BundleError("supplied executable uses an unqualified interpreter; no musl/glibc assumption made")
            loader_listing = command([trusted_loader, "--list", executable])
            for line in loader_listing.decode("utf-8").splitlines():
                line = line.strip()
                if not line or re.fullmatch(r"linux-vdso\.so\.\d+ \(0x[0-9a-f]+\)", line):
                    continue
                match = re.fullmatch(r"([^\s]+) => (/[^\s]+) \(0x[0-9a-f]+\)", line)
                if match:
                    library, location = match.groups()
                else:
                    match = re.fullmatch(r"(/[^\s]+) \(0x[0-9a-f]+\)", line)
                    if not match:
                        raise BundleError("unresolved or unsupported dynamic-loader dependency line")
                    location = match.group(1)
                    library = Path(location).name
                if library in resolved_dependencies and resolved_dependencies[library] != location:
                    raise BundleError("ambiguous actual loader dependency mapping")
                resolved_dependencies[library] = location
                register(Path(location), Path(location))
            register(trusted_loader, Path(metadata["interpreter"]))
            for dependency in metadata["needed"]:
                if dependency not in resolved_dependencies:
                    raise BundleError("direct ELF dependency absent from loader resolution")
            for library, location in resolved_dependencies.items():
                child, child_listing = elf_metadata(Path(location))
                new_file(output / "metadata" / f"elf-{name}-dependency-{library}.txt", child_listing)
                for dependency in child["needed"]:
                    if dependency not in resolved_dependencies and dependency != trusted_loader.name:
                        raise BundleError("transitive ELF dependency absent from loader resolution")
        elif metadata["needed"]:
            raise BundleError("ELF has dynamic dependencies without an interpreter")
        new_file(output / "metadata" / f"loader-{name}.txt", loader_listing)
        metadata.update({"install_path": str(install), "resolved_dependencies": resolved_dependencies,
                         "loader_listing_sha256": digest(loader_listing),
                         "linkage": "dynamic" if metadata["interpreter"] else "static_elf_startup"})
        reports[name] = metadata
    result = {
        "scope": "observed_host_ELF_startup_dependency_closure",
        "executables": reports, "runtime_files": sorted(files.values(), key=lambda item: item["install_path"]),
        "inspection_tools": {
            "readelf": {"path": str(tool("readelf")), "sha256": digest(tool("readelf").read_bytes())},
            "host_git_interpreter": {"path": str(trusted_loader), "sha256": digest(trusted_loader.read_bytes())},
        },
        "loader_environment_overrides_removed": True,
        "executable_main_or_constructors_called_for_dependency_inspection": False,
        "container_runtime_qualified": False,
        "not_covered": ["runtime dlopen/NSS plugins and application data", "future image/base ABI compatibility",
                        "future entrypoint shell/timeout/taskset/uploader tools", "kernel and procfs behavior"],
    }
    new_json(output / "metadata/runtime-dependencies.json", result)
    return result


def source_metadata(binary, repository, revision, output, name, phase):
    raw_manifest = command([binary, "source-manifest"])
    manifest = json_object(raw_manifest)
    entries = inventory(manifest)
    raw_check = command([binary, "source-check", repository])
    check = json_object(raw_check)
    embedded = raw_manifest[:-1] if raw_manifest.endswith(b"\n") else raw_manifest
    binary_hash = digest(binary.read_bytes())
    if (check.get("schema") != "prime31-aux-matched.source-check/v1"
            or check.get("verified") is not True or check.get("source_revision") != revision
            or check.get("running_binary_sha256") != binary_hash
            or check.get("source_manifest_sha256") != digest(embedded)
            or check.get("all_inventoried_sources_present_in_head") is not True
            or check.get("checked_file_count") != len(entries)):
        raise BundleError("native source-check identity did not match supplied binary/source metadata")
    checked = check.get("checked_files")
    if not isinstance(checked, list) or len(checked) != len(entries):
        raise BundleError("native source-check file inventory missing")
    observed = {item.get("path"): (item.get("sha256"), item.get("bytes")) for item in checked}
    if observed != entries:
        raise BundleError("native source-check inventory differs from embedded source manifest")
    new_file(output / "metadata" / f"{name}-{phase}-source-manifest.json", embedded)
    new_file(output / "metadata" / f"{name}-{phase}-source-check.json", raw_check)
    return entries, binary_hash, digest(embedded)


def construct(args):
    repository = Path(args.repository).resolve(strict=True)
    if Path(git(repository, "rev-parse", "--show-toplevel").decode().strip()).resolve() != repository:
        raise BundleError("--repository must be the checkout root")
    clean_source(repository)
    revision = revision_of(repository)
    destination = Path(args.output)
    if not destination.is_absolute():
        raise BundleError("absolute output path required")
    if destination.resolve(strict=False).is_relative_to(repository):
        raise BundleError("bundle output must be outside the source checkout")
    selected = set()
    native_expected = {}
    for name, value in (("prime31-isogeny-choices", args.choice_binary),
                        ("prime31-ic-incumbent", args.incumbent_binary)):
        path = Path(value)
        if not path.is_absolute() or path.is_symlink() or not path.is_file() or not os.access(path, os.X_OK):
            raise BundleError("supplied binaries must be existing absolute regular executable files")
        elf_metadata(path)
        native_expected[name] = (path.resolve(strict=True), digest(path.read_bytes()))
    data = {}
    for path in [args.protocol, args.selection, *args.cell, *args.extra_path]:
        relative_path(path)
        content, _, _ = head_blob(repository, revision, path)
        if (repository / path).is_symlink() or (repository / path).read_bytes() != content:
            raise BundleError("selected working file differs from tracked HEAD bytes")
        selected.add(path)
        data[path] = content
    protocol = json_object(data[args.protocol])
    selection = json_object(data[args.selection])
    if (protocol.get("schema") != "experiment.protocol/v1"
            or selection.get("schema") != "prime31-isogeny-choice.selection/v1"
            or selection.get("canonical_protocol_sha256") != digest(data[args.protocol])
            or selection.get("stage") not in ("screening", "complete_recovery")):
        raise BundleError("selection does not name the exact supplied protocol bytes")
    actual_cells = []
    for path in args.cell:
        cell = json_object(data[path])
        if (cell.get("schema") not in ("prime31-isogeny-choice.cell/v3", "prime31-complete-ic-incumbent.cell/v1")
                or not isinstance(cell.get("cell_id"), str) or not cell["cell_id"]
                or cell.get("stage") != selection.get("stage")
                or type(cell.get("execution_seconds")) is not int or cell["execution_seconds"] != 120
                or type(cell.get("memory_mib")) is not int or cell["memory_mib"] != 4096):
            raise BundleError("unsupported cell envelope or selected stage mismatch")
        actual_cells.append((cell["cell_id"], digest(data[path])))
    expected_cells = selection.get("cells")
    if not isinstance(expected_cells, list) or not all(isinstance(item, dict) for item in expected_cells):
        raise BundleError("explicit selected cell list required")
    expected_cells = [(item.get("cell_id"), item.get("input_sha256")) for item in expected_cells]
    if not all(isinstance(name, str) and name and isinstance(hash_value, str)
               and re.fullmatch(r"[0-9a-f]{64}", hash_value) for name, hash_value in expected_cells):
        raise BundleError("selected cells require explicit IDs and exact input SHA-256 values")
    if (len(set(name for name, _ in actual_cells)) != len(actual_cells)
            or sorted(actual_cells) != sorted(expected_cells)):
        raise BundleError("every selected cell must be supplied exactly once with matching bytes")
    # Include this constructor's real tracked source without exporting local config.
    for path in (Path(__file__).resolve(), Path(__file__).with_name("prepare-runtime-bundle.sh").resolve()):
        if not path.is_relative_to(repository):
            raise BundleError("constructor must run from the exact source checkout it bundles")
        relative = relative_path(path.relative_to(repository).as_posix())
        if head_blob(repository, revision, relative)[0] != path.read_bytes():
            raise BundleError("constructor source must be tracked and unchanged")
        selected.add(relative)
    output = fresh_directory(destination)
    new_json(output / "construction-start.json", {"source_revision": revision, "scope": "metadata_source_bundle_only"})
    try:
        copied = {}
        identities = {}
        for name, (original, expected_hash) in native_expected.items():
            target = output / "rootfs/usr/local/bin" / name
            content = original.read_bytes()
            if digest(content) != expected_hash:
                raise BundleError("supplied executable changed during preparation")
            new_file(target, content, 0o755)
            copied[name] = target
            entries, binary_hash, manifest_hash = source_metadata(target, repository, revision, output, name, "original")
            for path, identity in entries.items():
                if path in identities and identities[path] != identity:
                    raise BundleError("native binaries contain conflicting source bytes")
                tracked = head_blob(repository, revision, path)[0]
                if (digest(tracked), len(tracked)) != identity:
                    raise BundleError("embedded native inventory differs from exact HEAD source")
                identities[path] = identity
                selected.add(path)
        source, source_report = export_source(repository, output, selected, revision)
        runtime = inspect_runtime(copied, output)
        for name, binary in copied.items():
            source_metadata(binary, source, revision, output, name, "sanitized")
        clean_source(repository)
        if revision_of(repository) != revision:
            raise BundleError("source revision changed during construction")
        for name, (original, expected_hash) in native_expected.items():
            if digest(original.read_bytes()) != expected_hash:
                raise BundleError("supplied executable changed during construction")
        report = {
            "scope": "source_payload_packaging_only", "complete": True,
            "source": source_report, "selected_protocol_path": args.protocol,
            "selected_selection_path": args.selection, "selected_cell_paths": args.cell,
            "native_binary_sha256": {name: value[1] for name, value in native_expected.items()},
            "runtime_manifest_path": "metadata/runtime-dependencies.json",
            "native_linkage": {name: runtime["executables"][name]["linkage"] for name in copied},
            "source_checks_per_binary": ["original_checkout", "sanitized_real_sparse_checkout"],
            "build_executed": False, "scientific_arithmetic_executed": False,
            "launch_admitted": False, "image_published": False, "container_runtime_qualified": False,
            "canonical_protocol_validation_performed": False, "execution_receipts_verified": False,
            "publication_readback_verified": False,
            "binary_role_assignment": "caller_supplied_arguments; source metadata does not independently prove compiler target selection",
            "native_source_checks_runtime": "current_host; no container execution performed",
        }
        new_json(output / "bundle.json", report)
        records = []
        for path in sorted(output.rglob("*")):
            if path.is_symlink():
                raise BundleError("unexpected symlink in generated payload")
            if path.is_file():
                records.append(f"{digest(path.read_bytes())}  {path.relative_to(output).as_posix()}")
        new_file(output / "SHA256SUMS", ("\n".join(records) + "\n").encode())
        return report
    except Exception as error:
        new_json(output / "construction-failure.json", {"complete": False, "launch_admitted": False,
                 "scope": "metadata_source_bundle_only", "error_class": type(error).__name__,
                 "error": str(error) if isinstance(error, BundleError) else "unexpected constructor failure"})
        raise


def parser():
    result = argparse.ArgumentParser(description=__doc__)
    result.add_argument("--repository", required=True)
    result.add_argument("--choice-binary", required=True)
    result.add_argument("--incumbent-binary", required=True)
    result.add_argument("--cell", action="append", required=True, help="exact tracked repository-relative path; repeat for every selected cell")
    result.add_argument("--protocol", required=True, help="exact tracked repository-relative path")
    result.add_argument("--selection", required=True, help="exact tracked repository-relative path")
    result.add_argument("--extra-path", action="append", default=[], help="additional explicit tracked closure path")
    result.add_argument("--output", required=True, help="fresh absolute directory outside the checkout")
    return result


if __name__ == "__main__":
    try:
        print(json.dumps(construct(parser().parse_args()), sort_keys=True, indent=2))
    except (BundleError, OSError, ValueError) as error:
        print(f"prepare-runtime-bundle: {error}", file=sys.stderr)
        raise SystemExit(65)
