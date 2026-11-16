fn doubling_width_eligible(r:u64,m:usize)->bool{r>=7&&r<=u64::MAX/2&&m>=3&&m%2==1&&(r-1)as u128%(8*m as u128)==0}
impl RecoveryProducts<'_>{
 fn planned_doubling(r:u64,m:usize,cap:usize)->Result<Self,Error>{
  if !doubling_width_eligible(r,m){return Err(Error::Domain)}
  let length=(2*m)as u64;
  Ok(Self{r,m,cap,inner:EngineStorage::Owned(std::cell::RefCell::new(IntervalProducts::new(r,length,cap)?))})
 }
}
impl RecoveryModes<'_>{
 fn planned_width(m:&StreamModes)->Result<Self,Error>{Ok(Self{r:m.r,m:m.m,zeta:m.zeta,block:m.block,products:RecoveryProducts::planned_doubling(m.r,m.m,m.block)?})}
}
impl StreamModes{
 fn recover_planned_width(&self,source:&source::Source,curve:group::Curve,base:group::Point,target:Option<group::Point>,radii:&[u64],cap:usize,extra:&[u64],max_calls:usize)->Result<(Option<u64>,PullStats,&'static str),Error>{
  if doubling_width_eligible(self.r,self.m)&&radii.len()>=2&&radii[0].checked_add(radii[1])==Some(self.r){RecoveryModes::planned_width(self)?.recover_shared_rescue(source,curve,base,target,radii,cap,extra,max_calls)}
  else{self.recover_shared_rescue(source,curve,base,target,radii,cap,extra,max_calls)}
 }
}
include!("width-plan-checks.rs");
