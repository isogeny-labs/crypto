# Plan: index-calculus speed-up for genus 2-16

Synthesised 2026-11-17 from the four research notes in `research/` (A literature,
hyperelliptic; B lab baselines; C engineering levers; D non-hyperelliptic). Every number
quoted here is `[pcc:unverified]`: the notes could only read search snippets, not full
papers (eprint, arXiv and HAL were not reachable), and lab numbers are quoted, not re-run.

## 1. What the research says, in five lines

1. **Exponents.** Fixed-genus hyperelliptic IC has been stuck at q^(2-2/g) (double large
   prime) since 2007. Non-hyperelliptic curves do better: the exponent is 2-2/e, where e is
   how many residual points of a relation must land in the base. e = g for generic
   relations, g-1 for canonical or degree-(g+1) plane models, and smaller for special
   low-degree models (note D, section on levers).
2. **The hidden constant.** Relations need a degree-e divisor to split completely, which
   happens with probability about 1/e!. From genus 6 upward this factorial dominates.
3. **Where the cost goes.** With the base retuned, linear algebra's share of total cost is
   about (g-2)/g (note C, section 5.0): 33% at g=3, 75% at g=8, 88% at g=16. So relation
   levers fade with genus and linear-algebra levers grow.
4. **Realistic engineering gains** are 2-4x at g=3-4 and 2-3x at g=8-16 (note C), from
   sieved relations, structured elimination before Wiedemann, and closed-form root tests.
5. **The lab has no IC instrument above genus 5** (note B). Genus 6-16 has never been
   measured, only modelled.

## 2. The opportunity

The largest gains are not constant-factor tuning at genus 3-5, where the lab has already
pushed. They are:

- **(G1) A first real baseline for genus 6-16.** Without it no speed-up there can be
  measured at all.
- **(G2) Exponent gains from curve models with small e** (non-hyperelliptic), which change
  the exponent rather than the constant.
- **(G3) Attacking the e! factor** at larger genus, by allowing degree-2 places in the base
  or as large primes. This is unpublished for fixed genus (note A), so it is a genuine open
  question rather than a known result.
- **(G4) Linear-algebra levers** (structured elimination, filtering) for g >= 6, where LA is
  most of the cost.

## 3. Phases

Each phase ends with a frozen protocol, a pilot showing every gate is reachable, a run, an
independent assessment, and a commit. Runs are kept under ~20 minutes and each cell is
written as soon as it finishes, because this container restarts about every 30 minutes and
kills unfinished processes.

**P0. Recover the newest lab instrument.** Build `genus_ladder` from commit `fffa1b1243`
(the newest version, with arms `v`, `w`, `f`, `s`, `P`; note B), not the older `a552dc2ee8`
copy. Gate: reproduce one recorded cell exactly (count identity). This is the g = 2-5
reference.

**P1. A standalone instrument for genus 2-16 (hyperelliptic).** New Rust crate in this
directory, single-threaded, counted field multiplications. Contents: Cantor arithmetic for
any g <= 16 over F_p (p < 2^31), degree-1 factor base, single and double large primes,
Pollard rho with the negation map as the matched reference, and a sparse solve with an
optional structured-elimination pass. Gates: (a) count identity against `genus_ladder` at
g = 2-5 on shared cells; (b) every solve recovers and verifies a planted logarithm.
Deliverable: the first measured IC/rho table for g = 6-16, at the largest p that fits the
run window per genus.
*Update 2026-11-17:* built as a patch to the lab's `genus_ladder` rather than a fresh crate.
The group order comes from F_{p^k} point counts. Arms D and E (degree-2 places) were added
early, because the pilot showed degree-1 bases collapse at the small p where #J is
computable. See `P1/DESIGN.md`.

**P2. Levers, each as a paired arm in the P1 instrument.**
- L-SGE: structured elimination before the solve (expected to matter most at g >= 6).
- L-BASE: retune the base constant per genus (the L03 finding generalised).
- L-DEG2: degree-2 places as large primes or base elements (attacks e!; open question).
- L-SIEVE: sieved relations via function families (note C, rank 1), with heavier rows
  charged to linear algebra.
Each is measured as a ratio to the P1 baseline on the same curves and seeds, and reported
as a distance to matched rho, never as a pass/fail on its own.

**P3. Non-hyperelliptic models with small e.** Start where the research is concrete:
genus 4 via a plane quintic with two nodes, and genus 6 via smooth plane quintics (e = 4,
exponent 4/3 per note D). Line or low-degree-curve relations, same cost unit, same matched
rho convention (no free negation credit; the L10 lesson). Compared with the hyperelliptic
curve of the same genus in P1.

## 4. Rules carried over from the last pass

- Freeze the protocol only after a pilot shows every gate passes at the study's own sizes.
- Count each relation once; check rank before linear algebra.
- Charge each arm for its own set-up.
- Report every lever both in field multiplications and in the GAE band, because the two
  units rank IC differently (note B: genus 4 is far below rho in one and not in the other).

## 5. Order of work

P0, then P1 at g = 2-5 (to pass the count-identity gate), then P1 at g = 6-16, then P2 and
P3 in parallel. P1 is the critical path: every later claim needs its baseline.
