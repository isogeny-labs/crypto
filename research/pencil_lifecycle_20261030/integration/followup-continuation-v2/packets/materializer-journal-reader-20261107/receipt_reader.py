"""Bounded read-only receipt consistency verifier. Never authenticates ownership.

Linux/POSIX dir-fd and O_NOFOLLOW are required. No producer code is imported.
Unreferenced journal tails are reported, never interpreted as committed records.
"""
import hashlib
import json
import os
from pathlib import Path
import stat

MAX_RECEIPT = 16 * 1024 * 1024
MAX_PREFIX = 16 * 1024 * 1024
MAX_JOURNAL = 64 * 1024 * 1024
MAX_RECORDS = 100_000


class InvalidReceipt(ValueError):
    pass


def require(condition, message):
    if not condition:
        raise InvalidReceipt(message)


def strict_json(raw):
    def pairs(items):
        result = {}
        for key, value in items:
            require(key not in result, 'duplicate JSON key')
            result[key] = value
        return result
    def constant(value):
        raise InvalidReceipt('nonfinite JSON constant: ' + value)
    try:
        return json.loads(raw.decode('utf-8'), object_pairs_hook=pairs,
                          parse_constant=constant, parse_float=finite_float)
    except (UnicodeError, ValueError, RecursionError) as exc:
        raise InvalidReceipt('invalid JSON: ' + str(exc)) from exc


def finite_float(value):
    import math
    result = float(value)
    require(math.isfinite(result), 'nonfinite JSON number')
    return result


def directory_fd(directory):
    path = Path(os.path.abspath(directory))
    # abspath normalizes lexical ..; reject it in the caller spelling as well.
    require('..' not in Path(directory).parts, 'parent traversal')
    fd = os.open('/', os.O_RDONLY | os.O_DIRECTORY)
    try:
        for part in path.parts[1:]:
            new = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW,
                          dir_fd=fd)
            os.close(fd)
            fd = new
        return fd
    except BaseException:
        os.close(fd)
        raise


def read_regular(fd, name, limit, prefix=None):
    child = os.open(name, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=fd)
    try:
        before = os.fstat(child)
        require(stat.S_ISREG(before.st_mode), 'not a regular file')
        require(before.st_size <= limit, 'file size exceeds bound')
        length = before.st_size if prefix is None else prefix
        require(length <= before.st_size, 'truncated prefix')
        chunks = []
        remaining = length
        while remaining:
            chunk = os.read(child, min(remaining, 65536))
            require(bool(chunk), 'short read')
            chunks.append(chunk)
            remaining -= len(chunk)
        after = os.fstat(child)
        identity = lambda s: (s.st_dev, s.st_ino, s.st_size, s.st_mtime_ns, s.st_ctime_ns)
        require(identity(before) == identity(after), 'file changed during read')
        return b''.join(chunks), before.st_size
    finally:
        os.close(child)


def natural(value, ceiling):
    require(type(value) is int and 0 <= value <= ceiling, 'invalid bounded integer')
    return value


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(',', ':'), allow_nan=False)


def verify(directory):
    """Return consistency only; all filesystem/format errors reject the input.

    An incomplete checkpoint with no pointer acknowledges zero journal records;
    this says nothing about spawned children. Legacy receipts are accepted only
    as inline evidence, without claiming the newer journal integrity guarantee.
    """
    fd = directory_fd(directory)
    try:
        raw, _ = read_regular(fd, 'receipt.json', MAX_RECEIPT)
        receipt = strict_json(raw)
        require(type(receipt) is dict, 'receipt must be an object')
        status = receipt.get('status')
        require(status in ('incomplete', 'failed', 'passed'), 'unknown status')
        storage = receipt.get('child_cleanup_storage')
        require(storage is not None or 'child_cleanup_storage' not in receipt, 'null storage')
        require(storage in (None, 'journal-prefix/v1', 'inline-final-and-journal-prefix/v1'),
                'unknown storage')
        final = storage == 'inline-final-and-journal-prefix/v1'
        inline = receipt.get('child_cleanup', [])
        require(type(inline) is list and len(inline) <= MAX_RECORDS and
                all(type(row) is dict for row in inline), 'invalid inline records')
        pointer = receipt.get('child_cleanup_journal')
        if storage is None:
            require('child_cleanup_journal' not in receipt, 'legacy receipt has journal pointer')
            require('child_cleanup' in receipt, 'legacy inline evidence missing')
            return dict(consistency='legacy_inline_only', reported_status=status,
                        inline_records=len(inline), terminal_owner_authenticated=False,
                        terminal_established=False, journal_verified=False, cleanup_semantics_verified=False)
        if not final:
            require('child_cleanup' not in receipt, 'intermediate receipt contains inline records')
        records = []
        tail = None
        if pointer is not None:
            require(type(pointer) is dict and set(pointer) == {'path', 'bytes', 'sha256', 'records'},
                    'invalid pointer shape')
            require(pointer['path'] == 'child-cleanup.jsonl', 'invalid journal basename')
            length = natural(pointer['bytes'], MAX_PREFIX)
            count = natural(pointer['records'], MAX_RECORDS)
            digest = pointer['sha256']
            require(type(digest) is str and len(digest) == 64 and
                    all(c in '0123456789abcdef' for c in digest), 'invalid digest')
            data, size = read_regular(fd, 'child-cleanup.jsonl', MAX_JOURNAL, length)
            require(hashlib.sha256(data).hexdigest() == digest, 'prefix digest mismatch')
            require(not data or data.endswith(b'\n'), 'prefix is not newline terminated')
            lines = data.split(b'\n')[:-1] if data else []
            require(len(lines) == count, 'prefix count mismatch')
            records = [strict_json(line) for line in lines]
            require(all(type(row) is dict for row in records), 'journal rows must be objects')
            tail = size - length
        else:
            require('child_cleanup_journal' not in receipt, 'explicit null pointer')
        if final:
            require(status in ('passed', 'failed'), 'final status must be passed or failed')
            require('child_cleanup' in receipt or not records, 'final inline evidence missing')
            require(len(inline) >= len(records) and
                    all(canonical(a) == canonical(b) for a, b in zip(inline, records)),
                    'inline does not extend committed prefix')
            if status == 'passed':
                require(pointer is not None and len(inline) == len(records) and tail == 0,
                        'passed final requires complete matching journal')
            elif status != 'failed':
                require(len(inline) == len(records), 'only failed final may extend prefix')
        consistency = ('complete' if status == 'passed' else 'failed') if final and status != 'incomplete' else 'incomplete'
        return dict(consistency='storage_' + consistency, reported_status=status,
                    cleanup_semantics_verified=False,
                    storage=storage, committed_records=len(records), inline_records=len(inline),
                    unreferenced_tail_bytes=tail, journal_verified=pointer is not None,
                    terminal_owner_authenticated=False, terminal_established=False)
    finally:
        os.close(fd)


if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('directory')
    args = parser.parse_args()
    try:
        print(json.dumps(verify(args.directory), sort_keys=True))
    except (InvalidReceipt, OSError) as exc:
        parser.exit(1, 'receipt rejected: ' + str(exc) + '\n')
