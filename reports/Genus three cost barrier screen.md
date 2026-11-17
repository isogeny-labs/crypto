# Quartic geometry survives the cold-cost failure

The surviving mathematical mechanism is **genus-3 transfer to a non-hyperelliptic plane quartic, followed by a scalable implementation of Diem’s line-based index calculus**. Its established destination exponent is linear in field size up to logarithms; assigning the specialized algorithm exponent 4/3 is incorrect. The complete-cold failure rejects the present implementation on its registered input, while leaving this conditional mechanism mathematically viable. The immediate prerequisite is exact source Weil-polynomial recovery without quadratic extension-field enumeration, gated by independently computed Hasse–Witt residues and a uniqueness certificate. That prerequisite addresses one component; construction, maps, useful relation coverage, sparse solving and descent still need complete bounds. **Neither a numerical crossover nor a full 10× improvement across genus 2+ is established.** This report synthesizes existing notes and records; no scientific execution was performed. [pcc:unverified]

## Complete-cold costs reject a map-only successor

The earlier RESULTS record was absent from the working tree but was read directly from published commit `07d5141c2c7c`, at `research/hec_coldcost13_20261115/RESULTS.md`. Its 20 cells comprise five repeats of each of four arms on one public genus-3 pair. Cold quotient-plus-mapped IC had median **1780.974708 ms**, versus **0.797166 ms** for selected native Compact: a 2234.13-fold cost ratio. Meeting the original 10× objective on this case requires candidate cost at most **0.0797166 ms**, approximately a 22,341-fold reduction from the recorded candidate median. These are local ARM L0/unisolated, producer-unvalidated observations; repeats are not independent curve samples, extrema are not confidence intervals, and phase medians are not additive. ([Cost-screen note](../research_notes/Genus%20three%20cost%20barrier%20screen/regime.md)) [pcc:unverified]

The per-cell total after subtracting both maps has median **240.507208 ms**; relations, descent and verification alone have median **52.211917 ms**. Thus free maps with the remainder unchanged still fail the requested threshold. This is a descriptive counterfactual about recorded costs, not an impossibility theorem for another algorithm. Source Weil recovery accounts for a reported median 4.234041 ms, so removing it alone cannot close the gap either. ([Weil-screen note](../research_notes/Genus%20three%20cost%20barrier%20screen/weil.md)) [pcc:unverified]

## Diem’s linear exponent survives, implementation bounds do not

