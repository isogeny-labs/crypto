//! Strict admitted search/model replay owned by the original central Frame.
//! Diagnostic search transitions are not a checked CDCL or UNSAT derivation.
use super::*;
use serde::Deserialize;
use crate::cryptanalysis::sat as native;
use native::Lit;
use provenance::checked_model::{self, VerificationAdmission};
#[path = "sat_search_segment.rs"]
mod segment;
impl VerificationAdmission for ReplayBudget {
    fn addition(&mut self) -> Result<(), provenance::Error> {
        // Actual signed additions consume this same central replay work pool.
        self.reserve_checking(64, 0, Instant::now()).map_err(|_| provenance::Error::Admission)
    }
}
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum SolverEvent {
    Snapshot {
        version: u32,
        variables: u32,
        trail_limits: Vec<usize>,
        already_unsat: bool,
    },
    Clause {
        clause_id: u32,
        literals: Vec<Lit>,
        existing: bool,
        detached: bool,
    },
    XorConstraint {
        row_id: usize,
        mask: Vec<u64>,
        rhs: bool,
    },
    /// A newly added constraint refuted immediately, before arena storage.
    /// The clause is retained before root simplification; XOR duplicates are
    /// retained so a consumer can independently check their cancellation.
    FormulaContradiction {
        level: usize,
        constraint: SolverEventConstraint,
    },
    XorMatrixReset {},
    XorMatrixRow {
        row_id: usize,
        mask: Vec<u64>,
        rhs: bool,
    },
    XorRowXor {
        destination: usize,
        source: usize,
    },
    XorImplication {
        row_id: usize,
        literal: Lit,
        explanation: Vec<Lit>,
    },
    XorConflict {
        row_id: usize,
        explanation: Vec<Lit>,
    },
    SolveBegin {
        conflicts: u64,
        conflict_budget: u64,
    },
    Decision {
        literal: Lit,
        level: usize,
        trail_index: usize,
        activity_bits: u64,
        priority: bool,
    },
    Assignment {
        literal: Lit,
        level: i32,
        trail_index: usize,
        initial: bool,
        reason: SolverEventReason,
    },
    Conflict {
        level: usize,
        reason: SolverEventReason,
        literals: Vec<Lit>,
    },
    AnalysisReason {
        resolved_literal: Lit,
        reason: SolverEventReason,
        literals: Vec<Lit>,
    },
    Learned {
        clause_id: u32,
        literals: Vec<Lit>,
        backjump_level: i32,
    },
    Backjump {
        from_level: usize,
        to_level: usize,
        retained_trail: usize,
        removed: Vec<Lit>,
    },
    Restart {
        restart: u64,
        conflicts: u64,
    },
    DeleteClause {
        clause_id: u32,
    },
    SolveEnd {
        result: String,
    },
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum SolverEventConstraint {
    Clause { literals: Vec<Lit> },
    Xor { variables: Vec<u32>, rhs: bool },
}

#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum SolverEventReason {
    Decision {},
    Clause {
        clause_id: u32,
    },
    /// Copy the explanation immediately: native XOR reason buffers are reused.
    Xor {
        explanation: Vec<Lit>,
    },
}

