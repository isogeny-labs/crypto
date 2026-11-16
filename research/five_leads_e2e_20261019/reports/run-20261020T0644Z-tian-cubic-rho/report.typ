#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Tian F523³ source-rho control

*Receipt written:* 2026-10-20T06:44Z UTC. *Command:* `tian-cubic-rho` from `native/Cargo.toml`. *Host:* shared Intel Xeon 6975P-C; L0 operation counts only.

Five independent seeded negation-rho walks recovered the published scalar 1987 for Q=1987P on the prime-order elliptic curve over F523³. Every answer was checked by recomputing kP. The cubic-field inversion uses the norm formula and charges one base-field inversion; 1,024 sampled nonzero inverses were multiplied back to one before the runs. The five complete M/S/I vectors, native walk counts and source points are in `correctness/tian-cubic-rho-v2.jsonl`.

#figure(image("rho-multiplications.svg", width: 100%), caption: [Recorded base-field multiplication counts for five verified source-rho solves; squarings and inversions are retained separately in the raw vectors.])

The destination group-law check is a separate run. These controls alone do not execute the cubic transfer attack; no wall-speed or external-system comparison is claimed.
