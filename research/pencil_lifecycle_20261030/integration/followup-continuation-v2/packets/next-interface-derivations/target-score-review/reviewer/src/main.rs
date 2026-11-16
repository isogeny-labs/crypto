#[allow(dead_code)] mod three_leads;
use three_leads::{Ring,SignedGraph};
use std::panic::{catch_unwind,AssertUnwindSafe};
fn anchored()->(SignedGraph,usize){let mut g=SignedGraph::new(Ring::new(7));let w=g.node();assert_eq!(g.edge(0,1,w,1,3,2),None);(g,w)}
fn main(){
 let mut free=SignedGraph::new(Ring::new(7));let w=free.node();
 assert_eq!(free.target_rank_score(w,1,w,-1,0),0);
 assert_eq!(free.edge(w,1,w,-1,0,0),None);
 let free_bad=catch_unwind(AssertUnwindSafe(||free.edge(w,1,w,-1,1,0)));assert!(free_bad.is_err());
 let(mut pos,w)=anchored();assert_eq!(pos.target_rank_score(w,1,w,1,4),0);
 assert_eq!(pos.edge(w,1,w,1,6,4),None);
 let pos_bad=catch_unwind(AssertUnwindSafe(||pos.edge(w,1,w,1,0,4)));assert!(pos_bad.is_err());
 let(mut target,w)=anchored();assert_eq!(target.target_rank_score(w,1,w,1,5),3);assert_eq!(target.edge(w,1,w,1,0,5),Some(6));
 let(mut opposite,w)=anchored();assert_eq!(opposite.target_rank_score(w,1,w,-1,1),3);assert_eq!(opposite.edge(w,1,w,-1,1,1),Some(6));
 let(mut neg,w)=anchored();assert_eq!(neg.target_rank_score(w,-1,w,-1,3),0);assert_eq!(neg.edge(w,-1,w,-1,1,3),None);
 let neg_bad=catch_unwind(AssertUnwindSafe(||neg.edge(w,-1,w,-1,2,3)));assert!(neg_bad.is_err());
 println!("{}",serde_json::json!({"schema":"pencil.target-score-independent-controls/v1","all_passed":true,"fixture_groups":5,"expected_assertions_caught":3,"checks":["same-root opposite score0 distinguishes constants only at insertion","anchored positive score0 consistent and contradictory constants","anchored positive residual score3 returns6","anchored opposite score3 returns6","anchored negative score0 consistent and contradictory constants"],"universal_theorem":false,"scientific_execution":false,"timing":null}));
}
