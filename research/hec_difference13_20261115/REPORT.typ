#set page(paper: "a4", margin: 18mm)
#set text(font: "Helvetica", size: 10pt)
#text(size: 21pt, weight: "bold")[Decoding difference pairs from second jets]
#v(6pt)
Genus 3 · reconstructed Q1 over F127 · correctness gate · 15 November 2026

The gradient and Hessian of a reduced difference-surface equation recover the
unordered divisor pair on a generic transverse canonical chord. The native gate
uniquely decoded all 3,825 admitted rational-point pairs on Q1.

#image("PAIRS.svg", width: 100%)

*Geometric calculation.* At P − Q the gradient gives the canonical chord.
The Hessian pairs the P and Q directions to zero. For the complementary chord
pair its value is a nonzero root-difference product multiplied by 2. The four
pairs sharing an endpoint also give nonzero values. Odd characteristic and a
transverse chord therefore force uniqueness. A sum-surface jet instead retains
the residual pair. PROOF.md records the complete calculation.

*Coverage.* Of 8,256 pairs among 129 rational points, 306 were outside the selected
chart, zero failed the next derivative condition and 4,125 lacked four rational
chord intersections. All 3,825 remaining pairs passed; they are correlated because
several share a chord. Sum controls retained exactly two pairs in every case;
zero-Hessian controls retained six and were rejected as ambiguous.

*Execution.* Source f476d0dedabd, checkpoint 20261115T161127Z preceded a single
release run. Exit 0, empty stderr, first eight complete cases and aggregate
exclusions retained in RUN.txt. Point enumeration reuses pinned target code;
Hessians use the proved target derivative formula. No timing comparison or
independent derivative implementation is asserted.

*Connection to the quotient.* A separate written specialization and divisor-class
argument identifies the translated Gamma00 divisor with ψ(T) + (C − C), where T is the registered rational two-torsion point. A source zero x therefore decodes ψ(x − T). The next gate
must evaluate the actual translated source-section jets, check their constants
and derivative conventions, and apply this decoder. General public inputs still
need a decomposition into difference-surface points and consistent signs.

#text(size: 8pt)[Sources: Pauly–Previato, Singularities of 2theta-divisors, introduction (1.2);
Tian, arXiv:2007.03172 §4.2. Local evidence: PROOF.md, PROTOCOL.md, RESULTS.md,
RUN.txt and PAIRS.svg in research/hec_difference13_20261115. Standalone scientific
runner is not automatically executed by native CI.]
