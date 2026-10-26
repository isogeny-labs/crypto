"""Source-bound live worker build admitting the current in-repository crates.

V1 and v2 remain byte-identical for archived build records. V3 additionally
admits tools/factor-base, now a direct root dependency. Every admitted crate
is still fully hashed (excluding target/); other path and Git dependencies
remain rejected. Records bind this version's own builder digest.
"""
import argparse
import json
from pathlib import Path
import re
import shutil
import subprocess
import tarfile

from generic_build import ROOT, WORKER, command, controlled_environment, digest, verify_build_record
from identity import sha256, write_immutable
from oracle import require

ADMITTED_PATH_CRATES = {'tools/volcano-cli', 'isogeny_algos', 'tools/factor-base'}


def source_manifest(root, metadata):
    require(not (root / 'build.rs').exists(), 'root build script needs explicit admission')
    names = [Path('Cargo.toml'), Path('Cargo.lock'), Path(WORKER)]
    names += sorted(path.relative_to(root) for path in (root / 'src').rglob('*') if path.is_file())
    included = set()
    for name in names:
        if name.suffix == '.rs':
            text = (root / name).read_text()
            for relative in re.findall(r'include(?:_str|_bytes)?!\(\s*"([^"]+)"', text):
                path = (root / name).parent / relative
                require(path.resolve().is_relative_to(root), 'include input escapes source root')
                included.add(path.resolve().relative_to(root))
    names = sorted(set(names) | included)
    require(not any((root / path).is_symlink() for path in names), 'symlinked root build input')
    root_files = {path.as_posix(): digest(root / path) for path in names}
    dependencies = []
    for package in metadata['packages']:
        directory = Path(package['manifest_path']).parent
        if directory == root:
            continue
        if package['source'] is None:
            relative = directory.resolve()
            require(relative.is_relative_to(root)
                    and relative.relative_to(root).as_posix() in ADMITTED_PATH_CRATES,
                    'path/git dependency needs explicit build admission')
            source = 'path+' + relative.relative_to(root).as_posix()
            skip = directory / 'target'
        else:
            require(package['source'].startswith('registry+'),
                    'path/git dependency needs explicit build admission')
            source, skip = package['source'], None
        files = sorted(path for path in directory.rglob('*') if path.is_file()
                       and path.name not in {'.cargo-ok', '.cargo_vcs_info.json'}
                       and not (skip is not None and path.is_relative_to(skip)))
        require(not any(path.is_symlink() for path in files), 'symlinked dependency input')
        dependencies.append(dict(package=package['name'], version=package['version'], source=source,
                                 files={path.relative_to(directory).as_posix(): digest(path) for path in files}))
    dependencies.sort(key=lambda p: (p['package'], p['version']))
    return dict(schema_version=1, root_files=root_files, dependencies=dependencies)


def build(root, out):
    root, out = root.resolve(), out.resolve()
    out.mkdir(parents=True, exist_ok=False)
    env = controlled_environment(root)
    rustc = command(['rustc', '-Vv'], root, env)
    cargo = command(['cargo', '-Vv'], root, env)
    cfg = command(['rustc', '--print', 'cfg'], root, env).splitlines()
    target = {name: next(json.loads(line.split('=', 1)[1]) for line in cfg if line.startswith(name + '='))
              for name in ('target_arch', 'target_os')}
    metadata = json.loads(command(['cargo', 'metadata', '--locked', '--offline', '--format-version', '1'], root, env))
    source = source_manifest(root, metadata)
    write_immutable(out / 'source-manifest.json', source)
    with tarfile.open(out / 'root-source.tar.gz', 'x:gz') as archive:
        for name in source['root_files']:
            archive.add(root / name, arcname=name, recursive=False)
    native_tools = {}
    for name in ('CC', 'AR'):
        cp = subprocess.run([env[name], '--version'], cwd=root, env=env,
                            capture_output=True, text=True, check=False)
        native_tools[name] = dict(binary_sha256=digest(env[name]), exit_code=cp.returncode,
                                  version_stdout=cp.stdout, version_stderr=cp.stderr)
    args = ['cargo', 'build', '--locked', '--offline', '--release', '--no-default-features',
            '--example', 'ic_tournament_worker']
    policy = dict(schema_version=1, source_manifest_sha256=sha256(source),
                  rustc=rustc, cargo=cargo, native_tools=native_tools, arguments=args,
                  **target, flags=dict(rustflags='', incremental=False, features=[], profile='release'),
                  scope='local toolchain build; OS and system libraries are not hermetic')
    build_hash = sha256(policy)
    env.update(IC_GENERIC_SOURCE_MANIFEST_SHA256=sha256(source), IC_GENERIC_BUILD_SHA256=build_hash)
    write_immutable(out / 'build-policy.json', policy)
    with (out / 'build.log').open('x') as log:
        cp = subprocess.run(args, cwd=root, env=env, stdout=log, stderr=subprocess.STDOUT, check=False)
    write_immutable(out / 'build-exit.json', dict(exit_code=cp.returncode))
    require(cp.returncode == 0, 'worker build failed; log retained')
    require(source_manifest(root, metadata) == source, 'source changed during build')
    executable = Path(metadata['target_directory']) / 'release/examples/ic_tournament_worker'
    shutil.copy2(executable, out / 'worker')
    identity = json.loads(command([str(out / 'worker'), '--build-identity'], root, env))
    record = dict(schema_version=1, source_manifest_sha256=sha256(source), build_sha256=build_hash,
                  build=policy, identity=identity, worker_sha256=digest(out / 'worker'),
                  builder_sha256=digest(__file__))
    verify_build_record(record, source)
    write_immutable(out / 'build-record.json', record)
    return record


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=ROOT)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    result = build(args.root, args.out)
    print(json.dumps({key: result[key] for key in ('build_sha256', 'source_manifest_sha256', 'worker_sha256')}))
