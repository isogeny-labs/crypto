//! # Isogeny-volcano tools: profile, level-1 evaluation, binary-field steps.
//!
//! The command-line surface is `crypto volcano …` (`src/cli_volcano.rs`).
//!
//! * [`profile`] — Frobenius discriminant, conductor, per-prime volcano depth,
//!   class numbers of the orders, and the group structure at every level.
//! * [`level1`] — `Φ_ℓ(X, j₀)` over `ℤ` for a class-number-one CM crater by CRT
//!   (Vélu on curves with full rational `ℓ`-torsion), the step PARI cannot do
//!   in characteristic 2 for large `ℓ`.
//! * [`f2poly`], [`binary`] — factoring the mod-2 polynomial into Frobenius
//!   orbits and building/checking the level-1 curves over `F_{2^n}`.
//! * [`fp`] — prime-field volcanoes: CM instances, exact exploration from the crater,
//!   blind descent to the floor, per-level curves for ecbench.
//! * [`arith`] — 63-bit Montgomery field and short-Weierstrass points.

pub mod arith;
pub mod binary;
pub mod f2poly;
pub mod fp;
pub mod level1;
pub mod profile;
