// Generator preprocessing only; all original curve/source evidence is unchanged.
#include "reduced-s3-prior.hpp"

namespace reduced {
using namespace s3yield;
std::vector<I> vec(const Json &j) { std::vector<I> v; for(auto &x:j.items)v.push_back(x.integer());return v; }
Point point(const Json &j) {auto v=vec(j);require(v.size()==2,"point shape");return {v[0],v[1]};}
Json poly(const Polynomial &f) {auto j=arr();for(auto [m,c]:f)j.items.push_back(nums({m[0],m[1],c}));return j;}
Polynomial remainder(const Curve &c,I z,const std::vector<I>&xs) {
  I p=c.f.p,s=xs[0]+xs[1],t=xs[0]*xs[1];Polynomial h;
  term(h,{1,1},s*s-4*s*z-2*z*z-2*c.a,p);
  I B=-s*t+2*z*t+s*z*z-2*c.a*z-4*c.b;
  term(h,{1,0},B,p);term(h,{0,1},B,p);
  term(h,{0,0},t*t-2*t*z*z+c.a*c.a-4*c.b*z,p);return h;
}
Json homogeneous(const std::vector<Polynomial>&eqs,I p) {
  auto H=arr();Json cutoff;
  for(I D=0;D<=8;++D) {
    std::vector<Mono> ms;for(I x=D;x>=0;--x)ms.push_back({x,D-x});
    std::map<Mono,I> index;for(I i=0;i<I(ms.size());++i)index[ms[i]]=i;
    Echelon rows(p);
    for(auto &f:eqs)if(!f.empty()&&degree(f)<=D) {
      I df=degree(f);
      for(I x=D-df;x>=0;--x) {
        Mono shift{x,D-df-x};std::vector<I> row(ms.size());
        for(auto [m,c]:f)if(m[0]+m[1]==df)
          row[index.at({m[0]+shift[0],m[1]+shift[1]})]=c;
        rows.insert(row);
      }
    }
    I h=I(ms.size()-rows.pivots.size());H.items.push_back(num(h));
    if(h==0&&cutoff.kind==Json::Null)cutoff=num(D);
  }
  return obj({{"hilbert",H},{"cutoff",cutoff}});
}
Json affine(const std::vector<Polynomial>&eqs,const std::vector<Mono>&roots,I p) {
  I start=0;for(auto &f:eqs)if(!f.empty())start=std::max(start,degree(f));
  auto journal=arr();Json closure,unit_at;
  for(I D=start;D<=8;++D) {
    auto ms=monomials(D);std::map<Mono,I> index;
    for(I i=0;i<I(ms.size());++i)index[ms[i]]=i;
    Echelon rows(p),eval(p);
    for(auto &f:eqs)if(!f.empty()&&degree(f)<=D)
      for(auto s:monomials(D-degree(f))) {
        std::vector<I> row(ms.size());
        for(auto [m,c]:f)row[index.at({m[0]+s[0],m[1]+s[1]})]=c;
        rows.insert(row);
      }
    for(auto xy:roots) {
      std::vector<I> row;for(auto m:ms)row.push_back(mod(ipow(xy[0],m[0],p)*ipow(xy[1],m[1],p),p));eval.insert(row);
    }
    std::vector<I> one(ms.size());one[index.at({0,0})]=1;bool unit=rows.contains(one);
    if(unit&&unit_at.kind==Json::Null)unit_at=num(D);
    journal.items.push_back(nums({D,I(rows.pivots.size()),I(ms.size()),I(eval.pivots.size()),I(unit)}));
    if(rows.pivots.size()+eval.pivots.size()==ms.size()) {
      require(eval.pivots.size()==roots.size(),"evaluation rank");closure=num(D);break;
    }
  }
  require(roots.empty()||unit_at.kind==Json::Null,"false unit");
  return obj({{"closure",closure},{"refutation",unit_at},{"journal",journal}});
}
Json arm(const Curve &c,const Json &source,I r) {
  auto xs=vec(source.at("base_x"));require(xs.size()==2&&xs[0]!=xs[1],"two distinct roots");
  auto P=point(source.at("generator"));std::vector<Point> base;
  require(!P.infinity()&&c.f.mul(P.y,P.y)==c.rhs(P.x)&&c.scalar(r,P).infinity(),"valid subgroup generator");
  for(auto &v:source.at("base_points").items)base.push_back(point(v));
  require(base.size()==2,"base length");
  std::vector<I> logs;
  for(size_t i=0;i<base.size();++i) {auto Q=base[i];require(Q.x==xs[i],"base x binding");
    require(c.f.mul(Q.y,Q.y)==c.rhs(Q.x)&&c.scalar(r,Q).infinity(),"invalid base point");
    I n=0;while(n<r&&c.scalar(n,P)!=Q)++n;require(n<r,"subgroup binding");logs.push_back(n);
  }
  auto gx=base_poly(xs,0,c.f.p),gy=base_poly(xs,1,c.f.p);
  Echelon ranks(r);auto targets=arr();
  for(I n=1;n<r;++n) {
    auto R=c.scalar(n,P);require(!R.infinity(),"target order");auto h=remainder(c,R.x,xs),f=system_s3(c.a,c.b,R.x,c.f.p);
    I before=I(ranks.pivots.size());std::set<std::vector<I>> relations;std::vector<Mono> roots;
    for(I i=0;i<2;++i)for(I j=0;j<2;++j) {
      bool lift=false;
      for(I si:{-1,1})for(I sj:{-1,1})if(c.add(c.scalar(si,base[i]),c.scalar(sj,base[j]))==R) {
        lift=true;std::vector<I> row(2);row[i]=mod(row[i]+si,r);row[j]=mod(row[j]+sj,r);
        require(mod(row[0]*logs[0]+row[1]*logs[1],r)==n,"false relation");relations.insert(row);ranks.insert(row);
      }
      require((evaluate(h,xs[i],xs[j],c.f.p)==0)==lift,"reduced point lift");
      require((evaluate(f,xs[i],xs[j],c.f.p)==0)==lift,"original point lift");if(lift)roots.push_back({xs[i],xs[j]});
    }
    auto rows=arr();for(auto &v:relations)rows.items.push_back(nums(v));
    targets.items.push_back(obj({{"n",num(n)},{"target",nums({R.x,R.y})},{"remainder",poly(h)},
      {"remainder_degree",h.empty()?num(-1):num(degree(h))},{"rows",rows},{"before",num(before)},
      {"after",num(I(ranks.pivots.size()))},{"root_pairs",num(I(roots.size()))},
      {"homogeneous",homogeneous({h,gx,gy},c.f.p)},{"affine",affine({h,gx,gy},roots,c.f.p)},
      {"unreduced",degrees({f,gx,gy},roots,c.f.p,2)}}));
  }
  return obj({{"generator",source.at("generator")},{"base_points",source.at("base_points")},
    {"base_x",source.at("base_x")},{"targets",targets}});
}
bool lower(const Json&a,const Json&b) {return a.kind!=Json::Null&&b.kind!=Json::Null&&a.integer()<b.integer();}
void compare(const std::string &left,const std::string &right,const Json &L,const Json &R,Json &out) {
  require(L.at("targets").items.size()==R.at("targets").items.size(),"target counts");
  for(size_t i=0;i<L.at("targets").items.size();++i) {
    auto &a=L.at("targets").items[i];auto &b=R.at("targets").items[i];
    I da=a.at("after").integer()-a.at("before").integer(),db=b.at("after").integer()-b.at("before").integer();
    auto &ha=a.at("homogeneous").at("cutoff");auto &hb=b.at("homogeneous").at("cutoff");
    bool useful=db>da;bool hom=lower(hb,ha);bool closure=lower(b.at("affine").at("closure"),a.at("affine").at("closure"));
    out.items.push_back(obj({{"main",str(left)},{"twist",str(right)},{"n",a.at("n")},
      {"main_useful",num(da)},{"twist_useful",num(db)},{"main_cutoff",ha},{"twist_cutoff",hb},
      {"joint_homogeneous",truth(useful&&hom)},{"joint_closure",truth(useful&&closure)}}));
  }
}
} // namespace reduced
int main(int argc,char **argv) {try {
  using namespace reduced;require(argc==3,"usage: reduced-s3 ORIGINAL_CASES FRESH_OUTPUT");
  fs::path input=argv[1],output=argv[2];require(!fs::exists(output),"fresh output required");fs::create_directories(output);
  std::vector<fs::path> files;for(auto &e:fs::directory_iterator(input))if(e.path().filename()!="index.json"&&e.path().extension()==".json")files.push_back(e.path());std::sort(files.begin(),files.end());
  std::map<std::string,Json> cases;std::vector<std::string> population,fixture;auto pins=arr();
  for(auto &file:files) {
    auto bytes=read(file);auto source=parse(bytes);if(source.at("size").integer()!=2)continue;
    I p=source.at("p").integer(),a=source.at("a").integer(),b=source.at("b").integer(),d=source.at("d").integer(),r=source.at("r").integer();
    Field k(p);Curve E(k,a,b),T(k,mod(d*d*a,p),mod(d*d*d*b,p));
    auto c=obj({{"p",num(p)},{"a",num(a)},{"b",num(b)},{"d",num(d)},{"r",num(r)},
      {"source_sha256",str(digest(bytes))},{"source_case",str(file.filename().string())},{"scaling",source.at("scaling")},
      {"main",arm(E,source.at("main"),r)},{"twist",arm(T,source.at("twist"),r)}});
    for(auto role:{"matched","pulled_back_twist_base"})if(source.fields.count(role)&&source.at(role).kind!=Json::Null)
      c.fields[role]=arm(std::string(role)=="matched"?T:E,source.at(role),r);
    if(c.fields.count("matched")) {
      for(auto pair:{std::pair{"main","matched"},std::pair{"twist","pulled_back_twist_base"}})
        for(size_t n=0;n<c.at(pair.first).at("targets").items.size();++n) {
          auto x=c.at(pair.first).at("targets").items[n],y=c.at(pair.second).at("targets").items[n];
          for(auto key:{"target","remainder"}){x.fields.erase(key);y.fields.erase(key);}
          require(dump(x)==dump(y),"transported rank/degree mismatch");
        }
    }
    std::string name=file.filename().string();cases[name]=c;
    (p==43?fixture:population).push_back(name);
    std::ofstream(output/name)<<dump(c)<<'\n';pins.items.push_back(obj({{"case",str(name)},{"sha256",str(digest(dump(c)+'\n'))}}));
  }
  require(population.size()==88&&fixture.size()==10,"source coverage");auto comparisons=arr();
  for(auto &s:population)compare(s,s,cases.at(s).at("main"),cases.at(s).at("twist"),comparisons);
  for(auto &l:fixture)for(auto &r:fixture)compare(l,r,cases.at(l).at("main"),cases.at(r).at("twist"),comparisons);
  I joint=0,closure=0;for(auto &c:comparisons.items){joint+=c.at("joint_homogeneous").boolean();closure+=c.at("joint_closure").boolean();}
  auto index=obj({{"schema",str("quadratic-twist-reduced-s3/v1")},{"cases",pins},{"comparisons",comparisons},
    {"comparison_count",num(188)},{"target_comparisons",num(I(comparisons.items.size()))},{"joint_homogeneous_gains",num(joint)},{"joint_closure_gains",num(closure)}});
  std::ofstream(output/"index.json")<<dump(index)<<'\n';std::cout<<"98 cases, 188 comparisons, "<<comparisons.items.size()<<" target comparisons, "<<joint<<" joint homogeneous gains, "<<closure<<" joint closure gains\n";
}catch(const std::exception&e){std::cerr<<e.what()<<'\n';return 1;}}
