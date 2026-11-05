//! Distinct independently audited affine-domain finite rows. Raw rows unchanged.
use super::*;
use provenance::boolean_equations::{self as equations,affine_domain as affine};
use affine::matrix_rows as rows;
use std::mem::size_of;
#[derive(Clone, Copy)]
pub(in crate::ml_dataset::solvers) struct Caps {
    pub expansion:equations::Limits, pub audit:affine::audit::Limits,
    pub reduction:rows::Limits, pub checker:rows::CheckerLimits,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::ml_dataset::solvers) enum Failure {
    Admission(Stop),MissingMap,Expansion(affine::Failure),Audit(affine::audit::Failure),
    Reduction(rows::Failure),Certification(rows::Failure),Publication(Stop),
}
#[cfg(test)] mod tests {
 use super::*;
 pub(super) fn caps()->Caps {
    let expansion=equations::Limits{variables:4096,generators:4096,terms:4096,degree:64,work:4_000_000,bytes:8_000_000};
    Caps{expansion,audit:expansion,
        reduction:rows::Limits{variables:4096,generators:4096,window:1,candidates:4096,basis:4096,terms:4096,degree:64,proof:4096,wire:65536,work:4_000_000,bytes:8_000_000},
        checker:rows::CheckerLimits{variables:4096,generators:4096,polynomial_terms:4096,proof_terms:4096,accumulator_entries:4096,caps:[65536,262144,262144,262144,262144,65536,1024,1048576,262144]}}
 }
}
struct OriginalParts {
    public_sha256: [u8; 32],
    factor_sha256: [u8; 32],
    budget: Budget,
    binding: [u8; 32],
    launch_sha256: [u8; 32],
    math: Math,
    checked: Option<provenance::CheckedMap>,
    unchecked: Option<provenance::Map>,
    pattern: Option<u32>,
    map_bytes: Vec<u8>,
    map_ack: Option<capture::CommittedEventPosition>,
    publication_attempted: bool,
    map_published: bool,
    capture_limits: capture::CaptureLimits,
    boolean_attempted: bool,
}
enum OwnedState {
    Before,
    Expanded(affine::Attempt),
    Audited(affine::audit::Audited),
    Reduced(rows::OwnedRows),
    Certified(rows::OwnedCertificate),
}
pub(in crate::ml_dataset::solvers) struct OwnedAttempt {
    original: OriginalParts,
    sink: capture::CaptureEmitter,
    state: OwnedState,
    failure: Option<Failure>,
    deadline: Instant,
    diagnostic: Option<String>,
    selected_row: usize,
    publication: Publication,
}
impl OwnedAttempt {
    pub(in crate::ml_dataset::solvers) fn failure(&self) -> Option<Failure> {
        self.failure
    }
    pub(in crate::ml_dataset::solvers) fn charged(&self) -> (u64, u64) {
        (
            self.original.budget.checking_work,
            self.original.budget.adapter_bytes,
        )
    }
    pub(in crate::ml_dataset::solvers) fn first_stop(&self) -> Option<Stop> {
        self.original.budget.stopped
    }
    pub(in crate::ml_dataset::solvers) fn certificate(&self) -> Option<&rows::OwnedCertificate> {
        match &self.state {
            OwnedState::Certified(c) => Some(c),
            _ => None,
        }
    }
    fn stop(&mut self, failure: Failure, stop: Stop) {
        self.failure = Some(failure);
        let _: Result<(), Stop> = self.original.budget.fail(stop);
    }
    fn run(&mut self, row: usize, caps: Caps) {
        let Some(map) = self.original.checked.take() else {
            self.stop(Failure::MissingMap, Stop::Phase);
            return;
        };
        let expansion =
            affine::construct(map, caps.expansion, &mut Admit(&mut self.original.budget));
        if let Err(f) = expansion.status() {
            self.state = OwnedState::Expanded(expansion);
            self.stop(Failure::Expansion(f), Stop::NativeFailure);
            return;
        }
        let audited=affine::audit::audit(expansion,caps.audit,&mut Admit(&mut self.original.budget));
        if let Err(f)=audited.status(){self.state=OwnedState::Audited(audited);self.stop(Failure::Audit(f),Stop::NativeFailure);return}
        let reduced = rows::reduce_unit(
            audited,
            caps.reduction,
            &mut Admit(&mut self.original.budget),
        );
        if let Some(f) = reduced.failure() {
            self.state = OwnedState::Reduced(reduced);
            self.stop(Failure::Reduction(f), Stop::NativeFailure);
            return;
        }
        let certificate = rows::certify_owned(
            reduced,
            row,
            caps.reduction,
            caps.checker,
            self.deadline,
            &mut Admit(&mut self.original.budget),
        );
        let failure = certificate.failure();
        self.state = OwnedState::Certified(certificate);
        if let Some(f) = failure {
            self.stop(Failure::Certification(f), Stop::NativeFailure)
        } else if let Err(s) = self.original.budget.check(Instant::now()) {
            self.failure = Some(Failure::Admission(s))
        }
    }
}
impl OriginalOwner<'_> {
    /// Consumes the original owner and actual bound sink. It always returns
    /// custody, including on initial frame admission or sink-identity failure.
    pub(super) fn into_affine_row(
        mut self,
        sink: capture::CaptureEmitter,
        row: usize,
        caps: Caps,
    ) -> OwnedAttempt {
        let repeated = self.boolean_attempted;
        self.boolean_attempted = true;
        let deadline = if matches!(
            self.budget.phase,
            super::super::Phase::Setup | super::super::Phase::Construction
        ) {
            self.budget.construction_deadline
        } else {
            self.budget.deadline
        };
        let mut diagnostic = None;
        let admission = (|| -> Result<(), Stop> {
            if repeated {
                return self.budget.fail(Stop::Phase);
            }
            self.budget.reserve_checking(
                8192,
                size_of::<OwnedAttempt>() as u64 + 65536,
                Instant::now(),
            )?;
            let Some(binding) = self.original.binding else {
                return self.budget.fail(Stop::InvalidLimits);
            };
            if let Err(error) = sink.verify_directory(&self.original.root.join("live")) {
                diagnostic = Some(error);
                return self.budget.fail(Stop::Observer);
            }
            if let Err(error) = sink.verify_attempt_limits(binding, &self.capture_limits) {
                diagnostic = Some(error);
                return self.budget.fail(Stop::Observer);
            }
            self.budget.check(Instant::now())
        })();
        let OriginalOwner {
            original,
            budget,
            binding,
            launch_sha256,
            math,
            checked,
            retained_unchecked,
            pattern,
            retained_bytes,
            map_ack,
            publication_attempted,
            map_published,
            capture_limits,
            boolean_attempted,
        } = self;
        let mut out = OwnedAttempt {
            original: OriginalParts {
                public_sha256: original.public_sha256,
                factor_sha256: original.factor_base_sha256,
                budget,
                binding,
                launch_sha256,
                math,
                checked,
                unchecked: retained_unchecked,
                pattern,
                map_bytes: retained_bytes,
                map_ack,
                publication_attempted,
                map_published,
                capture_limits,
                boolean_attempted,
            },
            sink,
            state: OwnedState::Before,
            failure: None,
            deadline,
            diagnostic,
            selected_row: row,
            publication: Publication::default(),
        };
        if let Err(s) = admission {
            out.failure = Some(Failure::Admission(s));
            return out;
        }
        out.run(row, caps);
        out
    }
}

