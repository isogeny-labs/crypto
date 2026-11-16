from pathlib import Path
p=Path(__file__).parent
(p/'REPORT.md').write_text('''# Widened modular powers and generated inverse certificates

T-1154, 2026-10-31. Producer formal/source-model packet. Independent source-fidelity review pending.

For arbitrary UInt64 operands, their product fits in128 bits. The checked BitVec128 model of source widening, multiplication, remainder and narrowing equals exact natural modular multiplication for positive modulus; the remainder fits back into UInt64. This primitive does not need the earlier p<2^31 restriction.

The binary-loop proof models the source parity test, one-bit right shift and skipped final squaring. Its invariant is decoded accumulator times decoded base raised to the remaining exponent. A fuel theorem assumes e<2^fuel;64 steps cover every UInt64 exponent. For1<p and canonical base, the modeled modpow returns a canonical word decoding to the expected power, including exponent zero.

For prime p and nonzero canonical input a, exponent p-2 generates a canonical inverse word. The proof uses the standard finite-group-cardinality/Fermat argument through already imported lemmas. Under the prior p<2^31 multiplication-verifier bound, the generated word satisfies InverseWitness. The final theorem instantiates the earlier coefficient-update proof with this generated inverse, removing its supplied-inverse premise.

Fourteen widened-loop declarations, six inversion/composition declarations and eighteen fixed controls are checked, followed by38 named axiom readbacks. Controls include p17 inverse2->9, identity/negative-one inverses, exponents0/1 and2^64-1, zero base, maximum operand products and the composed coefficient-slot result. Widened2^63*2 modulo5 is1 while narrow UInt64 multiplication gives0. For composite p15,a2, the p-2 generator returns2 and fails the inverse certificate. Zero modulus, modulus one and zero inversion input remain outside the respective theorem domains.

The literal source excerpts bind Field::bi, the Field::inv dispatch and modpow at three_leads.rs195-200,307-332,588-600. The source path is odd-characteristic n=1 arithmetic without diagnostic tables. BitVec128 is the explicit logical model for Rust u128 operations; this is not a compiler or full-array execution proof. Independent source-fidelity review, array-loop composition and geometric class transport remain separate obligations.

The first widened proof attempt needed the explicit UInt64.ofNat conversion lemma. The first Fermat attempt needed an implicit argument instantiated. Corrected attempts and controls passed; nonfatal tactic-deprecation warnings are retained. Two exploratory cache planners failed on import parsing; no cache was copied, downloaded or built. The proof instead reused existing group-cardinality imports. Cache-planning error summaries are retained separately from native Lean receipts. Every Lean check records native exit and unchanged input bindings; readback admits only standard Lean axioms.

Primary sources: Lean Bitvectors logical semantics (https://lean-lang.org/doc/reference/latest/Basic-Types/Bitvectors/); Mathlib FiniteField.pow_card_sub_one_eq_one and ZMod Fermat documentation (https://leanprover-community.github.io/mathlib4_docs/Mathlib/FieldTheory/Finite/Basic.html), checked against pinned source. Established identities are reused; no novelty claim.

No scientific campaign, curve enumeration, timing or cost ratio. Frozen cohort and campaign limits unchanged. The diagram represents proof dependencies and scope; independent binary rechecking is not claimed.
''')
(p/'report.tex').write_text(r'''\documentclass[10pt]{article}
\usepackage[margin=.65in]{geometry}\usepackage{amsmath,amssymb,tikz,hyperref}
\setlength{\parindent}{0pt}\setlength{\parskip}{5pt}
\begin{document}
\textbf{Widened modular powers and generated inverse certificates}\hfill2026-10-31\\T-1154. Producer source-model proof; independent source-fidelity review pending.
\section*{Checked chain}
For arbitrary UInt64 operands, $ab<2^{128}$. The BitVec128 model of widening, multiplying, reducing and narrowing equals exact natural modular multiplication for positive modulus. Its remainder fits back into UInt64; this primitive needs no $p<2^{31}$ bound.

The binary-loop invariant is $r\,a^e$. Parity, right shift and skipped final squaring preserve it. The fuel theorem assumes $e<2^{\mathrm{fuel}}$, so64 steps cover every UInt64 exponent. For $1<p$ and canonical base, the result is canonical and decodes to $a^e$.
\begin{center}\begin{tikzpicture}[every node/.style={draw,align=center,font=\small}]
\node (a) at (0,0) {128-bit multiply\\exact modular value};
\node (b) at (4,0) {Binary loop\\all UInt64 exponents};
\node (c) at (8,0) {Prime $p$, exponent$p-2$\\generated inverse};
\node (d) at (4,-1.1) {Word inverse certificate and coefficient update\\additional $p<2^{31}$ verifier bound};
\draw[->](a)--(b);\draw[->](b)--(c);\draw[->](c)--(d);
\end{tikzpicture}\end{center}
For prime$p$ and nonzero canonical input, the standard finite-group-cardinality argument gives the inverse. Under $p<2^{31}$, its word product supplies the preceding InverseWitness. The composed coefficient-update theorem now uses this generated inverse.
\section*{Controls and readback}
Fourteen loop declarations, six inversion/composition declarations and eighteen controls receive38 named axiom readbacks. Controls include inverse$2\mapsto9$ modulo17, identity/negative-one inverses, exponents0/1/$2^{64}-1$, zero base, maximum operands and the composed slot result.

The widened value $2^{63}\cdot2\bmod5$ is1; narrow UInt64 multiplication gives0. At composite$p=15,a=2$, exponent$p-2$ returns2 and fails the inverse certificate. Zero modulus, modulus one and zero inversion input are excluded from their respective theorem domains.
\section*{Source and evidence}
Source excerpts/hashes bind \texttt{three\_leads.rs}:195--200,307--332,588--600. The source dispatch is odd-characteristic$n=1$ without diagnostic tables. BitVec128 models Rust u128 arithmetic; this is not compiled-binary equivalence. Full array-loop composition and geometric class transport remain separate bindings.

The first widened attempt needed the explicit ofNat conversion lemma; the first Fermat attempt needed an implicit argument instantiated. Corrected attempts and controls passed. Nonfatal deprecation warnings remain. Two exploratory cache planners failed import parsing; no cache was copied or built. Existing imported group-cardinality lemmas sufficed. Error summaries and native Lean receipts are retained separately.

Native exits, unchanged input hashes and standard-axiom readback accompany the inspected PDF, editable diagram, status JSON and custody. Existing compiled dependencies remain in the trust boundary.

Sources: \href{https://lean-lang.org/doc/reference/latest/Basic-Types/Bitvectors/}{Lean Bitvectors} and \href{https://leanprover-community.github.io/mathlib4_docs/Mathlib/FieldTheory/Finite/Basic.html}{Mathlib finite-field/Fermat identities}, compared with pinned source. No scientific campaign, enumeration, timing or cost ratio. Cohort and campaign limits unchanged.
\end{document}
''')
(p/'flow.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="1000" height="280"><rect width="1000" height="280" fill="white"/><g font-family="sans-serif" text-anchor="middle" fill="#17364a"><text x="500" y="28" font-size="20">T-1154 · Generated modular inverse correspondence</text><g stroke="#17364a" fill="#edf5f7"><rect x="20" y="65" width="260" height="65"/><rect x="370" y="65" width="260" height="65"/><rect x="720" y="65" width="260" height="65"/><rect x="275" y="180" width="450" height="55"/></g><text x="150" y="92">BitVec128 widened multiply</text><text x="150" y="115">exact for positive modulus</text><text x="500" y="92">Binary-loop invariant</text><text x="500" y="115">all UInt64 exponents · 64 fuel</text><text x="850" y="92">Prime p, exponent p − 2</text><text x="850" y="115">canonical generated inverse</text><path d="M280 97H365m265 0h85M850 130L690 180" stroke="#17364a"/><text x="500" y="202">InverseWitness + coefficient-update composition</text><text x="500" y="224">existing narrow verifier bound: p &lt; 2³¹</text><text x="500" y="265">Producer proofs · independent fidelity review pending · no performance measurements</text></g></svg>''')
# Normalize prose spacing next to mathematical and numerical tokens.
for name in ['REPORT.md','report.tex']:
 f=p/name;s=f.read_text()
 for a,b in {'in128':'in 128','so64':'so 64','prime$p$':'prime $p$','exponent$p-2$':'exponent $p-2$','receive38':'receive 38','inverse$2':'inverse $2','modulo17':'modulo 17','exponents0/1':'exponents 0/1','is1':'is 1','gives0':'gives 0','composite$p':'composite $p','returns2':'returns 2','odd-characteristic$n':'odd-characteristic $n','For1<p':'For 1<p',';64':'; 64','by38':'by 38'}.items():s=s.replace(a,b)
 f.write_text(s)
