//! Independently checked consequences of retained pre-native polynomial systems.
//! This channel has its own cumulative budget; censoring never stops branch replay.

use std::collections::BTreeMap;

use serde_json::{json, Value};

use super::prefix_algebra::{self, IdentityStatus, Multiple, Poly, Work};
use super::prime_chain_prefix::graph;

const MAX_OPERATIONS: u64 = 10_000_000;
const MAX_ROWS: u64 = 4096;
const MAX_PAYLOAD_BYTES: u64 = 64 * 1024 * 1024;
const MAX_TERMINAL_BYTES: usize = 4096;
const ORIGIN: &str = "separately_searched_generator_identity_for_retained_native_basis_row";
const WORK_SCOPE: &str = "direct identity coefficient multiplication and accumulation; excludes parsing, primality, graph construction and I/O; not native instructions";
const PAYLOAD_SCOPE: &str = "sum of serialized optional feature and record JSON bytes for each emitted nonterminal occurrence, before sorter envelopes and deduplication; excludes terminal censoring record";

pub(super) struct Node<'a> {
    pub initial: &'a Value,
    pub active: &'a [usize],
    pub bindings: &'a BTreeMap<usize, u64>,
    pub sources: &'a [Poly],
    pub basis: &'a [Poly],
    pub proofs: &'a [Vec<Multiple>],
    pub p: u64,
    pub sequence: u64,
    pub node_id: u64,
    pub equality_work: &'a Work,
}

#[derive(Clone, Copy)]
struct Budget {
    operations: u64,
    rows: u64,
    payload_bytes: u64,
}

#[derive(Default, serde::Serialize)]
struct Counts {
    examined_target_rows: u64,
    verified_identity_rows: u64,
    admitted_nonzero_rows: u64,
    admitted_zero_rows: u64,
    admitted_degree_drop_rows: u64,
    unknown_suffix_records: u64,
    direct_identity_coefficient_operations: u64,
    serialized_payload_bytes: u64,
    terminal_record_bytes: u64,
}

pub(super) struct Channel {
    budget: Budget,
    counts: Counts,
    disabled: bool,
}

impl Default for Channel {
    fn default() -> Self {
        Self {
            budget: Budget {
                operations: MAX_OPERATIONS,
                rows: MAX_ROWS,
                payload_bytes: MAX_PAYLOAD_BYTES,
            },
            counts: Counts::default(),
            disabled: false,
        }
    }
}

impl Channel {
    pub(super) fn summary(&self) -> Value {
        json!({"schema":"ml-dataset.algebraic-channel-summary/v1",
            "admission_status":if self.disabled {"bounded_unknown_suffix"} else {"retained_rows_checked"},
            "counts":self.counts,
            "limits":{"direct_identity_coefficient_operations":self.budget.operations,
                "examined_target_rows":self.budget.rows,"serialized_payload_bytes":self.budget.payload_bytes,
                "terminal_record_bytes":MAX_TERMINAL_BYTES},
            "work_scope":WORK_SCOPE,"serialized_payload_scope":PAYLOAD_SCOPE,
            "source_completion_claimed":false,"native_execution_trace_verified":false,
            "native_instruction_count_verified":false,"closure_completeness_verified":false,
            "exact_first_fall_degree":null,"exact_last_fall_degree":null})
    }

    fn censor(
        &mut self,
        node: &Node<'_>,
        row: usize,
        reason: &str,
        audit: Option<&prefix_algebra::IdentityAudit>,
        emit: &mut impl FnMut(Option<Value>, Value) -> Result<(), String>,
    ) -> Result<(), String> {
        if self.disabled {
            return Ok(());
        }
        let record = json!({"kind":"polynomial_consequence","admission_status":"bounded_unknown_suffix",
            "source_node_id":node.node_id,"action_sequence":node.sequence,"source_basis_row":row,
            "reason":reason,"identity_audit":audit,"certified_target":null,"multiplier_proof":null,
            "derivation_origin":ORIGIN,"native_operation_witness":false,
            "direct_identity_coefficient_operations_so_far":self.counts.direct_identity_coefficient_operations,
            "examined_target_rows_so_far":self.counts.examined_target_rows,
            "exact_first_fall_degree":null,"exact_last_fall_degree":null,
            "minimal_derivation_degree":null,"nonmembership_verified":false});
        let bytes = serde_json::to_vec(&record)
            .map_err(|e| e.to_string())?
            .len();
        if bytes > MAX_TERMINAL_BYTES {
            return Err("algebraic terminal record exceeds its separate bound".into());
        }
        emit(None, record)?;
        self.counts.unknown_suffix_records += 1;
        self.counts.terminal_record_bytes += bytes as u64;
        self.disabled = true;
        Ok(())
    }

