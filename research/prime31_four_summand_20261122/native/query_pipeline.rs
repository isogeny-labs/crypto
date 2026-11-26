//! Caller-driven M31 public queries with immediate signed-row consumption.
//!
//! The public input is a curve, a factor base and a point T. Each call supplies
//! only public labels a,b and constructs R=[a]G+[b]T with existing arithmetic.
//! The existing relation adapter checks every equality and recovers a scalar
//! only from its exact target functional followed by literal point replay.
//! No query generator, factor-base logarithms or scalar hint is accepted.

use crate::extractor::{extract_cofactor_visit, Limits, ResultRow, Status};
use crate::relation_adapter::{AdapterError, AdapterWork, RelationAdapter, RelationReceipt};
use crypto_lib::cryptanalysis::f4_fp::F4ResourceLimits;
use prime31_algebraic_ic::{Arithmetic, Counts, Curve, Point};
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum PipelineError {
    CoordinateFieldMustBeM31,
    InvalidDistinctAbscissaBase,
    Adapter(AdapterError),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum QueryState {
    TargetRecovered,
    AlreadyRecovered,
    QueryExhausted,
    Incomplete,
    ConsumerError,
}

#[derive(Clone, Debug, Serialize)]
pub struct QueryReceipt {
    pub a: u64,
    pub b: u64,
    pub query_point: Point,
    pub state: QueryState,
    pub rank_before: usize,
    pub rank_after: usize,
    pub recovered_scalar: Option<u64>,
    /// These are the rows actually delivered before stop, not an inferred
    /// count from graph topology or a buffered post-extraction pass.
    pub consumed: Vec<RelationReceipt>,
    pub extraction: Option<ResultRow>,
    pub consumer_error: Option<AdapterError>,
    pub query_arithmetic: Counts,
    pub cumulative_adapter_work: AdapterWork,
    /// Allocation, callback scheduling and inherited native work are not all
    /// priced. A receipt is not a complete cold-cost measurement.
    pub phase_accounting_complete: bool,
}

pub struct QueryPipeline {
    curve: Curve,
    base: Vec<(u64, u64)>,
    target: Point,
    adapter: RelationAdapter,
}

impl QueryPipeline {
    pub fn new(
        curve: Curve,
        base: Vec<(u64, u64)>,
        target: Point,
    ) -> Result<Self, PipelineError> {
        if curve.p != 2_147_483_647 {
            return Err(PipelineError::CoordinateFieldMustBeM31);
        }
        let mut xs = BTreeSet::new();
        if base.iter().any(|&(x, y)| y == 0 || !xs.insert(x)) {
            return Err(PipelineError::InvalidDistinctAbscissaBase);
        }
        let adapter = RelationAdapter::new(curve, base.clone(), target)
            .map_err(PipelineError::Adapter)?;
        Ok(Self { curve, base, target, adapter })
    }

    pub fn rank(&self) -> usize { self.adapter.rank() }

    pub fn scalar(&self) -> Option<u64> { self.adapter.scalar() }

    pub fn adapter_work(&self) -> AdapterWork { self.adapter.work() }

    pub fn setup_arithmetic_counts(&self) -> Counts {
        self.adapter.setup_arithmetic_counts()
    }

    /// One query with bounded extractor resources. The caller owns query selection and any global
    /// campaign budget. This method does not loop or retry on incomplete F4.
    /// A recovered target stops the producer immediately, leaving its chart
    /// explicitly ConsumerStopped rather than asserting exhaustive completion.
    /// Extractor deadlines are cooperative: setup, query arithmetic, and an
    /// in-progress consumer cannot be preempted. A hard elapsed-time limit
    /// requires an outer process supervisor.
    pub fn submit(
        &mut self,
        a: u64,
        b: u64,
        limits: Limits,
        resources: F4ResourceLimits,
    ) -> QueryReceipt {
        let (a, b) = (a % self.curve.n, b % self.curve.n);
        let rank_before = self.adapter.rank();
        let mut receipt = QueryReceipt {
            a, b, query_point: None, state: QueryState::AlreadyRecovered,
            rank_before, rank_after: rank_before,
            recovered_scalar: self.adapter.scalar(), consumed: Vec::new(),
            extraction: None, consumer_error: None,
            query_arithmetic: Counts::default(),
            cumulative_adapter_work: self.adapter.work(),
            phase_accounting_complete: false,
        };
        if receipt.recovered_scalar.is_some() {
            return receipt;
        }
        let mut arithmetic = Arithmetic::default();
        let ag = arithmetic.scalar(self.curve, a, Some(self.curve.g));
        let bt = arithmetic.scalar(self.curve, b, self.target);
        receipt.query_point = arithmetic.add(self.curve, ag, bt);
        receipt.query_arithmetic = arithmetic.counts;
        let adapter = &mut self.adapter;
        let extraction = extract_cofactor_visit(
            self.curve, &self.base, receipt.query_point, limits, resources,
            |relation| match adapter.accept(relation, a, b) {
                Ok(row) => {
                    let stop = row.recovered_scalar.is_some();
                    receipt.consumed.push(row);
                    stop
                }
                Err(error) => {
                    receipt.consumer_error = Some(error);
                    true
                }
            },
        );
        receipt.rank_after = self.adapter.rank();
        receipt.recovered_scalar = self.adapter.scalar();
        receipt.cumulative_adapter_work = self.adapter.work();
        receipt.state = if receipt.consumer_error.is_some() {
            QueryState::ConsumerError
        } else if receipt.recovered_scalar.is_some() {
            QueryState::TargetRecovered
        } else if matches!(extraction.status, Status::Complete | Status::Empty) {
            QueryState::QueryExhausted
        } else {
            QueryState::Incomplete
        };
        receipt.extraction = Some(extraction);
        receipt
    }
}
