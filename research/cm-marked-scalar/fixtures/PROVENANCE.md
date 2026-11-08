# Input custody

These files are unchanged copies of exact-arithmetic input/certificate records from the local study cm-isogeny-attack-20261107, rounds 37 and 38. Their historical producer was Python with FLINT. The native continuation must replay them without that producer.

- published-curves.json: Round37 inputs.json; NIST SP 800-186 sections 3.2.1.4-5.
- marked-relations.json: Round38 marked-relations.json; exact primitive seventh roots, CRT lifts, norms and subgroup fixtures.
- p-384-closure.json and p-521-closure.json: Round38 closure certificates; explicit forward/return polynomials, endpoint scaling, point fixtures and initial noninteger relation.

The companion SHA-256 manifest binds these copies. Historical construction timing and setup operation counts were not measured; historical field-operation counts are not native runtime results. Prior failed verifier attempts remain in the original sealed Round38 evidence and are not silently described as successful runs.

Parameter source: https://nvlpubs.nist.gov/nistpubs/SpecialPublications/NIST.SP.800-186.pdf

Standard quotient/isogeny background: https://ocw.mit.edu/courses/18-783-elliptic-curves-fall-2025/mit18_783_f25_lec05.pdf
