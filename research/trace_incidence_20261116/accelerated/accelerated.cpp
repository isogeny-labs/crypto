#include "accelerated.hpp"
#include <flint/nmod_poly.h>
#include <flint/ulong_extras.h>
#include <algorithm>
#include <random>
#include <set>
#include <stdexcept>
#include <unordered_map>

namespace paper_ec::accelerated {
namespace {
struct CPoint {Word x,y,infinity;};
struct CLabel {CPoint point;Word alpha,beta;};
struct Witness {Word informative,i,j,k;std::int64_t second_sign,third_sign;Word scalar;};
extern "C" {
void* ti_context_new(Word,Word,Word,Word,CPoint,CPoint,Word);
void ti_context_free(void*);
int ti_combine(void*,const Word*,std::size_t,CPoint*,Word);
int ti_support(void*,const CLabel*,std::size_t,const CLabel*,std::size_t,const CLabel*,std::size_t,unsigned char*,Witness*,Word,Word);
int ti_counts(const void*,FastCounts*);
}
CPoint encode(Point p) {return {p.x,p.y,p.infinity?Word(1):Word(0)};}
Point decode(CPoint p) {return {p.x,p.y,p.infinity!=0};}
void require(bool b,const char* s) {if(!b)throw std::invalid_argument(s);}
Word neg(Word a,Word p) {return a?p-a:0;}
Word mul(Word a,Word b,Word p) {return Word((__uint128_t(a)*b)%p);}
void count(Stats* s,const std::string& k,Word n=1) {if(s)s->add(k,n);}
struct Poly {
    nmod_poly_t n;
    explicit Poly(Word p) {nmod_poly_init(n,p);}
    Poly(Word p,const Coefficients& c):Poly(p) {for(std::size_t i=0;i<c.size();++i)nmod_poly_set_coeff_ui(n,i,c[i]);}
    Poly(const Poly& b):Poly(b.n->mod.n) {nmod_poly_set(n,b.n);}
    Poly(Poly&& b) noexcept:Poly(b.n->mod.n) {nmod_poly_swap(n,b.n);}
    Poly& operator=(Poly&& b) noexcept {nmod_poly_swap(n,b.n);return *this;}
    ~Poly(){nmod_poly_clear(n);}
    Word p() const{return n->mod.n;}
    slong degree() const{return nmod_poly_degree(n);}
    Coefficients coefficients() const {Coefficients c(nmod_poly_length(n));for(std::size_t i=0;i<c.size();++i)c[i]=nmod_poly_get_coeff_ui(n,i);return c;}
};
Poly product_roots(Word p,const std::vector<Word>& roots,std::size_t a,std::size_t b) {
    if(a==b)return Poly(p,{1});
    if(b-a==1)return Poly(p,{neg(roots[a],p),1});
    const auto m=a+(b-a)/2;auto l=product_roots(p,roots,a,m),r=product_roots(p,roots,m,b);Poly out(p);nmod_poly_mul(out.n,l.n,r.n);return out;
}
std::vector<Word> xs(const std::vector<LabelledPoint>& pool) {std::vector<Word> out;out.reserve(pool.size());for(const auto& l:pool)out.push_back(l.point.x);return out;}
Poly add(const Poly& a,const Poly& b) {Poly c(a.p());nmod_poly_add(c.n,a.n,b.n);return c;}
Poly sub(const Poly& a,const Poly& b) {Poly c(a.p());nmod_poly_sub(c.n,a.n,b.n);return c;}
Poly scale(const Poly& a,Word v) {Poly c(a.p());nmod_poly_scalar_mul_nmod(c.n,a.n,v);return c;}
struct Ring {
    const Poly& modulus;Poly preinverse;Stats* stats;
    Ring(const Poly& f,Stats* s):modulus(f),preinverse(f.p()),stats(s) {
        Poly reversed(f.p());nmod_poly_reverse(reversed.n,f.n,f.degree()+1);
        nmod_poly_inv_series(preinverse.n,reversed.n,f.degree()+1);count(stats,"accelerated.modulus_preinverses");
    }
    Poly reduce(const Poly& a) const {Poly out(a.p());nmod_poly_rem(out.n,a.n,modulus.n);return out;}
    Poly times(const Poly& a,const Poly& b) const {
        Poly out(a.p());nmod_poly_mulmod_preinv(out.n,a.n,b.n,modulus.n,preinverse.n);
        count(stats,"accelerated.quotient_products");return out;
    }
    // Brent–Kung requires deg(outer)<deg(modulus). F2 is monic of exactly
    // that degree on equal-sized pools. Split coefficient blocks explicitly;
    // calling generic compose_mod here silently selects quadratic Horner.
    Poly compose(const Poly& outer,const Poly& t) const {
        const auto d=modulus.degree();require(d>0,"constant incidence modulus");
        if(d==1) {
            Poly out(t.p(),{nmod_poly_evaluate_nmod(outer.n,nmod_poly_get_coeff_ui(t.n,0))});
            count(stats,"accelerated.scalar_compositions");return out;
        }
        // Tail shortcut avoids a power t^d on the usual equal-degree case.
        if(outer.degree()==d) {
            Poly tail(t.p()),out(t.p());nmod_poly_shift_right(tail.n,outer.n,1);
            nmod_poly_compose_mod_brent_kung_preinv(out.n,tail.n,t.n,modulus.n,preinverse.n);
            count(stats,"accelerated.brent_kung_compositions");
            return add(times(out,t),Poly(t.p(),{nmod_poly_get_coeff_ui(outer.n,0)}));
        }
        if(outer.degree()<d) {
            Poly out(t.p());nmod_poly_compose_mod_brent_kung_preinv(out.n,outer.n,t.n,modulus.n,preinverse.n);
            count(stats,"accelerated.brent_kung_compositions");return out;
        }
        // Unequal pools: compose degree<d blocks, Horner in t^d. This avoids
        // invalid FLINT calls without claiming the equal-size bound here.
        Poly td(t.p(),{1}),base(t);auto exponent=d;
        while(exponent) {if(exponent&1)td=times(td,base);exponent>>=1;if(exponent)base=times(base,base);}
        Poly out(t.p());
        for(slong block=outer.degree()/d;block>=0;--block) {
            Poly part(t.p()),value(t.p());
            for(slong j=0;j<d;++j)nmod_poly_set_coeff_ui(part.n,j,nmod_poly_get_coeff_ui(outer.n,block*d+j));
            nmod_poly_compose_mod_brent_kung_preinv(value.n,part.n,t.n,modulus.n,preinverse.n);
            count(stats,"accelerated.brent_kung_compositions");out=add(times(out,td),value);
        }
        return out;
    }
};
void public_validate(const LabelledPools& p) {
    const auto& c=p.target;const auto r=p.subgroup_order;
    require(c.p>3&&c.p<(Word(1)<<62)&&r>3&&r<(Word(1)<<62)&&n_is_prime(c.p)&&n_is_prime(r),"invalid accelerated prime domain");
    require(c.a<c.p&&c.b<c.p,"noncanonical curve");
    require((4*__uint128_t(mul(mul(c.a,c.a,c.p),c.a,c.p))+27*__uint128_t(mul(c.b,c.b,c.p)))%c.p!=0,"singular curve");
    require(!p.generator.infinity&&!p.public_target.infinity&&p.generator.x<c.p&&p.generator.y<c.p&&p.public_target.x<c.p&&p.public_target.y<c.p,"invalid public coordinates");
    require(on_curve(c,p.generator)&&on_curve(c,p.public_target),"invalid public points");
}
} // namespace

FastContext::FastContext(const LabelledPools& p,bool prepare):curve_(p.target),order_(p.subgroup_order),generator_(p.generator),target_(p.public_target) {
    public_validate(p);
    context_=ti_context_new(p.target.p,p.target.a,p.target.b,p.subgroup_order,encode(p.generator),encode(p.public_target),prepare);
    require(context_!=nullptr,"invalid public subgroup context");
}
FastContext::~FastContext() {ti_context_free(context_);}
void FastContext::require_matching(const LabelledPools& p) const {
    require(curve_.p==p.target.p&&curve_.a==p.target.a&&curve_.b==p.target.b&&order_==p.subgroup_order&&generator_==p.generator&&target_==p.public_target,"fast context belongs to another target");
}
FastCounts FastContext::counts() const {FastCounts c;require(ti_counts(context_,&c)==0,"counts ABI failed");return c;}
std::vector<Point> FastContext::combine(const std::vector<std::array<Word,2>>& labels,bool batch) {
    if(labels.empty())return {};
    static_assert(sizeof(std::array<Word,2>)==2*sizeof(Word));
    std::vector<CPoint> raw(labels.size());
    require(ti_combine(context_,labels.front().data(),labels.size(),raw.data(),batch)==0,"combine ABI failed");
    std::vector<Point> out;out.reserve(raw.size());for(auto p:raw)out.push_back(decode(p));return out;
}
LabelledPools generate(LabelledPools out,std::size_t B,Word seed,Generator method,GenerationCounts& counts,FastContext* fast) {
    public_validate(out);require(B>0&&B<=65536&&6*__uint128_t(B)<out.subgroup_order,"pool capacity");
    require(method==Generator::LegacyAffine||fast,"missing fast generator context");
    if(fast)fast->require_matching(out);
    if(method==Generator::LegacyAffine)require(point_mul(out.target,out.subgroup_order,out.generator).infinity&&point_mul(out.target,out.subgroup_order,out.public_target).infinity,"invalid generation subgroup");
    for(auto& p:out.pools)p.clear();
    counts={};
    std::mt19937_64 random(seed);std::uniform_int_distribution<Word> draw(0,out.subgroup_order-1);std::set<Word> used;
    for(auto& pool:out.pools)while(pool.size()<B) {
        // Draw exactly the remaining deficit, so rejection order and total
        // RNG consumption match the original scalar implementation exactly.
        const auto deficit=B-pool.size();std::vector<std::array<Word,2>> labels;labels.reserve(deficit);
        for(std::size_t i=0;i<deficit;++i){require(++counts.draws<1000000,"generation draw limit");labels.push_back({draw(random),draw(random)});}
        std::vector<Point> points;
        if(method==Generator::LegacyAffine) {
            for(const auto& l:labels)points.push_back(point_add(out.target,point_mul(out.target,l[0],out.generator),point_mul(out.target,l[1],out.public_target)));
        } else points=fast->combine(labels,method==Generator::BatchedWindow);
        for(std::size_t i=0;i<points.size();++i) {
            const auto p=points[i];if(p.infinity){++counts.rejected_infinity;continue;}
            if(!used.insert(p.x).second){++counts.rejected_coordinate;continue;}
            pool.push_back({p,labels[i][0],labels[i][1]});
        }
    }
    return out;
}
void validate(const LabelledPools& p) {
    public_validate(p);
    require(point_mul(p.target,p.subgroup_order,p.generator).infinity&&point_mul(p.target,p.subgroup_order,p.public_target).infinity,"invalid public subgroup");
    std::set<Word> seen;
    for(const auto& pool:p.pools) {
        require(!pool.empty(),"empty labelled pool");
        for(const auto& l:pool)require(!l.point.infinity&&l.point.x<p.target.p&&l.point.y<p.target.p&&on_curve(p.target,l.point)&&l.alpha<p.subgroup_order&&l.beta<p.subgroup_order&&seen.insert(l.point.x).second,"invalid or duplicate pool entry");
    }
}
Coefficients composition_gcd(const LabelledPools& p,Stats* stats) {
    validate(p);const auto prime=p.target.p;
    const auto second=xs(p.pools[1]),third=xs(p.pools[2]),first=xs(p.pools[0]);
    const auto f2=product_roots(prime,second,0,second.size()),f3=product_roots(prime,third,0,third.size());
    Ring ring(f3,stats);Poly ordinate(prime),z=ring.reduce(Poly(prime,{0,1}));
    std::vector<Word> ys;for(const auto& l:p.pools[2])ys.push_back(l.point.y);
    nmod_poly_interpolate_nmod_vec_fast(ordinate.n,third.data(),ys.data(),third.size());
    std::vector<Word> values(first.size()),prefix(first.size());
    nmod_poly_evaluate_nmod_vec_fast(values.data(),f3.n,first.data(),first.size());
    Word acc=1;
    for(std::size_t i=0;i<values.size();++i){require(values[i]!=0,"nonunit denominator");prefix[i]=acc;acc=mul(acc,values[i],prime);}
    acc=n_invmod(acc,prime);count(stats,"accelerated.denominator_scalar_inversions");
    for(std::size_t i=values.size();i-->0;){const auto value=values[i];values[i]=mul(acc,prefix[i],prime);acc=mul(acc,value,prime);}
    const auto h=ring.reduce(Poly(prime,{p.target.b,p.target.a,0,1}));Poly accumulated=ring.reduce(Poly(prime,{1}));
    for(std::size_t i=0;i<first.size();++i) {
        const auto x=first[i],y=p.pools[0][i].point.y;const auto d=f3.degree();Poly inverse(prime);
        // Synthetic quotient (F3(Z)-F3(x))/(Z-x), leading coefficient1.
        Word q=1;nmod_poly_set_coeff_ui(inverse.n,d-1,q);
        for(slong j=d-1;j>0;--j){q=(__uint128_t(mul(x,q,prime))+nmod_poly_get_coeff_ui(f3.n,j))%prime;nmod_poly_set_coeff_ui(inverse.n,j-1,q);}
        inverse=scale(inverse,neg(values[i],prime));
        const auto inv2=ring.times(inverse,inverse);
        const auto base=sub(sub(ring.times(add(h,Poly(prime,{mul(y,y,prime)})),inv2),Poly(prime,{x})),z);
        const auto cross=scale(ring.times(ordinate,inv2),mul(2,y,prime));
        const auto plus=ring.compose(f2,add(base,cross)),minus=ring.compose(f2,sub(base,cross));
        accumulated=ring.times(accumulated,ring.times(plus,minus));count(stats,"accelerated.first_pool_iterations");
    }
    Poly g(prime);nmod_poly_gcd(g.n,f3.n,accumulated.n);count(stats,"accelerated.final_gcds");return g.coefficients();
}
IncidenceAnswer recover(const LabelledPools& p,const Coefficients& gcd,Stats* stats) {
    require(!gcd.empty(),"zero incidence GCD");const auto prime=p.target.p,r=p.subgroup_order;Poly g(prime,gcd);
    IncidenceAnswer out;out.incidence_gcd=gcd;out.coordinate_relation=g.degree()>0;if(!out.coordinate_relation)return out;
    std::unordered_map<Word,std::size_t> index;for(std::size_t j=0;j<p.pools[1].size();++j)index[p.pools[1][j].point.x]=j;
    std::vector<std::size_t> roots;for(std::size_t k=0;k<p.pools[2].size();++k)if(nmod_poly_evaluate_nmod(g.n,p.pools[2][k].point.x)==0)roots.push_back(k);
    std::sort(roots.begin(),roots.end(),[&](auto a,auto b){return p.pools[2][a].point.x<p.pools[2][b].point.x;});
    require(roots.size()==std::size_t(g.degree()),"GCD not split squarefree in third pool");
    for(const auto k:roots)for(std::size_t i=0;i<p.pools[0].size();++i)for(const int s1:{1,-1}) {
        const auto &a=p.pools[0][i],&c=p.pools[2][k];const auto pa=s1==1?a.point:point_neg(p.target,a.point);
        const auto needed=point_neg(p.target,point_add(p.target,pa,c.point,stats));count(stats,"accelerated.witness_queries");
        if(needed.infinity)continue;
        const auto it=index.find(needed.x);if(it==index.end())continue;const auto j=it->second;const auto& b=p.pools[1][j];
        const int s2=needed.y==b.point.y?1:needed.y==neg(b.point.y,prime)?-1:0;if(!s2)continue;
        const auto pb=s2==1?b.point:point_neg(p.target,b.point);
        require(point_add(p.target,point_add(p.target,pa,pb,stats),c.point,stats).infinity,"bad geometric witness");
        const Word A=(__uint128_t(s1==1?a.alpha:neg(a.alpha,r))+(s2==1?b.alpha:neg(b.alpha,r))+c.alpha)%r;
        const Word B=(__uint128_t(s1==1?a.beta:neg(a.beta,r))+(s2==1?b.beta:neg(b.beta,r))+c.beta)%r;
        if(!B){require(!A,"inconsistent uninformative labels");count(stats,"accelerated.uninformative_witnesses");continue;}
        out.recovered_scalar=mul(neg(A,r),n_invmod(B,r),r);
        require(point_mul(p.target,out.recovered_scalar,p.generator,stats)==p.public_target,"bad public scalar recovery");
        out.informative_relation=true;out.positions={i,j,k};out.signs={s1,s2,1};return out;
    }
    return out;
}
IncidenceAnswer FastContext::pairs(const LabelledPools& p,bool bloom) {return pair_search(p,bloom,true);}
IncidenceAnswer FastContext::first_witness(const LabelledPools& p,bool bloom) {return pair_search(p,bloom,false);}
IncidenceAnswer FastContext::pair_search(const LabelledPools& p,bool bloom,bool complete) {
    require_matching(p);validate(p);std::array<std::vector<CLabel>,3> raw;
    for(std::size_t j=0;j<3;++j)for(const auto& l:p.pools[j])raw[j].push_back({encode(l.point),l.alpha,l.beta});
    std::vector<unsigned char> hits(raw[2].size());Witness w{};
    require(ti_support(context_,raw[0].data(),raw[0].size(),raw[1].data(),raw[1].size(),raw[2].data(),raw[2].size(),hits.data(),&w,bloom,complete)==0,"pair support or label check failed");
    std::vector<Word> roots;for(std::size_t k=0;k<hits.size();++k)if(hits[k])roots.push_back(p.pools[2][k].point.x);
    IncidenceAnswer out;if(complete)out.incidence_gcd=product_roots(p.target.p,roots,0,roots.size()).coefficients();out.coordinate_relation=!roots.empty();
    out.informative_relation=w.informative!=0;
    if(out.informative_relation){out.positions={std::size_t(w.i),std::size_t(w.j),std::size_t(w.k)};out.signs={1,int(w.second_sign),int(w.third_sign)};out.recovered_scalar=w.scalar;}
    return out;
}
IncidenceAnswer solve(const LabelledPools& p,Norm norm,FastContext* ctx,Stats* stats) {
    if(norm==Norm::Composition)return recover(p,composition_gcd(p,stats),stats);
    require(ctx,"missing pair context");return ctx->pairs(p,norm==Norm::StreamedPairsBloom);
}
} // namespace paper_ec::accelerated
