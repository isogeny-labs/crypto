#[path = "../../../../../src/binary_ecc/curve.rs"]
pub mod curve;
#[cfg(not(any(feature = "legacy-halftrace", feature = "incumbent")))]
#[path = "../../../../../src/binary_ecc/f2m.rs"]
pub mod f2m;
#[cfg(feature = "legacy-halftrace")]
#[path = "../../../baseline_f2m_snapshot.rs"]
pub mod f2m;
#[cfg(feature = "incumbent")]
#[path = "../../../incumbent_f2m_snapshot.rs"]
pub mod f2m;
#[cfg(not(any(feature = "legacy-halftrace", feature = "incumbent")))]
#[path = "../../../../../src/binary_ecc/operation_tally.rs"]
pub mod operation_tally;
#[cfg(feature = "legacy-halftrace")]
#[path = "../../../baseline_operation_tally_snapshot.rs"]
pub mod operation_tally;
#[cfg(feature = "incumbent")]
#[path = "../../../incumbent_operation_tally_snapshot.rs"]
pub mod operation_tally;
#[cfg(not(any(feature = "legacy-halftrace", feature = "incumbent")))]
#[path = "../../../../../src/binary_ecc/trace_zero.rs"]
pub mod trace_zero;
#[cfg(any(feature = "legacy-halftrace", feature = "incumbent"))]
#[path = "../../../baseline_trace_zero_snapshot.rs"]
pub mod trace_zero;
pub use curve::{BinaryCurve, BinaryPoint};
pub use f2m::{F2mElement, IrreduciblePoly};
pub use trace_zero::{
    TraceZeroAffineMode, TraceZeroAffineSystem, TraceZeroCoordinates, TraceZeroError,
    TraceZeroVariety,
};
