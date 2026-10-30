#include "phase-adaptive-prior.hpp"
// New flat Macaulay / bounded Buchberger instrument. The old oracle-based
// adaptive::solve and adaptive::roots functions are never called here.
namespace qualified {
using namespace adaptive;
bool divides(Mono a,Mono b){return a[0]<=b[0]&&a[1]<=b[1];}
Row moved(const Row&r,Mono m){Row z;z.f=shift(r.f,m);for(I j=0;j<3;++j)z.w[j]=shift(r.w[j],m);return z;}
bool normal(Row r,const std::vector<Row>&g,I p,const std::array<Polynomial,3>&f,I cap,I&peak,I&denied){
 if(expanded(r,f)>cap){denied=std::max(denied,expanded(r,f));return false;}
 peak=std::max(peak,expanded(r,f));
 for(I step=0;!r.f.empty();++step){require(step<1000,"division limit");auto m=lead(r.f);bool changed=false;
  for(auto &h:g)if(divides(lead(h.f),m)){auto l=lead(h.f);Mono s{m[0]-l[0],m[1]-l[1]};I bound=expanded(h,f)+s[0]+s[1];if(bound>cap){denied=std::max(denied,bound);return false;}auto z=moved(h,s);peak=std::max(peak,expanded(z,f));subtract(r,z,r.f.at(m),p);changed=true;break;}
  if(!changed)return false;
 }return true;
}
Json solve_cap(const std::array<Polynomial,3>&f,const std::vector<I>&xs,I cap){
 const I p=37;std::vector<Row>pool;for(I j=0;j<3;++j){Row r;r.f=f[j];r.w[j]=constant(1,p);pool.push_back(r);}auto trace=arr();I peak=0;std::map<std::string,Row> retained;
 for(I d=0;d<=cap;++d){auto b=basis(pool,d,p,f,peak);for(auto&r:b){auto key=dump(poly(r.f));auto it=retained.find(key);if(it!=retained.end()&&expanded(it->second,f)<=expanded(r,f))r=it->second;else retained[key]=r;}std::vector<Row>g;for(auto&r:b){bool redundant=false;for(auto&t:b)if(lead(t.f)!=lead(r.f)&&divides(lead(t.f),lead(r.f)))redundant=true;if(!redundant)g.push_back(r);}
  bool unit=false;for(auto&r:g)unit|=r.f==constant(1,p);bool ok=unit;I denied=0;
  if(!unit&&!g.empty()){ok=true;for(auto&r:pool)if(!normal(r,g,p,f,d,peak,denied)){ok=false;break;}
   if(ok)for(size_t i=0;i<g.size()&&ok;++i)for(size_t j=i+1;j<g.size()&&ok;++j){auto a=lead(g[i].f),b0=lead(g[j].f);Mono l{std::max(a[0],b0[0]),std::max(a[1],b0[1])},u{l[0]-a[0],l[1]-a[1]},v{l[0]-b0[0],l[1]-b0[1]};I bound=std::max(expanded(g[i],f)+u[0]+u[1],expanded(g[j],f)+v[0]+v[1]);if(bound>d){denied=std::max(denied,bound);ok=false;break;}auto z=moved(g[i],u);auto w=moved(g[j],v);peak=std::max(peak,bound);subtract(z,w,1,p);ok=normal(z,g,p,f,d,peak,denied);}
  }
  require(peak<=d,"hidden witness degree");trace.items.push_back(obj({{"d",num(d)},{"rank",num(I(b.size()))},{"peak",num(peak)},{"denied_bound",num(denied)},{"certified",truth(ok)}}));
  if(ok){auto cert=arr(),rr=arr();for(auto&r:g)cert.items.push_back(row_json(r,f));if(!unit)for(I x:xs)for(I y:xs)if(evaluate(f[0],x,y,p)==0&&evaluate(f[1],x,y,p)==0&&evaluate(f[2],x,y,p)==0)rr.items.push_back(nums({x,y}));std::sort(rr.items.begin(),rr.items.end(),[](const Json&a,const Json&b){return vec(a)<vec(b);});return obj({{"status",str(unit?"unit":"gb")},{"matrix_degree",num(d)},{"expanded_peak",num(peak)},{"certificate",cert},{"roots",rr},{"trace",trace}});}
 }return obj({{"status",str("cap")},{"matrix_degree",Json{}},{"expanded_peak",num(peak)},{"certificate",arr()},{"roots",arr()},{"trace",trace}});
}
Json rows_for(const Curve&c,Point q,const std::vector<Point>&base,const Json*roots){auto rows=arr(),lifts=arr();std::set<std::vector<I>>rs;
 for(I i=0;i<I(base.size());++i)for(I j=0;j<I(base.size());++j){bool allowed=roots==nullptr;if(roots)for(auto&r:roots->items)allowed|=vec(r)==std::vector<I>{base[i].x,base[j].x};if(!allowed)continue;
  for(I si:{-1,1})for(I sj:{-1,1}){auto a=si<0?c.neg(base[i]):base[i],b=sj<0?c.neg(base[j]):base[j];if(c.add(a,b)==q){std::vector<I>v(base.size());v[i]=mod(v[i]+si,19);v[j]=mod(v[j]+sj,19);rs.insert(v);lifts.items.push_back(nums({i,j,si,sj}));}}
 }for(auto&r:rs)rows.items.push_back(nums(r));return obj({{"rows",rows},{"lifts",lifts}});
}
struct Fixture {
 Field k{37};Curve e{k,7,19},t{k,28,33};Point P,U;
 Fixture(){P=generator(e,19);U=phi(P);}
 Point phi(Point q)const{if(q.infinity()||q.x==4)return {};I w=ipow(mod(q.x-4,37),35,37);return {mod(q.x+18*w,37),mod(q.y*(1-18*w*w),37)};}
 Point dual(Point q)const{if(q.infinity()||q.x==29)return {};I w=ipow(mod(q.x-29,37),35,37);return {mod(28*(q.x+35*w),37),mod(14*q.y*(1-35*w*w),37)};}
 Point inverse(Point q)const{return e.scalar(10,dual(q));}
};
Json fixture_record(const Fixture&f){auto ep=f.e.points(),tp=f.t.points();require(ep.size()==38&&tp.size()==38,"actual group cardinalities");require(!f.P.infinity()&&!f.U.infinity()&&f.e.scalar(19,f.P).infinity()&&f.t.scalar(19,f.U).infinity(),"generators r19");require(ipow(35,18,37)==36,"nonsquare twist");require(f.t.a==mod(35*35*f.e.a,37)&&f.t.b==mod(35*35*35*f.e.b,37),"actual d=-2 twist");
 auto ej=arr(),tj=arr(),fa=arr(),da=arr(),scalars=arr();
 for(auto q:ep){auto a=f.phi(q),b=f.dual(a);require(f.t.contains(a)&&b==f.e.scalar(2,q),"forward/dual composition");ej.items.push_back(obj({{"point",nums({q.x,q.y})},{"image",nums({a.x,a.y})},{"dual_image",nums({b.x,b.y})}}));}
 for(auto q:tp){auto a=f.dual(q),b=f.phi(a);require(f.e.contains(a)&&b==f.t.scalar(2,q),"dual/forward composition");tj.items.push_back(obj({{"point",nums({q.x,q.y})},{"dual",nums({a.x,a.y})},{"image_dual",nums({b.x,b.y})}}));}
 for(I i=0;i<38;++i)for(I j=0;j<38;++j){auto lhs=f.phi(f.e.add(ep[i],ep[j])),rhs=f.t.add(f.phi(ep[i]),f.phi(ep[j]));require(lhs==rhs,"actual forward addition");fa.items.push_back(nums({i,j,lhs.x,lhs.y,rhs.x,rhs.y}));auto a=f.dual(f.t.add(tp[i],tp[j])),b=f.e.add(f.dual(tp[i]),f.dual(tp[j]));require(a==b,"actual dual addition");da.items.push_back(nums({i,j,a.x,a.y,b.x,b.y}));}
 for(I n=0;n<19;++n){auto q=f.e.scalar(n,f.P),r=f.t.scalar(n,f.U),a=f.inverse(r);require(f.phi(q)==r&&a==q,"scalar-preserving coordinate inverse");auto wrong=f.e.scalar(9,f.dual(r));require(n==0||wrong!=q,"false inverse multiplier control");scalars.items.push_back(obj({{"n",num(n)},{"source",nums({q.x,q.y})},{"target",nums({r.x,r.y})},{"inverse",nums({a.x,a.y})},{"wrong9",nums({wrong.x,wrong.y})}}));}
 auto changes=arr();for(I u=1;u<37;++u){bool ok=mod(ipow(u,4,37)*7,37)==28&&mod(ipow(u,6,37)*19,37)==33;require(!ok,"non-isomorphic short scaling");changes.items.push_back(obj({{"u",num(u)},{"matches",truth(ok)}}));}
 Point K{4,0};require(f.e.contains(K)&&!f.e.scalar(19,K).infinity()&&f.phi(K).infinity(),"material odd-subgroup domain");
 return obj({{"schema",str("quadratic-twist-cm8-37-b3-transfer/v1")},{"p",num(37)},{"r",num(19)},{"source_curve",nums({7,19})},{"target_curve",nums({28,33})},{"generator",nums({f.P.x,f.P.y})},{"image_generator",nums({f.U.x,f.U.y})},{"source_points",ej},{"target_points",tj},{"forward_addition",fa},{"dual_addition",da},{"scalars",scalars},{"short_scalings",changes},{"kernel",nums({K.x,K.y})}});
}
std::map<std::string,Json> inputs(const Fixture&f){std::map<std::string,Json>out;
 for(bool target:{false,true}){const auto&c=target?f.t:f.e;Point G=target?f.U:f.P;auto xs=subgroup_xs(c,G,19);require(xs.size()==9,"nine unsigned classes");I index=0;
  for(I i=0;i<9;++i)for(I j=i+1;j<9;++j)for(I h=j+1;h<9;++h){auto base=representatives(c,{xs[i],xs[j],xs[h]});std::vector<Point>other;for(auto q:base)other.push_back(target?f.inverse(q):f.phi(q));
   for(bool mate:{false,true}){auto label=std::string(mate?(target?"pullback":"matched"):(target?"twist":"main"))+std::to_string(index);auto pts=arr();for(auto q:mate?other:base)pts.items.push_back(nums({q.x,q.y}));Point g=mate?(target?f.P:f.U):G;auto v=obj({{"generator",nums({g.x,g.y})},{"base_points",pts}});v.fields["input_sha256"]=str(digest(dump(v)));out[label]=v;}++index;}
 }require(out.size()==336,"all84 policies x4 roles");return out;
}
Json query(const Curve&c,Point q,const std::vector<Point>&base,bool reduced,I cap){std::vector<I>xs;for(auto b:base)xs.push_back(b.x);auto f=system_s3(c.a,c.b,q.x,37);require(base.size()==3&&!reduced,"single original encoding, cubic equality control");for(auto& term:f)require(term.first[0]<3&&term.first[1]<3,"S3 cubic remainder identity");auto cert=solve_cap({f,base_poly(xs,0,37),base_poly(xs,1,37)},xs,cap);auto rows=rows_for(c,q,base,&cert.at("roots"));auto reference=rows_for(c,q,base,nullptr);if(cert.at("status").string()!="cap")require(dump(rows)==dump(reference),"certified point completeness");Echelon rank(19);for(auto&r:rows.at("rows").items)rank.insert(vec(r));return obj({{"point",nums({q.x,q.y})},{"solver",cert},{"admitted",rows},{"reference",reference},{"rank",num(I(rank.pivots.size()))}});}
Json configuration(const Fixture&f,const Json&input,const std::string&label,bool reduced,I cap){bool mapped=label.find("twist")==0||label.find("matched")==0;const auto&c=mapped?f.t:f.e;Point G=point(input.at("generator"));std::vector<Point>base;for(auto&q:input.at("base_points").items)base.push_back(point(q));I b=I(base.size());auto queries=arr();for(I m=1;m<19;++m)queries.items.push_back(query(c,c.scalar(m,G),base,reduced,cap));
 auto fresh=[&](Point q){auto a=query(c,q,base,reduced,cap);I m=1;while(m<19&&c.scalar(m,G)!=q)++m;require(m<19&&dump(a)==dump(queries.items.at(size_t(m-1))),"fresh query binding");return std::pair{a,m};};
 Echelon eq(19);auto setup=arr();for(I m=1;m<19&&I(eq.pivots.size())<b;++m){auto a=fresh(c.scalar(m,G)).first;for(auto&r:a.at("admitted").at("rows").items){auto v=vec(r);v.push_back(m);eq.insert(v);}setup.items.push_back(obj({{"m",num(m)},{"rank",num(I(eq.pivots.size()))}}));}
 bool ready=I(eq.pivots.size())==b&&!eq.pivots.count(b);std::vector<I>logs; if(ready){logs.resize(b);for(auto it=eq.pivots.rbegin();it!=eq.pivots.rend();++it){I j=it->first,v=it->second[b];for(I k=j+1;k<b;++k)v-=it->second[k]*logs[k];logs[j]=mod(v,19);}for(I i=0;i<b;++i)require(c.scalar(logs[i],G)==base[i],"public setup logs");}
 auto recover=[&](Point Q){auto attempts=arr();I answer=-1;Point M=mapped?f.phi(Q):Q;if(ready)for(I shift=0;shift<19&&answer<0;++shift){Point R=c.add(M,c.scalar(shift,G));if(R.infinity()){attempts.items.push_back(obj({{"shift",num(shift)},{"identity",truth()}}));continue;}auto [a,m]=fresh(R);attempts.items.push_back(obj({{"shift",num(shift)},{"identity",truth(false)},{"m",num(m)}}));if(!a.at("admitted").at("lifts").items.empty()){auto l=vec(a.at("admitted").at("lifts").items.front());answer=mod(l[2]*logs[l[0]]+l[3]*logs[l[1]]-shift,19);require(f.e.scalar(answer,f.P)==Q,"ORIGINAL main scalar");}}
 return obj({{"source",nums({Q.x,Q.y})},{"answer",answer<0?Json{}:num(answer)},{"status",str(!ready?"setup-blocked":answer<0?"target-failed":"verified")},{"attempts",attempts}});};auto targets=arr();for(I n=1;n<19;++n)targets.items.push_back(recover(f.e.scalar(n,f.P)));
 auto direct=arr();for(I n=1;n<19;++n){Point Q=f.e.scalar(n,f.P);I a=1;while(a<19&&f.e.scalar(a,f.P)!=Q)++a;require(a<19,"direct enumeration");direct.items.push_back(obj({{"source",nums({Q.x,Q.y})},{"answer",num(a)},{"tests",num(a)}}));}
 return obj({{"arm",str(label)},{"b",num(b)},{"mode",str(reduced?"reduced":"original")},{"cap",num(cap)},{"cubic_remainder_identical",truth()},{"source_sha256",input.at("input_sha256")},{"queries",queries},{"setup",setup},{"ready",truth(ready)},{"logs",nums(logs)},{"targets",targets},{"direct_generic_reference",direct}});
}
}
int main(int argc,char**argv){try{using namespace qualified;require(argc==4,"cm8-37-b3 transfer|census ROOT FRESH_OUTPUT");std::string mode=argv[1];fs::path root=argv[2],out=argv[3];require(!fs::exists(out),"fresh output");Fixture f;auto transfer=fixture_record(f);
 if(mode=="transfer"){std::ofstream(out)<<dump(transfer)<<'\n';std::cout<<"38+38 points,2888 addition pairs,19 scalar images,coordinate inverse10D\n";return 0;}
 require(mode=="census","known mode");fs::create_directories(out);auto arms=inputs(f);auto pins=arr();I count=0,verified=0,blocked=0;
 for(auto&[label,input]:arms)for(bool reduced:{false})for(I cap=0;cap<=8;++cap){auto a=configuration(f,input,label,reduced,cap);blocked+=!a.at("ready").boolean();for(auto&t:a.at("targets").items)verified+=t.at("status").string()=="verified";auto file="p37-b3-"+label+(reduced?"-reduced-":"-original-")+std::to_string(cap)+".json";auto data=dump(a)+'\n';std::ofstream(out/file)<<data;pins.items.push_back(obj({{"name",str(file)},{"sha256",str(digest(data))}}));++count;}
 std::sort(pins.items.begin(),pins.items.end(),[](const Json&a,const Json&b){return a.at("name").string()<b.at("name").string();});require(count==3024,"all336 arms x1 x9");auto sources=arr();for(auto p:{"native/cm8-37-b3.cpp","native/phase-adaptive-prior.hpp","CM8_37_B3_20261030_PROTOCOL.md"}){std::string path="research/quadratic_twist_evidence_20261021/"+std::string(p);sources.items.push_back(obj({{"path",str(path)},{"sha256",str(digest(read(root/path)))}}));}
 auto index=obj({{"schema",str("quadratic-twist-cm8-37-b3-census/v1")},{"configurations",num(count)},{"source_targets",num(18*count)},{"verified_recoveries",num(verified)},{"blocked_setups",num(blocked)},{"files",pins},{"sources",sources}});std::ofstream(out/"index.json")<<dump(index)<<'\n';std::cout<<count<<" configurations, "<<verified<<" verified source recoveries, "<<blocked<<" blocked setups\n";
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
