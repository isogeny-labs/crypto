from pathlib import Path
import shutil,re,json,hashlib,difflib
P=Path(__file__).parent; B=P.parent; A=B/'owning-search-composed-source-20261107/assembled'; D=B/'retained-exclusion-verifier-proposal-20261107'; C=P/'checks';C.mkdir(exist_ok=True)
for name in ['sparse.rs','scheduler.rs','search.rs','bridge.rs','bqi.rs','check.rs','continuation.rs','exclusions.rs','signed_lift.rs','checked_model.shim.rs']:
 shutil.copyfile(D/name,C/name)
for name in ['capture_wire.rs']:shutil.copyfile(P/name,C/name)
def block(s,start):
 a=s.index(start);i=s.index('{',a);depth=1;j=i+1
 while depth:
  depth+=(s[j]=='{')-(s[j]=='}');j+=1
 return s[a:j]
owner=(A/'src/ml_dataset/original_boolean_search.rs').read_text()
stop=block((A/'src/ml_dataset/koblitz_m3.rs').read_text(),'pub(super) enum Stop').replace('pub(super)','pub')
fail=block(owner,'pub(in crate::ml_dataset::solvers) enum Failure').replace('pub(in crate::ml_dataset::solvers) enum Failure','pub enum OwnerFailure')
lift=block(owner,'pub(in crate::ml_dataset::solvers) struct LiftRecord').replace('pub(in crate::ml_dataset::solvers)','pub')
proj=(A/'src/ml_dataset/capture_projection.rs').read_text()
identity=block(proj,"pub(in crate::ml_dataset::solvers) struct SourceIdentity").replace('pub(in crate::ml_dataset::solvers)','pub')
status=block(proj,'pub(in crate::ml_dataset::solvers) enum Status').replace('pub(in crate::ml_dataset::solvers)','pub')
cap=(A/'src/ml_dataset/capture.rs').read_text();ack=block(cap,'pub(super) struct CommittedEventPosition').replace('pub(super)','pub')
h=(D/'harness.rs').read_text().split('#[cfg(test)]mod tests{')[0]
h+='\nuse sparse as equations;\nmod provenance{pub use super::Error;}\n#[derive(Debug,Clone,Copy)]'+stop+'\n#[derive(Debug,Clone,Copy)]'+fail+'\n'+lift+'\nmod capture{'+ack+'}\nmod capture_projection{use super::capture;'+identity+'\n#[derive(Debug,Clone,Copy)]'+status+'}\n#[path="encoding.test.rs"] mod encoding;\n'
(C/'harness.rs').write_text(h)
e=(P/'capture_encoding.rs').read_text();start=e.index('pub(in crate::ml_dataset::solvers) fn with_parts');end=e.index('macro_rules! object',start)
e=e[:start]+e[end:];e=e.replace('OwnedAttempt,Budget,','').replace('Failure as OwnerFailure,Stop','OwnerFailure,Stop').replace('pub(in crate::ml_dataset::solvers)','pub');e+='\n#[cfg(test)] #[path="encoder_tests.rs"] mod tests;\n';(C/'encoding.test.rs').write_text(e)
# Identical accessors to those of the existing borrowing Expansion.
base=(A/'src/cryptanalysis/checked_boolean_equations.rs').read_text();at=base.index('impl OwnedExpansion {');end=base.index('\n}',at)
access='''\n    /// Partial rows are diagnostics, never a complete admitted generator.\n    pub fn retained(&self) -> (&Polynomial, &Polynomial, Stage, usize, usize) {\n        (&self.attempt.partial, &self.attempt.scratch, self.attempt.stage, self.attempt.row, self.attempt.literal)\n    }'''
proposed=base[:end]+access+base[end:]
(P/'checked_boolean_equations.base.rs').write_text(base);(P/'checked_boolean_equations.proposed.rs').write_text(proposed)
(P/'expansion-accessor.patch').write_text(''.join(difflib.unified_diff(base.splitlines(True),proposed.splitlines(True),fromfile='a/src/cryptanalysis/checked_boolean_equations.rs',tofile='b/src/cryptanalysis/checked_boolean_equations.rs')))
(P/'EXTRACTION_RECEIPT.json').write_text(json.dumps({'scope':'Actual serializer walks and root encoding, with with_parts omitted because no authentic owner fixture; visibility only adjusted. Actual donor retained data types and kernels, historical structural CheckedMap harness. No authentic original owner or CaptureEmitter execution.','methods_removed':['with_parts'],'type_fields_extracted':['Stop','OwnerFailure','LiftRecord','SourceIdentity','Status','CommittedEventPosition'],'donor':str(D),'source':str(A)},indent=2)+'\n')
