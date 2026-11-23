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

pub fn rank(rows: Vec<Vec<F>>, columns: usize) -> Vec<usize> {
    reduced(rows, columns).1
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
