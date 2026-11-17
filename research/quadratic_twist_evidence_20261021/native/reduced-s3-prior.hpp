// Verbatim helper snapshot from s3-yield.cpp lines1-211, excluding its main.
// This addition preserves the frozen source and avoids importing a second main.
// Exact finite S3 systems; no timings or extrapolated solver costs.
#define TWIST_FULL_NO_MAIN
#include "full.cpp"

namespace s3yield {
using Mono = std::array<I, 2>;
using Polynomial = std::map<Mono, I>;
using Matrix = std::vector<std::vector<I>>;

void term(Polynomial &f, Mono m, I c, I p) {
  f[m] = mod(f[m] + c, p);
  if (!f[m]) f.erase(m);
}
Polynomial plus(Polynomial a, const Polynomial &b, I p, I scale = 1) {
  for (auto [m,c] : b) term(a,m,scale*c,p);
  return a;
}
Polynomial times(const Polynomial &a, const Polynomial &b, I p) {
  Polynomial f;
  for (auto [m,c] : a) for (auto [n,d] : b)
    term(f,{m[0]+n[0],m[1]+n[1]},c*d,p);
  return f;
}
Polynomial constant(I c, I p) {
  Polynomial f; term(f,{0,0},c,p); return f;
}
Polynomial system_s3(I a, I b, I z, I p) {
  Polynomial x{{{1,0},1}}, y{{{0,1},1}};
  auto delta=plus(x,y,p,-1), xy=times(x,y,p);
  auto f=times(times(delta,delta,p),constant(z*z,p),p);
  auto middle=plus(times(plus(x,y,p),plus(xy,constant(a,p),p),p),constant(2*b,p),p);
  f=plus(f,middle,p,-2*z);
  auto last=plus(xy,constant(-a,p),p);
  f=plus(f,times(last,last,p),p);
  return plus(f,plus(x,y,p),p,-4*b);
}
Polynomial base_poly(const std::vector<I> &xs, I variable, I p) {
  auto g=constant(1,p);
  for (I x : xs) {
    Polynomial t=constant(-x,p); Mono m{0,0}; m[variable]=1;
    term(t,m,1,p); g=times(g,t,p);
  }
  return g;
}
I evaluate(const Polynomial &f,I x,I y,I p) {
  I out=0;
  for(auto [m,c]:f) out=mod(out+c*ipow(x,m[0],p)*ipow(y,m[1],p),p);
  return out;
}
I degree(const Polynomial &f) {
  I d=0; for(auto [m,c]:f){(void)c;d=std::max(d,m[0]+m[1]);}return d;
}
std::vector<Mono> monomials(I d) {
  std::vector<Mono> ms;
  for(I n=d;n>=0;--n) for(I x=n;x>=0;--x) ms.push_back({x,n-x});
  return ms;
}
struct Echelon {
  I p; std::map<I,std::vector<I>> pivots;
  explicit Echelon(I prime):p(prime){}
  bool insert(std::vector<I> row) {
    for(I j=0;j<I(row.size());++j) if(row[j]) {
      auto it=pivots.find(j);
      if(it!=pivots.end()) {
        I scale=row[j];
        for(I k=j;k<I(row.size());++k) row[k]=mod(row[k]-scale*it->second[k],p);
      } else {
        I inv=ipow(row[j],p-2,p);
        for(auto &x:row)x=mod(x*inv,p);
        pivots.emplace(j,std::move(row));return true;
      }
    }
    return false;
  }
  bool contains(std::vector<I> row) const {
    for(auto &[j,v]:pivots) if(row[j]) {
      I scale=row[j];for(I k=j;k<I(row.size());++k)row[k]=mod(row[k]-scale*v[k],p);
    }
    return std::all_of(row.begin(),row.end(),[](I x){return x==0;});
  }
};
Json degrees(const std::array<Polynomial,3> &equations,
             const std::vector<Mono> &roots,I p,I b) {
  auto journal=arr();Json closure,refutation;
  for(I D=std::max<I>(4,b);D<=12;++D) {
    auto ms=monomials(D);std::map<Mono,I> index;
    for(I j=0;j<I(ms.size());++j)index[ms[j]]=j;
    Echelon matrix(p),evaluation(p);
    for(auto xy:roots) {
      std::vector<I> row;
      for(auto m:ms)row.push_back(mod(ipow(xy[0],m[0],p)*ipow(xy[1],m[1],p),p));
      evaluation.insert(row);
    }
    for(const auto &f:equations) for(auto shift:monomials(D-degree(f))) {
      std::vector<I> row(ms.size());
      for(auto [m,c]:f) row[index.at({m[0]+shift[0],m[1]+shift[1]})]=c;
      matrix.insert(row);
    }
    std::vector<I> one(ms.size());one[index.at({0,0})]=1;
    bool unit=matrix.contains(one);
    if(unit&&refutation.kind==Json::Null)refutation=num(D);
    journal.items.push_back(nums({D,I(matrix.pivots.size()),I(ms.size()),
                                  I(evaluation.pivots.size()),I(unit)}));
    if(matrix.pivots.size()+evaluation.pivots.size()==ms.size()) {
      require(evaluation.pivots.size()==roots.size(),"root evaluation lacks interpolation rank");
      closure=num(D);break;
    }
  }
  require(roots.empty() || refutation.kind==Json::Null,"false refutation");
  return obj({{"closure",closure},{"refutation",refutation},{"journal",journal}});
}
Point generator(const Curve &c,I r) {
  for(auto P:c.points())if(!P.infinity()&&c.scalar(r,P).infinity())return P;
  throw std::runtime_error("no nonzero r-torsion generator");
}
Point scale_point(const Field &k,Point P,I s) {
  return P.infinity()?P:Point{k.mul(k.pow(s,2),P.x),k.mul(k.pow(s,3),P.y)};
}
std::vector<I> subgroup_xs(const Curve &c,Point P,I r) {
  std::set<I> xs;
  for(I n=1;n<r;++n)xs.insert(c.scalar(n,P).x);
  return {xs.begin(),xs.end()};
}
std::vector<Point> representatives(const Curve &c,const std::vector<I> &xs) {
  std::vector<Point> out;
  for(I x:xs) {
    auto points=c.f.roots[c.rhs(x)];
    require(!points.empty(),"base x has no point");out.push_back({x,points.front()});
  }
  return out;
}
Json arm(const Curve &c,Point P,I r,const std::vector<I> &xs,
         const std::vector<Point> &transported = {}) {
  auto points=transported.empty()?representatives(c,xs):transported;
  std::vector<I> labels;
  for(auto B:points) {
    I n=0;while(n<r&&c.scalar(n,P)!=B)++n;
    require(n<r,"base point outside selected subgroup");labels.push_back(n);
  }
  Echelon relation_rank(r);auto targets=arr();I raw=0,signed_rows=0,useful_targets=0;
  I prefix_rank=0,prefix_useful=0;Json full_at;
  auto gx=base_poly(xs,0,c.f.p),gy=base_poly(xs,1,c.f.p);
  for(I n=1;n<r;++n) {
    auto R=c.scalar(n,P);auto f=system_s3(c.a,c.b,R.x,c.f.p);
    require(degree(f)==4&&f.at({2,2})==1,"S3 top monomial");
    for(auto [m,co]:f){(void)co;require(m==Mono{2,2}||m[0]+m[1]<4,"extra S3 top term");}
    std::vector<Mono> roots;std::set<std::vector<I>> coefficient_rows;
    I rows=0,before=I(relation_rank.pivots.size());
    for(I i=0;i<I(xs.size());++i)for(I j=0;j<I(xs.size());++j) {
      bool lifts=false;
      for(I si:{-1,1})for(I sj:{-1,1})
        if(c.add(c.scalar(si,points[i]),c.scalar(sj,points[j]))==R) {
          lifts=true;++rows;std::vector<I> row(xs.size());
          row[i]=mod(row[i]+si,r);row[j]=mod(row[j]+sj,r);
          I rhs=0;for(I k=0;k<I(xs.size());++k)rhs=mod(rhs+row[k]*labels[k],r);
          require(rhs==n,"false signed coefficient relation");relation_rank.insert(row);
          coefficient_rows.insert(row);
        }
      require((evaluate(f,xs[i],xs[j],c.f.p)==0)==lifts,"S3 root lacks actual point lift");
      if(lifts)roots.push_back({xs[i],xs[j]});
    }
    I after=I(relation_rank.pivots.size());
    raw+=I(roots.size());signed_rows+=rows;useful_targets+=after>before;
    if(n<=3){prefix_rank=after;prefix_useful+=after>before;}
    if(after==I(xs.size())&&full_at.kind==Json::Null)full_at=num(n);
    auto row_json=arr();for(const auto &row:coefficient_rows)row_json.items.push_back(nums(row));
    targets.items.push_back(obj({{"n",num(n)},{"root_pairs",num(I(roots.size()))},
      {"signed_rows",num(rows)},{"before",num(before)},{"after",num(after)},
      {"coefficient_rows",row_json},
      {"degree",degrees({f,gx,gy},roots,c.f.p,I(xs.size()))}}));
  }
  auto bp=arr();for(auto B:points)bp.items.push_back(nums({B.x,B.y}));
  return obj({{"generator",nums({P.x,P.y})},{"base_x",nums(xs)},{"base_points",bp},
    {"labels_for_audit_only",nums(labels)},{"targets",targets},
    {"summary",obj({{"root_pairs",num(raw)},{"signed_rows",num(signed_rows)},
      {"rank",num(I(relation_rank.pivots.size()))},{"useful_targets",num(useful_targets)},
      {"prefix_rank",num(prefix_rank)},{"prefix_useful_targets",num(prefix_useful)},
      {"first_full_rank",full_at}})}});
}
std::map<std::tuple<I,I,I>,std::string> names(const Json &registry) {
  std::map<std::tuple<I,I,I>,std::string> out;
  for(auto &m:registry.at("curves").items)if(m.at("family").string()=="prime") {
    const auto &ps=m.at("params");
    if(!ps.fields.count("p")||!ps.fields.count("a")||!ps.fields.count("b"))continue;
    std::vector<I> v;bool small=true;
    for(auto key:{"p","a","b"}) {
      const auto &x=ps.at(key);
      if(x.kind==Json::Number)v.push_back(x.integer());
      else if(x.kind==Json::String&&x.string().size()<4)v.push_back(std::stoll(x.string()));
      else small=false;
    }
    if(small)out[{v[0],v[1],v[2]}]=m.at("slug").string();
  }return out;
}
std::vector<std::vector<I>> subsets(const std::vector<I> &xs,I b) {
  std::vector<std::vector<I>> out;
  for(I mask=0;mask<(I(1)<<xs.size());++mask) {
    std::vector<I> sub;
    for(I i=0;i<I(xs.size());++i)if(mask&(I(1)<<i))sub.push_back(xs[i]);
    if(I(sub.size())==b)out.push_back(sub);
  }std::sort(out.begin(),out.end());return out;
}
Json hilbert(I b) {
  std::vector<I> h;
  for(I D=0;D<=b+2;++D) {
    I n=0;for(I i=0;i<=D;++i){I j=D-i;n+=i<b&&j<b&&(i<2||j<2);}h.push_back(n);
  }
  require(h[b]>0&&h[b+1]==0,"homogeneous regularity cutoff");
  return nums(h);
}
} // namespace s3yield