    pub(super) fn emit_node(
        &mut self,
        node: Node<'_>,
        emit: &mut impl FnMut(Option<Value>, Value) -> Result<(), String>,
    ) -> Result<(), String> {
        if self.disabled {
            return Ok(());
        }
        if node.basis.len() != node.proofs.len() {
            return Err("algebraic target and certificate counts differ".into());
        }
        // This context is constructed only from the verified pre-native frame.
        // Output row indices and every property of the returned basis stay restricted.
        let feature = json!({"task":"prime-polynomial-consequence/v1","initial":node.initial,
            "ring":{"modulus":node.p.to_string(),"variables":node.active.len(),"quotient_rules":[]},
            "active_original_variables":node.active,
            "fixed_bindings":node.bindings.iter().map(|(i,x)|json!({"original_variable":i,"value":x.to_string()})).collect::<Vec<_>>(),
            "source_equations":graph(node.sources,node.active.len())});
        for (row, (target, proof)) in node.basis.iter().zip(node.proofs).enumerate() {
            if self.counts.examined_target_rows >= self.budget.rows {
                return self.censor(&node, row, "channel_target_row_cap", None, emit);
            }
            self.counts.examined_target_rows += 1;
            let audit = prefix_algebra::verify_identity(
                node.sources,
                target,
                proof,
                node.active.len(),
                node.p,
                prefix_algebra::Limits {
                    operations: self
                        .budget
                        .operations
                        .saturating_sub(self.counts.direct_identity_coefficient_operations),
                    ..prefix_algebra::Limits::default()
                },
            )?;
            self.counts.direct_identity_coefficient_operations += audit.work.coefficient_operations;
            let (result_degree, max_degree, drop) = match &audit.status {
                IdentityStatus::Verified {
                    result_degree,
                    max_generator_multiple_degree,
                    witnessed_degree_drop,
                } => (
                    *result_degree,
                    *max_generator_multiple_degree,
                    *witnessed_degree_drop,
                ),
                IdentityStatus::UnknownCap { .. } => {
                    return self.censor(&node, row, "identity_verifier_cap", Some(&audit), emit);
                }
            };
            self.counts.verified_identity_rows += 1;
            let zero = target.is_empty();
            let record = json!({"kind":"polynomial_consequence",
                "admission_status":if zero {"verified_zero"} else {"verified_nonzero"},
                "source_node_id":node.node_id,"action_sequence":node.sequence,"source_basis_row":row,
                "certified_target":graph(std::slice::from_ref(target),node.active.len()),
                "multiplier_proof":proof,"identity_audit":audit,
                "result_degree":result_degree,"max_generator_multiple_degree":max_degree,
                "witnessed_degree_drop":!zero && drop,"trivial_zero_target":zero,
                "derivation_origin":ORIGIN,"native_operation_witness":false,
                "equality_audit_reference":{"source_node_id":node.node_id,"sequence":node.sequence},
                "equality_search_and_prior_identity_verification_work":node.equality_work,
                "equality_work_scope":"combined bounded generator-multiple search and prior bidirectional identity verification; shared node cost, do not sum across target rows",
                "identity_work_scope":WORK_SCOPE,
                "exact_first_fall_degree":null,"exact_last_fall_degree":null,
                "minimal_derivation_degree":null,"closure_completeness_verified":false,
                "nonmembership_verified":false,"native_instruction_count_verified":false});
            let public = (!zero).then(|| feature.clone());
            let feature_bytes = public
                .as_ref()
                .map(serde_json::to_vec)
                .transpose()
                .map_err(|e| e.to_string())?
                .map_or(0, |bytes| bytes.len() as u64);
            let record_bytes = serde_json::to_vec(&record)
                .map_err(|e| e.to_string())?
                .len() as u64;
            let bytes = feature_bytes
                .checked_add(record_bytes)
                .ok_or("algebraic payload size overflow")?;
            if bytes
                > self
                    .budget
                    .payload_bytes
                    .saturating_sub(self.counts.serialized_payload_bytes)
            {
                return self.censor(
                    &node,
                    row,
                    "channel_serialized_payload_cap",
                    Some(&audit),
                    emit,
                );
            }
            emit(public, record)?;
            self.counts.serialized_payload_bytes += bytes;
            if zero {
                self.counts.admitted_zero_rows += 1;
            } else {
                self.counts.admitted_nonzero_rows += 1;
                self.counts.admitted_degree_drop_rows += u64::from(drop);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Fixture {
        initial: Value,
        active: Vec<usize>,
        bindings: BTreeMap<usize, u64>,
        sources: Vec<Poly>,
        basis: Vec<Poly>,
        proofs: Vec<Vec<Multiple>>,
        work: Work,
        p: u64,
    }

    fn multiple(generator: usize, coefficient: u64) -> Multiple {
        Multiple {
            generator,
            exponents: vec![0, 0],
            coefficient: coefficient.to_string(),
        }
    }

    impl Fixture {
        fn cancellation() -> Self {
            // y = (x^2+y)-x^2, with a degree-two witnessed representation.
            Self {
                initial: json!({"public":"retained-fixture","n":7}),
                active: vec![0, 1],
                bindings: BTreeMap::new(),
                sources: vec![
                    vec![(vec![2, 0], 1), (vec![0, 1], 1)],
                    vec![(vec![2, 0], 1)],
                ],
                basis: vec![vec![(vec![0, 1], 1)]],
                proofs: vec![vec![multiple(0, 1), multiple(1, 126)]],
                work: Work::default(),
                p: 127,
            }
        }

        fn node(&self) -> Node<'_> {
            Node {
                initial: &self.initial,
                active: &self.active,
                bindings: &self.bindings,
                sources: &self.sources,
                basis: &self.basis,
                proofs: &self.proofs,
                p: self.p,
                sequence: 2,
                node_id: 0,
                equality_work: &self.work,
            }
        }
    }

    fn collect(
        channel: &mut Channel,
        node: Node<'_>,
    ) -> Result<Vec<(Option<Value>, Value)>, String> {
        let mut rows = Vec::new();
        channel.emit_node(node, &mut |feature, record| {
            rows.push((feature, record));
            Ok(())
        })?;
        Ok(rows)
    }

    #[test]
    fn cancellation_is_a_restricted_witness_not_an_exact_fall_label() {
        let fixture = Fixture::cancellation();
        let mut channel = Channel::default();
        let rows = collect(&mut channel, fixture.node()).unwrap();
        assert_eq!(rows.len(), 1);
        let (feature, record) = &rows[0];
        assert_eq!(record["result_degree"], 1);
        assert_eq!(record["max_generator_multiple_degree"], 2);
        assert_eq!(record["witnessed_degree_drop"], true);
        assert!(record["exact_first_fall_degree"].is_null());
        assert!(record["exact_last_fall_degree"].is_null());
        assert_eq!(record["native_operation_witness"], false);
        let public = feature.as_ref().unwrap();
        assert_eq!(public["ring"]["modulus"], "127");
        assert_eq!(public["ring"]["quotient_rules"], json!([]));
        for forbidden in [
            "certified_target",
            "multiplier_proof",
            "identity_audit",
            "source_basis_row",
            "witnessed_degree_drop",
        ] {
            assert!(public.get(forbidden).is_none());
        }
        assert_eq!(channel.summary()["counts"]["admitted_degree_drop_rows"], 1);
    }

    #[test]
    fn public_context_ignores_target_provenance_and_costs() {
        let mut fixture = Fixture::cancellation();
        let first = collect(&mut Channel::default(), fixture.node()).unwrap();
        fixture.basis.push(fixture.sources[0].clone());
        fixture.proofs.push(vec![multiple(0, 1)]);
        fixture.work.coefficient_operations = 999;
        let mut node = fixture.node();
        node.node_id = 42;
        node.sequence = 999;
        let changed = collect(&mut Channel::default(), node).unwrap();
        assert_eq!(first[0].0, changed[0].0);
        assert_eq!(changed[0].0, changed[1].0);
        assert_ne!(first[0].1, changed[0].1);
    }

    #[test]
    fn zero_rows_stay_restricted_without_degree_drop() {
        let mut fixture = Fixture::cancellation();
        fixture.sources.push(Vec::new());
        fixture.basis = vec![Vec::new()];
        fixture.proofs = vec![vec![Multiple {
            generator: 2,
            exponents: vec![8, 0],
            coefficient: "1".into(),
        }]];
        let mut channel = Channel::default();
        let rows = collect(&mut channel, fixture.node()).unwrap();
        assert!(rows[0].0.is_none());
        assert_eq!(rows[0].1["admission_status"], "verified_zero");
        assert!(rows[0].1["result_degree"].is_null());
        assert!(rows[0].1["max_generator_multiple_degree"].is_null());
        assert_eq!(rows[0].1["witnessed_degree_drop"], false);
        assert_eq!(channel.summary()["counts"]["admitted_nonzero_rows"], 0);
    }

    #[test]
    fn operation_and_row_caps_retain_prior_rows_and_disable_only_once() {
        for operation_cap in [true, false] {
            let mut fixture = Fixture::cancellation();
            fixture.basis.push(fixture.basis[0].clone());
            fixture.proofs.push(fixture.proofs[0].clone());
            let mut channel = Channel::default();
            if operation_cap {
                channel.budget.operations = 7;
            } else {
                channel.budget.rows = 1;
            }
            let rows = collect(&mut channel, fixture.node()).unwrap();
            assert_eq!(rows.len(), 2);
            assert_eq!(rows[0].1["admission_status"], "verified_nonzero");
            assert_eq!(rows[1].1["admission_status"], "bounded_unknown_suffix");
            assert!(rows[1].0.is_none());
            assert!(rows[1].1["certified_target"].is_null());
            assert!(rows[1].1["multiplier_proof"].is_null());
            assert!(collect(&mut channel, fixture.node()).unwrap().is_empty());
            assert_eq!(channel.summary()["counts"]["unknown_suffix_records"], 1);
            if operation_cap {
                assert_eq!(
                    channel.summary()["counts"]["direct_identity_coefficient_operations"],
                    7
                );
            }
        }
    }

    #[test]
    fn payload_cap_preserves_exact_declared_byte_accounting() {
        let mut fixture = Fixture::cancellation();
        let mut reference = Channel::default();
        let one = collect(&mut reference, fixture.node()).unwrap();
        let exact_bytes = serde_json::to_vec(one[0].0.as_ref().unwrap())
            .unwrap()
            .len()
            + serde_json::to_vec(&one[0].1).unwrap().len();
        assert_eq!(
            reference.counts.serialized_payload_bytes,
            exact_bytes as u64
        );
        fixture.basis.push(fixture.basis[0].clone());
        fixture.proofs.push(fixture.proofs[0].clone());
        let mut channel = Channel::default();
        channel.budget.payload_bytes = exact_bytes as u64;
        let rows = collect(&mut channel, fixture.node()).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], one[0]);
        assert_eq!(rows[1].1["reason"], "channel_serialized_payload_cap");
        assert_eq!(channel.counts.serialized_payload_bytes, exact_bytes as u64);
        assert!(channel.counts.terminal_record_bytes <= MAX_TERMINAL_BYTES as u64);
        assert_eq!(channel.counts.verified_identity_rows, 2);
        assert_eq!(channel.counts.admitted_nonzero_rows, 1);
    }