#[cfg(test)]
mod owned_tests {
    use super::super::super::super::verified_worker_original_tests::Fixture;
    use super::tests::caps;
    use super::*;
    use crate::ml_dataset::solvers;
    fn genuine_owned(twist: bool, refuse_wire: bool) -> OwnedAttempt { genuine_owned_fixture(twist,refuse_wire).0 }
    fn genuine_owned_fixture(twist: bool, refuse_wire: bool) -> (OwnedAttempt,Fixture,Vec<crate::ml_dataset::types::Artifact>) {
        let f = Fixture::with_original_row_shape(23, twist, "f4", 16384);
        execution::create(&f.root).unwrap();
        let mut registry = Vec::new();
        let generated = crate::ml_dataset::telemetry::measured(
            &f.root,
            &mut registry,
            "curve-construction",
            || crate::ml_dataset::curves::generate(&f.config),
        )
        .unwrap();
        let token = solvers::verify_and_store_worker_originals(
            &f.config,
            &generated,
            &f.root,
            Some(&f.binding),
            &mut registry,
        )
        .unwrap();
        let context = execution::admit(&f.root).unwrap().unwrap();
        let (_, limits) = crate::ml_dataset::admit_capture_context(&f.root)
            .unwrap()
            .unwrap();
        let mut sink = capture::CaptureEmitter::create(
            &f.root.join("live"),
            f.binding.clone(),
            limits.clone(),
        )
        .unwrap();
        let mut owner = match OriginalOwner::admit(&token, &generated, &context, &sink, &limits) {
            Ok(o) => o,
            Err(_) => panic!("genuine original required"),
        };
        owner
            .build_map(
                1,
                provenance::Limits {
                    variables: 4096,
                    codes: 16,
                    domain_clauses: 4096,
                    work: 4_000_000,
                    bytes: 8_000_000,
                },
            )
            .unwrap();
        owner.persist_map(&mut sink).unwrap();
        let before = (owner.budget.checking_work, owner.budget.adapter_bytes);
        let deadline = owner.budget.construction_deadline;
        let mut c = caps();
        if refuse_wire {
            c.reduction.wire = 0
        }
        let result = owner.into_affine_row(sink, 0, c);
        assert!(result.charged().0 > before.0);
        assert_eq!(result.deadline, deadline);
        assert!(result.original.map_ack.is_some());
        assert!(!result.original.map_bytes.is_empty());
        for (path,role) in crate::ml_dataset::CAPTURE_CONTEXT.into_iter().chain([(execution::PATH,execution::ROLE)]) {
            let(sha256,bytes)=crate::store::hash_file(&f.root.join(path)).unwrap();
            registry.push(crate::ml_dataset::types::Artifact{path:path.into(),role:role.into(),encoding:"json".into(),sha256,bytes});
        }
        // The token and generated case drop; retain Fixture for filesystem followups.
        (result,f,registry)
    }
    #[test]
    fn affine_original_unit_window_certificate_both_twists_requires_success() {
        for twist in [false,true] {
            let(owner,fixture,_)=genuine_owned_fixture(twist,false);
            assert_eq!(owner.failure(),None);assert_eq!(owner.first_stop(),None);
            let cert=owner.certificate().unwrap();let audited=cert.rows().source();
            assert!(audited.status().is_ok());let affine=audited.source();
            assert!(affine.status().is_ok());let identity=affine.identity().unwrap();
            assert_eq!(identity.schema,affine::SCHEMA);assert_eq!(identity.order,affine::ORDER);
            assert_eq!(identity.original_binding,owner.original.binding);
            assert_eq!(identity.original_binding,affine.source().original_binding());
            assert!(cert.encoding().unwrap().matches_identity(identity));
            assert!(cert.audit().unwrap().result.is_ok());
            assert!(cert.rows().rows()>0);assert!(owner.charged().0>0);
            assert!(!owner.publication.attempted);assert!(!owner.publication.committed);
            assert!(fixture.root.join("public.json").is_file());
        }
    }
    #[test]
    fn owned_actual_certificate_moves_through_worker_and_parent_refusal_without_copy_or_reset() {
        for twist in [false, true] {
            let owner = genuine_owned(twist, false);
            fn needs_static<T: 'static>(_: &T) {}
            needs_static(&owner);
            assert_eq!(owner.failure(), None);
            let certificate = owner.certificate().unwrap();
            assert!(certificate.audit().unwrap().result.is_ok());
            let raw = certificate.raw().unwrap().as_ptr();
            let charges = owner.charged();
            let output = solvers::WorkerSolveOutput::from_original_affine_rows(owner);
            let mut retained = solvers::WorkerResidualCustody::default();
            let mut registry = Vec::new();
            let result = crate::ml_dataset::finish_retained_worker_output(
                &std::env::temp_dir(),
                &mut registry,
                output,
                Ok(()),
                &mut retained,
            );
            assert!(result.is_err());
            let kept = retained.affine_rows.as_ref().unwrap();
            assert_eq!(kept.charged(), charges);
            assert_eq!(kept.certificate().unwrap().raw().unwrap().as_ptr(), raw);
            assert_eq!(kept.failure(), None);
            assert!(registry.is_empty());
            assert!(retained.affine_rows_report_unavailable());
        }
    }
    #[test]
    fn owned_encoder_cap_failure_keeps_rows_map_and_original_ledger_after_parent_adoption() {
        let owner = genuine_owned(false, true);
        assert_eq!(
            owner.failure(),
            Some(Failure::Certification(rows::Failure::Matrix(rows::MatrixFailure::Cap)))
        );
        assert_eq!(owner.first_stop(), Some(Stop::NativeFailure));
        assert!(owner.certificate().unwrap().rows().rows() > 0);
        let charges = owner.charged();
        let output = solvers::WorkerSolveOutput::from_original_affine_rows(owner);
        let mut retained = solvers::WorkerResidualCustody::default();
        let mut registry = Vec::new();
        assert!(crate::ml_dataset::finish_retained_worker_output(
            &std::env::temp_dir(),
            &mut registry,
            output,
            Err("parent failure control".into()),
            &mut retained
        )
        .is_err());
        let kept = retained.affine_rows.as_ref().unwrap();
        assert_eq!(kept.charged(), charges);
        assert_eq!(kept.first_stop(), Some(Stop::NativeFailure));
        assert!(
            kept.certificate()
                .unwrap()
                .rows()
                .source()
                .source()
                .source()
                .variables()
                > 0
        );
        assert!(kept.original.map_ack.is_some());
    }

