use super::{
    budget::Budget,
    filter::{add, mul},
    schema::{Complete, Rank},
};
use anyhow::{ensure, Result};

fn power(mut a: u64, mut e: u64, r: u64, b: &mut Budget) -> Result<u64> {
    let mut x = 1;
    while e != 0 {
        b.work(2)?;
        if e & 1 != 0 {
            x = mul(x, a, r);
        }
        a = mul(a, a, r);
        e >>= 1;
    }
    Ok(x)
}
/// Prime-field Gauss-Jordan with explicit admission before dense allocation.
/// Caller independently validates primality; no producer reducer is used.
pub fn reduce(rows: &[Complete], columns: usize, r: u64, b: &mut Budget) -> Result<Rank> {
    ensure!(
        (1..=4097).contains(&columns) && rows.len() <= 4096,
        "rank shape"
    );
    b.matrix(rows.len(), columns)?;
    let mut a = vec![vec![0u64; columns]; rows.len()];
    let mut rhs = Vec::with_capacity(rows.len());
    for (i, row) in rows.iter().enumerate() {
        ensure!(
            row.rhs < r && row.small.windows(2).all(|w| w[0].0 < w[1].0),
            "noncanonical matrix row"
        );
        for &(j, k) in &row.small {
            ensure!((j as usize) < columns && k > 0 && k < r, "matrix term");
            a[i][j as usize] = k;
        }
        rhs.push(row.rhs);
    }
    let mut pivots = vec![None; columns];
    let mut rank = 0;
    for (col, position) in pivots.iter_mut().enumerate() {
        b.work(rows.len().saturating_sub(rank))?;
        let Some(found) = (rank..rows.len()).find(|&i| a[i][col] != 0) else {
            continue;
        };
        a.swap(rank, found);
        rhs.swap(rank, found);
        let inverse = power(a[rank][col], r - 2, r, b)?;
        ensure!(
            mul(a[rank][col], inverse, r) == 1,
            "rank modulus is not a field"
        );
        b.work(columns - col + 1)?;
        for value in &mut a[rank][col..] {
            *value = mul(*value, inverse, r);
        }
        rhs[rank] = mul(rhs[rank], inverse, r);
        for i in 0..rows.len() {
            b.work(1)?;
            if i == rank || a[i][col] == 0 {
                continue;
            }
            let scale = r - a[i][col];
            b.work(columns - col + 1)?;
            let (destination, pivot) = if i < rank {
                let (before, after) = a.split_at_mut(rank);
                (&mut before[i], &after[0])
            } else {
                let (before, after) = a.split_at_mut(i);
                (&mut after[0], &before[rank])
            };
            for (value, source) in destination[col..].iter_mut().zip(&pivot[col..]) {
                *value = add(*value, mul(scale, *source, r), r);
            }
            rhs[i] = add(rhs[i], mul(scale, rhs[rank], r), r);
        }
        *position = Some(rank);
        rank += 1;
    }
    ensure!(
        rhs.iter().skip(rank).all(|v| *v == 0),
        "inconsistent completed equations"
    );
    let mut determined = vec![false; columns];
    let mut values = vec![0; columns];
    for (j, pivot) in pivots.iter().enumerate() {
        if let Some(i) = *pivot {
            b.work(columns)?;
            values[j] = rhs[i];
            determined[j] = pivots
                .iter()
                .enumerate()
                .all(|(k, p)| p.is_some() || a[i][k] == 0);
        }
    }
    Ok(Rank {
        columns,
        rank,
        determined,
        values,
        rref: a
            .iter()
            .map(|row| row.iter().map(u64::to_string).collect())
            .collect(),
        rhs: rhs.iter().map(u64::to_string).collect(),
    })
}
