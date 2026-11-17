struct ProjectorRecovery{scalar:Option<u64>,calls:usize,route:&'static str,degree:Option<usize>,stats:ProjectorStats}
fn recover_projector(source:&source::Source,curve:group::Curve,base:group::Point,target:Option<group::Point>,r:u64,m:usize,zeta:u64,radii:&[u64],max_calls:usize,node_cap:usize,caps:Caps)->Result<ProjectorRecovery,Error>{
 let finish=|k,calls,route,degree,stats|{if multiple(curve,base,k)!=target{return Err(Error::Arithmetic)}Ok(ProjectorRecovery{scalar:Some(k),calls,route,degree,stats})};
 match collect_public(source,curve,base,target,r,m,zeta,radii,max_calls)?{
  Collection::Known{scalar,calls}=>finish(scalar,calls,"endpoint",None,ProjectorStats::default()),
  Collection::CallCap{calls}=>Ok(ProjectorRecovery{scalar:None,calls,route:"call-cap",degree:None,stats:ProjectorStats::default()}),
  Collection::Transcript{sign,observations,calls}=>{let problem=match ProjectorProblem::build(r,m,sign,&observations,node_cap){Ok(x)=>x,Err(Error::Limit)=>return Ok(ProjectorRecovery{scalar:None,calls,route:"node-cap",degree:None,stats:ProjectorStats::default()}),Err(e)=>return Err(e)};
   let(q,_,stats)=match problem.extract(caps){Ok(x)=>x,Err((Error::Limit,stats))=>return Ok(ProjectorRecovery{scalar:None,calls,route:"projector-cap",degree:None,stats}),Err((e,_))=>return Err(e)};
   let degree=q.len()-1;if degree==0{return Err(Error::Arithmetic)}if degree==1{let y=(r-q[0])%r;if let Some(k)=problem.sys.phase_at(y)?{return finish(k,calls,"projector-singleton",Some(degree),stats)}}
   Ok(ProjectorRecovery{scalar:None,calls,route:"image-unresolved",degree:Some(degree),stats})
  }
 }
}
