#include "phase-adaptive-prior.hpp"

// Exact algorithm requests, not machine instructions, time, or GAE.
namespace phase {
bool known_mode=false;
std::vector<I> checked_xs;
bool cache_mode=false;
bool direct_mode=false;
std::array<std::array<Point,2>,2> signed_cache;
using namespace adaptive;
const std::vector<std::string> keys={
  "solver_calls","membership_evals","constraint_evals","eval_term_visits",
  "basis_calls","generated_rows","dense_column_slots","monomial_copies",
  "pivot_normalizations","scaled_terms","subtraction_steps","term_updates",
  "membership_subtractions","evaluation_entries","evaluation_row_updates",
  "evaluation_row_scales","evaluation_inverses","linear_row_updates",
  "linear_row_scales","linear_inverses","linear_backsub_updates",
  "point_add_requests","point_negations","scalar_iterations","signed_lifts",
  "map_multiplications","membership_builds","remainder_builds","scalar_recoveries","root_entries_validated","root_order_checks","root_polynomial_bindings","cache_entries_built","cache_entry_validations","cache_binding_checks","cache_reads","sign_operand_reads","sign_dispatches"};
struct Demand {
  std::map<std::string,I> n;
  Demand(){for(auto&k:keys)n[k]=0;}
  I& operator[](const std::string&k){return n.at(k);}
  void merge(const Demand&b){for(auto&k:keys)n[k]+=b.n.at(k);}
  Json json()const{auto j=obj({});for(auto&[k,v]:n)j.fields[k]=num(v);return j;}
};
struct NumSpace {
  I p;std::string prefix;Demand& ct;std::map<I,std::vector<I>> pivots;
  NumSpace(I modulus,std::string label,Demand&d):p(modulus),prefix(std::move(label)),ct(d){}
  bool insert(std::vector<I> r){
    for(I j=0;j<I(r.size());++j)if(r[j]){
      auto it=pivots.find(j);
      if(it!=pivots.end()){
        I a=r[j];for(I k=j;k<I(r.size());++k){++ct[prefix+"_row_updates"];r[k]=mod(r[k]-a*it->second[k],p);}
      }else{
        ++ct[prefix+"_inverses"];I a=ipow(r[j],p-2,p);
        for(auto&x:r){++ct[prefix+"_row_scales"];x=mod(x*a,p);}
        pivots[j]=r;return true;
      }
    }return false;
  }
};
I row_size(const Row&r){I n=I(r.f.size());for(auto&w:r.w)n+=I(w.size());return n;}
Row normalize(Row r,I c,I p,Demand&ct){++ct["pivot_normalizations"];ct["scaled_terms"]+=row_size(r);return adaptive::scale(r,c,p);}
void eliminate(Row&a,const Row&b,I c,I p,Demand&ct){++ct["subtraction_steps"];ct["term_updates"]+=row_size(b);adaptive::subtract(a,b,c,p);}
std::vector<Row> basis(const std::vector<Row>&pool,I D,I p,const std::array<Polynomial,3>&original,I&peak,Demand&ct){
  ++ct["basis_calls"];std::map<Mono,Row> pivots;I cols=I(monomials(D).size());
  for(auto&g:pool)if(!g.f.empty()&&degree(g.f)<=D)for(auto m:monomials(D-degree(g.f))){
    ++ct["generated_rows"];ct["dense_column_slots"]+=cols;ct["monomial_copies"]+=row_size(g);
    Row r;r.f=shift(g.f,m);for(I j=0;j<3;++j)r.w[j]=shift(g.w[j],m);
    peak=std::max(peak,expanded(r,original));require(peak<=32,"expanded cap");
    while(!r.f.empty()){
      Mono l=lead(r.f);I c=r.f.at(l);auto it=pivots.find(l);
      if(it==pivots.end()){pivots[l]=normalize(r,ipow(c,p-2,p),p,ct);break;}
      eliminate(r,it->second,c,p,ct);
    }
  }
  std::vector<Mono> ms;for(auto&[m,r]:pivots){(void)r;ms.push_back(m);}std::sort(ms.begin(),ms.end(),order);
  for(auto m:ms)for(auto n:ms)if(order(m,n)&&pivots[n].f.count(m))eliminate(pivots[n],pivots[m],pivots[n].f.at(m),p,ct);
  std::reverse(ms.begin(),ms.end());std::vector<Row> out;for(auto m:ms)out.push_back(pivots[m]);return out;
}
bool member(Polynomial f,const std::vector<Row>&B,I p,Demand&ct){for(auto&r:B){auto l=lead(r.f);if(f.count(l)){++ct["membership_subtractions"];ct["term_updates"]+=I(r.f.size());f=plus(f,r.f,p,-f.at(l));}}return f.empty();}
I evaluate_counted(const Polynomial&f,I x,I y,I p,bool membership,Demand&ct){++ct[membership?"membership_evals":"constraint_evals"];ct["eval_term_visits"]+=I(f.size());return evaluate(f,x,y,p);}
Json solve(const std::array<Polynomial,3>&f,I p,Demand&ct){
  ++ct["solver_calls"];std::vector<Row>pool;I input_max=0;
  for(I j=0;j<3;++j){Row r;r.f=f[j];r.w[j]=constant(1,p);pool.push_back(r);if(!f[j].empty())input_max=std::max(input_max,degree(f[j]));}
  std::vector<Mono> rr;if(known_mode){for(I x:checked_xs)for(I y:checked_xs)if(evaluate_counted(f[0],x,y,p,false,ct)==0)rr.push_back({x,y});}
  else for(I x=0;x<p;++x)if(evaluate_counted(f[1],x,0,p,true,ct)==0)for(I y=0;y<p;++y)if(evaluate_counted(f[2],0,y,p,true,ct)==0&&evaluate_counted(f[0],x,y,p,false,ct)==0)rr.push_back({x,y});
  auto roots_json=arr();for(auto xy:rr)roots_json.items.push_back(nums({xy[0],xy[1]}));auto trace=arr();I peak=0;
  for(I D=0;D<=8;++D)for(I round=0;round<64;++round){
    I before=I(pool.size());auto B=phase::basis(pool,D,p,f,peak,ct);auto ms=monomials(D);NumSpace evaluation(p,"evaluation",ct);
    for(auto xy:rr){std::vector<I>row;for(auto m:ms){++ct["evaluation_entries"];row.push_back(mod(ipow(xy[0],m[0],p)*ipow(xy[1],m[1],p),p));}evaluation.insert(row);}
    bool unit=false;for(auto&r:B)unit|=r.f==constant(1,p);
    bool closure=D>=input_max&&B.size()+evaluation.pivots.size()==ms.size()&&evaluation.pivots.size()==rr.size();
    auto promoted=arr();auto record=[&](){trace.items.push_back(obj({{"degree",num(D)},{"round",num(round)},{"pool_before",num(before)},{"rank",num(I(B.size()))},{"columns",num(I(ms.size()))},{"evaluation_rank",num(I(evaluation.pivots.size()))},{"promoted",promoted}}));};
    if(unit||closure){require(unit==rr.empty(),"terminal root status");record();auto cert=arr();I final_exp=0;
      for(auto&r:B)if(!unit||r.f==constant(1,p)){cert.items.push_back(row_json(r,f));final_exp=std::max(final_exp,expanded(r,f));}
      return obj({{"status",str(unit?"unit":"closure")},{"matrix_degree",num(D)},{"input_max_degree",num(input_max)},{"expanded_peak",num(peak)},{"certificate_expanded_degree",num(final_exp)},{"roots",roots_json},{"certificate",cert},{"trace",trace}});
    }
    std::sort(B.begin(),B.end(),[](const Row&a,const Row&b){I da=degree(a.f),db=degree(b.f);return da!=db?da<db:dump(poly(a.f))<dump(poly(b.f));});
    for(auto&r:B)if(degree(r.f)<D){auto low=phase::basis(pool,degree(r.f),p,f,peak,ct);if(!member(r.f,low,p,ct)){promoted.items.push_back(row_json(r,f));pool.push_back(r);}}
    record();if(promoted.items.empty())break;require(round<63,"round cap");
  }throw std::runtime_error("solver cap");
}
Point neg(const Curve&c,Point P,Demand&ct){++ct["point_negations"];return c.neg(P);}
Point add(const Curve&c,Point P,Point Q,Demand&ct){++ct["point_add_requests"];return c.add(P,Q);}
Point mul(const Curve&c,I n,Point P,Demand&ct){if(n<0){P=phase::neg(c,P,ct);n=-n;}Point R;for(;n;n/=2){++ct["scalar_iterations"];if(n%2)R=phase::add(c,R,P,ct);P=phase::add(c,P,P,ct);}return R;}
Json pj(Point P){return nums({P.x,P.y});}
struct Query {Json record;std::vector<std::array<I,4>>lifts;std::set<std::vector<I>>rows;};
Query query(const Curve&c,Point R,const Json&src,const Json&old,const std::vector<Point>&base,const std::vector<I>&xs,const Polynomial&gx,const Polynomial&gy,Demand&ct){
  require(!R.infinity(),"query at infinity");++ct["remainder_builds"];auto h=remainder(c,R.x,xs);auto cert=phase::solve({h,gx,gy},43,ct);
  // Audit-only binding by actual x coordinate. No scalar lookup is returned to recovery.
  bool bound=false;for(size_t i=0;i<src.at("targets").items.size();++i)if(src.at("targets").items[i].at("target").items[0].integer()==R.x){require(dump(cert)==dump(old.at("targets").items[i].at("reduced")),"frozen certificate mismatch");bound=true;break;}require(bound,"missing frozen query");
  Query q;q.record=obj({{"target",pj(R)},{"solver_sha256",str(digest(dump(cert)))},{"matrix_degree",cert.at("matrix_degree")},{"expanded_peak",cert.at("expanded_peak")},{"root_pairs",num(I(cert.at("roots").items.size()))}});
  auto tested=arr();
  for(auto&root:cert.at("roots").items){auto xy=vec(root);for(I i=0;i<2;++i)for(I j=0;j<2;++j)if(xs[i]==xy[0]&&xs[j]==xy[1])for(I si:{-1,1})for(I sj:{-1,1}){
    ++ct["signed_lifts"];ct["sign_operand_reads"]+=2;ct["sign_dispatches"]+=2;Point A,B;if(direct_mode){A=si<0?phase::neg(c,base[i],ct):base[i];B=sj<0?phase::neg(c,base[j],ct):base[j];}else if(cache_mode){ct["cache_reads"]+=2;A=signed_cache[i][si<0?0:1];B=signed_cache[j][sj<0?0:1];}else{A=phase::mul(c,si,base[i],ct);B=phase::mul(c,sj,base[j],ct);}Point sum=phase::add(c,A,B,ct);tested.items.push_back(obj({{"i",num(i)},{"j",num(j)},{"si",num(si)},{"sj",num(sj)},{"A",pj(A)},{"B",pj(B)},{"sum",pj(sum)},{"accepted",truth(sum==R)}}));if(sum==R){q.lifts.push_back({i,j,si,sj});std::vector<I>row(2);row[i]=mod(row[i]+si,11);row[j]=mod(row[j]+sj,11);q.rows.insert(row);}
  }}
  auto rj=arr(),lj=arr();for(auto&r:q.rows)rj.items.push_back(nums(r));for(auto&l:q.lifts)lj.items.push_back(nums({l[0],l[1],l[2],l[3]}));q.record.fields["rows"]=rj;q.record.fields["lifts"]=lj;q.record.fields["sign_trace_sha256"]=str(digest(dump(tested)));return q;
}
Json run_role(I mode,const std::string&role,const Json&src,const Json&old){
  bool cached=mode==1;bool known=true;known_mode=true;cache_mode=cached;direct_mode=mode==2;
  Field k(43);Curve E(k,1,0),T(k,4,0);bool mapped=role=="twist"||role=="matched";const Curve&c=mapped?T:E;
  Point P{31,18},G=point(src.at("generator"));auto xs=vec(src.at("base_x"));std::vector<Point>base;for(auto&b:src.at("base_points").items)base.push_back(point(b));
  Demand setup;setup["membership_builds"]+=2;auto gx=base_poly(xs,0,43),gy=base_poly(xs,1,43);Json root_contract;
  if(known){
    require(xs.size()==2&&base.size()==2,"two root entries");
    for(I i=0;i<2;++i){++setup["root_entries_validated"];require(xs[i]>=0&&xs[i]<43&&!base[i].infinity()&&base[i].x==xs[i],"root coordinate binding");}
    checked_xs=xs;std::sort(checked_xs.begin(),checked_xs.end());
    ++setup["root_order_checks"];require(checked_xs[0]<checked_xs[1],"distinct query roots");
    for(I v=0;v<2;++v){++setup["root_polynomial_bindings"];Polynomial expected;term(expected,v==0?Mono{2,0}:Mono{0,2},1,43);term(expected,v==0?Mono{1,0}:Mono{0,1},-xs[0]-xs[1],43);term(expected,{0,0},xs[0]*xs[1],43);require(expected==(v==0?gx:gy),"expanded membership binding");}
    for(I x:checked_xs){require(evaluate_counted(gx,x,0,43,true,setup)==0,"false x root");require(evaluate_counted(gy,0,x,43,true,setup)==0,"false y root");}
    root_contract=obj({{"roots",nums(checked_xs)},{"gx",poly(gx)},{"gy",poly(gy)}});
  }
  auto cache_contract=arr();
  if(cached){
    for(I i=0;i<2;++i)for(I j=0;j<2;++j){
      Point q=j==0?phase::neg(c,base[i],setup):base[i];++setup["cache_entries_built"];signed_cache[i][j]=q;
      ++setup["cache_entry_validations"];require(!q.infinity()&&q.x>=0&&q.x<43&&q.y>=0&&q.y<43&&c.contains(q),"invalid cache point");
      ++setup["cache_binding_checks"];if(j==1)require(q==base[i],"false plus binding");else require(phase::add(c,base[i],q,setup).infinity(),"false inverse binding");
      cache_contract.items.push_back(obj({{"base_index",num(i)},{"sign",num(j==0?-1:1)},{"point",pj(q)}}));
    }
  }
  NumSpace linear(11,"linear",setup);auto queries=arr();
  for(I m=1;m<=10&&linear.pivots.size()<2;++m){Demand d;Point R=phase::mul(c,m,G,d);auto q=query(c,R,src,old,base,xs,gx,gy,d);I before=I(linear.pivots.size());
    // Use the query ledger for relation insertion, then merge into setup exactly once.
    NumSpace local(11,"linear",d);local.pivots=linear.pivots;
    for(auto row:q.rows){row.push_back(m);local.insert(row);}linear.pivots=local.pivots;
    q.record.fields["m"]=num(m);q.record.fields["before"]=num(before);q.record.fields["after"]=num(I(linear.pivots.size()));q.record.fields["demand"]=d.json();queries.items.push_back(q.record);setup.merge(d);
  }
  require(linear.pivots.size()==2&&!linear.pivots.count(2),"setup rank cap");std::vector<I>logs(2);
  for(auto it=linear.pivots.rbegin();it!=linear.pivots.rend();++it){I j=it->first,v=it->second[2];for(I m=j+1;m<2;++m){++setup["linear_backsub_updates"];v-=it->second[m]*logs[m];}logs[j]=mod(v,11);}
  for(I j=0;j<2;++j)require(phase::mul(c,logs[j],G,setup)==base[j],"setup log check");
  auto setup_json=obj({{"queries",queries},{"base_logs",nums(logs)},{"demand",setup.json()}});
  auto recover=[&](Point Q){
    Demand online,verification;Point M=Q;if(mapped&&!Q.infinity()){online["map_multiplications"]+=2;M={mod(41*Q.x,43),mod(11*Q.y,43)};}auto attempts=arr();I answer=-1;
    for(I shift=0;shift<=10&&answer<0;++shift){Demand d;Point S=phase::mul(c,shift,G,d),R=phase::add(c,M,S,d);Json record;
      if(R.infinity())record=obj({{"shift",num(shift)},{"identity",truth()}});
      else{auto q=query(c,R,src,old,base,xs,gx,gy,d);record=q.record;record.fields["shift"]=num(shift);record.fields["identity"]=truth(false);
        if(!q.lifts.empty()){auto l=q.lifts.front();++d["scalar_recoveries"];answer=mod(l[2]*logs[l[0]]+l[3]*logs[l[1]]-shift,11);}
      }
      record.fields["demand"]=d.json();attempts.items.push_back(record);online.merge(d);
    }
    if(answer>=0)require(phase::mul(E,answer,P,verification)==Q,"false source recovery");
    Demand cold=setup;cold.merge(online);cold.merge(verification);
    return obj({{"source",pj(Q)},{"mapped",pj(M)},{"answer",num(answer)},{"status",str(answer>=0?"verified":"failed")},{"attempts",attempts},{"online",online.json()},{"verification",verification.json()},{"cold",cold.json()}});
  };
  if(known)setup_json.fields["root_contract"]=root_contract;
  if(cached)setup_json.fields["signed_cache"]=cache_contract;
  auto targets=arr();for(I n=1;n<=10;++n){auto row=recover(E.scalar(n,P));row.fields["n"]=num(n);targets.items.push_back(row);}
  return obj({{"setup",setup_json},{"targets",targets}});
}
} // namespace phase
Json old_vector_shape(Json j){
  for(auto&x:j.items)x=old_vector_shape(x);
  for(auto&[k,v]:j.fields){(void)k;v=old_vector_shape(v);}
  for(auto k:{"sign_operand_reads","sign_dispatches","sign_trace_sha256"})j.fields.erase(k);
  return j;
}
int main(int argc,char**argv){try{
  using namespace phase;require(argc==5,"direct-sign REDUCED_CASES ADAPTIVE_CASES SIGNED_CASES FRESH_OUTPUT");fs::path source=argv[1],prior=argv[2],signed_prior=argv[3],out=argv[4];require(!fs::exists(out),"fresh output");fs::create_directories(out);
  std::vector<fs::path>files;for(auto&e:fs::directory_iterator(source))if(e.path().filename().string().find("p43-")==0)files.push_back(e.path());std::sort(files.begin(),files.end());require(files.size()==10,"F43 policy coverage");
  auto pins=arr();I setups=0,targets=0,successes=0;
  for(auto&file:files){auto name=file.filename().string(),sb=read(file),ab=read(prior/name),pb=read(signed_prior/name);auto src=parse(sb),old=parse(ab),baseline=parse(pb);auto record=obj({{"source_sha256",str(digest(sb))},{"adaptive_sha256",str(digest(ab))},{"signed_sha256",str(digest(pb))}});
    for(I mode:{0,1,2}){auto arm=obj({});for(auto role:{"main","twist","matched","pulled_back_twist_base"}){
      auto result=run_role(mode,role,src.at(role),old.at(role));if(mode<2)require(dump(old_vector_shape(result))==dump(baseline.at(mode==0?"reference":"signed_cache").at(role)),"frozen signed-case mismatch");
      ++setups;for(auto&t:result.at("targets").items){++targets;successes+=t.at("status").string()=="verified";}arm.fields[role]=result;
    }record.fields[mode==0?"binary":mode==1?"cached":"direct"]=arm;}
    auto bytes=dump(record)+'\n';std::ofstream(out/name)<<bytes;pins.items.push_back(obj({{"case",str(name)},{"sha256",str(digest(bytes))}}));
  }
  auto index=obj({{"schema",str("quadratic-twist-direct-sign/v1")},{"cases",pins},{"policies",num(10)},{"setups",num(setups)},{"targets",num(targets)},{"verified",num(successes)},{"unit",str("unpriced algorithm request vector; no scalar time/GAE")}});std::ofstream(out/"index.json")<<dump(index)<<'\n';
  std::cout<<setups<<" setups, "<<targets<<" separate targets, "<<successes<<" source scalars verified; binary, cached and direct signs\n";
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