    #[test]
    fn committed_finite_row_receipt_requires_actual_both_twist_central_replay(){
        for twist in [false,true]{
            let(owner,fixture,mut artifacts)=genuine_owned_fixture(twist,false);
            let root=&fixture.root; let config=&fixture.config;
            assert!(root.join("public.json").is_file());
            assert_eq!(owner.failure(),None);
            let before=owner.sink.committed_event_count();
            let published=owner.publish();
            assert_eq!(published.failure(),None);
            assert!(published.publication.committed);
            assert_eq!(published.publication.ack.as_ref().unwrap().event_count,before+1);
            let output=solvers::WorkerSolveOutput::from_original_affine_rows(published);
            let mut retained=solvers::WorkerResidualCustody::default();
            let outcome=crate::ml_dataset::finish_retained_worker_output(&root,&mut artifacts,output,Ok(()),&mut retained).unwrap();
            artifacts.push(crate::ml_dataset::storage::write_json(&root,"result.json","solver-outcome",&outcome).unwrap());
            assert!(retained.affine_rows.as_ref().unwrap().publication.committed);
            assert_eq!(outcome["status"],"partial");
            let policy=super::super::affine_central::Policy{work:1_000_000_000_000,bytes:8_000_000_000,deadline:Instant::now()+std::time::Duration::from_secs(120),stream_bytes:128<<20,payload_bytes:128<<20};
            let c=caps();
            let checked=super::super::affine_central::verify_affine_row(&root,&config,&outcome,&artifacts,policy,super::super::affine_central::RowLimits{expansion:c.expansion,audit:c.audit,checker:c.checker});
            match checked {Ok(super::super::affine_central::RowOutcome::Verified(v))=>{assert!(v.audit().result.is_ok());assert!(!v.audit().source_authenticated);assert!(v.usage().0>0)},_=>panic!("actual original/receipt/source-bound replay required")}
        }
    }
    #[test]
    fn same_affine_owner_revalidates_actual_bytes_and_retains_sticky_failure(){
        for twist in [false,true]{
            let(owner,fixture,mut artifacts)=genuine_owned_fixture(twist,false);
            let root=&fixture.root; let config=&fixture.config;
            assert!(root.join("public.json").is_file());
            assert_eq!(owner.failure(),None);
            let before=owner.sink.committed_event_count();
            let published=owner.publish();
            assert_eq!(published.failure(),None);
            assert!(published.publication.committed);
            assert_eq!(published.publication.ack.as_ref().unwrap().event_count,before+1);
            let output=solvers::WorkerSolveOutput::from_original_affine_rows(published);
            let mut retained=solvers::WorkerResidualCustody::default();
            let outcome=crate::ml_dataset::finish_retained_worker_output(&root,&mut artifacts,output,Ok(()),&mut retained).unwrap();
            artifacts.push(crate::ml_dataset::storage::write_json(&root,"result.json","solver-outcome",&outcome).unwrap());
            assert!(retained.affine_rows.as_ref().unwrap().publication.committed);
            assert_eq!(outcome["status"],"partial");
            let policy=super::super::affine_central::Policy{work:1_000_000_000_000,bytes:8_000_000_000,deadline:Instant::now()+std::time::Duration::from_secs(120),stream_bytes:128<<20,payload_bytes:128<<20};
            let c=caps();
            let checked=super::super::affine_central::verify_affine_row(&root,&config,&outcome,&artifacts,policy,super::super::affine_central::RowLimits{expansion:c.expansion,audit:c.audit,checker:c.checker});
            match checked {Ok(super::super::affine_central::RowOutcome::Verified(mut v))=>{
                let before=v.usage();v.revalidate(&root,&config,&outcome,&artifacts).unwrap();assert!(v.usage().0>before.0);
                let path=root.join("live/payloads").join(receipt::path(1,0));
                let mut bytes=std::fs::read(&path).unwrap();bytes.push(b' ');std::fs::write(&path,bytes).unwrap();
                assert!(v.revalidate(&root,&config,&outcome,&artifacts).is_err());let failed=v.usage();
                assert!(v.revalidate(&root,&config,&outcome,&artifacts).is_err());assert_eq!(v.usage(),failed);
            },_=>panic!("actual original/receipt/source-bound replay required")}
        }
    }
    #[test]
    fn receipt_report_admission_refusal_keeps_rows_and_does_not_advance_capture(){
        let mut owner=genuine_owned(false,false);
        let before=owner.sink.committed_event_position();
        // Deliberate already-exhausted original deadline, never a fresh cap.
        owner.original.budget.construction_deadline=Instant::now();
        let charges=owner.charged();
        let published=owner.publish();
        assert_eq!(published.failure(),Some(Failure::Publication(Stop::Deadline)));
        assert_eq!(published.charged(),charges);
        assert_eq!(published.sink.committed_event_position(),before);
        assert!(published.certificate().is_some());
        assert!(!published.publication.committed);
    }

