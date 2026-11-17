//! Exact linear algebra; vectors are rows and matrices act on columns.
use quotient_calibration_preflight::field::F;

pub type Row = Vec<F>;

pub fn dot(a: &[F], b: &[F]) -> F {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).fold(F::ZERO, |s, (x, y)| {
        if *x == F::ZERO || *y == F::ZERO {
            s
        } else {
            s + *x * *y
        }
    })
}

pub fn combination(coefficients: &[F], rows: &[Row], columns: usize) -> Row {
    assert_eq!(coefficients.len(), rows.len());
    let mut result = vec![F::ZERO; columns];
    for (c, row) in coefficients.iter().zip(rows) {
        if *c == F::ZERO {
            continue;
        }
        for (a, b) in result.iter_mut().zip(row) {
            if *b != F::ZERO {
                *a = *a + *c * *b;
            }
        }
    }
    result
}

#[derive(Clone, Debug)]
pub struct Span {
    pub columns: usize,
    pub rows: Vec<Row>,
    pub pivots: Vec<usize>,
}

impl Span {
    pub fn new(columns: usize) -> Self {
        Self {
            columns,
            rows: vec![],
            pivots: vec![],
        }
    }
    pub fn from_rows(columns: usize, rows: impl IntoIterator<Item = Row>) -> Self {
        let mut result = Self::new(columns);
        for row in rows {
            result.insert(row);
        }
        result
    }
    pub fn full(n: usize) -> Self {
        Self::from_rows(
            n,
            (0..n).map(|i| {
                let mut r = vec![F::ZERO; n];
                r[i] = F::c(1);
                r
            }),
        )
    }
    pub fn remainder(&self, mut row: Row) -> Row {
        assert_eq!(row.len(), self.columns);
        for (basis, pivot) in self.rows.iter().zip(&self.pivots) {
            let c = row[*pivot];
            if c == F::ZERO {
                continue;
            }
            row[*pivot] = F::ZERO;
            for j in *pivot + 1..self.columns {
                if basis[j] != F::ZERO {
                    row[j] = row[j] - c * basis[j];
                }
            }
        }
        row
    }
    pub fn insert(&mut self, row: Row) -> bool {
        let mut row = self.remainder(row);
        let Some(pivot) = row.iter().position(|x| *x != F::ZERO) else {
            return false;
        };
        let inverse = row[pivot].inv();
        for x in &mut row[pivot..] {
            if *x != F::ZERO {
                *x = *x * inverse;
            }
        }
        for old in &mut self.rows {
            let c = old[pivot];
            if c == F::ZERO {
                continue;
            }
            old[pivot] = F::ZERO;
            for j in pivot + 1..self.columns {
                if row[j] != F::ZERO {
                    old[j] = old[j] - c * row[j];
                }
            }
        }
        let position = self.pivots.partition_point(|p| *p < pivot);
        self.rows.insert(position, row);
        self.pivots.insert(position, pivot);
        true
    }
    pub fn free_columns(&self) -> Vec<usize> {
        (0..self.columns)
            .filter(|i| self.pivots.binary_search(i).is_err())
            .collect()
    }
    pub fn kernel(&self) -> Vec<Row> {
        self.free_columns()
            .into_iter()
            .map(|free| {
                let mut v = vec![F::ZERO; self.columns];
                v[free] = F::c(1);
                for (row, pivot) in self.rows.iter().zip(&self.pivots) {
                    v[*pivot] = -row[free];
                }
                v
            })
            .collect()
    }
    pub fn quotient(&self, row: Row) -> Row {
        let remainder = self.remainder(row);
        self.free_columns()
            .into_iter()
            .map(|i| remainder[i])
            .collect()
    }
}

/// Intersect a kernel as constraints arrive, without eliminating thousands
/// of redundant rows once the kernel has become small.
pub struct Kernel {
    pub rows: Vec<Row>,
    columns: usize,
}
impl Kernel {
    pub fn new(n: usize) -> Self {
        Self {
            rows: Span::full(n).rows,
            columns: n,
        }
    }
    pub fn constrain(&mut self, equation: &[F]) {
        assert_eq!(equation.len(), self.columns);
        let values: Row = self.rows.iter().map(|r| dot(r, equation)).collect();
        let Some(p) = values.iter().position(|v| *v != F::ZERO) else {
            return;
        };
        let pivot = self.rows[p].clone();
        let inverse = values[p].inv();
        for (j, row) in self.rows.iter_mut().enumerate() {
            if j == p || values[j] == F::ZERO {
                continue;
            }
            let c = values[j] * inverse;
            for (a, b) in row.iter_mut().zip(&pivot) {
                if *b != F::ZERO {
                    *a = *a - c * *b;
                }
            }
        }
        self.rows.remove(p);
    }
}

pub fn inverse(rows: &[Row]) -> Vec<Row> {
    let n = rows.len();
    assert!(rows.iter().all(|r| r.len() == n));
    let augmented: Vec<Row> = rows
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let mut v = r.clone();
            v.resize(2 * n, F::ZERO);
            v[n + i] = F::c(1);
            v
        })
        .collect();
    let span = Span::from_rows(2 * n, augmented);
    assert_eq!(span.pivots, (0..n).collect::<Vec<_>>(), "singular matrix");
    span.rows.into_iter().map(|r| r[n..].to_vec()).collect()
}
pub fn multiply(a: &[Row], b: &[Row]) -> Vec<Row> {
    assert!(!b.is_empty());
    a.iter().map(|r| combination(r, b, b[0].len())).collect()
}
pub fn apply(a: &[Row], b: &[F]) -> Row {
    a.iter().map(|r| dot(r, b)).collect()
}
pub fn normalized(mut a: Row) -> Row {
    let scale = a
        .iter()
        .find(|v| **v != F::ZERO)
        .expect("all-zero projective vector")
        .inv();
    for v in &mut a {
        *v = *v * scale;
    }
    a
}
pub fn projective_equal(a: &[F], b: &[F]) -> bool {
    if a.len() != b.len() || !a.iter().any(|x| *x != F::ZERO) || !b.iter().any(|x| *x != F::ZERO) {
        return false;
    }
    let p = a.iter().position(|x| *x != F::ZERO).unwrap();
    b[p] != F::ZERO && a.iter().zip(b).all(|(x, y)| *x * b[p] == *y * a[p])
}
