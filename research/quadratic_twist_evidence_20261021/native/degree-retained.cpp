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
 const I p=13;std::vector<Row>pool;for(I j=0;j<3;++j){Row r;r.f=f[j];r.w[j]=constant(1,p);pool.push_back(r);}auto trace=arr();I peak=0;std::map<std::string,Row> retained;
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
  for(I si:{-1,1})for(I sj:{-1,1}){auto a=si<0?c.neg(base[i]):base[i],b=sj<0?c.neg(base[j]):base[j];if(c.add(a,b)==q){std::vector<I>v(base.size());v[i]=mod(v[i]+si,7);v[j]=mod(v[j]+sj,7);rs.insert(v);lifts.items.push_back(nums({i,j,si,sj}));}}
 }for(auto&r:rs)rows.items.push_back(nums(r));return obj({{"rows",rows},{"lifts",lifts}});
}
struct Fixture{Field k{13};Curve e{k,1,4},t{k,4,6};Point P{0,2},U{8,11};Point phi(Point q)const{if(q.infinity()||q.x==10)return {};I v=ipow(mod(q.x-10,13),11,13);return {mod(12*(q.x+2*v),13),mod(8*q.y*(1-2*v*v),13)};}};
Json query(const Curve&c,Point q,const std::vector<Point>&base,bool reduced,I cap){std::vector<I>xs;for(auto b:base)xs.push_back(b.x);auto f=system_s3(c.a,c.b,q.x,13);if(reduced&&base.size()==2)f=remainder(c,q.x,xs);auto cert=solve_cap({f,base_poly(xs,0,13),base_poly(xs,1,13)},xs,cap);auto rows=rows_for(c,q,base,&cert.at("roots"));auto reference=rows_for(c,q,base,nullptr);if(cert.at("status").string()!="cap")require(dump(rows)==dump(reference),"certified point completeness");Echelon rank(7);for(auto&r:rows.at("rows").items)rank.insert(vec(r));return obj({{"point",nums({q.x,q.y})},{"solver",cert},{"admitted",rows},{"reference",reference},{"rank",num(I(rank.pivots.size()))}});}
Json configuration(const Fixture&f,const Json&input,const std::string&label,bool reduced,I cap){bool mapped=label.find("twist")==0||label.find("matched")==0;const auto&c=mapped?f.t:f.e;Point G=point(input.at("generator"));std::vector<Point>base;for(auto&q:input.at("base_points").items)base.push_back(point(q));I b=I(base.size());auto queries=arr();for(I m=1;m<7;++m)queries.items.push_back(query(c,c.scalar(m,G),base,reduced,cap));
 auto fresh=[&](Point q){auto a=query(c,q,base,reduced,cap);I m=1;while(m<7&&c.scalar(m,G)!=q)++m;require(m<7&&dump(a)==dump(queries.items.at(size_t(m-1))),"fresh query binding");return std::pair{a,m};};
 Echelon eq(7);auto setup=arr();for(I m=1;m<7&&I(eq.pivots.size())<b;++m){auto a=fresh(c.scalar(m,G)).first;for(auto&r:a.at("admitted").at("rows").items){auto v=vec(r);v.push_back(m);eq.insert(v);}setup.items.push_back(obj({{"m",num(m)},{"rank",num(I(eq.pivots.size()))}}));}
 bool ready=I(eq.pivots.size())==b&&!eq.pivots.count(b);std::vector<I>logs; if(ready){logs.resize(b);for(auto it=eq.pivots.rbegin();it!=eq.pivots.rend();++it){I j=it->first,v=it->second[b];for(I k=j+1;k<b;++k)v-=it->second[k]*logs[k];logs[j]=mod(v,7);}for(I i=0;i<b;++i)require(c.scalar(logs[i],G)==base[i],"public setup logs");}
 auto recover=[&](Point Q){auto attempts=arr();I answer=-1;Point M=mapped?f.phi(Q):Q;if(ready)for(I shift=0;shift<7&&answer<0;++shift){Point R=c.add(M,c.scalar(shift,G));if(R.infinity()){attempts.items.push_back(obj({{"shift",num(shift)},{"identity",truth()}}));continue;}auto [a,m]=fresh(R);attempts.items.push_back(obj({{"shift",num(shift)},{"identity",truth(false)},{"m",num(m)}}));if(!a.at("admitted").at("lifts").items.empty()){auto l=vec(a.at("admitted").at("lifts").items.front());answer=mod(l[2]*logs[l[0]]+l[3]*logs[l[1]]-shift,7);require(f.e.scalar(answer,f.P)==Q,"ORIGINAL main scalar");}}
 return obj({{"source",nums({Q.x,Q.y})},{"answer",answer<0?Json{}:num(answer)},{"status",str(!ready?"setup-blocked":answer<0?"target-failed":"verified")},{"attempts",attempts}});};auto targets=arr();for(I n=1;n<7;++n)targets.items.push_back(recover(f.e.scalar(n,f.P)));
 auto direct=arr();for(I n=1;n<7;++n){Point Q=f.e.scalar(n,f.P);I a=1;while(a<7&&f.e.scalar(a,f.P)!=Q)++a;require(a<7,"direct enumeration");direct.items.push_back(obj({{"source",nums({Q.x,Q.y})},{"answer",num(a)},{"tests",num(a)}}));}
 return obj({{"arm",str(label)},{"b",num(b)},{"mode",str(reduced?"reduced":"original")},{"cap",num(cap)},{"source_sha256",input.at("input_sha256")},{"queries",queries},{"setup",setup},{"ready",truth(ready)},{"logs",nums(logs)},{"targets",targets},{"direct_generic_reference",direct}});
}
}
int main(int argc,char**argv){try{using namespace qualified;require(argc==3,"degree-qualified ROOT FRESH_DIR");fs::path root=argv[1],out=argv[2],src=root/"research/quadratic_twist_evidence_20261021/native/uniform-20261029/cases";require(!fs::exists(out),"fresh output");fs::create_directories(out);Fixture f;for(I n=0;n<7;++n)require(f.phi(f.e.scalar(n,f.P))==f.t.scalar(n,f.U),"actual source map");auto pins=arr();I count=0,verified=0,blocked=0;
 for(auto&entry:fs::directory_iterator(src)){auto name=entry.path().filename().string();if(name.rfind("p13-",0)!=0)continue;auto bytes=read(entry.path());auto input=parse(bytes);input.fields["input_sha256"]=str(digest(bytes));auto label=name.substr(name.find('-',4)+1);label=label.substr(0,label.size()-5);
  for(bool reduced:{false,true})for(I cap=0;cap<=8;++cap){auto a=configuration(f,input,label,reduced,cap);blocked+=!a.at("ready").boolean();for(auto&t:a.at("targets").items)verified+=t.at("status").string()=="verified";auto file=name.substr(0,name.size()-5)+(reduced?"-reduced-":"-original-")+std::to_string(cap)+".json";auto data=dump(a)+'\n';std::ofstream(out/file)<<data;pins.items.push_back(obj({{"name",str(file)},{"sha256",str(digest(data))}}));++count;}}
 std::sort(pins.items.begin(),pins.items.end(),[](const Json&a,const Json&b){return a.at("name").string()<b.at("name").string();});require(count==288,"all16 arms x2 x9");auto sources=arr();for(auto p:{"native/degree-retained.cpp","native/phase-adaptive-prior.hpp","DEGREE_RETAINED_20261030_PROTOCOL.md"}){std::string path="research/quadratic_twist_evidence_20261021/"+std::string(p);sources.items.push_back(obj({{"path",str(path)},{"sha256",str(digest(read(root/path)))}}));}
 auto index=obj({{"schema",str("quadratic-twist-degree-qualified/v1")},{"configurations",num(count)},{"source_targets",num(6*count)},{"verified_recoveries",num(verified)},{"blocked_setups",num(blocked)},{"files",pins},{"sources",sources}});std::ofstream(out/"index.json")<<dump(index)<<'\n';std::cout<<count<<" configurations, "<<verified<<" verified source recoveries, "<<blocked<<" blocked setups\n";
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
