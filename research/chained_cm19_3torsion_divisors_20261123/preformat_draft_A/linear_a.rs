use crate::field::{F, ONE, ZERO};

pub fn reduced(mut rows: Vec<Vec<F>>, columns: usize) -> (Vec<Vec<F>>, Vec<usize>) {
    assert!(rows.iter().all(|r| r.len() == columns));
    let mut pivots = Vec::new();
    for col in 0..columns {
        let next = (pivots.len()..rows.len()).find(|&i| rows[i][col] != ZERO);
        let Some(next) = next else { continue };
        let row = pivots.len();
        rows.swap(row, next);
        let inverse = rows[row][col].inverse();
        for value in &mut rows[row] {
            *value = value.times(inverse);
        }
        let pivot = rows[row].clone();
        for (i, other) in rows.iter_mut().enumerate() {
            if i != row {
                let factor = other[col];
                for (value, &basis) in other.iter_mut().zip(&pivot) {
                    *value = value.minus(factor.times(basis));
                }
            }
        }
        pivots.push(col);
    }
    (rows, pivots)
}

pub fn unique_conic(rows: Vec<Vec<F>>) -> (usize, Option<[F; 6]>) {
    let original = rows.clone();
    let (r, pivots) = reduced(rows, 6);
    if pivots.len() != 5 {
        return (pivots.len(), None);
    }
    let free = (0..6).find(|c| !pivots.contains(c)).expect("one free column");
    let mut vector = [ZERO; 6];
    vector[free] = ONE;
    for (row, &pivot) in pivots.iter().enumerate() {
        vector[pivot] = r[row][free].negative();
    }
    let first = vector.iter().copied().find(|&x| x != ZERO).expect("nonzero kernel");
    vector = vector.map(|x| x.over(first));
    for row in original {
        assert_eq!(row.into_iter().zip(vector).fold(ZERO, |s, (a, b)| s.plus(a.times(b))), ZERO);
    }
    (pivots.len(), Some(vector))
}

pub struct RankCertificate {
    pub pivots: Vec<usize>,
    pub original_rows: Vec<usize>,
    pub nonzero_minor: F,
    pub nullspace: Vec<Vec<F>>,
}

fn determinant(mut a: Vec<Vec<F>>) -> F {
    let size = a.len();
    assert!(a.iter().all(|r| r.len() == size));
    let mut out = ONE;
    for col in 0..size {
        let Some(pivot) = (col..size).find(|&r| a[r][col] != ZERO) else { return ZERO };
        if pivot != col {
            a.swap(col, pivot);
            out = out.negative();
        }
        let diagonal = a[col][col];
        out = out.times(diagonal);
        let base = a[col].clone();
        for row in &mut a[col + 1..] {
            let factor = row[col].over(diagonal);
            for (value, &b) in row.iter_mut().zip(&base) {
                *value = value.minus(factor.times(b));
            }
        }
    }
    out
}

pub fn certificate(original: Vec<Vec<F>>, columns: usize) -> RankCertificate {
    let mut a = original.clone();
    let mut order: Vec<_> = (0..a.len()).collect();
    let mut pivots = Vec::new();
    for col in 0..columns {
        let Some(pivot) = (pivots.len()..a.len()).find(|&r| a[r][col] != ZERO) else { continue };
        let row = pivots.len();
        a.swap(row, pivot);
        order.swap(row, pivot);
        let diagonal = a[row][col];
        let base = a[row].clone();
        for other in &mut a[row + 1..] {
            let factor = other[col].over(diagonal);
            for (value, &b) in other.iter_mut().zip(&base) {
                *value = value.minus(factor.times(b));
            }
        }
        pivots.push(col);
    }
    let original_rows = order[..pivots.len()].to_vec();
    let minor = original_rows.iter().map(|&r| pivots.iter().map(|&c| original[r][c]).collect()).collect();
    let nonzero_minor = determinant(minor);
    assert_ne!(nonzero_minor, ZERO);
    let (rref, other_pivots) = reduced(original.clone(), columns);
    assert_eq!(pivots, other_pivots);
    let mut nullspace = Vec::new();
    for free in (0..columns).filter(|c| !pivots.contains(c)) {
        let mut vector = vec![ZERO; columns];
        vector[free] = ONE;
        for (r, &pivot) in pivots.iter().enumerate() {
            vector[pivot] = rref[r][free].negative();
        }
        for row in &original {
            assert_eq!(row.iter().copied().zip(&vector).fold(ZERO, |s, (x, &y)| s.plus(x.times(y))), ZERO);
        }
        nullspace.push(vector);
    }
    assert_eq!(nullspace.len() + pivots.len(), columns);
    RankCertificate { pivots, original_rows, nonzero_minor, nullspace }
}
