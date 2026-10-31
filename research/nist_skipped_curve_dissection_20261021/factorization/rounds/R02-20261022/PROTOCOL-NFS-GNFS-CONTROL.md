# Qualify the actual GNFS path

The corrected 85-digit control produced the two expected prime factors,
but native logs show that `nfs(N)` dispatched it to SIQS at the tool's
default crossover of 95 digits. This validates expression/result handling,
not GNFS coverage. Preserve that successful SIQS fallback control.

Before production, run the same exact control through the actual GNFS
path with **`-gnfs -xover 81`**, two threads, seed 20261022, and the same
300-second native / 360-second outer bounds. The pinned polynomial-search
implementation supports inputs from 81 digits. These two selection flags
also apply to production/resume attempts, whose current inputs have 110
and 134 digits. No production attempt has started yet.

The bundled lattice-siever binaries were checked out without executable
mode. Grant user execution permission to those same pinned binaries and
retain their content hashes. No siever code is modified. A successful GNFS
control requires the proper factors and separate exact native verification;
setup failures remain setup failures. All production allocations and
acceptance criteria in the preceding protocols are unchanged.
