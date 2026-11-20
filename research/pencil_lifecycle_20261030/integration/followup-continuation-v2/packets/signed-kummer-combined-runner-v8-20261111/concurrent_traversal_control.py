#!/usr/bin/env python3
"""Charged, solver-free control for concurrent tree-accounting semantics."""
import json
import os
import shutil
import stat
import sys
import threading
from pathlib import Path

from concurrent_tree import scan_regular_tree


def coordinated_delete(path, *, directory):
    ready = threading.Event()
    done = threading.Event()
    errors = []
    triggered = []

    def remove():
        try:
            if not ready.wait(2):
                raise RuntimeError('deletion trigger was not delivered')
            if directory:
                shutil.rmtree(path)
            else:
                path.unlink()
        except BaseException as error:
            errors.append(error)
        finally:
            done.set()

    thread = threading.Thread(target=remove, daemon=True)
    thread.start()

    def trigger():
        if triggered:
            raise RuntimeError('deletion control triggered more than once')
        triggered.append(True)
        ready.set()
        if not done.wait(2):
            raise RuntimeError('concurrent deletion control timed out')
        thread.join(2)
        if thread.is_alive():
            raise RuntimeError('concurrent deletion thread survived trigger')
        if errors:
            raise errors[0]

    def hook(candidate, is_directory):
        if not triggered and candidate == path and is_directory == directory:
            trigger()

    def finish():
        if not triggered:
            # Never leave the helper thread alive, even on a failed assertion.
            ready.set()
        thread.join(2)
        if thread.is_alive():
            raise RuntimeError('concurrent deletion thread outlived control')
        if errors:
            raise errors[0]
        if not triggered:
            raise RuntimeError('concurrent deletion control was not triggered')

    return hook, trigger, finish


def require_rejection(label, expected, action):
    try:
        action()
    except expected as error:
        return {'label': label, 'rejected': True, 'error': type(error).__name__}
    raise RuntimeError(label + ' was not rejected')