    #[test]
    fn actual_report_storage_admission_refusal_is_retained_before_receipt_write(){
        let mut owner=genuine_owned(false,false);
        let raw=owner.certificate().unwrap().raw().unwrap().len() as u64;
        let scan=(raw+owner.original.map_bytes.len() as u64)*16;
        let b=&mut owner.original.budget;
        let remaining=b.native_work_limit-b.native.work-b.checking_work;
        assert!(remaining>scan+64);
        b.reserve_checking(remaining-scan-64,0,Instant::now()).unwrap();
        let before=owner.charged();let head=owner.sink.committed_event_position();
        let published=owner.publish();
        assert_eq!(published.failure(),Some(Failure::Publication(Stop::Work)));
        assert_eq!(published.charged().0,before.0+scan,"wire scan passed; storage/report quote refused");
        assert!(published.publication.metadata.is_none());assert!(published.publication.bytes.is_empty());
        assert_eq!(published.sink.committed_event_position(),head);
        assert!(published.certificate().is_some());
    }
    #[test]
    fn deadline_after_actual_row_ack_keeps_ack_and_certificate_but_refuses_report() {
        let (mut owner, fixture, mut registry) = genuine_owned_fixture(false, false);
        assert_eq!(owner.failure(), None);
        let before = owner.sink.committed_event_position();
        let raw = owner.certificate().unwrap().raw().unwrap().as_ptr();
        let charges = owner.charged();
        owner.publication.expire_after_ack = true;
        let mut published = owner.publish();
        assert_eq!(published.failure(), Some(Failure::Publication(Stop::Deadline)));
        assert_eq!(published.first_stop(), Some(Stop::Deadline));
        assert!(published.charged().0 > charges.0);
        assert!(published.charged().1 > charges.1);
        assert!(!published.publication.bytes.is_empty());
        assert!(published.publication.capture_error.is_none());
        let ack = published.publication.ack.clone().expect("actual payload acknowledged");
        assert_eq!(ack.event_count, before.event_count + 1);
        assert!(ack.byte_end > before.byte_end);
        assert_eq!(published.sink.committed_event_position(), ack);
        assert!(!published.publication.committed);
        assert!(published.outcome().is_none());
        let head = std::fs::read(fixture.root.join("live/committed.json")).unwrap();
        let charged = published.charged();
        // A repeated publication cannot erase the first stop or advance capture.
        let published = published.publish();
        assert_eq!(published.charged(), charged);
        let output = solvers::WorkerSolveOutput::from_original_affine_rows(published);
        let mut retained = solvers::WorkerResidualCustody::default();
        assert!(crate::ml_dataset::finish_retained_worker_output(
            &fixture.root, &mut registry, output, Ok(()), &mut retained,
        ).is_err());
        assert!(retained.affine_rows_report_unavailable());
        let kept = retained.affine_rows.as_ref().unwrap();
        assert_eq!(kept.charged(), charged);
        assert_eq!(kept.first_stop(), Some(Stop::Deadline));
        assert_eq!(kept.publication.ack.as_ref(), Some(&ack));
        assert_eq!(kept.certificate().unwrap().raw().unwrap().as_ptr(), raw);
        assert_eq!(head, std::fs::read(fixture.root.join("live/committed.json")).unwrap());
    }

