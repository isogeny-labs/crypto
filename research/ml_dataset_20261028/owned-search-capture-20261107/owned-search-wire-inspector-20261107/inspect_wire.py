#!/usr/bin/env python3
"""Bounded structural inspection of retained evidence; no mathematical verdict."""
import argparse
from dataclasses import dataclass
import hashlib
import json
from pathlib import Path
import struct

MAGIC = b'OWNSEARCH\x01'
ROOT = ('schema snapshot_index identity signed_domain signed_target failure first_stop '
        'diagnostic normalization_cursor expansion normalized continuation lift_records '
        'coverage exclusion_audit accepted_record status encoding_complete').split()
IDENTITY = ('public_sha256 factor_sha256 launch_sha256 binding_sha256 map_bytes map_ack '
            'publication_attempted map_published pattern').split()
SCHEMA = 'owned-search-retained-evidence/v1'

class Invalid(ValueError):
    pass

@dataclass(frozen=True)
class Limits:
    bytes: int = 128 * 1024 * 1024
    values: int = 2_000_000
    depth: int = 64
    string_bytes: int = 4096
    map_fields: int = 64

class Reader:
    def __init__(self, streams, cap):
        self.streams = iter(streams)
        self.current = next(self.streams, None)
        self.cap = cap
        self.count = 0
        self.digest = hashlib.sha256()
    def read(self, n):
        if n < 0 or n > 65536 or self.count + n > self.cap:
            raise Invalid('byte or scratch bound exceeded')
        parts = []
        left = n
        while left:
            if self.current is None:
                raise Invalid('truncated stream')
            part = self.current.read(left)
            if not part:
                self.current = next(self.streams, None)
                continue
            parts.append(part)
            left -= len(part)
        data = b''.join(parts)
        self.count += n
        self.digest.update(data)
        return data
    def raw(self, n, retain=False):
        if n > self.cap - self.count:
            raise Invalid('declared bytes exceed bound')
        if retain:
            return self.read(n)
        while n:
            k = min(n, 65536)
            self.read(k)
            n -= k
        return None
    def finish(self):
        # One extra byte is enough to reject, even when the admitted cap is exact.
        while self.current is not None:
            if self.current.read(1):
                raise Invalid('trailing bytes after root')
            self.current = next(self.streams, None)

class Inspector:
    def __init__(self, streams, limits):
        if any(v <= 0 for v in vars(limits).values()) or limits.depth > 128 or limits.string_bytes > 65536:
            raise Invalid('invalid inspector limits')
        self.limits = limits
        self.r = Reader(streams, limits.bytes)
        self.values = 0
        self.tags = [0] * 7
        self.identity = {}
        self.snapshot = None
        self.status = None
    def number(self):
        return struct.unpack('<Q', self.r.read(8))[0]
    def value(self, path=(), depth=0):
        if depth > self.limits.depth or self.values >= self.limits.values:
            raise Invalid('depth or value bound exceeded')
        self.values += 1
        tag = self.r.read(1)[0]
        if tag > 6:
            raise Invalid('unknown tag')
        self.tags[tag] += 1
        if not path and tag != 6:
            raise Invalid('root must be map')
        if path == ('identity',) and tag != 6:
            raise Invalid('identity must be map')
        result = None
        if tag == 0:
            pass
        elif tag == 1:
            bit = self.r.read(1)[0]
            if bit > 1:
                raise Invalid('invalid boolean')
            result = bool(bit)
        elif tag == 2:
            result = self.number()
        elif tag == 3:
            n = self.number()
            if n > self.limits.string_bytes:
                raise Invalid('string bound exceeded')
            try:
                result = self.r.read(n).decode('utf-8', errors='strict')
            except UnicodeDecodeError as e:
                raise Invalid('invalid UTF8') from e
        elif tag == 4:
            n = self.number()
            identity_hash = len(path) == 2 and path[0] == 'identity' and path[1].endswith('_sha256')
            if identity_hash and n != 32:
                raise Invalid('identity hash must have32 bytes')
            result = self.r.raw(n, retain=identity_hash)
        else:
            count = self.number()
            minimum = count * (2 if tag == 6 else 1)
            if minimum > self.limits.values - self.values:
                raise Invalid('declared value count exceeds bound')
            if tag == 5:
                for i in range(count):
                    self.value(path + (i,), depth + 1)
            else:
                if count > self.limits.map_fields:
                    raise Invalid('map field bound exceeded')
                expected = ROOT if not path else IDENTITY if path == ('identity',) else None
                if expected is not None and count != len(expected):
                    raise Invalid('wrong fixed map size')
                keys = set()
                for i in range(count):
                    key = self.value(path + ('<key>',), depth + 1)
                    if not isinstance(key, str) or key in keys:
                        raise Invalid('non-string or duplicate map key')
                    keys.add(key)
                    if expected is not None and key != expected[i]:
                        raise Invalid('wrong fixed key or order')
                    self.value(path + (key,), depth + 1)
        if path == ('schema',) and (tag != 3 or result != SCHEMA):
            raise Invalid('wrong schema')
        if path == ('encoding_complete',) and (tag != 1 or result is not True):
            raise Invalid('missing encoding completion sentinel')
        if path == ('snapshot_index',):
            if tag != 2:
                raise Invalid('snapshot index must be u64')
            self.snapshot = result
        if path == ('status',):
            if tag != 3:
                raise Invalid('status must be string')
            self.status = result
        if len(path) == 2 and path[0] == 'identity' and path[1].endswith('_sha256'):
            if tag != 4:
                raise Invalid('identity hash must be raw bytes')
            self.identity[path[1]] = result.hex()
        return result
    def inspect(self):
        if self.r.read(len(MAGIC)) != MAGIC:
            raise Invalid('wrong magic/version')
        self.value()
        self.r.finish()
        return {'schema': 'owned-search-wire-inspection/v1', 'structural_pass': True,
                'bytes': self.r.count, 'sha256': self.r.digest.hexdigest(),
                'values': self.values, 'tag_counts': self.tags,
                'snapshot_index': self.snapshot, 'source_hashes': self.identity,
                'producer_status': self.status, 'mathematical_replay': False,
                'capture_chain_verified': False}

def inspect(streams, limits=Limits()):
    return Inspector(streams, limits).inspect()

def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('chunks', nargs='+', type=Path, help='explicit byte order; not inferred from filenames')
    p.add_argument('--max-bytes', type=int, default=Limits.bytes)
    p.add_argument('--max-values', type=int, default=Limits.values)
    p.add_argument('--max-depth', type=int, default=Limits.depth)
    a = p.parse_args()
    if len(a.chunks) > 4096:
        p.error('at most4096 chunks per inspection')
    from contextlib import closing
    def files():
        for f in a.chunks:
            with f.open('rb') as stream:
                yield stream
    with closing(files()) as streams:
        result = inspect(streams, Limits(bytes=a.max_bytes, values=a.max_values, depth=a.max_depth))
    print(json.dumps(result, sort_keys=True))

if __name__ == '__main__':
    main()
