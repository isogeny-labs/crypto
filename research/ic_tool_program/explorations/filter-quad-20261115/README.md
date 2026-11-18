# Two extra filter bits for Top tables (exploration, dropped)

- **Status:** an exploration on this session's container, 2026-11-15, three
  isolated rounds an arm. This is a record, not rounds, and supports no
  claim. It ran on top of the doubled filter (one extra bit, PR #4360),
  through a temporary `KIC_FILTER_EXTRA_BITS` override that is not
  merged.
- **Question:** does a filter four times today's width, two extra bits,
  pay beyond the doubled one?
- **Result** ([`explore/`](explore/)), cycles a summand, median of three:

  | row | step | one extra bit | two extra bits |
  |:--|:--|:--|:--|
  | k0n53 | admitted rate | 0.61% | 0.14% |
  | k0n53 | admitted | 1.68 | 0.91 |
  | k0n53 | filter | 7.67 | 8.43 |
  | k0n61 | admitted rate | 0.42% | 0.10% |
  | k0n61 | admitted | 1.13 | 0.67 |
  | k0n61 | filter | 8.17 | 8.80 |

  Collect overlapped between the arms at both sizes: k0n53 154–211 ms
  against 151–189 ms, and k0n61 817–992 ms against 833–914 ms.
- **Reading:**
  - The wider filter's probe misses cost about what the fewer false
    positives save: −0.5 to −0.8 cycles admitted against +0.6 to +0.8
    cycles filter.
  - So one extra bit is where this lever stops, and two were not taken
    further.
