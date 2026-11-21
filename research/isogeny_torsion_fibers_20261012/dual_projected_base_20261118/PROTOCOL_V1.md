# Frozen reused-setup dual-isogeny panel

**Frozen 2026-11-18 PST before implementation or execution.** This is the
separate 64-target panel required by the [dual-transport protocol](../dual_projected_base_20261115/PROTOCOL_V1.md).
The earlier four-target cold result remains the primary N1 comparison. A
failure, timeout, cap event, or protocol change gets a new receipt or amendment;
it does not overwrite a completed child.

## Question, boundary, and falsifier

Does constructing the degree-three dual map and the B=64 projected factor
base once for 64 public targets reduce *whole-batch* counted work relative to
constructing the same base once with inverse Vélu, destination-direct
relations, and same-target signed rho? The 64 targets are all in the prime
order-229 subgroup of `E: y²=x³−21x+37` over `F_(211²)`. The destination is
`E′: y²=x³−81x+179`. The maps, dual kernel, exceptional cases, and
composition identities are those independently checked in the prior run.

The prespecified two-class generic floor is `sqrt(pi/4)=0.886227` group
operations per `sqrt(229)` for a single target. The reference is signed rho
on **each of the same 64 targets**, including failed attempts and final
verification. BSGS and rho A/A are controls. The main descriptive unit is
whole-batch additions and doublings divided by `64*sqrt(229)`; also report
the undivided whole-batch count. This is `S_group`, a partial operation view.
Complete portable `S` is unset until all primitive and process costs are
measured and calibrated. A cheaper setup phase alone is not a batch or cold
ECDLP improvement. A batch that repeats construction per target fails the
reuse gate.

## Inputs and information boundary

- The dual candidate receives only the witness-free
  [`DUAL_BASE_PUBLIC_V1.json`](../dual_projected_base_20261115/RUN_V1/DUAL_BASE_PUBLIC_V1.json),
  SHA-256 `59864a39ea05ab57823a7e072822ea781aaf625bfcc0a1752faeab63ef819383`.
  Inverse and destination controls receive the original full base, SHA-256
  `f4e0b303e37173c0a764e717dffe571facf0c39e492869712e0d11090d46b73f`.
- Freeze a new 64-target public file from the prior four public targets, SHA-256
  `e1969305462131722556dead399593b77a919a8017f0e035e3019986b645958e`.
  Preserve those four in positions 0–3. For the remaining 60, use native
  `StdRng` seed `0x2026_1118_0064`, draw residues in `1..229`, reject
  duplicates and the first four residues, and publish `[k]G` and its forward
  image. Put the 64 scalars in a separate checker file. The solver reads only
  public points; the checker reads both files. Freeze both output hashes before
  the first solve child. Distinctness, curve membership, subgroup order, map
  equality, and the checker/public hash link are mandatory gates.
- Use B=64 and the first 64 frozen destination columns. Use 3 rounds. For
  target position `i`, keep the prior trial seed formula
  `(0x2026_1015 ^ 211) + round*0x9e37_79b9 + (i+64)*0x0123_4567`, with
  `u64` wrapping arithmetic. Rotate the six-arm order by
  `(round+i) mod 6`. The arms are direct dual, direct inverse,
  destination direct, source BSGS, signed rho, and rho A/A.

## Execution and acceptance

Run one native child per arm and round: 18 children. Each child reads its
public input, constructs its map/base/table **once**, then solves all 64
targets sequentially. Charge setup, every failed construction candidate,
relations, row reduction, recovery, verification, process startup and I/O to
that child. Keep per-target group and base-field counters, trial and relation
counts, recovered scalar, witness rows, CPU and wall, plus child-level setup,
peak RSS, and totals. The cap is 1,832 relation trials or 32 rho retries of
4,580 steps per target, 300 seconds wall per child, and 512 MiB address space
where enforced. On Darwin, label memory and timing L0 if the cap and physical
core isolation are unavailable. Preserve every completed or censored target.

An independent native checker, using separate field and curve arithmetic,
must validate public input separation, every target scalar and mapped scalar,
all relation witnesses and row ranks, exact pairing of the dual and inverse
columns, and child phase sums. Every relation arm must reach rank 65 on every
target. Every returned scalar must pass `[k]G=Q` and its mapped equality.
Compare paired whole-batch totals only after all 18 children and replays pass.
Report failures and censored cells separately, never as completed ratios.
Preserve exact commands, source/input/binary hashes, raw children, replay
receipts, host and isolation record, audit, graph, SVG/PNG, and PDF. Copy the
performance visual into `benchmarks/` and index it when the shared index
reservation clears. Do not promote a complete portable price or isolated wall
ratio from this L0 counted-work panel.

The next accounting gate remains a disjoint native primitive census and
independent counter replay over this exact workload, followed by calibrated
conversion on both candidate and controls. The larger-order dual-versus-inverse
comparison and L2 isolated timing require separate frozen admission records.
