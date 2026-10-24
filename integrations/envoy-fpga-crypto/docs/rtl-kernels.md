# FPGA hash and module-lattice arithmetic kernels

These SystemVerilog modules are building blocks for the missing FPGA backend
of the Envoy extension. They are separate from the existing FPC1 host ABI.
They do not advertise an algorithm catalog, implement key slots, or execute
complete ML-KEM/ML-DSA operations. No host operation has been reclassified as
hardware offload by adding these files.

| Requested component | Current implementation | Remaining integration |
|---|---|---|
| Keccak permutation | `rtl/keccak_f1600.sv`, all 24 Keccak-f[1600] rounds | Connect to the algorithm controller and host transport |
| SHAKE128/SHAKE256 | `rtl/shake_stream.sv`, length-delimited byte streams and multi-block squeeze | Wider streams, throughput tuning, side-channel review |
| Module-lattice multiplication | `rtl/lattice_modmul.sv`, canonical modular multiplication for both primes | Physical DSP mapping, placement and timing validation |
| Forward/inverse NTT | `rtl/lattice_ntt.sv`, the 256-coefficient ML-KEM/ML-DSA transforms | Pointwise/base multiplication, sampling, packing and full algorithm controllers |
| Complete ML-KEM/ML-DSA offload | Host API and explicit software plugin remain available | Complete algorithm RTL, entropy, key custody, FPC1 dispatch and F2 shell |
| Production F2 deployment | Not implemented or measured | Vivado/AFI build, device tests and workload measurements |

## Interface contract

Every module uses a rising-edge clock and an active-low asynchronous control
reset. Assert `clear` for at least one rising edge to cancel and erase a
command. Accept `start` only when `ready` is high. Command inputs are latched;
later changes and additional `start` pulses while busy have no effect.
Reset deassertion must be synchronized in the integrating clock domain.

`keccak_f1600` consumes a 1600-bit state. Lane `(x,y)` is the 64-bit field
starting at bit `64*(x+5*y)`, with the least-significant byte first. A command
performs one complete round per clock for 24 clocks after acceptance and
pulses `done`. Intermediate state is internal. The result remains until the
next accepted command, reset, or clear; the working state is wiped at completion.

`shake_stream` latches `mode_256`, `input_length`, and `output_length` at
command acceptance. Stream exactly that many input bytes using
`in_valid/in_ready`, and accept exactly that many output bytes using
`out_valid/out_ready`. A producer holds a pending byte until accepted. A
consumer may stall indefinitely; output stays stable while stalled.
The SHAKE128 and SHAKE256 rates are 168 and 136 bytes, respectively.
FIPS 202 SHAKE domain separation/padding is internal. Empty messages,
messages ending exactly at a rate boundary, zero output length, and
multiple squeeze blocks are supported. All sponge state is wiped before
returning to idle, and the nested permutation is cleared after each result
is consumed. The invalid output bus reads zero.

`lattice_modmul` latches two unsigned canonical residues and the public mode:

| Mode | Modulus | Required inputs | Result |
|---|---|---|---|
| `mode_dsa=0` | 3329 | `0 <= a,b < 3329` | `a*b mod 3329` |
| `mode_dsa=1` | 8380417 | `0 <= a,b < 8380417` | `a*b mod 8380417` |

It uses fixed-stage multiplication and Barrett reduction with
`mu=floor(2^48/q)`. Canonical inputs satisfy `a*b < q^2 < 2^48`; the
approximate quotient underestimates the true quotient by at most one.
The provisional remainder therefore lies in `[0,2*q)`, so one conditional
subtraction is sufficient. Constants are 84552411147 and 33587228.
There is no runtime divider or data-dependent reduction loop. Valid commands
complete three clocks after their acceptance edge. Out-of-range operands
produce `done/error` and zero result on the acceptance edge. Successful
results remain until the next command/clear; intermediate registers are
wiped at completion.