def run_control(root):
    if os.getuid() != 1000 or os.getgid() != 1000:
        raise RuntimeError('control must run as the admitted unprivileged worker')
    root = Path(root)
    root.mkdir(mode=0o700, parents=False, exist_ok=False)
    mutable = root / 'mutable'
    immutable = root / 'immutable'
    durable = root / 'durable'
    for path in (mutable, immutable, durable):
        path.mkdir(mode=0o700)

    vanished_dir = mutable / 'rmetaControl'
    vanished_dir.mkdir()
    (vanished_dir / 'file').write_bytes(b'x')
    hook, _, finish = coordinated_delete(vanished_dir, directory=True)
    directory_scan = scan_regular_tree(root, mutable_missing_roots=(mutable,), before_lstat=hook)
    finish()
    assert any(path.endswith('/rmetaControl') for path in directory_scan['transient_missing'])

    # Exercise os.walk's real onerror branch: the parent enumeration and
    # directory lstat succeed, then the directory is removed immediately before
    # os.walk's descent scandir.  The real scandir raises ENOENT to onerror.
    descent = mutable / 'rmetaDescent'
    descent.mkdir()
    (descent / 'file').write_bytes(b'x')
    _, trigger, finish = coordinated_delete(descent, directory=True)
    real_scandir = os.scandir
    scandir_triggered = []

    def controlled_scandir(path):
        # shutil.rmtree's safe-fd implementation calls os.scandir(integer_fd)
        # in the deletion thread; pass that through without path conversion.
        candidate = Path(os.fsdecode(path)) if isinstance(path, (str, bytes, os.PathLike)) else None
        if not scandir_triggered and candidate == descent:
            scandir_triggered.append(True)
            trigger()
        return real_scandir(path)

    os.scandir = controlled_scandir
    try:
        descent_scan = scan_regular_tree(root, mutable_missing_roots=(mutable,))
    finally:
        os.scandir = real_scandir
        finish()
    assert scandir_triggered == [True]
    assert any(path.endswith('/rmetaDescent') for path in descent_scan['transient_missing'])

    vanished_file = mutable / 'temporary.rmeta'
    vanished_file.write_bytes(b'x')
    hook, _, finish = coordinated_delete(vanished_file, directory=False)
    file_scan = scan_regular_tree(root, mutable_missing_roots=(mutable,), before_lstat=hook)
    finish()
    assert any(path.endswith('/temporary.rmeta') for path in file_scan['transient_missing'])

    immutable_gone = immutable / 'must-not-disappear'
    immutable_gone.write_bytes(b'x')
    hook, _, finish = coordinated_delete(immutable_gone, directory=False)
    negatives = [require_rejection(
        'immutable disappearance',
        FileNotFoundError,
        lambda: scan_regular_tree(root, mutable_missing_roots=(mutable,), before_lstat=hook),
    )]
    finish()

    symlink = mutable / 'symlink'
    symlink.symlink_to(immutable, target_is_directory=True)
    negatives.append(require_rejection(
        'symlink', RuntimeError,
        lambda: scan_regular_tree(root, mutable_missing_roots=(mutable,)),
    ))
    symlink.unlink()

    fifo = mutable / 'fifo'
    os.mkfifo(fifo, 0o600)
    negatives.append(require_rejection(
        'special file', RuntimeError,
        lambda: scan_regular_tree(root, mutable_missing_roots=(mutable,)),
    ))
    fifo.unlink()

    denied = immutable / 'denied'
    denied.mkdir(mode=0o700)
    (denied / 'file').write_bytes(b'x')
    denied.chmod(0)
    try:
        negatives.append(require_rejection(
            'permission error', PermissionError,
            lambda: scan_regular_tree(root, mutable_missing_roots=(mutable,)),
        ))
    finally:
        denied.chmod(0o700)

    atomic = durable / '.worker-receipt.json.control'
    atomic.write_bytes(b'x')
    hook, _, finish = coordinated_delete(atomic, directory=False)
    atomic_scan = scan_regular_tree(
        durable,
        missing_name_predicate=lambda path: path.name.startswith('.worker-receipt.json.'),
        before_lstat=hook,
    )
    finish()
    assert any(path.endswith('/.worker-receipt.json.control') for path in atomic_scan['transient_missing'])

    arbitrary = durable / 'arbitrary'
    arbitrary.write_bytes(b'x')
    hook, _, finish = coordinated_delete(arbitrary, directory=False)
    negatives.append(require_rejection(
        'unrecognized durable disappearance', FileNotFoundError,
        lambda: scan_regular_tree(
            durable,
            missing_name_predicate=lambda path: path.name.startswith('.worker-receipt.json.'),
            before_lstat=hook,
        ),
    ))
    finish()

    final_scan = scan_regular_tree(root)
    assert final_scan['total_bytes'] <= 1024 * 1024
    assert not final_scan['transient_missing']

    result = {
        'schema': 'pencil.concurrent-tree-control/v1',
        'status': 'passed',
        'positive_controls': {
            'compiler_directory_disappearance': True,
            'compiler_descent_scandir_disappearance': True,
            'compiler_file_disappearance': True,
            'atomic_receipt_temp_disappearance': True,
        },
        'negative_controls': negatives,
        'fail_closed_count': len(negatives),
        'final_fixture_files': final_scan['files'],
        'final_fixture_bytes': final_scan['total_bytes'],
        'fixture_bytes_cap': 1024 * 1024,
        'source_root_mode': stat.S_IMODE(root.lstat().st_mode),
    }
    assert result['fail_closed_count'] == 5
    return result


def main():
    if len(sys.argv) != 2:
        raise SystemExit('usage: concurrent_traversal_control.py CONTROL_ROOT')
    result = run_control(sys.argv[1])
    print(json.dumps(result, sort_keys=True))


if __name__ == '__main__':
    main()
