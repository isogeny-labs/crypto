#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10pt)
= A concrete 263-bit prime conductor factor
13 November 2026 · Fixed published-instance replay and construction review.

The K-571 example of Koblitz–Koblitz–Menezes has a 263-bit prime factor in its
Frobenius conductor. PARI/GP 2.17.2 reconstructed the factor, produced an ECPP
certificate and accepted that certificate in a separate replay process. Changing
the subject caused rejection. These arithmetic observations are PCC statements,
with verdict not_covered; independent review remains required: [pcc:unverified].
Data claim: PCC-20261113-add2c4.

#text(size: 8.3pt)[
Prime: #raw("9021184135396238924389891945192676814518993645089807769277009849103733654828039", block: true)
]

The curve is K-571, $y^2+x y=x^3+1$ over $F_(2^571)$. The source paper gives
endomorphism conductor 1 and fundamental discriminant −7. The replay's
Frobenius conductor is the product of the displayed prime and 3952463, a
22-bit prime. Full trace, conductor and checks are retained in arithmetic.json.
This is a published factor, not a newly evaluated crossing.

#image("evidence-stages.svg", width: 100%)

== Acceptance criterion and missing components
The requested minimum remains an 80-bit prime endomorphism-conductor gap
between ordinary elliptic curves, crossed by a completely evaluated isogeny,
with matched endpoint ECDLP measurements. Both the existing 80-bit source and
this 263-bit published factor lack a supplied quotient and full point map.
No new ECDLP or scalar-multiplication timings were produced. A full K-571
floor has composite conductor; an exactly prime ratio needs an appropriate
intermediate endpoint, also absent here.

== Construction assessment
Galbraith–Gilchrist–Robert Algorithm 3 recovers a vertical map from two supplied
curves. It does not produce the missing endpoint. Its extra discriminant-divisor
mechanism has only 1 and 7 available at our maximal discriminant −7 endpoint.
Higher-dimensional evaluation still needs initial map information. Horizontal
invertible ideal actions do not resolve general conductor-changing evaluation;
The Isogeny Problems lists the ordinary non-invertible-ideal case as Problem 5.2.
This is a bounded assessment, not an impossibility theorem. [pcc:unverified]

#pagebreak()
== Replay and retained evidence
Use an empty output directory because GP write appends. Run check-published.gp
with GP 2.17.2, a 128 MB stack, and inspect both stderr and the output files;
GP can exit zero after an interpreter error. Then run check-certificate.gp in
the same directory. Commands are in ../README.md.

- arithmetic.json: exact trace, conductor, two factors and scope.
- prime263.cert: saved PARI primality certificate (2547 bytes).
- certificate-replay.json: subject binding, acceptance and changed-subject control.
- primecert-stack8M.stderr: initial default-stack exhaustion, retained as failure.
- source-review.json and source-hashes.txt: sources, locators and downloaded-byte hashes.

The 128 MB replay completed; no performance claim is inferred from its duration.
This is arithmetic verification of a fixed published instance, not a new search
or experiment. No quotient, transported generator or endpoint timing is present.
No new subgroup workload was introduced for K-571. [pcc:unverified]

== Sources
1. Koblitz, Koblitz and Menezes, Section 11.2, Example 4:
   #link("https://www.math.uwaterloo.ca/~ajmeneze/publications/ecc.pdf")[author PDF].
2. Galbraith, Climbing and descending tall isogeny volcanos, introduction,
   Section 5, Appendix A: #link("https://antsmath.org/ANTSXVI/papers/Galbraith.pdf")[ANTS submission PDF].
   This copy predates later IACR revisions.
3. Galbraith, Gilchrist and Robert, Algorithm 3, Proposition 5.1:
   #link("https://www.math.u-bordeaux.fr/~damienrobert/pro/publications/articles/climbing_volcanoes.pdf")[author PDF], dated 2025-08-15.
4. #link("https://isogeni.es/problems/full.pdf")[The Isogeny Problems], Problem 5.2.

Literature claim: PCC-20261113-80903b. KB unavailable: no MCP service exposed,
repository wrapper exited 127. Fallback: scoped repository evidence and primary
sources. ecdb unavailable in the fresh runtime; fixed-instance arithmetic replay
is recorded instead. No exhaustive world-record claim is made.

The existing readiness, complete-pipeline and CM-orbit figures have no changed
measurement inputs. No canonical benchmark graph or scoreboard row is promoted.
The next technical obligation is a complete quotient and non-kernel images for
the existing 80-bit source, with total construction and transport costs recorded.
