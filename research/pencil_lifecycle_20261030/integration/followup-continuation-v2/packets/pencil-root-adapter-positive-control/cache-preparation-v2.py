from pathlib import Path, PurePosixPath
import hashlib, json, os, re, resource, shutil, signal, stat, subprocess, tarfile, time, traceback
P = Path(__file__).resolve().parent
F = json.loads((P / 'cache-preparation-v2-freeze.json').read_text())
START = time.monotonic()
resource.setrlimit(resource.RLIMIT_AS, (2 * 1024**3, 2 * 1024**3))
W = Path(F['workspace']) / 'cache-stage-v2'
R = {'status': 'failed', 'compiler_invoked': False, 'limits': F['limits'], 'errors': [], 'phase': 'preflight'}
ROWS = {}
ACTIVE = None

def stop(sig, frame):
    raise RuntimeError('signal ' + str(sig))
for s in (signal.SIGTERM, signal.SIGINT):
    signal.signal(s, stop)

def guard():
    if time.monotonic() - START >= 160:
        raise RuntimeError('160-second work deadline')
    if shutil.disk_usage(W.parent).free < 6 * 1024**3:
        raise RuntimeError('free disk below 6GiB')

def digest(path):
    guard()
    h = hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(4 * 1024**2), b''):
            guard()
            h.update(chunk)
    return h.hexdigest()

def files(root):
    out = []
    for path in root.rglob('*'):
        guard()
        mode = path.lstat().st_mode
        if stat.S_ISREG(mode):
            out.append(path)
        elif not stat.S_ISDIR(mode):
            raise ValueError('nonregular source: ' + str(path))
        if len(out) > 100000:
            raise ValueError('100000 file limit')
    return sorted(out)


