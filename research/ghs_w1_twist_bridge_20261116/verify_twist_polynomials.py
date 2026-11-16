#!/usr/bin/env python3
"""Independent binary-polynomial check of the two genus-seven equations."""
v, u_plus, u_minus = 0x8f, 0x802f, 0xc07a
square = sum(1 << (2*i) for i in range(v.bit_length()) if v & (1 << i))
assert square == 0x4055
assert u_plus ^ u_minus == square
print(f'PASS v²={square:#x}, u_plus+u_minus={u_plus ^ u_minus:#x}')