Generic fixed-genus double-large-prime IC has exponent **2−2/g**, giving 4/3 at genus 3. Diem’s specialized non-hyperelliptic genus-3 algorithm instead has **expected Õ(q)** complexity; the expectation concerns algorithm randomness, not a random-curve ensemble. Geometrically, a line through two prescribed quartic points leaves a residual degree-two divisor. The earlier Diem–Thomé analysis is heuristic; Diem’s 2012 theorem establishes the linear exponent. The KB attribution of 4/3 to this specialized method must therefore be corrected rather than used to dismiss its potential advantage. ([GTTD](https://doi.org/10.1090/S0025-5718-06-01900-4); [Diem–Thomé](https://members.loria.fr/EThome/files/non-he-genus3.pdf); [Diem, Theorem 1](https://www.numdam.org/article/JTNB_2012__24_3_639_0.pdf)) [pcc:unverified]

An efficiently evaluable isogeny preserves scalar multiplication, so a map injective on the target prime-order subgroup preserves its logarithm. **The advantage comes from destination relation geometry**, conditional on applicable transfer and subgroup preservation. Smith’s established genus-3 construction uses a (2,2,2) kernel; the current order-27 (3,3,3) construction has different admission obligations. Smith’s conditional approximately 18.57% applicability prediction cannot be assigned to it. ([Smith, §§3, 8–9](https://arxiv.org/html/0806.2995)) [pcc:unverified]

The implementation reviewed in the notes does not inherit the linear theorem. Source counting enumerates the quadratic extension, yielding Θ(p²) character tests if generalized. Mapping uses capped cyclic scans without a justified hitting bound. A fixed 48-point factor base supplies no growing-family coverage guarantee; dense reduced elimination would cost approximately p^(3/2) if its dimension grew as √p. Fixed-degree extension arithmetic and fixed-dimensional Jacobian RREF chiefly inflate constants, whereas these enumeration and matrix choices create distinct scaling barriers. Kernel success, field construction, LP fill-in, rank and descent tails also lack implementation-level bounds. ([Source audit](../research_notes/Genus%20three%20cost%20barrier%20screen/regime.md)) [pcc:unverified]

## Weil recovery is an exactness-gated prerequisite

For squarefree degree-seven f over an odd prime field, write

\[
L(T)=1+a_1T+a_2T^2+a_3T^3+pa_2T^4+p^2a_1T^5+p^3T^6.
\]

With Wᵢⱼ=[x^(pi−j)]f^((p−1)/2), **L(T)≡det(I−TW) modulo p**. Hence N₁=p+1+a₁ and N₂=p²+1−a₁²+2a₂; a₁ is minus the Frobenius trace. Recovering residues and lifting them can replace extension-field point enumeration, but coefficient computation and certification remain charged work. ([Harvey–Sutherland](https://doi.org/10.1112/S1461157014000187); [Shi, §§5–6](https://arxiv.org/html/2602.00965v1)) [pcc:unverified]

The derived criterion fixes exact a₁ and a proved prime ℓ≠p dividing the Jacobian order, then considers b≡a₂ mod p with |b|≤15p. For each b, CRT constrains c≡a₃ mod p and c≡−A−(p+1)b mod ℓ, where A=p³+1+(p²+1)a₁. Retain exactly those pairs for which **h(x)=x³+a₁x²+(b−3p)x+c−2pa₁** has all three roots in [−2√p,2√p]. This is equivalent to the reciprocal polynomial being p-Weil. Exact endpoint and critical-point signs, including repeated roots, give a finite uniqueness test: one survivor certifies recovery because the actual curve polynomial is included; zero indicates inconsistent premises, and multiple survivors require additional certified information. ([Full derived criterion](../research_notes/Genus%20three%20cost%20barrier%20screen/weil.md)) [pcc:unverified]

**The fixture residues were not computed.** For p=127, a₁=1 and ℓ=1028893, the note conditionally derives uniqueness from the expected determinant **1+T+84T²+46T³ mod 127**, obtaining a₂=−43, a₃=−1224 and #J=2057786. Those residues are implied by the old recorded polynomial, not independent Hasse–Witt evidence. The immediate next prerequisite is independent residue computation on the exact public curve, followed by exact candidate counting, subgroup-divisibility validation and agreement with the archived polynomial. Any determinant mismatch or second survivor rejects this certificate. Published coefficient/lifting methods supply alternatives to quadratic enumeration, but their bounds are not measured replacement latencies here. ([Weil note](../research_notes/Genus%20three%20cost%20barrier%20screen/weil.md); [Harvey–Sutherland II](https://arxiv.org/html/1410.5222v1)) [pcc:unverified]

## Admission requires every charged phase and separate genera

A successor needs a stated input family, transfer applicability and failure probability, certified subgroup preservation, bounded kernel/quotient construction and general-class map evaluation, algebraic support extraction, useful independent relation coverage, controlled LP expansion and matrix nonzeros, sparse solving, and bounded target descent. For native cost T₀ on the same public pair, require **Tnew=Corder+Cconstruction+Cmaps+Crelations+CLP+CLA+Cdescent+Cverify≤T₀/10**, including setup, rejected attempts and retries. A model Ctransfer(p)+A(p)p≤B(p)p^(4/3)/10 expresses a possible regime; without constants and all phase bounds it predicts no crossover. A retained phase of exponent ≥4/3 defeats a claimed polynomial advantage over hyperelliptic doubleLP. ([Regime screen](../research_notes/Genus%20three%20cost%20barrier%20screen/regime.md)) [pcc:unverified]

Genus 2 cannot access non-hyperelliptic genus-2 geometry; genus 4 additionally needs a Jacobian-locus construction for its quotient. Point sieving with public labels and sparse recombination is established prior art and already a repository proposal, with useful-rank yield and expansion costs unresolved. The affine-pencil candidate reproduces ordinary composition/reduction. Neither supplies a newly admitted full-cost mechanism. Any later measurement requires frozen same-input comparisons against the fastest admitted native arm, full principal-identity and scalar checks, held-out curves, retained failures, matched resources and independent replay. ([Alternatives screen](../research_notes/Genus%20three%20cost%20barrier%20screen/alternatives.md); [Sieving primary source](https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf); [Smith, §9](https://arxiv.org/html/0806.2995#S9)) [pcc:unverified]

## Conclusion

The productive next step is **certified source recovery followed by a complete mathematical cost certificate for transfer plus scalable quartic IC**. Passing the residue gate would remove a concrete prerequisite; experiment admission still depends on the joint inequality and coverage bounds. The screen preserves a known genus-3 exponent mechanism while declining to promote it into a 10× result or a genus-2+ solution. [pcc:unverified]

Source scope: all three supplied notes and the earlier RESULTS at commit `07d5141c2c7c`; primary Diem, Smith and Shi pages were also opened. KB revision `9141e386254340a9bdba3036f2bb706a`, refreshed 2026-11-14T21:44:48Z, predates these local records; lexical searches for the quartic exponent and cold-cost barrier supplied pointers, not replacement evidence. No implementation, timing, residue computation or independent replay was performed for this report. [pcc:unverified]

## Executed follow-up, 15 November 2026

After this desk study, a separately frozen correctness worker independently
computed the expected Hasse-Witt matrix/residues, matching108polynomial oracle
cases. A separately committed exact-lift amendment recovered the unique
fixture polynomial and order2057786 without extension enumeration, conditional
on the reused public subgroup check. Both first executions exited0, retained
raw output and had pre-run S3 checkpoints. General ambiguous lifting remains
unsupported; no timing or10xgain measured. The earlier uncomputed-residue
statements describe this report's original screening stage, now superseded
by the [executed results](../research/hec_weil_recovery13_20261115/RESULTS.md).
[pcc:unverified]
