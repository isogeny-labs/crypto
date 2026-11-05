# Bitplane decoder handoff (T-1580/T-1581)

## What was done
Implemented exact transposed support/parity/coefficient planes with signed
popcounts and modular Horner accumulation. All237 native methods pass, none
filtered: full479-point identity,27 ternary repeated-offset cases,729 word-boundary
cases, domain/oracle controls and8 recovery receipts.33 prebuild locks pass.
Source imported unchanged; recovery differs only by two checked module routes.
One-page report rendered and visually inspected.

## Decisions made
Preserve zero support separately from multiplicity parity. Use i128 signed digit
accumulation and Euclidean reduction; preserve oracle call/error order and avoid
decoder-owned query heap allocations. Stored plane payload328 u64 entries=2624
bytes;144 popcount calls and9 modular Horner steps are derived counts, not timing.
Constructor transposition, integer/bit work and query scratch remain in the next
complete comparison. Existing coefficient packet and source unchanged.

## Open threads
Instrument complete decoder/setup costs, then separately bind and freeze a
matched recovery comparison once admission is established. Coefficient-program
construction and term-count scaling remain distinct tasks. Original4563-case
cohort and previous panels unchanged.

## Errors
No build/test/render failure.15 compiler warnings retained. Literature context
was read from the primary bit-serial arithmetic paper; FPGA results are not
attributed to this CPU implementation. No comparative panel/cloud job ran.