    #[test]
    fn wide_coefficients_are_strings_in_features_and_restricted_targets() {
        let mut fixture = Fixture::cancellation();
        fixture.p = (1 << 61) - 1;
        fixture.sources = vec![vec![(vec![1, 0], fixture.p - 1)]];
        fixture.basis = fixture.sources.clone();
        fixture.proofs = vec![vec![multiple(0, 1)]];
        let rows = collect(&mut Channel::default(), fixture.node()).unwrap();
        let coefficient = (fixture.p - 1).to_string();
        assert_eq!(
            rows[0].0.as_ref().unwrap()["source_equations"]["coefficient_edges"][0]["coefficient"],
            coefficient
        );
        assert_eq!(
            rows[0].1["certified_target"]["coefficient_edges"][0]["coefficient"],
            coefficient
        );
    }

    #[test]
    fn malformed_identity_is_a_hard_error_not_an_unknown_target() {
        for mutation in 0..3 {
            let mut fixture = Fixture::cancellation();
            match mutation {
                0 => fixture.proofs[0][0].coefficient = "2".into(),
                1 => fixture.proofs[0][0].generator = 99,
                _ => {
                    let duplicate = fixture.proofs[0][0].clone();
                    fixture.proofs[0].push(duplicate);
                }
            }
            assert!(collect(&mut Channel::default(), fixture.node()).is_err());
        }
    }
}
