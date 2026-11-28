//! Proposed private child of sat_original_owner. Not production-registered.
//! One published CheckedMap, one verified original, one cumulative Budget.
use super::*;
use provenance::boolean_equations as equations;
use std::mem::size_of;

// Scheduler's generic interface is private here. Callers cannot provide an owner.
mod sparse {
    pub use super::equations::{Charge, Monomial, Polynomial};
    pub trait Owner {
        fn reserve(&mut self, charge: Charge) -> Result<(), ()>;
        fn check(&mut self) -> Result<(), ()>;
    }
}
#[path = "dynamic_boolean_scheduler.rs"]
mod scheduler;
pub(in crate::ml_dataset::solvers) use scheduler::Limits as MatrixLimits;
struct MatrixAdmission<'b>(&'b mut Budget);
impl sparse::Owner for MatrixAdmission<'_> {
    fn reserve(&mut self, q: sparse::Charge) -> Result<(), ()> {
        self.0.reserve_checking(q.work,q.bytes,Instant::now()).map_err(|_| ())
    }
    fn check(&mut self) -> Result<(), ()> { self.0.check(Instant::now()).map_err(|_| ()) }
}
#[derive(Clone,Copy)]
pub(in crate::ml_dataset::solvers) struct Caps {
    pub expansion: equations::Limits,
    pub matrix: MatrixLimits,
}
#[derive(Clone,Copy,Debug)]
pub(in crate::ml_dataset::solvers) enum Failure {
    Admission(Stop), Expansion(equations::Failure), Matrix(scheduler::Failure),
}
/// Borrowed phase custody: cannot outlive, replace, or move the verified owner.
/// All buffers survive a phase refusal until this object itself is dropped.
/// Not yet worker transport / durable checkpoint serialization.
pub(in crate::ml_dataset::solvers) struct Attempt<'s,'o> {
    original: &'s VerifiedWorkerOriginal<'o>,
    ledger: &'s mut Budget,
    binding: [u8;32],
    pattern: Option<u32>,
    expansion: Option<equations::Expansion<'s>>,
    normalized: Vec<equations::Polynomial>,
    normalization_generator: usize,
    normalization_term: usize,
    normalization_sort: usize,
    matrix: Option<scheduler::Attempt>,
    original_deadline: Instant,
    failure: Option<Failure>,
}
impl Attempt<'_,'_> {
    pub(in crate::ml_dataset::solvers) fn failure(&self)->Option<Failure>{self.failure}
    pub(in crate::ml_dataset::solvers) fn first_stop(&self)->Option<Stop>{self.ledger.stopped}
    pub(in crate::ml_dataset::solvers) fn charged(&self)->(u64,u64){(self.ledger.checking_work,self.ledger.adapter_bytes)}
    fn fail(&mut self,f:Failure,s:Stop){self.failure=Some(f);let _:Result<(),Stop>=self.ledger.fail(s);}
    fn pay(&mut self,w:u64,b:usize)->Result<(),Stop>{
        let bytes=u64::try_from(b).map_err(|_|Stop::Overflow)?;
        self.ledger.reserve_checking(w,bytes,Instant::now())
    }
    fn normalize(&mut self)->Result<(),Stop>{
        let count=self.expansion.as_ref().ok_or(Stop::Phase)?.equations().len();
        self.pay(1,count.checked_mul(size_of::<equations::Polynomial>()).and_then(|n|n.checked_add(64)).ok_or(Stop::Overflow)?)?;
        self.normalized.try_reserve_exact(count).map_err(|_|Stop::Allocation)?;
        while self.normalization_generator<count {
            let g=self.normalization_generator;
            let terms=self.expansion.as_ref().ok_or(Stop::Phase)?.equations()[g].len();
            self.pay(1,terms.checked_mul(size_of::<equations::Monomial>()).and_then(|n|n.checked_add(64)).ok_or(Stop::Overflow)?)?;
            // Store every growing allocation before the next fallible operation.
            self.normalized.push(Vec::new());
            self.normalized[g].try_reserve_exact(terms).map_err(|_|Stop::Allocation)?;
            self.normalization_term=0;
            while self.normalization_term<terms {
                let t=self.normalization_term;
                let len=self.expansion.as_ref().ok_or(Stop::Phase)?.equations()[g][t].len();
                self.pay(u64::try_from(len).map_err(|_|Stop::Overflow)?.checked_add(1).ok_or(Stop::Overflow)?,len.checked_mul(size_of::<u32>()).and_then(|n|n.checked_add(64)).ok_or(Stop::Overflow)?)?;
                self.normalized[g].push(Vec::new());
                self.normalized[g][t].try_reserve_exact(len).map_err(|_|Stop::Allocation)?;
                self.normalized[g][t].extend_from_slice(&self.expansion.as_ref().ok_or(Stop::Phase)?.equations()[g][t]);
                self.normalization_term+=1;
            }
            // Expansion uses lex vectors; scheduler requires degree then lex.
            // Insertion swaps allocate nothing; all comparisons/swaps are prepaid.
            self.normalization_sort=1;
            while self.normalization_sort<terms {
                let mut j=self.normalization_sort;
                while j>0 {
                    let lens=self.normalized[g][j].len().checked_add(self.normalized[g][j-1].len()).ok_or(Stop::Overflow)?;
                    self.pay(u64::try_from(lens).map_err(|_|Stop::Overflow)?.checked_add(2).ok_or(Stop::Overflow)?,0)?;
                    let a=&self.normalized[g][j-1];let b=&self.normalized[g][j];
                    if a.len().cmp(&b.len()).then_with(||a.cmp(b))!=std::cmp::Ordering::Greater {break}
                    self.normalized[g].swap(j-1,j);j-=1;
                }
                self.normalization_sort+=1;
            }
            self.normalization_generator+=1;
        }
        self.ledger.check(Instant::now())
    }
}
impl<'o> OriginalOwner<'o> {
    /// No public map, generic owner, caller deadline or fresh Budget is accepted.
    /// Call only after genuine build_map + persist_map. Reuses the one-shot row
    /// gate so this cannot follow or precede another row attempt on this owner.
    pub(super) fn matrix_phase<'s>(&'s mut self,mut caps:Caps)->Attempt<'s,'o>{
        let repeated=self.boolean_attempted;self.boolean_attempted=true;
        let admitted=self.map_published && self.map_ack.is_some() && self.checked.is_some();
        let source=self.checked.as_ref();
        let original_deadline=self.budget.deadline;
        let mut out=Attempt{original:self.original,ledger:&mut self.budget,binding:self.binding,pattern:self.pattern,
            expansion:None,normalized:Vec::new(),normalization_generator:0,normalization_term:0,normalization_sort:0,
            matrix:None,original_deadline,failure:None};
        if repeated || !admitted {out.fail(Failure::Admission(Stop::Phase),Stop::Phase);return out}
        let prep=(||->Result<(),Stop>{
            out.pay(1,size_of::<Attempt<'s,'o>>())?;
            let phase=out.ledger.begin_native(false,Instant::now())?;
            caps.expansion.work=caps.expansion.work.min(phase.work);
            caps.expansion.bytes=caps.expansion.bytes.min(phase.bytes);
            Ok(())
        })();
        if let Err(s)=prep{out.fail(Failure::Admission(s),s);return out}
        let expanded=equations::expand(source.expect("checked above"),caps.expansion,&mut Admit(&mut *out.ledger));
        let status=expanded.status();out.expansion=Some(expanded);
        if let Err(f)=status{out.fail(Failure::Expansion(f),Stop::NativeFailure);return out}
        // Representation conversion is still construction and cannot escape its deadline.
        if let Err(s)=out.normalize(){out.fail(Failure::Admission(s),s);return out}
        if let Err(s)=out.ledger.finish_native(Instant::now()){out.fail(Failure::Admission(s),s);return out}
        let phase=match out.ledger.begin_native(true,Instant::now()){
            Ok(l)=>l,Err(s)=>{out.fail(Failure::Admission(s),s);return out}
        };
        caps.matrix.work=caps.matrix.work.min(phase.work);caps.matrix.bytes=caps.matrix.bytes.min(phase.bytes);
        let variables=out.expansion.as_ref().expect("retained expansion").variables();
        let matrix=scheduler::run(variables,&out.normalized,caps.matrix,out.original_deadline,&mut MatrixAdmission(&mut *out.ledger));
        let status=matrix.result;out.matrix=Some(matrix);
        if let Err(f)=status{out.fail(Failure::Matrix(f),Stop::NativeFailure);return out}
        if let Err(s)=out.ledger.finish_native(Instant::now()){out.fail(Failure::Admission(s),s)}
        out
    }
}
