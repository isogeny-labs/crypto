from pathlib import Path
import shutil,re,difflib,json,hashlib
B=Path('/home/ec2-user/ml-dataset-resume-20261029'); P=Path(__file__).parent
old=B/'original-owned-search-policy-entry-proposal-20261107'
p=(old/'original_boolean_search_policy.rs').read_text()
a=p.index('fn eligible(');z=p.index('/// Called by',a)
p=p[:a]+'''fn eligible(config: &CaseConfig, policy: ProposedPolicy) -> bool {
    eligible_inputs(config.channels.is_some(), &config.solver,
        super::super::profile(&config.family, config.n, config.m),
        config.factor_base_size > 0, config.work_limit, config.degree_bound, policy)
}
// Pure request gate used by the authentic CaseConfig projection above.
fn eligible_inputs(has_channels: bool, solver: &str, profile: bool,
    factor_base_nonempty: bool, work: u64, degree: u32, policy: ProposedPolicy) -> bool {
    match policy {
        ProposedPolicy::ConservativeAffineEnvelopeV1 => !has_channels
            && solver == "f4" && profile && factor_base_nonempty && work > 0 && degree > 0,
    }
}

'''+p[z:]
a=p.index('        let mut c: CaseConfig');p=p[:a]+'''        let policy = ProposedPolicy::ConservativeAffineEnvelopeV1;
        assert!(eligible_inputs(false,"f4",true,true,16384,3,policy));
        for solver in ["f5","sat",""] {
            assert!(!eligible_inputs(false,solver,true,true,16384,3,policy));
        }
        // Every existing channel schema has Some(channels), including v1-v4.
        assert!(!eligible_inputs(true,"f4",true,true,16384,3,policy));
        assert!(!eligible_inputs(false,"f4",false,true,16384,3,policy));
        assert!(!eligible_inputs(false,"f4",true,false,16384,3,policy));
        assert!(!eligible_inputs(false,"f4",true,true,0,3,policy));
        assert!(!eligible_inputs(false,"f4",true,true,16384,0,policy));
    }
}
'''
(P/'original_boolean_search_policy.rs').write_text(p)
base=(B/'original-owned-search-continuation-proposal-20261107/original_boolean_search.rs').read_text()
proposed=base.replace('use std::mem::size_of;','use std::mem::size_of;\n#[path="original_boolean_search_policy.rs"] mod policy;\npub(in crate::ml_dataset::solvers) use policy::ProposedPolicy;')
proposed=proposed.replace('sink:capture::CaptureEmitter,caps:Caps)->OwnedAttempt{','sink:capture::CaptureEmitter,choice:ProposedPolicy)->OwnedAttempt{')
proposed=proposed.replace('let admission=(||->Result<(),Stop>{','let admission=(||->Result<Caps,Stop>{')
proposed=proposed.replace('            self.budget.check(Instant::now())\n        })();','            let caps=policy::for_owner(&self,choice)?;\n            self.budget.check(Instant::now())?;\n            Ok(caps)\n        })();')
proposed=proposed.replace('        if let Err(s)=admission{out.fail(Failure::Admission(s),s);return out}\n        out.run(caps);out','        let caps=match admission{Ok(c)=>c,Err(s)=>{out.fail(Failure::Admission(s),s);return out}};\n        out.run(caps);out')
(P/'original_boolean_search.base.rs').write_text(base);(P/'original_boolean_search.proposed.rs').write_text(proposed)
(P/'policy-entry.patch').write_text(''.join(difflib.unified_diff(base.splitlines(True),proposed.splitlines(True),fromfile='a/src/ml_dataset/original_boolean_search.rs',tofile='b/src/ml_dataset/original_boolean_search.rs')))
J=B/'continuation-original-budget-joint-20261107'
for n in ['budget_core.rs','sparse.rs','scheduler.rs','search.rs','bridge.rs','bqi.rs','check.rs','continuation.rs']:shutil.copyfile(J/n,P/n)
source=B/'sat-cli-native-recovery-20261107/source/src/cryptanalysis/sparse_equations.rs'; s=source.read_text();a=s.index('#[derive(Clone, Copy, Debug)]\npub struct Limits');z=s.index('#[derive(Clone, Copy, Debug, Default)]',a);(P/'equation_limits.rs').write_text(s[a:z])
# Mechanical policy extraction: exclude only production import and owner wrapper;
# normalize module visibility and path qualifiers for a single-module harness.
a=p.index('/// Explicit source-level');q=p[a:];a=q.index('fn eligible(');z=q.index('// Pure request gate',a);q=q[:a]+q[z:];a=q.index('/// Called by');z=q.index('fn from_budget',a);q=q[:a]+q[z:];q=q.replace('pub(in crate::ml_dataset::solvers)','pub(super)').replace('super::super::super::super::Phase','Phase').replace('super::super::super::Phase','Phase');(P/'policy_core.rs').write_text(q)
(P/'harness.rs').write_text('''#![allow(dead_code)]
mod sparse; mod scheduler; mod bridge; mod bqi; mod check; mod continuation;
mod equations {include!("equation_limits.rs");}
mod original {
 include!("budget_core.rs");
 use crate::{equations,continuation,scheduler,check,bridge,bqi};
 use scheduler::search::SearchLimits;
 #[derive(Clone,Copy)] struct Caps {expansion:equations::Limits,continuation:continuation::Limits}
 include!("policy_core.rs");
}
''')
inputs=[old/'original_boolean_search_policy.rs',B/'original-owned-search-continuation-proposal-20261107/original_boolean_search.rs',source]+[J/n for n in ['budget_core.rs','sparse.rs','scheduler.rs','search.rs','bridge.rs','bqi.rs','check.rs','continuation.rs']]
(P/'SOURCE_BINDINGS.json').write_text(json.dumps([dict(path=str(x),sha256=hashlib.sha256(x.read_bytes()).hexdigest(),bytes=x.stat().st_size) for x in inputs],indent=2)+'\n')
