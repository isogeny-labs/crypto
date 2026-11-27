#!/usr/bin/env python3
"""Fit the measured PKM F4 cost law and compare it with rho's exponent.

Inputs are the measured numbers in ENCODING_WIDTH_DIAGNOSTIC_20261125.md and
the 13/14/15-bit closed-pipeline panels. Output is a table, not a claim.
"""
import math

# Measured F4 multiply-adds per call, interval arm (encoding diagnostic + T=7 probe).
C5 = 127.4e6
C6 = 2002.9e6
C7 = 79.748e9
RATIO = C6 / C5
RATIO7 = C7 / C6
LOG2_RATIO = math.log2(RATIO)

print(f"F4 cost ratio T=5->6: {RATIO:.2f}x   log2 = {LOG2_RATIO:.2f} bits")
print(f"F4 cost ratio T=6->7: {RATIO7:.2f}x   log2 = {math.log2(RATIO7):.2f} bits")
print("T=7 cost grows faster than the T=5->6 law; the wall steepens.")

# PKM m=2 heuristic: the factor base needs |V| ~ p^(1/2), i.e. T ~ 0.5 log2 p.
print("\nbits  T_req  C_F4 (MA)   log2 C_F4  C_F4 exponent in p   rho exponent")
for bits in (13, 15, 32, 59, 96, 128, 192, 256):
    T = 0.5 * bits
    C = C5 * RATIO7 ** (T - 5)
    print(f"{bits:4d} {T:6.1f} {C:12.3e} {math.log2(C):10.1f} "
          f"{math.log(C, 2) / bits:18.2f} {0.5:14.2f}")

print("\nFixed-T measured growth (T = 5): charged GAE per cold solve")
# 13-bit E2E pooled, 15-bit E2E pooled; 48 solves per arm-side.
for label, gae in (("13-bit pooled", 23.206e9), ("15-bit pooled", 67.97e9)):
    print(f"{label}: {gae/48:.3e} GAE/solve")
print("13->15 bits: "
      f"{(67.97e9/48)/(23.206e9/48):.2f}x over 2 bits, "
      f"exponent {math.log2((67.97e9)/(23.206e9))/2:.2f} per bit")
print("rho exponent: 0.5 per bit")

print("\nConclusion:")
print("- fixed T: the measured cost grows faster than rho's 0.5 exponent;")
print("- growing T to keep |V| ~ p^0.5 makes C_F4 ~ p^(log2(ratio)/2),")
print("  which is far above p^0.5, so the 32x constant cannot be subexponential.")
