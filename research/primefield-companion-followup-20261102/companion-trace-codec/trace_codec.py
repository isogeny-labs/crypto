"""Canonical binary trace fingerprint v1; no numerical kernel imports."""
import hashlib

MAGIC = b'companion-trace/v1\x00'


def uint(n):
    assert n >= 0
    out = bytearray()
    while n >= 128:
        out.append((n & 127) | 128)
        n >>= 7
    out.append(n)
    return bytes(out)


def chunks(value, depth=0):
    if depth > 128:
        raise ValueError('trace nesting exceeds 128')
    if value is None:
        yield b'n'
    elif type(value) is bool:
        yield b't' if value else b'f'
    elif type(value) is int:
        magnitude = abs(value)
        size = (magnitude.bit_length() + 7) // 8
        yield b'i' + bytes([int(value < 0)]) + uint(size)
        yield magnitude.to_bytes(size, 'big')
    elif type(value) is str:
        data = value.encode('utf-8')
        yield b's' + uint(len(data))
        yield data
    elif type(value) is list:
        yield b'l' + uint(len(value))
        for item in value:
            yield from chunks(item, depth + 1)
    elif type(value) is dict:
        if not all(type(k) is str for k in value):
            raise TypeError('dictionary keys must be strings')
        yield b'd' + uint(len(value))
        for key in sorted(value):
            yield from chunks(key, depth + 1)
            yield from chunks(value[key], depth + 1)
    else:
        raise TypeError('unsupported trace type: ' + type(value).__name__)


def encode(value):
    """Materialize only for codec tests/inspection; fingerprints stream chunks."""
    return MAGIC + b''.join(chunks(value))


def fingerprint(value):
    h = hashlib.sha256(MAGIC)
    size = len(MAGIC)
    for block in chunks(value):
        h.update(block)
        size += len(block)
    return dict(schema='companion-trace-fingerprint/v1', sha256=h.hexdigest(), bytes=size)


def decode(data):
    """Independent byte parser; rejects noncanonical and truncated encodings."""
    if type(data) is not bytes or not data.startswith(MAGIC):
        raise ValueError('invalid trace header')
    pos = len(MAGIC)

    def take(n):
        nonlocal pos
        if n < 0 or n > len(data) - pos:
            raise ValueError('truncated trace')
        value = data[pos:pos+n]
        pos += n
        return value

    def count():
        value, digits = 0, 0
        while True:
            byte = take(1)[0]
            value += (byte % 128) * (128 ** digits)
            digits += 1
            if digits > 10:
                raise ValueError('length exceeds parser envelope')
            if byte < 128:
                if digits > 1 and byte == 0:
                    raise ValueError('noncanonical length')
                return value

    def item(depth=0):
        if depth > 128:
            raise ValueError('trace nesting exceeds 128')
        tag = take(1)
        if tag == b'n': return None
        if tag == b't': return True
        if tag == b'f': return False
        if tag == b'i':
            sign = take(1)[0]
            magnitude = take(count())
            if sign not in (0, 1) or (magnitude and magnitude[0] == 0) or (not magnitude and sign):
                raise ValueError('noncanonical integer')
            value = int.from_bytes(magnitude, 'big')
            return -value if sign else value
        if tag == b's': return take(count()).decode('utf-8')
        if tag in (b'l', b'd'):
            length = count()
            if length > len(data) - pos:
                raise ValueError('impossible container length')
            if tag == b'l': return [item(depth + 1) for _ in range(length)]
            result = {}
            previous = None
            for _ in range(length):
                key = item(depth + 1)
                if type(key) is not str or (previous is not None and key <= previous):
                    raise ValueError('noncanonical dictionary keys')
                previous = key
                result[key] = item(depth + 1)
            return result
        raise ValueError('unknown trace tag')

    result = item()
    if pos != len(data):
        raise ValueError('trailing trace bytes')
    return result
