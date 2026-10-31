#set page(margin: 2cm)
#set text(size: 10pt)
= R6.8 Transport oracles on factor-base points (Proposition 5′)
Round-6 lead R6.8. Toy instances (p ≈ 2^18–2^20); no sub-rho claim; no deployed-curve claim. Full proof and tables: `RESULTS.md`.

== Result
*Proposition 5′ (proved).* An index-calculus-style attacker with transport oracles $T_psi$ on arbitrary formed points is simulated exactly, with the same success probability and no dearer decomposition calls, by an attacker on $E_1$ without oracles whose factor base is the union of pulled-back bases $F union union.big_psi hat(psi)(F'_psi)$, of size at most $(1+|Psi|)|F|$ and independent of the instance. Pullbacks only grow (Thm 4.2, Lemma 5); the degree enters through the fibre size $|M_k| <= deg psi$ and the image index $i <= min(deg psi, h)$. No exponent moves.

*Falsifier (measured, exact census): did not fire* in 0 of 16 cells (4 curves × |F| ∈ {72, 144} × m ∈ {2, 3}). Union class factor per unknown / best single base: 0.02–0.27, threshold > |Ψ| ∈ {1, 2}.

*Measured gains are oracle-free:* pulled-back bases gain the image index (≈3× on j = 0 with ℓ = 3, ≈2× for ℓ = 2 on the 6r curve), matched within 2% by filtering small-x points into ℓE (no oracle). At m = 2 the union costs 0.31–0.44 of plain (derived, §15.2 model), inside the predicted interval [1/(k·i), 1/k]. m = 3 at |F| = 144 is near saturation (D ≈ r), a toy-size effect.

*Verdict: confirmed.* No §15.4 row moves.

#figure(image("class_factor_per_unknown.svg", width: 100%), caption: [Class factor per unknown by arm (exact census, |F| = 144); red dashed: falsifier threshold best single × |Ψ|.])
#figure(image("cost_ratio.svg", width: 100%), caption: [Derived §15.2 cost relative to the plain base; dotted: 1/k.])

== Limits
Four toy curves, one seed; class-factor bound (5.2) heuristic; non-IC attackers, transport of non-formed points and the CJ-DLP side are out of scope.
