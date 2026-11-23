"""Fail-closed tree accounting with narrow compiler-temp disappearance handling."""
import errno
import os
import stat
from pathlib import Path


def _absolute(path):
    return Path(os.path.abspath(os.fspath(path)))


def _within(path, root):
    path = _absolute(path)
    root = _absolute(root)
    try:
        path.relative_to(root)
        return True
    except ValueError:
        return False


def _confirmed_absent(path):
    try:
        Path(path).lstat()
    except FileNotFoundError as error:
        if error.errno == errno.ENOENT:
            return True
        raise
    return False


def scan_regular_tree(
    root,
    *,
    mutable_missing_roots=(),
    missing_name_predicate=None,
    bucket=None,
    before_lstat=None,
    event_limit=32,
):
    """Account regular files while tolerating only confirmed, allowlisted ENOENT.

    ``mutable_missing_roots`` is intended for compiler-owned target/tmp trees.
    ``missing_name_predicate`` is intended for the known atomic receipt temp
    names.  Every observed directory, symlink, and special file is checked.
    Errors other than ENOENT are never suppressed.

    ``before_lstat`` exists only so the charged native control can coordinate a
    real concurrent deletion at the exact list/lstat boundary.  Production
    accounting never supplies it.
    """
    root = _absolute(root)
    mutable = tuple(_absolute(path) for path in mutable_missing_roots)
    root_mode = root.lstat().st_mode
    if not stat.S_ISDIR(root_mode) or stat.S_ISLNK(root_mode):
        raise RuntimeError('tree root is not a real directory: ' + str(root))

    result = {'total_bytes': 0, 'files': 0, 'buckets': {}, 'transient_missing': []}

    def record_missing(path):
        path = _absolute(path)
        allowed = any(_within(path, candidate) for candidate in mutable)
        if missing_name_predicate is not None and missing_name_predicate(path):
            allowed = True
        if not allowed or not _confirmed_absent(path):
            return False
        if len(result['transient_missing']) < event_limit:
            result['transient_missing'].append(str(path))
        return True

    def onerror(error):
        path = getattr(error, 'filename', None)
        if error.errno == errno.ENOENT and path is not None and record_missing(path):
            return
        raise error

    for base, dirs, names in os.walk(root, topdown=True, followlinks=False, onerror=onerror):
        base_path = Path(base)
        kept = []
        for name in dirs:
            path = base_path / name
            if before_lstat is not None:
                before_lstat(path, True)
            try:
                mode = path.lstat().st_mode
            except FileNotFoundError as error:
                if error.errno == errno.ENOENT and record_missing(path):
                    continue
                raise
            if stat.S_ISLNK(mode) or not stat.S_ISDIR(mode):
                raise RuntimeError('non-directory or symlink in tree: ' + str(path))
            kept.append(name)
        dirs[:] = kept
        for name in names:
            path = base_path / name
            if before_lstat is not None:
                before_lstat(path, False)
            try:
                file_stat = path.lstat()
            except FileNotFoundError as error:
                if error.errno == errno.ENOENT and record_missing(path):
                    continue
                raise
            if stat.S_ISLNK(file_stat.st_mode) or not stat.S_ISREG(file_stat.st_mode):
                raise RuntimeError('non-regular file or symlink in tree: ' + str(path))
            result['files'] += 1
            result['total_bytes'] += file_stat.st_size
            if bucket is not None:
                key = bucket(path)
                result['buckets'][key] = result['buckets'].get(key, 0) + file_stat.st_size
    return result
