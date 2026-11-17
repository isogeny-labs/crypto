#[allow(dead_code)]
mod three_leads;
use three_leads::{Ring,SignedGraph};
use serde_json::json;
fn main(){
 let ring=Ring::new(7); let mut g=SignedGraph::new(ring.clone());let u=g.node();
 let before=ring.counts.get();let op=g.operations;
 let score=g.target_rank_score(0,1,u,1,0);
 let after=ring.counts.get();let finds=g.operations-op;
 assert_eq!(score,0); assert_eq!(g.edge(0,1,u,1,0,0),None);
 assert_eq!(g.target_rank_score(0,1,u,1,1),3);
 assert_eq!(g.edge(0,1,u,1,1,1),Some(6));
 let mut control=SignedGraph::new(Ring::new(7));let v=control.node();
 assert_eq!(control.target_rank_score(0,1,v,1,1),2);
 assert_eq!(control.edge(0,1,v,1,1,1),None);
 let mut merge=SignedGraph::new(Ring::new(7));let a=merge.node();let b=merge.node();
 assert_eq!(merge.target_rank_score(a,1,b,1,1),1);
 assert_eq!(merge.edge(a,1,b,1,1,1),None);
 // Same-root opposite signs cancel nuisance and yield k=6.
 let mut same=SignedGraph::new(Ring::new(7));let w=same.node();
 assert_eq!(same.target_rank_score(w,1,w,-1,1),3);
 assert_eq!(same.edge(w,1,w,-1,1,1),Some(6));
 // Negative anchor: -2*w = 3+2*k; at k=6 this sets w=3.
 let mut neg=SignedGraph::new(Ring::new(7));let w=neg.node();
 assert_eq!(neg.target_rank_score(w,-1,w,-1,2),2);
 assert_eq!(neg.edge(w,-1,w,-1,3,2),None);
 assert_eq!(neg.target_rank_score(0,1,w,1,0),3);
 assert_eq!(neg.edge(0,1,w,1,3,0),Some(6));
 println!("{}",serde_json::to_string_pretty(&json!({"schema":"pencil.target-score-fixed-controls/v1","source_commit":"5cd0a45205e0896a5987364ccde900ec0813f477","scope":"fixed native graph controls over F7","checks":["zero-score anchor enables target equation","without anchor score2 has no return","score1 merge has no return","same-root opposite signs returns6","negative anchor then fixed comparison returns6"],"all_passed":true,"first_score_call_counts":{"scalar_add":after.add-before.add,"scalar_mul":after.mul-before.mul,"scalar_inv":after.inv-before.inv,"find_visits":finds},"scientific_execution":false,"timing":null,"cost_ratio":null})).unwrap());
}
