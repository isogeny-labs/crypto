static void verify_orbits(Runtime& r) {
 for(const auto& fixture:r.f.marked.at("orbit_checks").array){
  long k=fixture.at("k").integer();Big input_scalar(k);
  auto P=point(r.group),current=point(r.group),representative=point(r.group);
  require(EC_POINT_mul(r.group,P.get(),input_scalar.p,nullptr,nullptr,r.field.ctx),"orbit input");
  require(EC_POINT_copy(current.get(),P.get()),"orbit initial point");
  bool have=false;Affine best;Big transporter(1),best_transporter;std::set<std::string> seen;
  int actions=k?7:1;
  for(int j=0;j<actions;++j){
   for(int sign:{1,-1}){
    auto candidate=point(r.group);require(EC_POINT_copy(candidate.get(),current.get()),"orbit copy");
    Big label=transporter;if(sign<0){require(EC_POINT_invert(r.group,candidate.get(),r.field.ctx),"orbit sign");require(BN_mod_sub(label.p,Big().p,transporter.p,r.n.p,r.field.ctx),"signed transporter");}
    auto direct=point(r.group);require(EC_POINT_mul(r.group,direct.get(),nullptr,P.get(),label.p,r.field.ctx),"orbit action reference");
    require(EC_POINT_cmp(r.group,direct.get(),candidate.get(),r.field.ctx)==0,"orbit action and transporter");
    Affine a=r.affine(candidate.get());seen.insert(a.infinity?"O":a.x.str()+":"+a.y.str());
    if(!have||(!a.infinity&&(BN_cmp(a.x.p,best.x.p)<0||(a.x==best.x&&BN_cmp(a.y.p,best.y.p)<0)))){
     best=a;best_transporter=label;require(EC_POINT_copy(representative.get(),candidate.get()),"representative copy");have=true;
    }
   }
   if(j+1<actions){auto transported=r.transported(current.get()),next=point(r.group);Counts schedule;public_mul(r,next.get(),transported.get(),r.correction,5,schedule);current=std::move(next);require(BN_mod_mul(transporter.p,transporter.p,r.mu.p,r.n.p,r.field.ctx),"next transporter");}
  }
  require(seen.size()==size_t(k?14:1),"signed orbit size");
  if(k){const auto& saved=fixture.at("representative");require(best.x==Big(saved.at(size_t(0)).str())&&best.y==Big(saved.at(size_t(1)).str()),"saved affine orbit representative");}
  else require(best.infinity,"identity representative");
  require(best_transporter==Big(fixture.at("transporter").str()),"saved representative transporter");
 }
}