    #[test]
    fn paid_row_report_survives_actual_terminal_telemetry_write_failure() {
        let (owner, fixture, mut registry) = genuine_owned_fixture(false, false);
        assert_eq!(owner.failure(), None);
        let raw = owner.certificate().unwrap().raw().unwrap().as_ptr();
        let mut produced = None;
        let measurement = crate::ml_dataset::telemetry::measured(
            &fixture.root, &mut registry, "solver", || {
                let published = owner.publish();
                assert_eq!(published.failure(), None);
                assert!(published.publication.committed);
                let output = solvers::WorkerSolveOutput::from_original_affine_rows(published);
                assert_eq!(output.data.outcome["status"], "partial");
                assert_eq!(output.data.outcome["finite_row_receipt_committed"], true);
                assert!(!output.residual.affine_rows_report_unavailable());
                produced = Some(output);
                // Real start record and paid report exist; fault only the actual
                // terminal telemetry write, without a fabricated error result.
                std::fs::create_dir(fixture.root.join("telemetry/solver.json")).unwrap();
                Ok(())
            },
        );
        assert!(measurement.is_err());
        assert!(fixture.root.join("telemetry/solver-start.json").is_file());
        let output = produced.expect("paid row output survives measurement failure");
        let owner = output.residual.affine_rows.as_ref().unwrap();
        assert!(owner.publication.report_taken);
        let charges = owner.charged();
        let ack = owner.publication.ack.clone().unwrap();
        assert_eq!(output.data.outcome["receipt_sequence"], ack.event_count);
        assert_eq!(output.data.outcome["receipt_sha256"], ack.last_event_sha256);
        let head = std::fs::read(fixture.root.join("live/committed.json")).unwrap();
        let mut retained = solvers::WorkerResidualCustody::default();
        assert!(crate::ml_dataset::finish_retained_worker_output(
            &fixture.root, &mut registry, output, measurement, &mut retained,
        ).is_err());
        let kept = retained.affine_rows.as_mut().unwrap();
        assert!(kept.publication.committed);
        assert!(kept.publication.report_taken);
        assert_eq!(kept.failure(), None);
        assert_eq!(kept.charged(), charges);
        assert_eq!(kept.publication.ack.as_ref(), Some(&ack));
        assert_eq!(kept.certificate().unwrap().raw().unwrap().as_ptr(), raw);
        assert!(kept.outcome().is_none(), "paid report remains one-shot");
        assert_eq!(head, std::fs::read(fixture.root.join("live/committed.json")).unwrap());
    }

