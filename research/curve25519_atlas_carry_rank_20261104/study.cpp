// Development instrument: exact small-prime controls, not a Curve25519 solver.
// Native C++20 plus GMP for the RFC fixture. No timing or ECDLP cost claims.
#include <algorithm>
#include <array>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <random>
#include <regex>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include "curve25519_fixture.hpp"
using I = std::int64_t;
void require(bool b, const std::string& s) { if (!b) throw std::runtime_error(s); }
I mod(I a, I p) { I r=a%p; return r<0?r+p:r; }
std::string integer(I x) { return x<0?"(- "+std::to_string(-x)+")":std::to_string(x); }
I power(I a,I n,I p) { I r=1; for(;n;n/=2,a=a*a%p) if(n%2) r=r*a%p; return r; }
bool prime(I n) { if(n<2) return false; for(I d=2;d*d<=n;++d) if(n%d==0) return false; return true; }
struct Point {
    I x=0,y=0; bool inf=true;
    auto operator<=>(const Point&) const = default;
};
struct Curve {
    I p,a,b; // y^2 = x^3 + a*x^2 + b*x, p <= 65521.
    I f(I x) const { return mod(x*mod(x*x+a*x+b,p),p); }
    bool has(Point q) const { return q.inf || mod(q.y*q.y,p)==f(q.x); }
    Point neg(Point q) const { if(!q.inf) q.y=mod(-q.y,p); return q; }
    I inv(I x) const { require(mod(x,p)!=0,"division by zero"); return power(mod(x,p),p-2,p); }
    Point add(Point q,Point r) const {
        if(q.inf) return r;
        if(r.inf) return q;
        if(q.x==r.x && mod(q.y+r.y,p)==0) return {};
        I l=q.x==r.x ? mod((3*q.x*q.x+2*a*q.x+b)*inv(2*q.y),p)
                           : mod((r.y-q.y)*inv(r.x-q.x),p);
        I x=mod(l*l-a-q.x-r.x,p);
        return {x,mod(l*(q.x-x)-q.y,p),false};
    }
    Point mul(I n,Point q) const { Point r; for(;n;n/=2,q=add(q,q)) if(n%2) r=add(r,q); return r; }
    Curve quotient() const { return {p,mod(-2*a,p),mod(a*a-4*b,p)}; }
    Point phi(Point q) const {
        if(q.inf || q.x==0) return {};
        I z=inv(q.x);
        return {mod(q.x+a+b*z,p),mod(q.y*mod(1-b*mod(z*z,p),p),p),false};
    }
    Point dual(Point q) const {
        Point z=quotient().phi(q);
        if(!z.inf) { z.x=mod(z.x*inv(4),p); z.y=mod(z.y*inv(8),p); }
        return z;
    }
    std::vector<Point> points() const {
        std::vector<std::vector<I>> roots(static_cast<std::size_t>(p));
        for(I y=0;y<p;++y) roots[y*y%p].push_back(y);
        std::vector<Point> r(1);
        for(I x=0;x<p;++x) for(I y:roots[f(x)]) r.push_back({x,y,false});
        return r;
    }
};
Point canonical(const Curve& e,Point q) { return std::min(q,e.neg(q)); }
struct Fixture {
    Curve e; I order,ell,h; Point g; std::vector<Point> columns;
    explicit Fixture(I p):e{p,486662%p,1},order(0),ell(0),h(0) {
        require(p>=101 && p<=65521 && prime(p),"prime must be in [101,65521]");
        require(mod(e.a*e.a-4*e.b,p)!=0,"singular curve");
        auto all=e.points(); order=static_cast<I>(all.size());
        for(I d=2;d<=order;++d) if(order%d==0 && prime(d)) ell=d;
        require(ell>5 && order%(ell*ell)!=0,"need a simple odd prime subgroup");
        h=order/ell;
        for(Point q:all) { Point r=e.mul(h,q); if(!r.inf) {g=r;break;} }
        require(!g.inf && e.mul(ell,g).inf,"subgroup generator");
        std::set<Point> s;
        for(Point q:all) if(!q.inf && e.mul(ell,q).inf) s.insert(canonical(e,q));
        columns.assign(s.begin(),s.end());
        require(static_cast<I>(columns.size())==(ell-1)/2,"subgroup coverage");
    }
};
struct Arm { std::string name; Curve model; std::vector<Point> base,source; bool transported=false; };
std::vector<Arm> atlas(const Fixture& f,std::size_t n,std::uint64_t seed) {
    require(n>=3 && n<=f.columns.size(),"base column count outside subgroup capacity");
    std::vector<Point> native(f.columns.begin(),f.columns.begin()+n);
    auto target=f.columns;
    std::sort(target.begin(),target.end(),[&](Point x,Point y){return canonical(f.e.quotient(),f.e.phi(x))<canonical(f.e.quotient(),f.e.phi(y));});
    std::vector<Point> mixed(native.begin(),native.begin()+n/2);
    for(Point q:target) if(mixed.size()<n && std::find(mixed.begin(),mixed.end(),q)==mixed.end()) mixed.push_back(q);
    target.resize(n);
    auto random=f.columns; std::mt19937_64 rng(seed); std::shuffle(random.begin(),random.end(),rng); random.resize(n);
    std::vector<Arm> out;
    for(auto entry:std::vector<std::pair<std::string,std::vector<Point>>>{{"source",native},{"same_support",native},{"target_native",target},{"mixed",mixed},{"random",random}}) {
        bool transported=entry.first=="same_support" || entry.first=="target_native";
        auto base=entry.second;
        if(transported) for(auto& q:base) q=f.e.phi(q); // Keep sign orientation, no silent column rescaling.
        out.push_back({entry.first,transported?f.e.quotient():f.e,base,entry.second,transported});
    }
    return out;
}
// Rank is over the prime subgroup scalar field, not F_p.
struct Rank {
    I ell; std::map<std::size_t,std::vector<I>> rows;
    explicit Rank(I q):ell(q){}
    bool insert(std::vector<I> row) {
        for(auto& x:row) x=mod(x,ell);
        for(const auto& [pivot,r]:rows) {
            I c=row[pivot];
            for(std::size_t j=pivot;j<row.size();++j) row[j]=mod(row[j]-c*r[j],ell);
        }
        auto it=std::find_if(row.begin(),row.end(),[](I x){return x!=0;});
        if(it==row.end()) return false;
        auto pivot=static_cast<std::size_t>(it-row.begin()); I z=power(*it,ell-2,ell);
        for(auto& x:row) x=x*z%ell;
        rows.emplace(pivot,std::move(row)); return true;
    }
};
struct Stats { I probes=0,hits=0,new_rank=0; };
Stats collect(const Curve& e,I ell,const std::vector<Point>& base,const std::vector<Point>& targets,std::ostream* trace=nullptr) {
    // Generic decomposition diagnostic; never label this an algebraic oracle.
    std::map<Point,std::pair<std::size_t,I>> lookup;
    for(std::size_t k=0;k<base.size();++k) {
        require(!base[k].inf && base[k].y!=0,"base must contain odd-subgroup nonzero points");
        require(!lookup.contains(base[k]) && !lookup.contains(e.neg(base[k])),"duplicate sign orbit");
        lookup[base[k]]={k,1}; lookup[e.neg(base[k])]={k,-1};
    }
    Rank rank(ell); Stats s;
    std::size_t target_index=0;
    if(trace)*trace<<"target,i,j,k,si,sj,sk,rank_increment,cumulative_pair_probes\n";
    for(Point t:targets) {
        bool found=false;
        for(std::size_t i=0;i<base.size() && !found;++i) for(std::size_t j=i+1;j<base.size() && !found;++j)
        for(I si:{-1,1}) for(I sj:{-1,1}) {
            if(found) break;
            ++s.probes;
            Point u=e.add(si==1?base[i]:e.neg(base[i]),sj==1?base[j]:e.neg(base[j]));
            auto it=lookup.find(e.add(t,e.neg(u)));
            if(it==lookup.end() || it->second.first==i || it->second.first==j) continue;
            auto [k,sk]=it->second; std::vector<I> row(base.size()); row[i]=si;row[j]=sj;row[k]=sk;
            Point check=e.add(u,sk==1?base[k]:e.neg(base[k])); require(check==t,"relation witness");
            ++s.hits;bool independent=rank.insert(row);if(independent)++s.new_rank;
            if(trace)*trace<<target_index<<','<<i<<','<<j<<','<<k<<','<<si<<','<<sj<<','<<sk<<','<<independent<<','<<s.probes<<'\n';
            found=true; break;
        }
        ++target_index;
    }
    return s;
}
std::vector<I> digits(I n,I radix,std::size_t size) {
    std::vector<I> d(size); for(auto& x:d) {x=n%radix;n/=radix;} require(n==0,"digit overflow");return d;
}
bool carry_witness(I a,I b,I p,I radix) {
    std::size_t n=1; for(I t=p/radix;t;t/=radix) ++n;
    auto ad=digits(a,radix,n),bd=digits(b,radix,n),pd=digits(p,radix,n);
    auto qd=digits(a*b/p,radix,n),cd=digits(a*b%p,radix,n);
    I carry=0;
    for(std::size_t k=0;k<2*n;++k) {
        I s=carry-(k<n?cd[k]:0);
        for(std::size_t i=0;i<n;++i) if(k>=i && k-i<n) s+=ad[i]*bd[k-i]-pd[i]*qd[k-i];
        if(s%radix) return false;
        carry=s/radix;
        if(carry < -2*static_cast<I>(n)*radix*radix || carry > 2*static_cast<I>(n)*radix*radix) return false;
    }
    return carry==0;
}
// Exact QF_NIA relation queries. The two encodings have the same signed
// full-coordinate model set. Membership is an explicit finite-set selector.
class SMT {
    std::ostream& o; I p,radix; std::size_t n=1; bool carry; int serial=0;
    std::map<std::string,std::vector<std::string>> cache;
public:
    SMT(std::ostream& out,I field,bool c,I r):o(out),p(field),radix(r),carry(c) {
        require(r>=2 && r<=16,"radix must be in [2,16]");
        for(I t=p/r;t;t/=r) ++n;
        o<<"; Development diagnostic; check every returned model natively.\n(set-logic QF_NIA)\n";
    }
    void assertion(const std::string& x) {o<<"(assert "<<x<<")\n";}
    void bounded(const std::string& x,I lo,I hi) {
        o<<"(declare-const "<<x<<" Int)\n";
        assertion("(and (<= "+integer(lo)+" "+x+") (<= "+x+" "+integer(hi)+"))");
    }
    void field(const std::string& x) {bounded(x,0,p-1);}
    std::vector<std::string> limbs(const std::string& x) {
        if(cache.contains(x)) return cache.at(x);
        std::vector<std::string> d; std::string sum="(+"; I weight=1;
        for(std::size_t i=0;i<n;++i) {auto s=x+"d"+std::to_string(i); bounded(s,0,radix-1);d.push_back(s);sum+=" (* "+std::to_string(weight)+" "+s+")";weight*=radix;}
        assertion("(= "+x+" "+sum+"))");cache[x]=d; return d;
    }
    std::string mul(const std::string& a,const std::string& b) {
        std::string z="v"+std::to_string(serial++);field(z);
        if(!carry) {assertion("(= "+z+" (mod (* "+a+" "+b+") "+std::to_string(p)+"))");return z;}
        std::string q=z+"q";field(q);auto ad=limbs(a),bd=limbs(b),cd=limbs(z),qd=limbs(q);auto pd=digits(p,radix,n);
        std::vector<std::string> cs(2*n+1,"0"); I bound=2*static_cast<I>(n)*radix*radix;
        for(std::size_t k=1;k<2*n;++k) {cs[k]=z+"c"+std::to_string(k);bounded(cs[k],-bound,bound);}
        for(std::size_t k=0;k<2*n;++k) {
            std::string s="(+ "+cs[k]+" "+(k<n?"(- "+cd[k]+")":"0");
            for(std::size_t i=0;i<n;++i) if(k>=i && k-i<n)
                s+=" (* "+ad[i]+" "+bd[k-i]+") (- (* "+std::to_string(pd[i])+" "+qd[k-i]+"))";
            assertion("(= "+s+") (* "+std::to_string(radix)+" "+cs[k+1]+"))");
        }
        return z;
    }
    std::string linear(const std::string& expr) {
        std::string z="v"+std::to_string(serial++);field(z);
        assertion("(= "+z+" (mod "+expr+" "+std::to_string(p)+"))");return z;
    }
    struct P {std::string x,y,inf;};
    P point(const std::string& name,const Curve& e) {
        require(e.p==p,"point field mismatch");
        P q{name+"x",name+"y",name+"inf"};field(q.x);field(q.y);o<<"(declare-const "<<q.inf<<" Bool)\n";
        // Leaves come from a natively checked finite table. Complete addition
        // implies that internal nodes are on curve. Repeating their cubic curve
        // equations adds consequences, not model restrictions.
        assertion("(=> "+q.inf+" (and (= "+q.x+" 0) (= "+q.y+" 0)))");return q;
    }
    static std::string equal(P a,P b) {return "(and (= "+a.inf+" "+b.inf+") (= "+a.x+" "+b.x+") (= "+a.y+" "+b.y+"))";}
    void addition(P a,P b,P c,const Curve& e) {
        std::string l="slope"+std::to_string(serial++);field(l);
        auto dx=linear("(- "+b.x+" "+a.x+")"),dy=linear("(- "+b.y+" "+a.y+")");
        auto denom=linear("(* 2 "+a.y+")"),xx=mul(a.x,a.x);
        auto numer=linear("(+ (* 3 "+xx+") (* "+std::to_string(2*e.a)+" "+a.x+") "+std::to_string(e.b)+")");
        auto ld=mul(l,dx),lt=mul(l,denom),ll=mul(l,l);
        auto cx=linear("(- "+ll+" "+std::to_string(e.a)+" "+a.x+" "+b.x+")");
        auto axcx=linear("(- "+a.x+" "+c.x+")"),ly=mul(l,axcx),cy=linear("(- "+ly+" "+a.y+")");
        auto ys=linear("(+ "+a.y+" "+b.y+")");
        std::string slope="(ite (= "+a.x+" "+b.x+") (= "+lt+" "+numer+") (= "+ld+" "+dy+"))";
        std::string normal="(and (not "+c.inf+") "+slope+" (= "+c.x+" "+cx+") (= "+c.y+" "+cy+"))";
        assertion("(ite "+a.inf+" "+equal(b,c)+" (ite "+b.inf+" "+equal(a,c)+" (ite (and (= "+a.x+" "+b.x+") (= "+ys+" 0)) "+c.inf+" "+normal+")))");
    }
    void query(const Arm& arm,Point target,const std::vector<Point>& pinned={}) {
        require(arm.model.has(target),"off-curve query target");
        for(Point q:arm.base) require(!q.inf && arm.model.has(q) && q.y!=0,"invalid query base");
        auto a=point("a",arm.model),b=point("b",arm.model),c=point("c",arm.model),u=point("u",arm.model),r=point("r",arm.model);
        std::array<P,3> leaves{a,b,c};
        for(P leaf:leaves) {
            std::string s="(or";
            for(Point q:arm.base) for(Point z:{q,arm.model.neg(q)}) s+=" (and (= "+leaf.x+" "+std::to_string(z.x)+") (= "+leaf.y+" "+std::to_string(z.y)+"))";
            assertion("(not "+leaf.inf+")");assertion(s+")");
        }
        // Same distinct-column restriction as the generic diagnostic.
        for(std::size_t i=0;i<3;++i) for(std::size_t j=i+1;j<3;++j) assertion("(not (= "+leaves[i].x+" "+leaves[j].x+"))");
        assertion(std::string(target.inf?"":"(not ")+r.inf+(target.inf?"":")"));
        assertion("(= "+r.x+" "+std::to_string(target.x)+")");assertion("(= "+r.y+" "+std::to_string(target.y)+")");
        addition(a,b,u,arm.model);addition(u,c,r,arm.model);
        require(pinned.empty() || pinned.size()==3,"pinned fixture arity");
        for(std::size_t i=0;i<pinned.size();++i) {
            assertion("(= "+leaves[i].x+" "+std::to_string(pinned[i].x)+")");
            assertion("(= "+leaves[i].y+" "+std::to_string(pinned[i].y)+")");
        }
        o<<"(check-sat)\n(get-model)\n";
    }
};
void replay(const Curve& e,const std::vector<Point>& base,Point target,const std::filesystem::path& path) {
    std::ifstream in(path);require(in.good(),"missing solver output");
    std::string text((std::istreambuf_iterator<char>(in)),{});
    require(text.starts_with("sat\n"),"replay requires sat; unknown and unsat are not certificates");
    auto value=[&](const std::string& name) {
        std::regex pattern("\\(define-fun\\s+"+name+"\\s+\\(\\)\\s+Int\\s+([0-9]+)\\s*\\)");
        std::smatch match;require(std::regex_search(text,match,pattern),"missing canonical coordinate "+name);
        I x=std::stoll(match[1].str());require(x>=0 && x<e.p,"noncanonical coordinate");return x;
    };
    Point sum;std::set<I> used;
    for(std::string name:{"a","b","c"}) {
        Point q{value(name+"x"),value(name+"y"),false};require(e.has(q),"off-curve model");
        require(used.insert(q.x).second,"repeated sign orbit");
        require(std::any_of(base.begin(),base.end(),[&](Point b){return q==b || q==e.neg(b);}),"outside factor base");
        sum=e.add(sum,q);
    }
    require(sum==target,"solver witness does not sum to target");
}
void encoding_fixtures(const std::filesystem::path& out) {
    require(!std::filesystem::exists(out),"fixture output already exists");
    Fixture f(101);auto arms=atlas(f,3,42);std::filesystem::create_directories(out);
    std::ofstream cases(out/"cases.tsv");
    for(const auto& arm:arms) {
        Point sat=arm.model.add(arm.model.add(arm.base[0],arm.base[1]),arm.base[2]);
        Point unsat;bool found=false;
        for(I k=1;k<f.ell;++k) {
            Point r=f.e.mul(k,f.g);if(arm.transported)r=f.e.phi(r);
            if(collect(arm.model,f.ell,arm.base,{r}).hits==0) {unsat=r;found=true;break;}
        }
        require(found,"functional fixture needs an unsatisfiable query");
        for(bool carries:{false,true}) {
            std::string stem=arm.name+(carries?"-carry-pinned":"-mod-pinned");
            std::ofstream o(out/(stem+".smt2"));SMT smt(o,f.e.p,carries,4);smt.query(arm,sat,arm.base);
            cases<<stem<<"\tsat\t"<<arm.name<<'\t'<<sat.x<<'\t'<<sat.y<<'\t'<<sat.inf<<'\n';
        }
        for(bool carries:{false,true}) for(bool positive:{false,true}) {
            Point r=positive?sat:unsat;
            std::string stem=arm.name+(carries?"-carry":"-mod")+(positive?"-sat":"-unsat");
            std::ofstream o(out/(stem+".smt2"));SMT smt(o,f.e.p,carries,4);smt.query(arm,r);
            cases<<stem<<'\t'<<(positive?"sat":"unsat")<<'\t'<<arm.name<<'\t'<<r.x<<'\t'<<r.y<<'\t'<<r.inf<<'\n';
        }
    }
}
void replay_fixture(const std::string& name,Point target,const std::filesystem::path& path) {
    Fixture f(101);auto arms=atlas(f,3,42);
    auto a=std::find_if(arms.begin(),arms.end(),[&](const Arm& a){return a.name==name;});
    require(a!=arms.end(),"unknown fixture arm");replay(a->model,a->base,target,path);
    std::cout<<"native SAT witness replay passed (development fixture)\n";
}
void replay_run(I p,I a,I b,I ell,const std::filesystem::path& base_path,Point target,const std::filesystem::path& model) {
    require(p>=101 && p<=65521 && prime(p) && ell>3 && ell<2*p && prime(ell),"replay field/subgroup domain");
    require(a>=0 && a<p && b>0 && b<p && mod(b*(a*a-4*b),p)!=0,"replay nonsingular model");
    Curve e{p,a,b};
    require(target.x>=0 && target.x<p && target.y>=0 && target.y<p && (!target.inf || (target.x==0 && target.y==0)),"noncanonical replay target");
    require(e.has(target) && e.mul(ell,target).inf,"invalid subgroup target");
    std::ifstream in(base_path);require(in.good(),"missing base file");std::string line;
    std::getline(in,line);require(line=="column,source_x,source_y,model_x,model_y","base CSV schema");
    std::vector<Point> base;std::set<I> xs;
    while(std::getline(in,line)) {
        std::replace(line.begin(),line.end(),',',' ');std::istringstream row(line);I i,sx,sy,x,y;std::string extra;
        require(static_cast<bool>(row>>i>>sx>>sy>>x>>y) && !(row>>extra),"base CSV row");
        require(i==static_cast<I>(base.size()) && x>=0 && x<p && y>0 && y<p && xs.insert(x).second,"base identity");
        Point q{x,y,false};require(e.has(q) && e.mul(ell,q).inf,"invalid subgroup base");base.push_back(q);
    }
    require(base.size()>=3,"insufficient replay base");replay(e,base,target,model);
    std::cout<<"native model witness replay passed (small-prime domain; producer observation)\n";
}
void product_fixtures(const std::filesystem::path& out) {
    require(!std::filesystem::exists(out),"product fixture output already exists");
    std::filesystem::create_directories(out);std::ofstream cases(out/"cases.tsv");
    for(I a=0;a<7;++a)for(I b=0;b<7;++b)for(bool carry:{false,true})for(bool correct:{false,true}) {
        std::string stem=std::to_string(a)+"-"+std::to_string(b)+(carry?"-carry":"-mod")+(correct?"-sat":"-unsat");
        std::ofstream o(out/(stem+".smt2"));SMT smt(o,7,carry,2);smt.field("a");smt.field("b");
        smt.assertion("(= a "+std::to_string(a)+")");smt.assertion("(= b "+std::to_string(b)+")");
        auto c=smt.mul("a","b");I result=correct?a*b%7:(a*b+1)%7;
        smt.assertion("(= "+c+" "+std::to_string(result)+")");o<<"(check-sat)\n";
        cases<<stem<<'\t'<<(correct?"sat":"unsat")<<'\n';
    }
}
std::vector<I> target_scalars(I ell,std::size_t count,std::uint64_t seed) {
    require(count>0 && 2*count<=static_cast<std::size_t>((ell-1)/2),"insufficient disjoint target sign orbits");
    std::vector<I> scalars(static_cast<std::size_t>((ell-1)/2));
    for(std::size_t i=0;i<scalars.size();++i)scalars[i]=static_cast<I>(i)+1;
    std::mt19937_64 rng(seed^0x4f5554ULL);std::shuffle(scalars.begin(),scalars.end(),rng);scalars.resize(2*count);
    for(auto& s:scalars)if(rng()&1U)s=ell-s;
    return scalars;
}
void self_test() {
    curve25519_fixture();
    // Small exhaustive functional fixtures: no measured performance conclusions.
    for(I p:{101,211}) {
        Curve e{p,mod(486662,p),1};auto all=e.points();auto q=e.quotient();
        for(Point x:all) {
            require(q.has(e.phi(x)),"isogeny image on quotient");
            require(e.dual(e.phi(x))==e.mul(2,x),"dual composition");
            for(Point y:all) require(e.phi(e.add(x,y))==q.add(e.phi(x),e.phi(y)),"isogeny homomorphism");
        }
    }
    for(I p:{7,31,101}) for(I radix:{2,4,16}) for(I a=0;a<p;++a) for(I b=0;b<p;++b)
        require(carry_witness(a,b,p,radix),"schoolbook carry witness");
    Rank rank(101);require(rank.insert({1,2,3}),"rank insertion");require(!rank.insert({2,4,6}),"dependent row");require(rank.insert({0,1,0}),"independent row");
    Fixture f(1019);auto arms=atlas(f,5,42);
    {
        auto scalars=target_scalars(f.ell,1,42);std::set<I> orbits;
        for(I s:scalars)require(s>0 && s<f.ell && orbits.insert(std::min(s,f.ell-s)).second,"holdout sign-orbit leakage");
        require(scalars==target_scalars(f.ell,1,42),"target reproducibility");
    }
    std::vector<Point> ts;for(I k=1;k<=30;++k) ts.push_back(f.e.mul(k,f.g));
    auto a=collect(f.e,f.ell,arms[0].base,ts);for(auto& t:ts)t=f.e.phi(t);
    auto b=collect(f.e.quotient(),f.ell,arms[1].base,ts);
    require(a.hits==b.hits && a.probes==b.probes && a.new_rank==b.new_rank,"same support control");
    for(const auto& arm:arms) for(std::size_t i=0;i<arm.source.size();++i) {
        require(f.e.mul(f.ell,arm.source[i]).inf,"source subgroup");
        if(arm.transported) require(f.e.mul(power(2,f.ell-2,f.ell),f.e.dual(arm.base[i]))==arm.source[i],"inverse transport");
    }
    for(bool carries:{false,true}) {std::ostringstream o;SMT smt(o,f.e.p,carries,4);smt.query(arms[0],f.g);require(o.str().find("(check-sat)")!=std::string::npos,"query emission");}
    auto reject=[&](const std::string& text) {
        auto path=std::filesystem::temp_directory_path()/"atlas-replay-negative-test.txt";
        {std::ofstream out(path);out<<text;}
        bool caught=false;try {replay(f.e,arms[0].base,f.g,path);}catch(const std::exception&){caught=true;}
        std::filesystem::remove(path);require(caught,"replay accepted an invalid model");
    };
    reject("unknown\n");reject("unsat\n");reject("sat\n(model)\n");
    {
        auto path=std::filesystem::temp_directory_path()/"atlas-replay-positive-test.txt";
        Point target;std::ofstream out(path);out<<"sat\n(model\n";
        for(std::size_t i=0;i<3;++i) {
            Point q=arms[0].base[i];target=f.e.add(target,q);std::string name(1,static_cast<char>('a'+i));
            out<<"(define-fun "<<name<<"x () Int "<<q.x<<")\n(define-fun "<<name<<"y () Int "<<q.y<<")\n";
        }
        out<<")\n";out.close();replay(f.e,arms[0].base,target,path);
        auto csv_path=path;csv_path+=".csv";
        std::ofstream csv(csv_path);csv<<"column,source_x,source_y,model_x,model_y\n";
        for(std::size_t i=0;i<arms[0].base.size();++i) {
            Point q=arms[0].base[i];csv<<i<<','<<q.x<<','<<q.y<<','<<q.x<<','<<q.y<<'\n';
        }
        csv.close();replay_run(f.e.p,f.e.a,f.e.b,f.ell,csv_path,target,path);
        std::filesystem::remove(csv_path);std::filesystem::remove(path);
    }
    std::cout<<"functional self-tests passed (producer observation; no independent verdict)\n";
}
void run(I p,std::size_t n,std::size_t count,std::uint64_t seed,const std::filesystem::path& out) {
    require(count>0 && count<=128,"target count must be in [1,128]");
    require(!std::filesystem::exists(out),"output already exists; use a fresh run directory");
    Fixture f(p);auto arms=atlas(f,n,seed);auto scalars=target_scalars(f.ell,count,seed);
    std::vector<Point> train,hold;
    for(std::size_t i=0;i<count;++i) {train.push_back(f.e.mul(scalars[i],f.g));hold.push_back(f.e.mul(scalars[count+i],f.g));}
    std::filesystem::create_directories(out);std::ofstream rows(out/"generic.csv");
    rows<<"arm,p,group_order,ell,columns,panel,targets,pair_probes,hits,coefficient_rank\n";
    std::size_t selected=0;Stats best;
    for(std::size_t k=0;k<arms.size();++k) {
        const auto& arm=arms[k];auto tr=train,ho=hold;
        if(arm.transported) {for(auto& t:tr)t=f.e.phi(t);for(auto& t:ho)t=f.e.phi(t);}
        std::ofstream train_rows(out/(arm.name+"-train-relations.csv")),hold_rows(out/(arm.name+"-holdout-relations.csv"));
        auto s=collect(arm.model,f.ell,arm.base,tr,&train_rows),h=collect(arm.model,f.ell,arm.base,ho,&hold_rows);
        // Selection is on training data only; deterministic tie order. Diagnostic
        // utility = coefficient-rank increments per counted pair probe.
        if(k==0 || s.new_rank*best.probes>best.new_rank*s.probes) {selected=k;best=s;}
        for(auto pair:{std::pair{"train",s},std::pair{"holdout",h}})
            rows<<arm.name<<','<<p<<','<<f.order<<','<<f.ell<<','<<n<<','<<pair.first<<','<<count<<','<<pair.second.probes<<','<<pair.second.hits<<','<<pair.second.new_rank<<'\n';
        for(std::size_t t=0;t<ho.size();++t) for(bool carries:{false,true}) {
            std::ofstream o(out/(arm.name+"-holdout-"+std::to_string(t)+(carries?"-carry.smt2":"-mod.smt2")));
            SMT smt(o,p,carries,4);smt.query(arm,ho[t]);
        }
        std::ofstream base(out/(arm.name+"-base.csv"));base<<"column,source_x,source_y,model_x,model_y\n";
        for(std::size_t i=0;i<n;++i)base<<i<<','<<arm.source[i].x<<','<<arm.source[i].y<<','<<arm.base[i].x<<','<<arm.base[i].y<<'\n';
    }
    std::ofstream selected_file(out/"selection.txt");selected_file<<"training_only="<<arms[selected].name<<"\ncriterion=coefficient_rank/pair_probes\nclassification=generic_decomposition_diagnostic\n";
    std::ofstream targets(out/"targets.csv");targets<<"panel,index,known_answer_scalar,x,y\n";
    for(std::size_t i=0;i<2*count;++i) {auto t=f.e.mul(scalars[i],f.g);targets<<(i<count?"train":"holdout")<<','<<i%count<<','<<scalars[i]<<','<<t.x<<','<<t.y<<'\n';}
}
int main(int argc,char** argv) {
    try {
        if(argc==2 && std::string(argv[1])=="--self-test") {self_test();return 0;}
        if(argc==3 && std::string(argv[1])=="--encoding-fixtures") {encoding_fixtures(argv[2]);return 0;}
        if(argc==3 && std::string(argv[1])=="--product-fixtures") {product_fixtures(argv[2]);return 0;}
        if(argc==11 && std::string(argv[1])=="--replay") {
            replay_run(std::stoll(argv[2]),std::stoll(argv[3]),std::stoll(argv[4]),std::stoll(argv[5]),argv[6],
                       {std::stoll(argv[7]),std::stoll(argv[8]),std::stoi(argv[9])!=0},argv[10]);return 0;
        }
        if(argc==8 && std::string(argv[1])=="--replay-fixture") {
            replay_fixture(argv[2],{std::stoll(argv[3]),std::stoll(argv[4]),std::stoi(argv[5])!=0},argv[6]);
            require(std::string(argv[7])=="development-only","fixture replay scope");return 0;
        }
        if(argc==7 && std::string(argv[1])=="--run") {
            run(std::stoll(argv[2]),std::stoull(argv[3]),std::stoull(argv[4]),std::stoull(argv[5]),argv[6]);return 0;
        }
        std::cerr<<"usage: study --self-test | --run PRIME COLUMNS TARGETS_PER_PANEL SEED FRESH_OUTPUT\n";return 2;
    } catch(const std::exception& e) {std::cerr<<e.what()<<'\n';return 1;}
}
