#!/usr/bin/env python3
"""Narrow, fail-closed translator for the pinned Magma projective functions.

Only assignments, returns, conditionals, function definitions and field
arithmetic occur in the selected source blocks. Unsupported syntax aborts.
"""
from __future__ import annotations

import hashlib
import re
from pathlib import Path

AUTHOR_SHA = '7416a7fffaafff371738e7498a6795057372e1c477e0079297eebf2347c2e72a'
MODULUS = 0
TRACE: list[str] = []


class F(int):
    def __new__(cls, value: int):
        return int.__new__(cls, int(value) % MODULUS)

    def __add__(self, other): return F(int(self) + int(other))
    def __radd__(self, other): return F(int(other) + int(self))
    def __sub__(self, other): return F(int(self) - int(other))
    def __rsub__(self, other): return F(int(other) - int(self))
    def __mul__(self, other): return F(int(self) * int(other))
    def __rmul__(self, other): return F(int(other) * int(self))
    def __neg__(self): return F(-int(self))
    def __pow__(self, power, modulo=None):
        assert modulo is None
        return F(pow(int(self), int(power), MODULUS))
    def __truediv__(self, other):
        divisor = int(other) % MODULUS
        if divisor == 0: raise ZeroDivisionError('author field denominator zero')
        return F(int(self) * pow(divisor, -1, MODULUS))
    def __rtruediv__(self, other):
        divisor = int(self)
        if divisor == 0: raise ZeroDivisionError('author field denominator zero')
        return F(int(other) * pow(divisor, -1, MODULUS))


def _blocks(source: str) -> list[str]:
    prelude = source[source.index('swap:=function('):source.index('printCoords:=procedure(')]
    projective = source[source.index('ADD11DBL:=function('):source.index('// ############# TEST #############')]
    return [prelude, projective]


def translate(source_path: Path):
    data = source_path.read_bytes()
    assert hashlib.sha256(data).hexdigest() == AUTHOR_SHA, 'pinned author SHA-256'
    source = data.decode()
    output = ['# Generated strictly from the pinned Magma file', 'def _set_trace(trace):', '    global TRACE', '    TRACE = trace']
    names = []
    for block in _blocks(source):
        indent = 0
        in_function = False
        for raw in block.splitlines():
            raw = raw.split('//', 1)[0].strip()
            if not raw:
                continue
            if ':=function(' in raw:
                match = re.fullmatch(r'([A-Za-z][A-Za-z0-9]*):=function\(([^)]*)\)', raw)
                assert match and not in_function, ('function syntax', raw)
                name, params = match.groups()
                names.append(name)
                output.append(f'def {name}({params}):')
                output.append(f'    TRACE.append("{name}")')
                indent, in_function = 1, True
                continue
            assert in_function, ('statement outside function', raw)
            for stmt in raw.split(';'):
                stmt = stmt.strip()
                if not stmt:
                    continue
                if stmt == 'end function':
                    assert indent == 1, ('function indent', raw, indent)
                    indent, in_function = 0, False
                    continue
                if stmt == 'end if':
                    assert indent >= 2, ('if indent', raw, indent)
                    indent -= 1
                    continue
                if stmt == 'else':
                    assert indent >= 2
                    indent -= 1
                    output.append('    ' * indent + 'else:')
                    indent += 1
                    continue
                conditional = re.fullmatch(r'(if|elif) (.+) then', stmt)
                if conditional:
                    kind, expression = conditional.groups()
                    if kind == 'elif':
                        assert indent >= 2
                        indent -= 1
                    output.append('    ' * indent + f'{kind} {_expr(expression)}:')
                    indent += 1
                    continue
                if stmt.startswith('return '):
                    output.append('    ' * indent + 'return ' + _expr(stmt[7:]))
                    continue
                if ':=' in stmt:
                    lhs, rhs = stmt.split(':=', 1)
                    assert re.fullmatch(r'[A-Za-z0-9_, ]+', lhs), ('assignment lhs', lhs)
                    output.append('    ' * indent + lhs.strip() + ' = ' + _expr(rhs))
                    continue
                raise AssertionError(('unsupported Magma statement', stmt))
        assert not in_function and indent == 0
    expected = {'swap', 'getcoords', 'ADD11DBL', 'ADD11ADD', 'ADD11',
                'ADD12TRP', 'ADD12DBLADD', 'ADD12G', 'ADD12', 'DBL2B0',
                'DBL2C0', 'DBL2G', 'DBL2', 'ADD22PPMQ', 'ADD22B0',
                'ADD22C0', 'ADD22G', 'ADD22', 'ADDDIVISOR'}
    assert set(names) == expected, ('function coverage', sorted(set(names) ^ expected))
    program = '\n'.join(output) + '\n'
    namespace = {'TRACE': TRACE, '__': None}
    exec(compile(program, str(source_path) + ':strict-python-translation', 'exec'), namespace)
    return namespace, program


def _expr(value: str) -> str:
    value = value.strip().replace('^', '**')
    value = re.sub(r'\beq\b', '==', value)
    value = re.sub(r'\bne\b', '!=', value)
    value = re.sub(r'\b([A-Za-z][A-Za-z0-9]*)\[(\d+)\]',
                   lambda m: f'{m.group(1)}[{int(m.group(2))-1}]', value)
    assert not re.search(r'(?<![=])=(?!=)', value), ('unexpected equality', value)
    return value


def set_field(p: int) -> None:
    global MODULUS
    assert p in (7, 11)
    MODULUS = p