`lattice_ntt` latches the public mode and `inverse` flag, then accepts exactly
256 canonical 32-bit coefficient words. It emits 256 canonical words, in
order, and erases each stored coefficient as that word is accepted.
The internal multiplier is cleared after every result is captured. NTT
memory is scrubbed for 256 clocks after reset/clear or invalid input;
`ready` remains low during that scrub. Clear cancels without a completion
pulse; invalid input reports `done/error` only after erasure. No partial
result is emitted on invalid input.

## Transform representation

All coefficients are normal residues, not Montgomery-encoded values.
Twiddles are constants derived at elaboration from root 17 for ML-KEM and
root 1753 for ML-DSA; the constant-generation function's divisions do not
become runtime arithmetic.

For ML-DSA, output word `i` is the input polynomial evaluated at
`1753^(2*BitRev8(i)+1) mod 8380417`.
For ML-KEM, words `2*i` and `2*i+1` are the evaluations of the even- and
odd-coefficient polynomials at `17^(2*BitRev7(i)+1) mod 3329`.
ML-KEM's transform stops at quadratic factors; its transformed multiplication
requires the appropriate two-coefficient base multiplication. Treating it
as 256 independent scalar products is incorrect.

The inverse uses normalization factors `128^-1 mod 3329 = 3303` and
`256^-1 mod 8380417 = 8347681`. These interfaces can be shared across the
three parameter sets in each algorithm family because their dimensions,
rather than their polynomial degree or field, change.

## Reproduce validation

With Verilator, a C++17 compiler, Make and OpenSSL development files:

```sh
make lint-rtl test-rtl
```

Full validation with Verilator 5.020 (Ubuntu 24.04) and fatal source lint
with 5.032 are recorded in the
[CI compatibility receipt](receipts/2026-10-18-ci-compat/README.md).
The Makefile sets `--unroll-count 256` so the older frontend can elaborate
the 200-byte SHAKE state-copy and erase loops. Fatal `-Wall` lint and
assertions remain enabled; no warning category is suppressed.

The test harness is native C++. It checks the Keccak zero-state known answer,
SHAKE known answers and independent OpenSSL XOF results across input/output
rate boundaries. It compares NTT output to direct polynomial evaluation and
inverse output to direct interpolation, not a second butterfly implementation.
Polynomial-product tests compare transform-based products to schoolbook
negacyclic convolution. Pointwise/base multiplication in those tests is C++
oracle code; it is not a hardware polynomial-multiplication command.

Other tests exercise busy command rejection, latched configuration, input gaps,
output backpressure, invalid residues, cancellation during different processing
phases, erasure before reuse, and simultaneous independent kernels. Public
fixtures use deterministic seed `0x4650433252544c`. A fixed transaction-schedule
check covers differing coefficients for each mode and direction; this is not
a proof of side-channel resistance.

For a second frontend and generic synthesis replay, install sv2v and Yosys:

```sh
make test-rtl-synth
```

This lowers SystemVerilog, checks the design for structural problems, rejects
runtime divider/modulo/latch cells, maps to generic cells, and replays the
same external test groups against the generated Verilog. `build/rtl/synthesis.log`
retains synthesis output. This is not a formal equivalence proof or a
Xilinx/Vivado implementation. The generic flow currently realizes coefficient
storage as registers; BRAM inference and physical resource optimization remain
unverified. Vendor-generated C++ is built with its compatible warning settings;
the source RTL still uses fatal Verilator `-Wall` checks.
Erasure assertions check source RTL state; preservation of wipe logic through
the vendor implementation must still be verified.

These cores are unmasked. Data-independent control and buffer erasure do not
establish power/EM leakage resistance, fault-injection resistance, entropy
quality, FIPS validation, clock frequency, or end-to-end acceleration. The
current byte-wide SHAKE stream and single-butterfly NTT are functional initial
designs. Do not infer throughput or CPU speedup from simulator execution time.

Algorithm references: [FIPS 202](https://csrc.nist.gov/pubs/fips/202/final),
[FIPS 203](https://csrc.nist.gov/pubs/fips/203/final), and
[FIPS 204](https://csrc.nist.gov/pubs/fips/204/final).