    #[test]
    fn actual_payload_write_failure_preserves_encoded_receipt_and_certificate(){
        let(owner,fixture,_)=genuine_owned_fixture(false,false);
        let root=&fixture.root;
        assert!(root.join("public.json").is_file());
        let head=owner.sink.committed_event_position();
        let path=receipt::path(owner.original.pattern.unwrap(),owner.selected_row as u64);
        std::fs::create_dir(root.join("live/payloads").join(path)).unwrap();
        let published=owner.publish();
        assert_eq!(published.failure(),Some(Failure::Publication(Stop::Observer)));
        assert!(!published.publication.bytes.is_empty());
        assert!(published.publication.metadata.is_some());
        assert!(published.publication.capture_error.is_some());
        assert_eq!(published.sink.committed_event_position(),head);
        assert!(!published.publication.committed);assert!(published.certificate().is_some());
    }
}

#[path="affine_row_receipt.rs"]
pub(in crate::ml_dataset::solvers) mod receipt;
#[derive(Default)]
struct Publication {
    attempted:bool,metadata:Option<receipt::Metadata>,bytes:Vec<u8>,
    ack:Option<capture::CommittedEventPosition>,committed:bool,report_taken:bool,
    path:Option<String>,capture_error:Option<capture::CaptureError>,
    #[cfg(test)]
    expire_after_ack: bool,
}
struct ReceiptWriter<'a>{bytes:&'a mut Vec<u8>,cap:usize}
impl std::io::Write for ReceiptWriter<'_>{
    fn write(&mut self,b:&[u8])->std::io::Result<usize>{
        if self.bytes.len().checked_add(b.len()).filter(|n|*n<=self.cap).is_none(){return Err(std::io::ErrorKind::InvalidData.into())}
        self.bytes.extend_from_slice(b);Ok(b.len())
    }
    fn flush(&mut self)->std::io::Result<()>{Ok(())}
}
impl OwnedAttempt {
    pub(in crate::ml_dataset::solvers) fn publish(mut self)->Self {
        if self.failure.is_some(){return self}
        if let Err(stop)=self.publish_inner(){self.stop(Failure::Publication(stop),stop)}
        self
    }
    fn publish_inner(&mut self)->Result<(),Stop>{
        if self.publication.attempted{return Err(Stop::Phase)}
        self.publication.attempted=true;
        let OwnedState::Certified(certificate)=&self.state else{return Err(Stop::Phase)};
        if certificate.failure().is_some()||certificate.audit().is_none_or(|a|a.result.is_err())||!self.original.map_published{return Err(Stop::Phase)}
        let raw=certificate.raw().ok_or(Stop::Phase)?;
        // Pay the actual-wire scan/hash before inspecting its canonical alphabet.
        self.original.budget.reserve_checking((raw.len() as u64).checked_add(self.original.map_bytes.len() as u64).and_then(|n|n.checked_mul(16)).ok_or(Stop::Overflow)?,65536,Instant::now())?;
        let cap=receipt::cap(raw).ok_or(Stop::Serialization)?;
        if cap>self.original.capture_limits.max_payload_bytes{return Err(Stop::Bytes)}
        let cap_usize=usize::try_from(cap).map_err(|_|Stop::Overflow)?;
        let before=self.sink.committed_event_position();
        if self.original.map_ack.as_ref()!=Some(&before){return Err(Stop::Observer)}
        let storage=super::super::quote_parent_storage_length(cap,false,&self.original.capture_limits)?;
        let report=super::super::quote_parent_storage_length(receipt::REPORT_CAP,true,&self.original.capture_limits)?;
        let encoding=cap.checked_mul(16).ok_or(Stop::Overflow)?;
        self.original.budget.reserve_checking(storage.work.checked_add(report.work).and_then(|w|w.checked_add(encoding)).and_then(|w|w.checked_add(16*receipt::REPORT_CAP)).ok_or(Stop::Overflow)?,storage.bytes.checked_add(report.bytes).and_then(|b|b.checked_add(cap)).and_then(|b|b.checked_add(262144)).ok_or(Stop::Overflow)?,Instant::now())?;
        self.original.budget.reserve_trace(storage.trace.checked_add(report.trace).ok_or(Stop::Overflow)?,Instant::now())?;
        // No paid operation remains outside this bound before the snapshot is encoded.
        self.publication.bytes.try_reserve_exact(cap_usize).map_err(|_|Stop::Allocation)?;
        let mut map_event=[0;32];hex::decode_to_slice(&before.last_event_sha256,&mut map_event).map_err(|_|Stop::Serialization)?;
        let row=u64::try_from(self.selected_row).map_err(|_|Stop::Overflow)?;
        self.publication.path=Some(receipt::path(self.original.pattern.ok_or(Stop::Phase)?,row));
        let b=&self.original.budget;
        if let Some(stop)=b.stopped{return Err(stop)}
        self.publication.metadata=Some(receipt::Metadata{
            encoding:rows::EncodingBinding::from_identity(certificate.rows().source().source().identity().ok_or(Stop::Phase)?),
            original_binding:self.original.binding,launch_sha256:self.original.launch_sha256,
            pattern:self.original.pattern.unwrap(),row,window:0,
            variables:certificate.rows().source().source().variables(),generators:certificate.rows().source().source().equations().len() as u64,
            map_sha256:Sha256::digest(&self.original.map_bytes).into(),map_bytes:self.original.map_bytes.len() as u64,
            map_sequence:before.event_count,map_event_sha256:map_event,
            wire_sha256:Sha256::digest(raw).into(),wire_bytes:raw.len() as u64,
            limits:[b.native_work_limit,b.memory_limit,b.trace_limit,b.addition_limit],
            totals:[b.native.work,b.native.bytes,b.checking_work,b.checking_performed_work,b.adapter_bytes,b.trace_bytes,b.additions],
            phase:b.phase as u8,phase_native:[b.phase_native.work,b.phase_native.bytes],phase_limits:[b.phase_limits.work,b.phase_limits.bytes],
            first_stop:b.stopped,solver_completed:false,full_span_search:false,scalar_recovered:false,
        });
        let wire=receipt::Wire{schema:receipt::SCHEMA,metadata:self.publication.metadata.as_ref().unwrap(),wire:std::str::from_utf8(raw).map_err(|_|Stop::Serialization)?};
        serde_json::to_writer(ReceiptWriter{bytes:&mut self.publication.bytes,cap:cap_usize},&wire).map_err(|_|Stop::Serialization)?;
        self.original.budget.check(Instant::now())?;
        let result=self.sink.payload_borrowed(self.publication.path.as_ref().unwrap(),receipt::ROLE,"json",&self.publication.bytes);
        if let Err(error)=result{self.publication.capture_error=Some(error);return Err(Stop::Observer)}
        let after=self.sink.committed_event_position();
        let exact=after.event_count==before.event_count.checked_add(1).ok_or(Stop::Overflow)?&&after.binding_sha256==before.binding_sha256&&after.byte_end>before.byte_end;
        self.publication.ack=Some(after);
        if !exact{return Err(Stop::Observer)}
        // Test-only fault at the precise real acknowledgement boundary. The
        // ordinary clock/deadline check below must produce and retain the stop.
        #[cfg(test)]
        if self.publication.expire_after_ack {
            let expired = Instant::now();
            self.original.budget.construction_deadline = expired;
            self.original.budget.deadline = expired;
        }
        self.original.budget.check(Instant::now())?;
        self.publication.committed=true;Ok(())
    }
    pub(in crate::ml_dataset::solvers) fn outcome(&mut self)->Option<Value>{
        if !self.publication.committed||self.failure.is_some()||self.publication.report_taken{return None}
        if let Err(stop)=self.original.budget.check(Instant::now()){self.stop(Failure::Publication(stop),stop);return None}
        self.publication.report_taken=true;
        let ack=self.publication.ack.as_ref()?;
        Some(serde_json::json!({"adapter_id":"original-affine-unit-row-v1","strategy":"original-affine-unit-row-v1","status":"partial","finite_row_receipt_committed":true,"solver_completed":false,"scalar_recovered_without_witness":false,"ecdlp_pipeline_completed":false,"full_span_search":false,"pattern":self.original.pattern,"row":self.selected_row,"receipt_sequence":ack.event_count,"receipt_sha256":ack.last_event_sha256}))
    }
}
