use five_leads_study::three_leads::{Divisor,Jacobian,Ring};
use serde_json::{json,Value};use std::collections::HashMap;
/// Caller supplies valid D=aG+bQ in the SAME target context. This table does
/// not certify the label invariant or infer Q in <G> merely from rQ=0.
/// Degree2/distinct-root eligibility is the existing external collector gate.
pub struct Labels<'a>{j:&'a Jacobian,g:Divisor,q:Divisor,ring:Ring,first:HashMap<Divisor,(u64,u64)>}
impl<'a> Labels<'a>{pub fn new(j:&'a Jacobian,g:Divisor,q:Divisor,r:u64)->Self{
 assert!(r<=(1u64<<63));let ring=Ring::new(r); // native primality/r>2 assertion
 assert!(j.valid(g)&&j.valid(q));assert_ne!(g,Divisor::O);assert_eq!(j.mul(g,r),Divisor::O);assert_eq!(j.mul(q,r),Divisor::O);
 Self{j,g,q,ring,first:HashMap::new()}}
pub fn ingest(&mut self,d:Divisor,a:u64,b:u64)->Value{
 let before=self.ring.counts.get();let field_before=self.j.f.counts();let r=self.ring.r;
 if a>=r||b>=r{return json!({"status":"noncanonical_label","emit_graph_edge":false,"answer":null,"scalar_before":before,"scalar_after":self.ring.counts.get(),"field":self.j.f.counts()-field_before})}
 let key=d.min(self.j.neg(d));let flipped=d!=key;let(aa,bb)=if flipped{(self.ring.neg(a),self.ring.neg(b))}else{(a,b)};
 let(mut answer,mut verified)=(None,false);
 let(status,emit,delta)=if let Some(&(old_a,old_b))=self.first.get(&key){
  let da=self.ring.sub(aa,old_a);let db=self.ring.sub(bb,old_b);
  if db==0{(if da==0{"redundant"}else{"invariant_error"},false,Some([da,db]))}
  else{let candidate=self.ring.mul(self.ring.neg(da),self.ring.inv(db));verified=self.j.mul(self.g,candidate)==self.q;if verified{answer=Some(candidate)};(if verified{"candidate_verified"}else{"candidate_rejected"},false,Some([da,db]))}
 }else{self.first.insert(key,(aa,bb));("first",true,None)};
 json!({"status":status,"emit_graph_edge":emit,"answer":answer,"candidate_verified":verified,"canonical_divisor":key,"sign_flipped":flipped,"normalized_labels":[aa,bb],"delta":delta,"stored_pairs":self.first.len(),"scalar_before":before,"scalar_after":self.ring.counts.get(),"field":self.j.f.counts()-field_before})}
}
