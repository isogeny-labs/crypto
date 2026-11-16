# Primary source review: novelty constraint

Pinned upstream commit: 0ead8ca498bbd3add9d124537bcc2a8cba7f2697.

- https://github.com/isogeny-labs/crypto/blob/0ead8ca498bbd3add9d124537bcc2a8cba7f2697/src/cryptanalysis/semaev_prime_poly.rs
- https://github.com/isogeny-labs/crypto/blob/0ead8ca498bbd3add9d124537bcc2a8cba7f2697/research/gbrl/src/semaev.rs

The inspected source already constructs S3 and recursive formal Sylvester resultants. Its MPoly::restrict_to_line implements affine-line restriction with a degree bound. Its Fp backend is limited to p<2^62; the checkpoint adds wide-prime conditional solving, not a newly invented resultant reduction. The gbrl source includes S3/S4 and decomposition generators. Retire the basic resultant/line restriction as a novelty lead.

Source references include Semaev (2004), Diem (2011), Faugere/Gaudry/Huot/Renault (2014), Petit/Quisquater (2012). Those papers were not independently read in this cycle. This is a source-level prior-art finding, not completed literature clearance.
