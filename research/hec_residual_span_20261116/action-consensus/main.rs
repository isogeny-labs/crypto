#[allow(dead_code)]mod ring;
use ring::{Dsu,Ring,R,ZERO,ONE};
use hec_native_ic::fp::Fp;
struct Field{f:Fp,basis:[Option<[u64;3]>;2],dead:bool}
impl Field{fn new()->Self{Self{f:Fp::new(11),basis:[None,None],dead:false}}fn insert(&mut self,mut v:[u64;3]){if self.dead{return}for j in 0..2{let a=v[j];if a==0{continue}if let Some(p)=self.basis[j]{for k in j..3{v[k]=self.f.sub(v[k],self.f.mul(a,p[k]));}}else{let iv=self.f.inv(a);for x in &mut v{*x=self.f.mul(*x,iv);}self.basis[j]=Some(v);return}}if v[2]!=0{self.dead=true}}
fn status(&self)->Option<(usize,Option<u64>)>{if self.dead{None}else{Some((self.basis.iter().filter(|x|x.is_some()).count(),self.basis[1].map(|r|r[2])))}}}
fn check(s:&Dsu,fields:&[Field])->usize{let roots=[3,4,5,9];let mut common=None;let mut blocked=false;let mut alive=0;for(&x,f)in roots.iter().zip(fields){assert_eq!(s.status(x),f.status());if let Some((_,k))=f.status(){alive+=1;if let Some(k)=k{if common.is_some_and(|p|p!=k){blocked=true}common=Some(k);}else{blocked=true}}}let expected=if blocked||alive==0{None}else{common};assert_eq!(s.common_k(),expected);4}
fn main(){let r=Ring::new(11);for x in[3,4,5,9]{assert_eq!(r.phi.eval(x,&r.f),0);}let z=r.z(1);let a=r.sub(z,r.constant(3));let cases:Vec<(&str,Vec<[R;3]>,Option<u64>)>=vec![
("free",vec![],None),
("constant_k_four_actions",vec![[ZERO,ONE,r.constant(7)]],Some(7)),
("varying_k",vec![[ZERO,ONE,z]],None),
("empty_survivors",vec![[ZERO,ZERO,ONE]],None),
("narrow_to_one",vec![[ZERO,ONE,z],[ZERO,ZERO,a]],Some(3)),
("consistent_zero_pivot",vec![[ZERO,a,r.scale(a,7)],[ZERO,ONE,r.constant(7)]],Some(7)),
("inconsistent_component_only",vec![[a,ZERO,ONE],[ZERO,ONE,r.constant(7)]],Some(7)),
("inconsistent_equal_lhs",vec![[ONE,z,ONE],[ONE,z,r.constant(2)]],None)];
let(mut prefixes,mut components)=(0,0);println!("case\tprefix\tsurvivors\tcommon_k\tunique_action");for(name,rows,final_k)in cases{let mut s=Dsu::new(0,11);let mut fields:Vec<_>=(0..4).map(|_|Field::new()).collect();components+=check(&s,&fields);prefixes+=1;for(i,row)in rows.iter().enumerate(){s.insert(&row[..2],row[2]);for(&x,f)in[3,4,5,9].iter().zip(&mut fields){f.insert(row.map(|v|r.eval(v,x)));}components+=check(&s,&fields);prefixes+=1;println!("{name}\t{}\t{}\t{}\t{}",i+1,s.survivors(),s.common_k().map_or("none".into(),|k|k.to_string()),s.unique().map_or("none".into(),|x|x.0.to_string()));}assert_eq!(s.common_k(),final_k);}
eprintln!("scenarios=8 prefixes={prefixes} independent_field_component_checks={components} all_consensus_and_branch_status_checks_pass=true no_group_performance_claim=true");}
