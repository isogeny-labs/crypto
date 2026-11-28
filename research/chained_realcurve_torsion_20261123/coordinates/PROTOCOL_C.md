# Genuine-coordinate C: remove one unused import

B7d5a29e3 passes formatting but strict compilation1 reports only unused import Fld in main_b.rs (complete379B stderr retained). Clippy and actual coordinates were not reached. Original A/B and inherited arithmetic remain unchanged. [pcc:unverified]

C main_c.rs removes exactly that unused trait import; field_b.rs and curve_b.rs are reused unchanged. Caller/metadata/lock select C main only. All original protocol arithmetic, controls, limits, strict warnings/lints, seed, source-first guards, debug assertions and overflow checks are unchanged. No suppression or acceptance relaxation. Publish and fully read back before first C phases. [pcc:unverified]
