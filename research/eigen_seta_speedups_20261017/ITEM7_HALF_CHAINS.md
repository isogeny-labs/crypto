# Item 7: half-length chains on a smaller prime (design study; not implemented)

**Verdict: undetermined, leaning not feasible as stated. Not recommended
under the owner's bar ("no major reduction").** No code was written.

Method: workflow `wf_22303879-66d` (2026-10-18), read-only. All 8 agents ran
on Sonnet 5.5:
- four independent studies: decapsulation algebra, torsion-point attacks,
  parameters, literature;
- one synthesis;
- three adversarial verifiers, by lens: torsion leakage, hidden orientation
  (R1), feasibility.

Two of the three verifiers refuted the synthesis's framing. The full JSON
output is in `item7_workflow.json`. Every number below is **derived** unless
marked otherwise. Nothing here was measured.

## The idea

Decapsulation evaluates one (2^E, 2^E) Kani chain. E = 1076 at L1, because
the scheme needs 2^E > 7 d² f². That forces p + 1 = c · 2^(E+1) · 3^F3, a
1341-bit p with 21 limbs. Splitting the chain into two halves that need only
2^(E/2+2) rational torsion would allow a p of about 807 bits (13 limbs).

## What the study found

1. **Diagram correction.** The decapsulation Kani square is E_C × E_C → E_C × E_C
   (`eigenx:src/kem.rs:903-920`). It has no E_A-side half, so the first draft
   of this note described the wrong diagram. A split Φ = Φ2 ∘ Φ1 through a
   middle surface M is algebraically plausible. It would publish no new
   values: pk and ct keep their shape at torsion order 2^N', N' ≈ 540.
2. **Feasibility is not established.** The kernel today is built from
   rational order-2^E points (`kem.rs:895-920`). With only 2^(E/2+2) rational
   torsion, the second half's kernel cannot be computed directly. The
   proposed workaround needs a middle isomorphism ι: M → M′, computed from the
   dual of a half of Φ̂ and matched via a 4-torsion anchor. That ι is
   UNVERIFIED to exist uniquely.
3. **No precedent found.** No published technique doing this was found. The
   draft's attribution to SQIsign2D-West/East and the HD-representation work
   is unsupported. Some ePrint PDFs returned HTTP 403, so absence is not
   proven either.
4. **Speed.** The arithmetic ceiling is (13/21)² = 0.38, i.e. 2.6×. A
   straightforward design needs about three half chains (about 1.5E steps),
   which gives about 1.75× under schoolbook cost and about 1.4× under
   Karatsuba-like cost. Neither estimate has code behind it. Honest range:
   between roughly 1× and 2.6×.

## Security delta (L1; L3 and L5 behave the same)

| quantity | now | half-chain | note |
|---|---|---|---|
| sender MITM, classical | 128.4 | 128.4 | no dependence on p |
| mask guess + Kani, classical / quantum queries | 129.0 / 64.5 | 129.0 / 64.5 | holds while N′ ≫ d |
| hidden orientation (row E) | 138.5 | 138.5 | **true by construction**: the model row is f_bits/2 and cannot see the change |
| End(E_A) generic, classical / quantum queries | 447 / 335 | ≈ 269 / ≈ 202 | non-binding, but only 2× the floor |
| gate floor, MD 2^40 | 151.2 | ≈ 148.8–150.0 | −1.2 to −2.4 bits (smaller-field oracle) |
| **torsion over log2(d·f) ≈ 534** | **≈ 542 bits** | **≈ 6 bits** | the qualitative change |
| torsion vs deg ψ_A ≈ 2^557 | 2^1076 covers it | 2^540 does not | direction (helps or hurts) UNVERIFIED |

The modelled rows move by at most about 2.4 bits, which would count as
minor. The model cannot see the main risk, though. Today the torsion exceeds
the composite degree d·f by about 540 bits. After the redesign the margin is
about 6 bits. That margin covers the open mask-free-embedding question
(`docs/SECURITY.md` Q2) and attacks on torsion masking (ePrint 2026/1944,
read as a snippet only). The new p also changes the geometry of ψ_A against
√p, which is relevant to R1. None of these was analysed for a concrete new
prime. "No leak found in the model" is therefore **not** a clearance.

## Why this is not pursued now

Under the owner's bar, a change can ship only if its security reduction is
shown to be minor. Item 7 has unproven feasibility and an unknown speed-up.
It also has a qualitative loss of torsion margin that no analysis covers.

Items 1–6 (eigenx PR #5) remove 32% of decapsulation instructions with
**zero** security change, which is the better trade.

## What would reopen it

1. A written construction of ι, with its uniqueness proved or brute-forced in
   constant time.
2. An analysis of mask-free embeddings and torsion-masking attacks at
   N′ ≈ log2(d·f) + 6, and of the ψ_A-vs-√p change for R1.
3. Only then: a NUMS prime search, a new parameter set next to the current
   ones, and the KEM, KAT, CT and security_estimate gates.
