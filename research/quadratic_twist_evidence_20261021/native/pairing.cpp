// Preregistered twist-assisted pairing correctness/degeneracy sweep. No timings.
#define TWIST_FULL_NO_MAIN
#include "full.cpp"
#include <optional>

Json pj(Point p) { return nums({p.x,p.y}); }
Point monic_image(Point p,const Field& f,I d) {
  return p.infinity()?p:Point{f.div(p.x,d),f.div(p.y,f.mul(d,f.p))};
}
std::optional<I> checked_line(const Curve& c,Point a,Point b,Point x) {
  if(a.infinity()||b.infinity()||x.infinity()) return {};
  const auto& f=c.f;
  if(a.x==b.x && f.add(a.y,b.y)==0) {
    I z=f.sub(x.x,a.x); return z?std::optional<I>(z):std::nullopt;
  }
  I den=a==b?f.mul(2%f.p,a.y):f.sub(b.x,a.x);
  if(!den) return {};
  I m=a==b?f.div(f.add(f.mul(3%f.p,f.pow(a.x,2)),c.a),den)
            :f.div(f.sub(b.y,a.y),den);
  I num=f.sub(f.sub(x.y,a.y),f.mul(m,f.sub(x.x,a.x)));
  Point ab=c.add(a,b); I v=ab.infinity()?1:f.sub(x.x,ab.x);
  if(!num||!v) return {};
  return f.div(num,v);
}
std::optional<I> checked_miller(const Curve& c,Point p,Point x,I r) {
  if(p.infinity()||x.infinity()) return {};
  I z=1,bit=1; Point v=p;
  while(bit<=r/2) bit*=2;
  for(bit/=2;bit;bit/=2) {
    auto line=checked_line(c,v,v,x); if(!line) return {};
    z=c.f.mul(c.f.mul(z,z),*line); v=c.add(v,v);
    if(r&bit) {line=checked_line(c,v,p,x);if(!line)return{};
      z=c.f.mul(z,*line);v=c.add(v,p);}
  }
  require(v.infinity(),"wrong Miller endpoint"); return z;
}
std::optional<I> direct(const Curve& c,Point p,Point q,I r) {
  if(p.infinity()||q.infinity()) return 1;
  auto a=checked_miller(c,p,q,r),b=checked_miller(c,q,p,r);
  if(!a||!b) return {};
  I z=c.f.div(*a,*b);return r%2?c.f.neg(z):z;
}
std::optional<I> shifted_at(const Curve& c,Point p,Point q,I r,Point offset) {
  if(p.infinity()||q.infinity())return 1;
  auto a=checked_miller(c,p,c.add(q,offset),r);
  auto b=checked_miller(c,p,offset,r);
  auto d=checked_miller(c,q,c.neg(offset),r);
  auto e=checked_miller(c,q,c.add(p,c.neg(offset)),r);
  if(!a||!b||!d||!e)return{};
  return c.f.mul(c.f.div(*a,*b),c.f.div(*d,*e));
}
std::pair<I,Point> shifted(const Curve& c,Point p,Point q,I r,
                          const std::vector<Point>& offsets) {
  for(auto o:offsets) if(auto z=shifted_at(c,p,q,r,o))return{*z,o};
  throw std::runtime_error("no legal disjoint-divisor offset in complete E(K)");
}
std::map<std::tuple<I,I,I>,std::string> model_names(const Json& registry) {
  std::map<std::tuple<I,I,I>,std::string> out;
  for(const auto& m:registry.at("curves").items) {
    if(m.at("family").string()!="prime")continue;
    const auto& par=m.at("params");
    if(!par.fields.count("p")||!par.fields.count("a")||!par.fields.count("b"))continue;
    auto get=[&](const char* key)->std::optional<I>{const auto& v=par.at(key);
      if(v.kind==Json::Number)return v.integer();
      if(v.kind==Json::String&&v.string().size()<4)return std::stoll(v.string());
      return{};};
    auto p=get("p"),a=get("a"),b=get("b");
    if(p&&a&&b)out[{*p,*a,*b}]=m.at("slug").string();
  }return out;
}
Point cofactor_generator(const Curve& c,const std::vector<Point>& ps,I r) {
  for(auto p:ps){auto q=c.scalar(I(ps.size())/r,p);if(!q.infinity())return q;}
  throw std::runtime_error("cofactor projection has no nonzero image");
}
int main(int argc,char**argv){try{
  require(argc==3,"usage: pairing REGISTRY OUTPUT");
  auto raw=read(argv[1]);auto ids=model_names(parse(raw));auto models=arr();
  I shared=0,nonshared=0,aux=0,zero=0,nonzero=0;
  for(I p:{5,7,11,13,17,19}){I d=nonsquare(p);Field k(p),f(p,d);
    for(I a=0;a<p;++a)for(I b=0;b<p;++b){if(!mod(4*a*a*a+27*b*b,p))continue;
      I ta=mod(d*d*a,p),tb=mod(d*d*d*b,p);
      Curve e(k,a,b),t(k,ta,tb),c(f,a,b);auto ep=e.points(),tp=t.points();
      auto cases=arr();std::vector<Point> offsets;
      for(auto [r,exponent]:factors(I(ep.size()))){(void)exponent;if(r==2)continue;
        auto item=obj({{"r",num(r)},{"shared",truth(tp.size()%r==0)}});
        if(tp.size()%r){++nonshared;I torsion=0;
          for(auto x:tp)torsion+=t.scalar(r,x).infinity();
          require(torsion==1,"nonshared twist has nontrivial r torsion");
          item.fields["twist_r_torsion_points"]=num(torsion);
        }else{++shared;require((p+1)%r==0&&p%r!=1,"embedding degree is not two");
          auto P=cofactor_generator(e,ep,r);require(e.scalar(r,P).infinity(),"source order");
          if(offsets.empty())offsets=c.points();
          auto values=arr();Point wrong;bool has_wrong=false;
          for(auto T:tp){++aux;auto U=t.scalar(I(tp.size())/r,T);auto S=monic_image(U,f,d);
            require(c.contains(S)&&c.scalar(r,S).infinity()&&c.frob(S)==c.neg(S),"twist image");
            auto z=direct(c,P,S,r);require(z.has_value(),"minus image direct formula undefined");
            Point offset;
            if(S.infinity()){++zero;require(*z==1,"identity pairing");}
            else{++nonzero;require(*z!=1&&f.pow(*z,r)==1&&f.pow(*z,p+1)==1,"nondegeneracy/norm");
              auto checked=shifted(c,P,S,r,offsets);require(checked.first==*z,"direct/shift mismatch");offset=checked.second;}
            values.items.push_back(obj({{"T",pj(T)},{"cofactor_T",pj(U)},
              {"S",pj(S)},{"value",num(*z)},{"offset",pj(offset)}}));
            if(!has_wrong&&!t.scalar(r,T).infinity()){wrong=T;has_wrong=true;}
          }
          auto control=shifted(c,P,P,r,offsets);require(control.first==1,"same-subgroup not degenerate");
          item.fields["P"]=pj(P);item.fields["auxiliaries"]=values;
          item.fields["same_source_control"]=obj({{"value",num(control.first)},
            {"offset",pj(control.second)},{"direct_defined",truth(direct(c,P,P,r).has_value())}});
          item.fields["wrong_subgroup_control"]=has_wrong?pj(wrong):Json{};
        }cases.items.push_back(item);}
      models.items.push_back(obj({{"p",num(p)},{"a",num(a)},{"b",num(b)},{"d",num(d)},
        {"icv1",str(ids.at({p,a,b}))},{"twist_icv1",str(ids.at({p,ta,tb}))},
        {"base_order",num(I(ep.size()))},{"twist_order",num(I(tp.size()))},{"primes",cases}}));
    }}
  require(models.items.size()==942&&shared==274,"population changed");
  Field f(43,2);Curve c(f,1,0);Point P{31,18},Q{13,24},S{39,3*43};
  I g=*direct(c,P,S,11);require(g==2+25*43,"fixture g");auto logs=arr();
  for(I n=0;n<11;++n){auto target=c.scalar(n,P);I value=*direct(c,target,S,11);
    auto ic=index_calculus(f,g,value,11,430000+n);
    require(ic.at("recovered_log").integer()==n,"fixture IC recovery");
    logs.items.push_back(obj({{"scalar",num(n)},{"Q",pj(target)},
      {"target",num(value)},{"index_calculus",ic}}));}
  auto fixture=obj({{"p",num(43)},{"a",num(1)},{"b",num(0)},{"d",num(2)},{"r",num(11)},
    {"icv1",str(ids.at({43,1,0}))},{"twist_icv1",str(ids.at({43,4,0}))},
    {"P",pj(P)},{"Q",pj(Q)},{"monic_twist_point",pj({35,12})},{"S",pj(S)},
    {"g",num(g)},{"known_scalar",num(7)},{"logs",logs}});
  auto result=obj({{"schema",str("quadratic-twist-pairing-transfer/v1")},{"status",str("passed")},
    {"registry_sha256",str(digest(raw))},{"models",models},{"fixture",fixture},
    {"summary",obj({{"models",num(942)},{"shared_primes",num(shared)},{"nonshared_primes",num(nonshared)},
      {"auxiliary_points",num(aux)},{"cofactor_zero_images",num(zero)},{"nontrivial_images",num(nonzero)}})}});
  std::ofstream(argv[2])<<dump(result)<<'\n';std::cout<<"PASS native pairing sweep: "<<shared<<" shared cases, "<<aux<<" auxiliary points\n";
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
