//! Composition boundary for already checked homogeneous relations.
//! A fixed partition binds the graph and residual basis to one field and one
//! column numbering. Geometric verification remains the producer's obligation.
use crate::{residual, Error, Outcome, Sparse, SymbolicGraph};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Column {
    Small(usize),
    Large(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admission {
    Joined,
    Anchored,
    Independent,
    Dependent,
    RejectedThreeLarge,
    StoppedWorkingCap,
    StoppedRetainedCap,
    AlreadyStopped,
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Counts {
    pub submitted_rows: u128,
    pub submitted_terms: u128,
    pub admitted_rows: u128,
    pub rejected_three_large: u128,
    pub emitted_constraints: u128,
}

pub struct Collector {
    columns: Vec<Column>,
    small_columns: Vec<usize>,
    graph: SymbolicGraph,
    basis: residual::Basis,
    counts: Counts,
    stop: Option<Admission>,
    pending: Option<Sparse>,
}

impl Collector {
    /// `is_small[i]` fixes the classification of original column i. Labels,
    /// if present, must be ordinary small columns chosen by the caller.
    /// No adaptive partition or measured-performance default is supplied.
    pub fn new(p: u64, is_small: &[bool], limits: residual::Limits) -> Result<Self, Error> {
        let mut small_columns = Vec::new();
        let mut large_count = 0;
        let columns = is_small
            .iter()
            .enumerate()
            .map(|(i, &small)| {
                crate::meter::collection(1, 0, 1, 1);
                crate::meter::charge(|c| c.partition_terms += 1);
                if small {
                    crate::meter::collection(0, 0, 1, 0);
                    small_columns.push(i);
                    Column::Small(small_columns.len() - 1)
                } else {
                    let column = Column::Large(large_count);
                    large_count += 1;
                    column
                }
            })
            .collect();
        Ok(Self {
            columns,
            graph: SymbolicGraph::new(p, large_count, small_columns.len())?,
            basis: residual::Basis::new(p, small_columns.len(), limits)?,
            small_columns,
            counts: Counts::default(),
            stop: None,
            pending: None,
        })
    }

    pub fn counts(&self) -> &Counts {
        &self.counts
    }
    pub fn basis(&self) -> &residual::Basis {
        &self.basis
    }
    /// Original global column identifiers, in residual basis order.
    pub fn small_columns(&self) -> &[usize] {
        &self.small_columns
    }
    pub fn stopped(&self) -> Option<Admission> {
        self.stop
    }
    /// Unincorporated constraint retained for diagnostic/replay purposes.
    /// This payload is OUTSIDE the residual basis's coefficient limit.
    pub fn pending_constraint(&self) -> Option<&Sparse> {
        self.pending.as_ref()
    }

    /// No processing is permitted after a cap. Malformed columns fail before
    /// mutation. Rejected three-large rows were never added to either stage.
    pub fn insert(&mut self, terms: &[(usize, u64)]) -> Result<Admission, Error> {
        if self.stop.is_some() {
            return Ok(Admission::AlreadyStopped);
        }
        if terms.iter().any(|&(i, _)| {
            crate::meter::collection(1, 0, 0, 1);
            i >= self.columns.len()
        }) {
            return Err(Error::ColumnOutOfRange);
        }
        self.counts.submitted_rows += 1;
        self.counts.submitted_terms += terms.len() as u128;
        let mut small = Vec::new();
        let mut large = Vec::new();
        for &(i, coefficient) in terms {
            crate::meter::collection(2, 0, 1, 0); // input tuple, column tag, fresh partition tuple
            crate::meter::charge(|c| c.partition_terms += 1);
            match self.columns[i] {
                Column::Small(j) => small.push((j, coefficient)),
                Column::Large(j) => large.push((j, coefficient)),
            }
        }
        let result = match self.graph.insert(&large, &small) {
            Err(Error::MoreThanTwoLargeVariables) => {
                self.counts.rejected_three_large += 1;
                return Ok(Admission::RejectedThreeLarge);
            }
            other => other?,
        };
        let status = match result {
            Outcome::Joined => Admission::Joined,
            Outcome::Anchored => Admission::Anchored,
            Outcome::Constraint(row) => {
                self.counts.emitted_constraints += 1;
                let terms: Vec<_> = row
                    .iter()
                    .map(|(&i, &v)| {
                        crate::meter::collection(0, 1, 0, 0);
                        (i, v)
                    })
                    .collect();
                match self.basis.insert(&terms)? {
                    residual::Insert::Independent => Admission::Independent,
                    residual::Insert::Dependent => Admission::Dependent,
                    cap => {
                        let status = match cap {
                            residual::Insert::WorkingCap => Admission::StoppedWorkingCap,
                            residual::Insert::RetainedCap => Admission::StoppedRetainedCap,
                            _ => unreachable!(),
                        };
                        self.pending = Some(row);
                        self.stop = Some(status);
                        return Ok(status);
                    }
                }
            }
        };
        self.counts.admitted_rows += 1;
        Ok(status)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collection_trace_partition_and_rejected_prefix() {
        crate::meter::reset();
        let mut c = Collector::new(
            7,
            &[true, false, true],
            residual::Limits {
                retained_coefficients: 20,
                working_coefficients: 20,
            },
        )
        .unwrap();
        let n = crate::meter::snapshot();
        assert_eq!(n.collection_entry_visits, 3);
        assert_eq!(n.collection_elements_initialized, 10); // 3 tags + 2 small ids + 5 forest slots
        assert_eq!(n.collection_comparisons, 3);
        assert_eq!(n.partition_terms, 3); // diagnostic, not a second priced scan
        crate::meter::reset();
        assert_eq!(
            c.insert(&[(0, 1), (3, 1), (1, 1)]),
            Err(Error::ColumnOutOfRange)
        );
        let n = crate::meter::snapshot();
        assert_eq!(n.collection_entry_visits, 2);
        assert_eq!(n.collection_comparisons, 2);
        assert_eq!(n.collection_elements_initialized, 0);
        assert_eq!(n.partition_terms, 0);
    }

    #[test]
    fn working_cap_retains_original_constraint_and_stops() {
        let mut c = Collector::new(
            7,
            &[true; 5],
            residual::Limits {
                retained_coefficients: 20,
                working_coefficients: 3,
            },
        )
        .unwrap();
        assert_eq!(
            c.insert(&[(0, 1), (2, 1), (3, 1)]),
            Ok(Admission::Independent)
        );
        let before: Vec<_> = c.basis().rows().cloned().collect();
        let input = [(0, 1), (1, 2), (4, 1)];
        assert_eq!(c.insert(&input), Ok(Admission::StoppedWorkingCap));
        assert_eq!(c.pending_constraint(), Some(&input.into()));
        assert_eq!(c.basis().rows().cloned().collect::<Vec<_>>(), before);
        assert_eq!(c.insert(&[]), Ok(Admission::AlreadyStopped));
        assert_eq!(c.counts().submitted_rows, 2);
        assert_eq!(c.counts().admitted_rows, 1);
    }

    #[test]
    fn cancellation_precedes_admission_and_caps_are_terminal() {
        let mut c = Collector::new(
            7,
            &[false, false, false, true],
            residual::Limits {
                retained_coefficients: 0,
                working_coefficients: 1,
            },
        )
        .unwrap();
        assert_eq!(
            c.insert(&[(0, 1), (1, 1), (2, 1)]),
            Ok(Admission::RejectedThreeLarge)
        );
        assert_eq!(
            c.insert(&[(0, 1), (1, 1), (2, 1), (2, 6)]),
            Ok(Admission::Joined)
        );
        assert_eq!(c.insert(&[(3, 2)]), Ok(Admission::StoppedRetainedCap));
        assert_eq!(c.pending_constraint(), Some(&[(0, 2)].into()));
        assert_eq!(c.insert(&[(3, 1)]), Ok(Admission::AlreadyStopped));
        assert_eq!(c.counts().submitted_rows, 3);
        assert_eq!(c.counts().admitted_rows, 1);
        assert_eq!(c.counts().rejected_three_large, 1);
        assert_eq!(c.basis().rank(), 0);
    }

    #[test]
    fn interleaved_partition_and_negative_cycle() {
        let mut c = Collector::new(
            7,
            &[true, false, true, false],
            residual::Limits {
                retained_coefficients: 4,
                working_coefficients: 2,
            },
        )
        .unwrap();
        assert_eq!(c.small_columns(), &[0, 2]);
        assert_eq!(c.insert(&[(1, 1), (3, 1)]), Ok(Admission::Joined));
        assert_eq!(c.insert(&[(1, 1), (3, 6), (0, 1)]), Ok(Admission::Anchored));
        assert_eq!(c.insert(&[(1, 1), (2, 1)]), Ok(Admission::Independent));
        // x1=-4*s0; x1+s2=0 gives 3*s0+s2=0 over F7.
        assert_eq!(
            c.basis().rows().cloned().collect::<Vec<_>>(),
            vec![([(0, 1), (1, 5)]).into()]
        );
        assert_eq!(c.insert(&[(4, 1)]), Err(Error::ColumnOutOfRange));
        assert_eq!(c.counts().submitted_rows, 3);
    }
}