impl SolverEventConstraint { fn native(self) -> native::SolverEventConstraint { match self {
Self::Clause {literals} => native::SolverEventConstraint::Clause {literals},
Self::Xor {variables, rhs} => native::SolverEventConstraint::Xor {variables, rhs},
} } }
impl SolverEventReason { fn native(self) -> native::SolverEventReason { match self {
Self::Decision {} => native::SolverEventReason::Decision,
Self::Clause {clause_id} => native::SolverEventReason::Clause {clause_id},
Self::Xor {explanation} => native::SolverEventReason::Xor {explanation},
} } }
impl SolverEvent { fn native(self) -> native::SolverEvent { match self {
Self::Snapshot {version, variables, trail_limits, already_unsat} => native::SolverEvent::Snapshot {version, variables, trail_limits, already_unsat},
Self::Clause {clause_id, literals, existing, detached} => native::SolverEvent::Clause {clause_id, literals, existing, detached},
Self::XorConstraint {row_id, mask, rhs} => native::SolverEvent::XorConstraint {row_id, mask, rhs},
Self::FormulaContradiction {level, constraint} => native::SolverEvent::FormulaContradiction {level, constraint: constraint.native()},
Self::XorMatrixReset {} => native::SolverEvent::XorMatrixReset,
Self::XorMatrixRow {row_id, mask, rhs} => native::SolverEvent::XorMatrixRow {row_id, mask, rhs},
Self::XorRowXor {destination, source} => native::SolverEvent::XorRowXor {destination, source},
Self::XorImplication {row_id, literal, explanation} => native::SolverEvent::XorImplication {row_id, literal, explanation},
Self::XorConflict {row_id, explanation} => native::SolverEvent::XorConflict {row_id, explanation},
Self::SolveBegin {conflicts, conflict_budget} => native::SolverEvent::SolveBegin {conflicts, conflict_budget},
Self::Decision {literal, level, trail_index, activity_bits, priority} => native::SolverEvent::Decision {literal, level, trail_index, activity_bits, priority},
Self::Assignment {literal, level, trail_index, initial, reason} => native::SolverEvent::Assignment {literal, level, trail_index, initial, reason: reason.native()},
Self::Conflict {level, reason, literals} => native::SolverEvent::Conflict {level, reason: reason.native(), literals},
Self::AnalysisReason {resolved_literal, reason, literals} => native::SolverEvent::AnalysisReason {resolved_literal, reason: reason.native(), literals},
Self::Learned {clause_id, literals, backjump_level} => native::SolverEvent::Learned {clause_id, literals, backjump_level},
Self::Backjump {from_level, to_level, retained_trail, removed} => native::SolverEvent::Backjump {from_level, to_level, retained_trail, removed},
Self::Restart {restart, conflicts} => native::SolverEvent::Restart {restart, conflicts},
Self::DeleteClause {clause_id} => native::SolverEvent::DeleteClause {clause_id},
Self::SolveEnd {result} => native::SolverEvent::SolveEnd {result},
} } }
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Wire {
    schema: String,
    binding_sha256: String,
    sequence: u64,
    previous_event_sha256: String,
    event: Envelope,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Envelope { kind: String, payload: Payload }
#[derive(Deserialize)]
#[serde(tag="schema", deny_unknown_fields)]
enum Payload {
    #[serde(rename="binary-sat-search-event/v1")]
    Event {
        original_binding:[u8;32], pattern:u32,
        initial_checkpoint_sequence:u64, initial_checkpoint_sha256:String,
        native_index:usize, event:SolverEvent,
    },
    #[serde(rename="binary-sat-search-result/v1")]
    Model {
        original_binding:[u8;32], pattern:u32,
        initial_checkpoint_sequence:u64, initial_checkpoint_sha256:String,
        search_end_sequence:u64, search_end_sha256:String,
        model:Vec<bool>, verification:Progress,
        scalar_recovered:bool, all_patterns_negative:bool,
    },
    #[serde(rename="binary-sat-search-terminal/v1")]
    Terminal {
        original_binding:[u8;32], pattern:u32,
        initial_checkpoint_sequence:u64, initial_checkpoint_sha256:String,
        covered_sequence:u64, covered_sha256:String,
        invoked:bool, native_result:Option<NativeOutcome>,
        checked_signed_relation:bool, first_stop:Option<Stop>,
        limits:[u64;4], totals:[u64;7],
        scalar_recovered:bool, all_patterns_negative:bool,
    },
}
#[derive(Deserialize)]
#[serde(rename_all="snake_case")]
enum NativeOutcome { Sat, UnsatUncertified, Unknown }
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Progress {
    formula_rows:usize, blocks:Vec<u64>, chain_steps:usize,
    lift_choices:Vec<[Option<(u64,u64)>;2]>, path:Vec<(u64,u64)>,
    sum:Option<(u64,u64)>, sign_mask:u32, signed_relation:bool,
}
fn hash(value:&str)->Result<[u8;32],String> {
    if value.len()!=64 || !value.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
        return Err("search digest is not canonical".into())
    }
    let mut bytes=[0;32]; hex::decode_to_slice(value,&mut bytes).map_err(|_|"search digest")?; Ok(bytes)
}
pub(super) struct Replay {
    event_limit:usize,
    segment:Option<segment::Segment>,
    records:Vec<Wire>,
    pending:Option<Wire>,
    terminal:Option<usize>,
    native_pending:Option<native::SolverEvent>,
    claimed_pending:Option<checked_model::Progress>,
}
impl Replay {
    pub(super) fn new(event_limit:usize)->Self {
        Self {event_limit,segment:None,records:Vec::new(),pending:None,terminal:None,
            native_pending:None,claimed_pending:None}
    }
    pub(super) fn feed(&mut self,line:&[u8],binding:&str,original_binding:[u8;32],
        map:&provenance::CheckedMap,signed:&[(u64,u64)],target:Option<(u64,u64)>,
        checkpoint:&(u64,String),owner:&mut ReplayBudget)->Result<(),String> {
        if self.terminal.is_some() || self.pending.is_some() || self.event_limit==0 {
            return Err("search record after terminal/refusal".into())
        }
        // Aggregate traversal admission precedes this call; this additional quote
        // covers strict parse, retained event conversion and claimed Progress copies.
        let n=u64::try_from(line.len()).map_err(|_|"search line overflow")?;
        owner.reserve_checking(n.checked_mul(512).ok_or("search quote overflow")?,
            n.checked_mul(256).ok_or("search quote overflow")?,Instant::now())
            .map_err(|_|"search decode admission")?;
        if self.records.len()>=self.event_limit.checked_add(2).ok_or("search record overflow")? {
            return Err("search record cap".into())
        }
        if self.records.len()==self.records.capacity() {
            let count=self.records.len().checked_add(1).ok_or("search record capacity")?;
            owner.reserve_checking(count as u64,(count as u64).checked_mul(std::mem::size_of::<Wire>() as u64).ok_or("search owner overflow")?,Instant::now())
                .map_err(|_|"search record admission")?;
            self.records.try_reserve_exact(1).map_err(|_|"search record allocation")?;
        }
        self.pending=Some(serde_json::from_slice(line).map_err(|_|"strict search wire decode")?);
        if self.segment.is_none() {
            self.segment=Some(segment::Segment::new(segment::Expected{
                original_binding,pattern:map.pattern(),checkpoint_sequence:checkpoint.0,
                checkpoint_sha256:hash(&checkpoint.1)?,event_limit:self.event_limit,
            },owner).map_err(|_|"search segment admission")?);
        }
        let wire=self.pending.as_ref().unwrap();
        if wire.schema!="crypto.ml-dataset.live-event/v2" || wire.binding_sha256!=binding || wire.event.kind!="binary_sat_search" {
            return Err("search envelope identity".into())
        }
        let link=segment::Link{sequence:wire.sequence,previous_sha256:hash(&wire.previous_event_sha256)?,actual_line_sha256:Sha256::digest(line).into()};
        let segment=self.segment.as_mut().unwrap();
        match &wire.event.payload {
            Payload::Event{original_binding,pattern,initial_checkpoint_sequence,initial_checkpoint_sha256,native_index,event}=>{
                self.native_pending=Some(event.clone().native());
                segment.event(&link,&segment::Event{
                    original_binding:*original_binding,pattern:*pattern,
                    initial_checkpoint_sequence:*initial_checkpoint_sequence,
                    initial_checkpoint_sha256:hash(initial_checkpoint_sha256)?,native_index:*native_index,
                    event:self.native_pending.as_ref().unwrap(),
                },owner).map_err(|_|"search event join")?;
                self.native_pending=None;
            },
            Payload::Model{original_binding,pattern,initial_checkpoint_sequence,initial_checkpoint_sha256,
                search_end_sequence,search_end_sha256,model,verification,scalar_recovered,all_patterns_negative}=>{
                self.claimed_pending=Some(checked_model::Progress{
                    formula_rows:verification.formula_rows,blocks:verification.blocks.clone(),chain_steps:verification.chain_steps,
                    lift_choices:verification.lift_choices.clone(),path:verification.path.clone(),sum:verification.sum,
                    sign_mask:verification.sign_mask,signed_relation:verification.signed_relation,
                });
                segment.model_result(&link,&segment::ResultRecord{
                    original_binding:*original_binding,pattern:*pattern,
                    initial_checkpoint_sequence:*initial_checkpoint_sequence,initial_checkpoint_sha256:hash(initial_checkpoint_sha256)?,
                    search_end_sequence:*search_end_sequence,search_end_sha256:hash(search_end_sha256)?,model,
                    claimed_verification:self.claimed_pending.as_ref().unwrap(),scalar_recovered:*scalar_recovered,
                    all_patterns_negative:*all_patterns_negative,
                },map,signed,target,owner).map_err(|_|"independent signed model replay")?;
                self.claimed_pending=None;
            },
            Payload::Terminal{original_binding:ob,pattern,initial_checkpoint_sequence,initial_checkpoint_sha256,
                covered_sequence,covered_sha256,invoked,native_result,checked_signed_relation,first_stop,
                scalar_recovered,all_patterns_negative,..}=>{
                let endpoint=segment.endpoint();
                if *ob!=original_binding || *pattern!=map.pattern() || *initial_checkpoint_sequence!=checkpoint.0
                    || initial_checkpoint_sha256!=&checkpoint.1 || !invoked || first_stop.is_some()
                    || *covered_sequence!=endpoint.0 || hash(covered_sha256)?!=endpoint.1
                    || link.sequence!=endpoint.0.checked_add(1).ok_or("terminal sequence overflow")?
                    || link.previous_sha256!=endpoint.1 || *scalar_recovered || *all_patterns_negative {
                    return Err("search terminal identity or claims".into())
                }
                let matched=match native_result {
                    Some(NativeOutcome::Sat)=>*checked_signed_relation && segment.model_relation_checked(),
                    Some(NativeOutcome::UnsatUncertified)=>!*checked_signed_relation && segment.uncertified_native_unsat(),
                    _=>false,
                };
                if !matched {return Err("search terminal outcome differs from checked segment".into())}
                self.terminal=Some(self.records.len());
            },
        }
        DecodeBudget::check(owner,Instant::now()).map_err(|_|"search post-decode deadline")?;
        self.records.push(self.pending.take().unwrap());
        Ok(())
    }
    pub(super) fn finish(&self,initial:&ledger::Checkpoint,head:&capture::CommittedHead)->Result<bool,String> {
        let Some(i)=self.terminal else {return Ok(false)};
        let row=self.records.get(i).ok_or("terminal not retained")?;
        let Payload::Terminal{limits,totals,..}=&row.event.payload else {return Err("terminal type".into())};
        if limits!=&initial.limits || !terminal_minimum_paid(&initial.totals, totals) || totals.iter().zip(initial.totals).any(|(now,prior)|*now<prior)
            || totals[0].checked_add(totals[2]).filter(|v|*v<=limits[0]).is_none()
            || totals[1].checked_add(totals[4]).filter(|v|*v<=limits[1]).is_none()
            || totals[3]>totals[2] || totals[5]>limits[2] || totals[6]>limits[3]
            || row.sequence!=head.event_count || self.pending.is_some() {
            return Err("terminal ledger or final capture coverage".into())
        }
        // The authenticated traversal and repeated raw head join bind the exact
        // terminal line digest; this report never certifies native cost accuracy.
        Ok(true)
    }
    pub(super) fn model_relation_checked(&self)->bool {
        self.terminal.is_some() && self.segment.as_ref().is_some_and(|s|s.model_relation_checked())
    }
    pub(super) fn native_unsat_uncertified(&self)->bool {
        self.terminal.is_some() && self.segment.as_ref().is_some_and(|s|s.uncertified_native_unsat())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn begin()->Value {
        serde_json::json!({"schema":"crypto.ml-dataset.live-event/v2","binding_sha256":"0".repeat(64),
            "sequence":2,"previous_event_sha256":"0".repeat(64),"event":{"kind":"binary_sat_search",
            "payload":{"schema":"binary-sat-search-event/v1","original_binding":vec![0u8;32],"pattern":0,
            "initial_checkpoint_sequence":1,"initial_checkpoint_sha256":"0".repeat(64),"native_index":0,
            "event":{"kind":"solve_begin","conflicts":0,"conflict_budget":10}}}})
    }
    #[test]
    fn strict_real_search_wire_accepts_known_begin_and_rejects_unknown_fields() {
        let value=begin();
        assert!(serde_json::from_value::<Wire>(value.clone()).is_ok());
        for location in [0,1,2,3] {
            let mut changed=value.clone();
            let target=match location {0=>&mut changed,1=>&mut changed["event"],
                2=>&mut changed["event"]["payload"],_=>&mut changed["event"]["payload"]["event"]};
            target["invented_completion"]=Value::Bool(true);
            assert!(serde_json::from_value::<Wire>(changed).is_err());
        }
    }
    #[test]
    fn strict_reason_and_claim_progress_reject_hidden_fields() {
        let mut value=begin();
        value["event"]["payload"]["event"]=serde_json::json!({"kind":"assignment","literal":1,
            "level":0,"trail_index":0,"initial":false,"reason":{"kind":"decision","hidden_scalar":1}});
        assert!(serde_json::from_value::<Wire>(value).is_err());
        let claim=serde_json::json!({"formula_rows":0,"blocks":[],"chain_steps":0,"lift_choices":[],
            "path":[],"sum":null,"sign_mask":0,"signed_relation":false,"degree":4});
        assert!(serde_json::from_value::<Progress>(claim).is_err());
    }
    #[test]
    fn empty_event_variants_accept_native_wire_and_reject_extra_fields() {
        let reason=serde_json::json!({"kind":"decision"});
        assert!(matches!(serde_json::from_value::<SolverEventReason>(reason).unwrap().native(), native::SolverEventReason::Decision));
        assert!(serde_json::from_value::<SolverEventReason>(serde_json::json!({"kind":"decision","hidden_scalar":1})).is_err());
        let reset=serde_json::json!({"kind":"xor_matrix_reset"});
        assert!(matches!(serde_json::from_value::<SolverEvent>(reset).unwrap().native(), native::SolverEvent::XorMatrixReset));
        assert!(serde_json::from_value::<SolverEvent>(serde_json::json!({"kind":"xor_matrix_reset","hidden_scalar":1})).is_err());
    }
    #[test]
    fn canonical_digest_rejects_alternate_encodings() {
        assert_eq!(hash(&"0".repeat(64)).unwrap(),[0;32]);
        assert!(hash(&"A".repeat(64)).is_err());
        assert!(hash(&"0".repeat(63)).is_err());
    }
    #[test]
    fn model_additions_use_existing_sticky_cumulative_replay_owner() {
        let deadline=Instant::now()+std::time::Duration::from_secs(30);
        let mut owner=ReplayBudget{policy:Policy{work:64,bytes:64,deadline,stream_bytes:64,payload_bytes:64},
            work:0,bytes:0,first_stop:None};
        assert!(VerificationAdmission::addition(&mut owner).is_ok());
        assert_eq!(owner.work,64);
        assert!(VerificationAdmission::addition(&mut owner).is_err());
        assert_eq!(owner.first_stop,Some(Stop::Work));
        assert_eq!(owner.policy.deadline,deadline);
        assert_eq!(owner.work,64);
        assert!(Admission::check(&mut owner).is_err());
    }
}

// Mandatory same-owner terminal tariff from reviewed producer implementation.
// This lower bound is necessary ledger consistency, not native cost measurement.
fn terminal_minimum_paid(initial:&[u64;7], terminal:&[u64;7])->bool {
    [(2usize,294912u64),(4,2048),(5,20480)].into_iter().all(|(i,delta)|
        initial[i].checked_add(delta).is_some_and(|minimum|terminal[i]>=minimum))
}
#[cfg(test)]
mod terminal_tariff_tests {
    use super::*;
    #[test]
    fn rehashed_terminal_cannot_claim_unpaid_or_overflowed_fixed_tariff() {
        let initial=[1,2,3,4,5,6,7];
        assert!(!terminal_minimum_paid(&initial,&initial));
        let mut valid=initial;
        valid[2]+=294912;valid[4]+=2048;valid[5]+=20480;
        assert!(terminal_minimum_paid(&initial,&valid));
        for i in [2usize,4,5] {let mut changed=valid;changed[i]-=1;
            assert!(!terminal_minimum_paid(&initial,&changed));}
        let overflow=[u64::MAX;7];assert!(!terminal_minimum_paid(&overflow,&overflow));
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProducerReport {
    invoked:bool,native_result:Option<NativeOutcome>,checked_signed_relation:bool,
    first_stop:Option<Stop>,terminal_stop:Option<Stop>,terminal_acknowledged:bool,
    terminal_accepted:bool,totals:[u64;7],
}
impl Replay {
    pub(super) fn check_report(&self,value:&Value)->Result<(),String>{
        if value.get("first_stop").is_none()||value.get("terminal_stop").is_none(){return Err("missing explicit search stop fields".into())}
        let report:ProducerReport=serde_json::from_value(value.clone()).map_err(|_|"strict search report")?;
        if !report.invoked || report.first_stop.is_some() || report.terminal_stop.is_some()
            || !report.terminal_acknowledged || !report.terminal_accepted {
            return Err("producer stopped or terminal unavailable".into())
        }
        let row=self.terminal.and_then(|i|self.records.get(i)).ok_or("producer report lacks terminal")?;
        let Payload::Terminal{totals,..}=&row.event.payload else {return Err("terminal kind".into())};
        // Terminal prepayment includes its own publication, so terminal/report
        // totals must be exact; a post-ack failure cannot hide behind an old seal.
        if &report.totals!=totals {return Err("producer report ledger differs from terminal".into())}
        let valid=match report.native_result {
            Some(NativeOutcome::Sat)=>report.checked_signed_relation && self.model_relation_checked(),
            Some(NativeOutcome::UnsatUncertified)=>!report.checked_signed_relation && self.native_unsat_uncertified(),
            _=>false,
        };
        if !valid {return Err("producer report differs from independently replayed scope".into())}
        Ok(())
    }
    pub(super) fn restricted_projection(&self)->Value {
        let progress=self.segment.as_ref().map(|s|&s.verification);
        serde_json::json!({"schema":"crypto.ml-dataset.sat-model-relation/v1",
            "visibility":"restricted","model_relation_checked":self.model_relation_checked(),
            "native_unsat_uncertified":self.native_unsat_uncertified(),
            "signed_relation":progress.filter(|_|self.model_relation_checked()),
            "causal_search_verified":false,"unsat_proved":false,"scalar_recovered":false,
            "all_patterns_negative":false,"solver_completed":false})
    }
}
