//! CADO-NFS `bwc` input and solution verification for sparse systems.
//!
//! The binary matrix and ASCII right-hand-side layouts match the existing
//! `la_engine_bench` CADO control. This module does not invoke CADO or filter
//! relation matrices. The caller must supply a prime modulus and a full-rank
//! system with at least as many rows as columns; a curve DLP still needs
//! group-side checking.

use std::io::{self, Write};

use super::koblitz_sparse_la::SparseRow;
use super::residual_walk::is_prime_u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CadoExportSummary {
    pub dimension: usize,
    pub nonzeros: usize,
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

/// Write CADO's per-row `(len, (column, signed i32 coefficient)*)` binary
/// matrix and `rows 1 prime` followed by decimal residues in the RHS text.
/// The modulus must be a known odd prime below `2^63`.
pub fn write_rectangular_system(
    rows: &[SparseRow<u64>],
    columns: usize,
    modulus: u64,
    matrix: &mut impl Write,
    rhs: &mut impl Write,
) -> io::Result<CadoExportSummary> {
    if modulus < 3 || modulus >= (1u64 << 63) || !is_prime_u64(modulus) {
        return Err(invalid("modulus must be an odd prime below 2^63"));
    }
    if columns == 0 || columns > u32::MAX as usize || rows.len() < columns {
        return Err(invalid(
            "CADO bwc export needs at least as many rows as columns",
        ));
    }
    let mut nonzeros = 0;
    for row in rows {
        let length = u32::try_from(row.entries.len()).map_err(|_| invalid("row too long"))?;
        matrix.write_all(&length.to_le_bytes())?;
        let mut previous = None;
        for &(column, coefficient) in &row.entries {
            if column as usize >= columns || previous.is_some_and(|last| column <= last) {
                return Err(invalid("row columns must be sorted, unique and in range"));
            }
            if coefficient == 0 || coefficient >= modulus {
                return Err(invalid("row coefficients must be nonzero residues"));
            }
            let signed = if coefficient > modulus / 2 {
                coefficient as i128 - modulus as i128
            } else {
                coefficient as i128
            };
            let signed = i32::try_from(signed)
                .map_err(|_| invalid("coefficient exceeds CADO signed 32-bit format"))?;
            matrix.write_all(&column.to_le_bytes())?;
            matrix.write_all(&signed.to_le_bytes())?;
            previous = Some(column);
        }
        if row.rhs >= modulus {
            return Err(invalid("right-hand side is not a reduced residue"));
        }
        nonzeros += row.entries.len();
    }
    writeln!(rhs, "{} 1 {}", rows.len(), modulus)?;
    for row in rows {
        writeln!(rhs, "{}", row.rhs)?;
    }
    Ok(CadoExportSummary {
        dimension: columns,
        nonzeros,
    })
}

/// Square-system compatibility entry point.
pub fn write_square_system(
    rows: &[SparseRow<u64>],
    columns: usize,
    modulus: u64,
    matrix: &mut impl Write,
    rhs: &mut impl Write,
) -> io::Result<CadoExportSummary> {
    if rows.len() != columns {
        return Err(invalid("CADO bwc export requires a square matrix"));
    }
    write_rectangular_system(rows, columns, modulus, matrix, rhs)
}

fn inverse(value: u64, modulus: u64) -> Option<u64> {
    let (mut old_r, mut r) = (modulus as i128, value as i128);
    let (mut old_t, mut t) = (0i128, 1i128);
    while r != 0 {
        let q = old_r / r;
        (old_r, r) = (r, old_r - q * r);
        (old_t, t) = (t, old_t - q * t);
    }
    (old_r == 1).then_some(old_t.rem_euclid(modulus as i128) as u64)
}

/// Decode CADO's `K.sols*` words and verify `M x = RHS` against every row.
/// CADO stores a right-nullspace vector `(x_raw, y)` for `(M || RHS)`, so
/// the solution is `x_raw / (-y)`. Some builds pad the middle; the RHS word
/// remains last. Only the first `columns` coordinates are read as `x_raw`.
pub fn verify_rectangular_solution(
    rows: &[SparseRow<u64>],
    columns: usize,
    modulus: u64,
    bytes: &[u8],
) -> Result<Vec<u64>, &'static str> {
    if modulus < 3 || modulus >= (1u64 << 63) || !is_prime_u64(modulus) {
        return Err("invalid modulus");
    }
    if columns == 0 || rows.len() < columns {
        return Err("CADO solution needs at least as many rows as columns");
    }
    if bytes.len() % 8 != 0 || bytes.len() / 8 < columns + 1 {
        return Err("CADO solution has a bad word count");
    }
    let words: Vec<u64> = bytes
        .chunks_exact(8)
        .map(|chunk| u64::from_le_bytes(chunk.try_into().expect("eight-byte chunk")))
        .collect();
    let y = words[words.len() - 1] % modulus;
    let minus_y = if y == 0 { 0 } else { modulus - y };
    let scale = inverse(minus_y, modulus).ok_or("CADO RHS coordinate is not invertible")?;
    let solution: Vec<_> = words[..columns]
        .iter()
        .map(|&word| {
            ((u128::from(word % modulus) * u128::from(scale)) % u128::from(modulus)) as u64
        })
        .collect();
    for row in rows {
        if row.rhs >= modulus {
            return Err("row right-hand side is not reduced");
        }
        let mut lhs = 0u64;
        for &(column, coefficient) in &row.entries {
            let Some(&value) = solution.get(column as usize) else {
                return Err("row column exceeds solution width");
            };
            if coefficient >= modulus {
                return Err("row coefficient is not reduced");
            }
            lhs = ((u128::from(lhs) + u128::from(coefficient) * u128::from(value))
                % u128::from(modulus)) as u64;
        }
        if lhs != row.rhs {
            return Err("CADO solution fails an input relation");
        }
    }
    Ok(solution)
}

/// Square-system compatibility entry point.
pub fn verify_solution(
    rows: &[SparseRow<u64>],
    columns: usize,
    modulus: u64,
    bytes: &[u8],
) -> Result<Vec<u64>, &'static str> {
    if rows.len() != columns {
        return Err("CADO solution needs a square input system");
    }
    verify_rectangular_solution(rows, columns, modulus, bytes)
}
