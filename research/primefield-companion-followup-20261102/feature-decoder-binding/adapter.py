"""Fixed-program binding to the preserved public sign-oracle decoder."""
from dataclasses import dataclass
import hashlib
import json
from pathlib import Path
from preserved_decoder import evaluate as original_evaluate
from program_check import verify

PROGRAM_SHA256 = '1f7e916829de51311f2e7e5109766dc3e34a3e1d2e8bb93b517f23ae496c8e36'


@dataclass(frozen=True)
class BoundProgram:
    source_bytes: bytes


def _source(raw):
    if type(raw) is not bytes or hashlib.sha256(raw).hexdigest() != PROGRAM_SHA256:
        raise ValueError('fixed verified program bytes required')
    return json.loads(raw)


def bind(raw=None):
    if raw is None:
        raw = (Path(__file__).resolve().parent/'program.json').read_bytes()
    program = _source(raw)
    verify(program)
    return BoundProgram(raw)


def legacy_view(bound):
    if type(bound) is not BoundProgram:
        raise ValueError('bound program required')
    program = _source(bound.source_bytes)
    q = 1+max(d for t in program['terms'] for d in t['offsets'])
    if program['r'] != 479 or q != 16:
        raise ValueError('fixed field/window binding')
    return dict(r=479, q=q, offsets=list(range(q)), degree=3, terms=program['terms'])


def decode(bound, oracle):
    """Oracle receives offsets only; source construction and Q ownership are external."""
    view = legacy_view(bound)
    def signed_oracle(offset):
        value = oracle(offset)
        if type(value) is not int or value not in (-1, 0, 1):
            raise ValueError('source oracle must return an integer in {-1,0,1}')
        return value
    scalar, counts = original_evaluate(view, signed_oracle)
    return scalar, dict(counts, program_sha256=PROGRAM_SHA256,
                        binding_bytes_hashed=len(bound.source_bytes), required_prefix=16)
