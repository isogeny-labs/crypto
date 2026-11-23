from pathlib import Path
p=Path(__file__).parent
(p/'REPORT.md').write_text('''# Prime-field word coefficient update

T-1149, 2026-10-31. Producer formal/source-binding packet; independent source-fidelity review pending.

The checked update decodes to o+c*choose(k,j)*a^j*b^(k-j)/s and remains canonical. Hypotheses: prime modulus 1<p<2^31; canonical input words o,c,a,b,s,iw;0<=j<=k<=5; nonzero s; and the directly checkable word certificate (s*iw)%p=1. The theorem derives iw's inverse decoding from that certificate using the existing bounded multiplication result. It certifies a supplied inverse word, not its generator.

Supporting results: the source-style binary exponentiation model unrolls symbolically for every exponent 0..5 and decodes to the expected power; three right shifts drain these exponents. Reducing the bounded chooseWord result modulo p preserves its ZMod value and yields a canonical word. Nested multiplication follows the source term's parenthesization; addition uses the existing no-overflow theorem. The prior coefficient-loop packet supplies the connection from such contributions to polynomial composition.

Nine main declarations and seventeen fixed controls receive named axiom readback. At p=17, a=2, b=3, c=4, scale=2, inverse=9, k=2, j=1, the word term is 7, agreeing with 24 mod17; updating prior slot 10 yields0. Controls cover reduced choose values, zero exponent/base, (-1)^5, invalid indices, wrong/zero/noncanonical inverse certificates, and an excluded overflowing multiplication. The noncanonical inverse26 passes the product test for scale 2 modulo 17 but fails the full canonical witness. Natural multiplication and UInt64 multiplication differ for2^63 times 2 modulo 5; the input is outside the theorem domain.

The source mapping is odd-characteristic n=1 arithmetic without diagnostic tables. Source Field::inv rejects zero and calls Field::bi, whose modpow uses u128 products. This packet verifies a supplied inverse through a word product certificate; the arbitrary-exponent inversion generator, literal vector execution/compiler equivalence and geometric class transport remain separate bindings. Field/loop source hashes and exact excerpts are retained.

Main attempt2 failed because an existing nonzero-decoding lemma needed its explicit import; later attempts passed. The first control compile required explicit unfolding of the witness predicate for decidability; corrected controls passed. No missing proof is filled by an axiom. Strict native exit and unchanged source/dependency hash checks apply to each attempt. Compiled dependencies remain part of the Lean trust boundary; independent binary rechecking is not claimed.

Primary references: existing UInt64Residue, CrossScaling and WordAdd11 proof sources; official Mathlib ZMod.Basic natCast_mod documentation (https://leanprover-community.github.io/mathlib4_docs/Mathlib/Data/ZMod/Basic.html). No novelty claim. No scientific campaign, timing, curve enumeration or cost ratio. Frozen cohort and campaign limits unchanged. Diagram depicts correspondence dependencies.
''')
(p/'report.tex').write_text(r'''\documentclass[10pt]{article}
\usepackage[margin=.7in]{geometry}\usepackage{amsmath,amssymb,tikz,hyperref}
\setlength{\parindent}{0pt}\setlength{\parskip}{5pt}
\begin{document}
\textbf{Prime-field word coefficient update}\hfill2026-10-31\\T-1149. Producer proof packet; independent source-fidelity review pending.
\section*{Checked update and hypotheses}
The word output remains canonical and decodes to
\[o+c\binom{k}{j}a^jb^{k-j}s^{-1}.\]
Hypotheses: prime $1<p<2^{31}$; canonical input words $o,c,a,b,s,i_w$; $0\le j\le k\le5$; $s\ne0$; and the checkable word certificate
\[ (s\cdot i_w)\bmod p=1.\]
Bounded multiplication decoding proves that the certificate supplies the field inverse. This verifies the supplied word, not the inversion generator.
\begin{center}\begin{tikzpicture}[every node/.style={draw,align=center,font=\small}]
\node (a) at (0,0) {Choose modulo $p$\\bounded powers};
\node (b) at (4,0) {Nested multiplication\\canonical term};
\node (c) at (8,0) {Slot addition\\decoded update};
\node (d) at (4,-1) {Inverse word certificate};
\draw[->](a)--(b);\draw[->](b)--(c);\draw[->](d)--(b);
\end{tikzpicture}\end{center}
\section*{Supporting proofs and controls}
The source-style binary power loop unrolls symbolically for exponents zero through five; three shifts drain the exponent. Bounded choose-word reduction preserves its residue-class value. Multiplication follows the source parenthesization; addition reuses the bounded word theorem.

Nine main declarations and seventeen controls receive separate named axiom readback. With $p=17,a=2,b=3,c=4,s=2,i_w=9,k=2,j=1$, the term is 7, agreeing with $24\bmod17$; prior slot 10 updates to 0. Controls cover zero exponent/base, $(-1)^5$, reduced choose, invalid indices and wrong/zero/noncanonical inverse certificates.

Inverse word 26 passes the product test for scale 2 modulo 17 but fails canonicality. For the excluded input $2^{63}$ times 2 modulo 5, the natural result is 1 and the UInt64 result is 0. These controls retain the input-bound distinction.
\section*{Source and evidence boundary}
The source mapping is odd-characteristic $n=1$ arithmetic without diagnostic tables. \texttt{Field::inv} rejects zero and calls \texttt{bi}; its general \texttt{modpow} uses u128 products. The arbitrary-exponent inversion generator, literal vector/compiler behavior and geometric class transport remain separate bindings. Exact excerpts and source hashes accompany this packet.

One main attempt needed an explicit import of the existing nonzero-decoding lemma. The first controls needed predicate unfolding for decidability; corrected checks passed. Native exits, unchanged bindings and axiom readback are retained with all attempts. Existing compiled dependencies remain in the trust boundary.

References: existing UInt64Residue/CrossScaling/WordAdd11 proofs and \href{https://leanprover-community.github.io/mathlib4_docs/Mathlib/Data/ZMod/Basic.html}{Mathlib ZMod.Basic}. No scientific campaign, curve enumeration, timing or cost ratio. Cohort and campaign limits unchanged. Diagram shows proof dependencies.
\end{document}
''')
(p/'flow.svg').write_text('''<svg xmlns="http://www.w3.org/2000/svg" width="950" height="260"><rect width="950" height="260" fill="white"/><g font-family="sans-serif" text-anchor="middle" fill="#17364a"><text x="475" y="30" font-size="20">T-1149 · Prime-field word coefficient update</text><g stroke="#17364a" fill="#edf5f7"><rect x="20" y="65" width="240" height="65"/><rect x="355" y="65" width="240" height="65"/><rect x="690" y="65" width="240" height="65"/><rect x="310" y="175" width="330" height="40"/></g><text x="140" y="92">Choose % p</text><text x="140" y="115">Power exponents 0…5</text><text x="475" y="92">Nested word products</text><text x="475" y="115">canonical term + decoding</text><text x="810" y="92">Slot addition</text><text x="810" y="115">canonical decoded update</text><path d="M260 97H350m245 0h90M475 175V135" stroke="#17364a"/><text x="475" y="200">Inverse certificate: canonical, s ≠ 0, s·iw % p = 1</text><text x="475" y="247">Prime 1 &lt; p &lt; 2³¹ · canonical inputs · independent fidelity review pending</text></g></svg>''')
