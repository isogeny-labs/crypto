"""Independent metadata validation for the proposed binary trace instrument.

Bounds encoded size from the declared trace schema and carry radix. Does not
reconstruct a trace or certify its hash preimage from digest-only records.
"""
from dataclasses import dataclass
import re


@dataclass(frozen=True)
class Bits:
    width: int


def length_size(n):
    return max(1, (n.bit_length() + 6) // 7)


def size(value):
    if isinstance(value, Bits):
        n = (value.width + 7) // 8
        return 2 + length_size(n) + n
    if type(value) is int:
        return size(Bits(abs(value).bit_length()))
    if type(value) is str:
        n = len(value.encode('utf-8'))
        return 1 + length_size(n) + n
    if type(value) is list:
        return 1 + length_size(len(value)) + sum(size(v) for v in value)
    if type(value) is dict:
        return 1 + length_size(len(value)) + sum(size(k) + size(v) for k, v in value.items())
    raise TypeError('unsupported size schema')


def byte_bounds(r, n, s, a, b):
    if any(type(x) is not int for x in (r, n, s, a, b)):
        raise ValueError('integer parameters required')
    if not (r > 3 and n > 0 and s > 0 and 0 <= a < r and 0 <= b < r and a != b):
        raise ValueError('invalid trace parameters')
    bound = s * n * n * (r-1)**4 * (a+1) * (b+1)
    digit_bytes = max(1, (bound.bit_length()+7)//8)
    widths = {'P': (2*n-1)*8*digit_bytes, 'Q': (2*n-1)*8*digit_bytes,
              'T': n*8*digit_bytes, 'Y': n*8*digit_bytes, 'acc': n*8*digit_bytes}
    def envelope(upper):
        trace = [dict(stage='setup', bound=bound, digit_bytes=digit_bytes,
                      inverse=Bits((r-1).bit_length() if upper else 1))]
        for j in range(s):
            trace.append(dict(stage='column', column=j,
                              **{k: Bits(w if upper else 0) for k, w in widths.items()}))
        trace.append(dict(stage='final', packed=Bits(n*8*digit_bytes if upper else 0)))
        return len(b'companion-trace/v1\x00') + size(trace)
    return envelope(False), envelope(True)


def validate(record, r, n, s, a, b):
    if type(record) is not dict or set(record) != {'schema', 'sha256', 'bytes'}:
        raise ValueError('fingerprint fields')
    if record['schema'] != 'companion-trace-fingerprint/v1':
        raise ValueError('fingerprint version')
    if type(record['sha256']) is not str or re.fullmatch('[0-9a-f]{64}', record['sha256']) is None:
        raise ValueError('fingerprint digest')
    low, high = byte_bounds(r, n, s, a, b)
    if type(record['bytes']) is not int or not low <= record['bytes'] <= high:
        raise ValueError('fingerprint byte envelope')
    return dict(min_bytes=low, max_bytes=high)