try:
    R['initial_free_bytes'] = shutil.disk_usage(W.parent).free
    assert R['initial_free_bytes'] >= 12 * 1024**3, 'initial free below12GiB'
    for name, expected in F['files'].items():
        assert digest(P / name) == expected, name
    assert digest(Path(F['lock_path'])) == F['lock_sha256'], 'lock changed'
    archives = json.loads((P / 'registry-archives.json').read_text())
    assert archives['count'] == 259 and not archives['missing'] and not archives['mismatch']
    lock_packages = {}
    for block in Path(F['lock_path']).read_text().split('[[package]]')[1:]:
        fields = dict(re.findall(r'^([a-z_]+) = "([^"\n]+)"$', block, re.M))
        if fields.get('source', '').startswith('registry+'):
            key = fields['name'] + '-' + fields['version'] + '.crate'
            assert key not in lock_packages, 'duplicate registry name/version'
            lock_packages[key] = fields['checksum']
    archive_packages = {}
    for row in archives['archives']:
        key = Path(row['path']).name
        assert key not in archive_packages, 'duplicate archive'
        archive_packages[key] = row['expected']
    assert archive_packages == lock_packages, 'archive rows not bijective with lock registry packages' 
    # Verify every immutable compressed input before creating an output tree.
    for row in archives['archives']:
        assert digest(Path(row['path'])) == row['expected'] == row['sha256'], row['path']
    cache = Path(F['cache_source'])
    cache_files = []
    for name in ('deps', 'build', '.fingerprint'):
        cache_files.extend(files(cache / name))
    cache_bytes = sum(f.stat().st_size for f in cache_files)
    assert cache_bytes <= 3 * 1024**3, '3GiB logical cache limit'
    W.mkdir(mode=0o700, exist_ok=False)
    vendor = W / 'vendor'
    vendor.mkdir()
    expanded = 0
    count = 0
    R['phase'] = 'vendor-extraction'
    for row in archives['archives']:
        guard()
        archive = Path(row['path'])
        prefix = archive.name.removesuffix('.crate')
        root = vendor / prefix
        root.mkdir()
        checksums = {}
        seen = set()
        with tarfile.open(archive, 'r:gz') as tar:
            for member in tar:
                guard()
                rel = PurePosixPath(member.name)
                assert not rel.is_absolute() and '..' not in rel.parts, member.name
                assert rel.parts and rel.parts[0] == prefix, member.name
                assert member.isdir() or member.isfile(), 'link/special entry: ' + member.name
                assert member.name not in seen, 'duplicate archive member'
                seen.add(member.name)
                tail = rel.parts[1:]
                if not tail:
                    assert member.isdir()
                    continue
                assert tail[-1] != '.cargo-checksum.json', 'unexpected vendor checksum input'
                dst = root.joinpath(*tail)
                assert dst.is_relative_to(root)
                if member.isdir():
                    dst.mkdir(parents=True, exist_ok=True)
                    continue
                expanded += member.size
                count += 1
                assert expanded <= 2 * 1024**3 and count <= 100000, 'vendor expansion cap'
                dst.parent.mkdir(parents=True, exist_ok=True)
                h = hashlib.sha256()
                written = 0
                with tar.extractfile(member) as stream, dst.open('xb') as out:
                    while True:
                        guard()
                        chunk = stream.read(1024**2)
                        if not chunk:
                            break
                        written += len(chunk)
                        assert written <= member.size
                        h.update(chunk)
                        out.write(chunk)
                assert written == member.size
                dst.chmod(0o755 if member.mode & 0o111 else 0o644)
                checksums['/'.join(tail)] = h.hexdigest()
                ROWS[str(dst)] = {'sha256': h.hexdigest(), 'bytes': written, 'archive': str(archive)}
        assert (root / 'Cargo.toml').is_file(), 'missing package manifest'
        checksum = root / '.cargo-checksum.json'
        checksum.write_text(json.dumps({'files': checksums, 'package': row['expected']}, sort_keys=True) + '\n')
        expanded += checksum.stat().st_size
        assert expanded <= 2 * 1024**3
        ROWS[str(checksum)] = {'sha256': digest(checksum), 'bytes': checksum.stat().st_size, 'generated_from_lock_authenticated_archive': True}
        assert digest(archive) == row['sha256'], 'archive changed during extraction'
    R['phase'] = 'cache-copy'
    target = W / 'target' / 'release'
    target.mkdir(parents=True)
    copied_bytes = 0
    for src in cache_files:
        guard()
        dst = target / src.relative_to(cache)
        dst.parent.mkdir(parents=True, exist_ok=True)
        # Only exact enumerated regular files; never copy newly added siblings.
        fd = os.open(src, os.O_RDONLY | os.O_NOFOLLOW)
        with os.fdopen(fd, 'rb') as stream, dst.open('xb') as out:
            before = os.fstat(stream.fileno())
            assert stat.S_ISREG(before.st_mode), str(src)
            h = hashlib.sha256()
            size = 0
            while True:
                guard()
                chunk = stream.read(1024**2)
                if not chunk:
                    break
                size += len(chunk)
                copied_bytes += len(chunk)
                assert size <= before.st_size and copied_bytes <= 3 * 1024**3, 'cache growth/size cap'
                h.update(chunk)
                out.write(chunk)
            after = os.fstat(stream.fileno())
            assert size == before.st_size
            assert (before.st_size, before.st_mtime_ns, before.st_ctime_ns) == (after.st_size, after.st_mtime_ns, after.st_ctime_ns), str(src)
        assert (src.lstat().st_dev, src.lstat().st_ino) == (before.st_dev, before.st_ino), 'cache source replaced'
        dst.chmod(stat.S_IMODE(before.st_mode) & 0o777)
        os.utime(dst, ns=(before.st_atime_ns, before.st_mtime_ns))
        assert digest(dst) == h.hexdigest()
        assert (before.st_dev, before.st_ino) != (dst.stat().st_dev, dst.stat().st_ino), 'hardlink'
        ROWS[str(dst)] = {'source': str(src), 'sha256': h.hexdigest(), 'bytes': size}
    assert sum(row['bytes'] for path, row in ROWS.items() if str(target) + '/' in path) == copied_bytes
    R['phase'] = 'final-readback'
    for path, row in ROWS.items():
        assert digest(Path(path)) == row['sha256'], path
    cargo = W / 'cargo-home'
    cargo.mkdir()
    (cargo / 'config.toml').write_text('[source.crates-io]\nreplace-with="vendored-sources"\n[source.vendored-sources]\ndirectory=' + json.dumps(str(vendor)) + '\n[net]\noffline=true\n')
    (W / 'tmp').mkdir()
    for name, expected in F['files'].items():
        assert digest(P / name) == expected, 'post: ' + name
    assert digest(Path(F['lock_path'])) == F['lock_sha256']
    guard()
    R.update(status='passed', expanded_vendor_bytes=expanded, vendor_file_count=count, cache_bytes=cache_bytes, cache_file_count=len(cache_files))
except BaseException as error:
    R['errors'].append({'type': type(error).__name__, 'repr': repr(error), 'filename': getattr(error, 'filename', None), 'traceback': traceback.format_exc()})
finally:
    if ACTIVE is not None:
        try:
            if ACTIVE.poll() is None:
                os.killpg(ACTIVE.pid, signal.SIGKILL)
            ACTIVE.wait(timeout=2)
        except BaseException as error:
            R['status'] = 'failed'
            R['errors'].append({'cleanup': repr(error)})
    R.update(elapsed_seconds=time.monotonic()-START, final_free_bytes=shutil.disk_usage(W.parent).free, verified_file_count=len(ROWS), retained_partial=str(W), earlier_source_preparation_elapsed='unknown; outside this bounded attempt')
    (P / 'cache-preparation-v2-inputs.json').write_text(json.dumps(ROWS, indent=2)+'\n')
    (P / 'cache-preparation-v2-receipt.json').write_text(json.dumps(R, indent=2)+'\n')
    print(json.dumps(R))
raise SystemExit(0 if R['status'] == 'passed' else 1)
