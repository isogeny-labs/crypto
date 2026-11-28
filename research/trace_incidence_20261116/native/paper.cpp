#include "paper.hpp"
// Supplied primary source: arXiv:2610.32508, Definition 1.2, Algorithms
// 1/2/4/5/6 and Remark 5.4. Datum construction is separate from the online
// algorithm; see sections 6.1 and 8.4 for the unresolved general construction.
// CRT uses a balanced tree; exceptional known representatives are peeled in
// one pass. These preserve the paper's exact outputs without root factoring.
#include <flint/nmod_poly.h>
#include <flint/ulong_extras.h>
#include <algorithm>
#include <limits>
#include <numeric>
#include <set>
#include <stdexcept>
#include <utility>

namespace paper_ec {
namespace {
Word addw(Word a, Word b, Word p) { return static_cast<Word>((static_cast<__uint128_t>(a)+b)%p); }
Word subw(Word a, Word b, Word p) { return a>=b ? a-b : p-(b-a); }
Word mulw(Word a, Word b, Word p) { return static_cast<Word>(static_cast<__uint128_t>(a)*b%p); }
Word poww(Word a, Word e, Word p) {
    Word r=1;
    while(e) { if(e&1) r=mulw(r,a,p); a=mulw(a,a,p); e>>=1; }
    return r;
}
Word invw(Word a,Word p) { if(!a) throw std::invalid_argument("zero scalar inverse"); return poww(a,p-2,p); }
Word exact_power(Word a,unsigned e) {
    Word r=1;
    for(unsigned i=0;i<e;++i) {
        if(a && r>std::numeric_limits<Word>::max()/a) throw std::invalid_argument("integer power overflow");
        r*=a;
    }
    return r;
}
unsigned valuation(Word n,Word ell) { unsigned v=0; while(n && n%ell==0) {n/=ell;++v;} return v; }
void count(Stats* s,const char* key,Word n=1) { if(s) s->add(key,n); }

struct Poly {
    nmod_poly_t value;
    Word p;
    explicit Poly(Word prime):p(prime) { nmod_poly_init(value,p); }
    Poly(Word prime,const Coefficients& c):Poly(prime) {
        for(std::size_t i=0;i<c.size();++i) nmod_poly_set_coeff_ui(value,static_cast<slong>(i),c[i]%p);
    }
    Poly(const Poly& a):Poly(a.p) { nmod_poly_set(value,a.value); }
    Poly(Poly&& a) noexcept:Poly(a.p) { nmod_poly_swap(value,a.value); }
    Poly& operator=(const Poly& a) { if(this!=&a) { if(p!=a.p) throw std::logic_error("poly field mismatch"); nmod_poly_set(value,a.value); } return *this; }
    Poly& operator=(Poly&& a) noexcept { nmod_poly_swap(value,a.value); return *this; }
    ~Poly() { nmod_poly_clear(value); }
    slong degree() const { return nmod_poly_degree(value); }
    Word at(Word x) const { return nmod_poly_evaluate_nmod(value,x); }
    Coefficients coefficients() const {
        Coefficients c(static_cast<std::size_t>(std::max<slong>(0,degree()+1)));
        for(std::size_t i=0;i<c.size();++i) c[i]=nmod_poly_get_coeff_ui(value,static_cast<slong>(i));
        return c;
    }
};
Poly constant(Word p,Word a) { return Poly(p,Coefficients{a%p}); }
Poly xpoly(Word p) { return Poly(p,Coefficients{0,1}); }
Poly plus(const Poly&a,const Poly&b) { Poly c(a.p);nmod_poly_add(c.value,a.value,b.value);return c; }
Poly minus(const Poly&a,const Poly&b) { Poly c(a.p);nmod_poly_sub(c.value,a.value,b.value);return c; }
Poly scale(const Poly&a,Word b) { Poly c(a.p);nmod_poly_scalar_mul_nmod(c.value,a.value,b%a.p);return c; }
Poly product(const Poly&a,const Poly&b) { Poly c(a.p);nmod_poly_mul(c.value,a.value,b.value);return c; }
Poly remainder(const Poly&a,const Poly&mod) { Poly c(a.p);nmod_poly_rem(c.value,a.value,mod.value);return c; }
Poly gcd(const Poly&a,const Poly&b,Stats*s) { Poly c(a.p);nmod_poly_gcd(c.value,a.value,b.value);count(s,"paper.polynomial_gcd");return c; }
Poly quotient_exact(const Poly&a,const Poly&b) {
    Poly q(a.p),r(a.p);nmod_poly_divrem(q.value,r.value,a.value,b.value);
    if(r.degree()>=0) throw std::logic_error("inexact polynomial division");
    return q;
}
struct Ring {
    const Poly& mod; Word p; Stats* stats;
    Ring(const Poly& u,Stats*s):mod(u),p(u.p),stats(s) { if(u.degree()<1) throw std::invalid_argument("quotient modulus must be nonconstant"); }
    Poly reduce(const Poly&a) const { return remainder(a,mod); }
    Poly c(Word a) const { return constant(p,a); }
    Poly mul(const Poly&a,const Poly&b) const {
        Poly c(p);nmod_poly_mulmod(c.value,a.value,b.value,mod.value);
        count(stats,"paper.quotient_multiplications");return c;
    }
    Poly power(const Poly&a,Word n) const {
        Poly c(p),base=reduce(a);nmod_poly_powmod_ui_binexp(c.value,base.value,n,mod.value);
        count(stats,"paper.quotient_power_calls");return c;
    }
    Poly inverse(const Poly&a) const {
        Poly c(p);if(!nmod_poly_invmod(c.value,a.value,mod.value)) throw std::invalid_argument("nonunit quotient inverse");
        count(stats,"paper.quotient_inversions");return c;
    }
};
Poly curve_polynomial(const Curve& e) { return Poly(e.p,Coefficients{e.b,e.a,0,1}); }
bool poly_equal(const Poly&a,const Poly&b) { return nmod_poly_equal(a.value,b.value)!=0; }

void check_factorization(Word n,const std::vector<PrimePower>& fs,Stats*s) {
    Word product_n=1;std::set<Word> seen;
    for(const auto& f:fs) {
        count(s,"datum.primality_checks");
        if(f.prime<2 || !f.exponent || !n_is_prime(f.prime) || !seen.insert(f.prime).second)
            throw std::invalid_argument("invalid prime factorization");
        Word pp=exact_power(f.prime,f.exponent);
        if(product_n>n/pp) throw std::invalid_argument("factorization product mismatch");
        product_n*=pp;
    }
    if(product_n!=n) throw std::invalid_argument("factorization product mismatch");
}
void check_basis(const Curve&e,Word n,const std::vector<PrimePower>& fs,const GroupBasis& b,Stats*s) {
    if(!b.first_order || !b.second_order || static_cast<__uint128_t>(b.first_order)*b.second_order!=n)
        throw std::invalid_argument("basis order product mismatch");
    const Point points[2]={b.first,b.second};const Word orders[2]={b.first_order,b.second_order};
    for(unsigned i=0;i<2;++i) {
        if(!on_curve(e,points[i]) || n%orders[i] || !point_mul(e,orders[i],points[i],s).infinity)
            throw std::invalid_argument("invalid basis point/order");
        for(const auto& f:fs) if(orders[i]%f.prime==0 && point_mul(e,orders[i]/f.prime,points[i],s).infinity)
            throw std::invalid_argument("basis order is not exact");
    }
    // A nontrivial intersection contains a common point of prime order.
    for(const auto& f:fs) if(b.first_order%f.prime==0 && b.second_order%f.prime==0) {
        Point t1=point_mul(e,b.first_order/f.prime,b.first,s),t2=point_mul(e,b.second_order/f.prime,b.second,s),q;
        for(Word j=0;j<f.prime;++j) {
            count(s,"datum.independence_tests");
            if(q==t2) throw std::invalid_argument("basis cyclic subgroups intersect");
            q=point_add(e,q,t1,s);
        }
    }
}

using Pieces=std::vector<std::pair<Word,Poly>>;
Pieces twoadic(const Poly&h,const Poly&f,Word beta,unsigned m,unsigned full,Stats*s) {
    count(s,"lift.twoadic_calls");
    if(f.degree()==0) return {};
    if(!m) { Pieces r;r.emplace_back(0,f);return r; }
    Ring ring(f,s);Word exponent=(f.p-1)/exact_power(2,full-m+1);
    Poly g=ring.power(h,exponent);Pieces result;
    for(Word i=0;i<2;++i) {
        Poly fi=gcd(minus(g,constant(f.p,i?f.p-1:1)),f,s);
        if(fi.degree()==0) continue;
        Poly hi=remainder(h,fi);if(i) hi=scale(hi,invw(beta,f.p));
        Pieces child=twoadic(hi,fi,mulw(beta,beta,f.p),m-1,full,s);
        for(auto& item:child) result.emplace_back(2*item.first+i,std::move(item.second));
    }
    return result;
}
std::pair<Poly,Poly> crt_embedding(const Pieces& pieces,const Poly&h,Word beta,Word gamma,unsigned parity,Stats*s) {
    Word p=h.p;
    std::vector<std::pair<Poly,Poly>> nodes;
    for(const auto& entry:pieces) if((entry.first&1)==parity) {
        Word j=entry.first;const Poly&f=entry.second;Ring ring(f,s);
        Word beta_inv_j=poww(invw(beta,p),j,p);
        Poly local=scale(ring.power(scale(remainder(h,f),beta_inv_j),gamma),poww(beta,j/2,p));
        nodes.emplace_back(f,std::move(local));count(s,"lift.crt_terms");
    }
    if(nodes.empty())return {constant(p,1),Poly(p)};
    // Balanced product/CRT tree: never divide a full-degree total modulus
    // separately for every leaf. Each level has total degree at most deg(f).
    while(nodes.size()>1) {
        std::vector<std::pair<Poly,Poly>> next;
        for(std::size_t i=0;i<nodes.size();i+=2) {
            if(i+1==nodes.size()) { next.push_back(std::move(nodes[i]));continue; }
            const auto& left=nodes[i];const auto& right=nodes[i+1];
            Ring ring(right.first,s);
            Poly correction=ring.mul(ring.reduce(minus(right.second,left.second)),
                ring.inverse(ring.reduce(left.first)));
            Poly value=plus(left.second,product(left.first,correction));
            Poly modulus=product(left.first,right.first);
            next.emplace_back(std::move(modulus),std::move(value));
            count(s,"lift.crt_merges");
        }
        nodes=std::move(next);
    }
    return std::move(nodes.front());
}

class DivisionRecurrence {
    const Curve& e;const Ring& ring;Poly x;Poly w4;
    std::map<Word,Poly> memo;
public:
    DivisionRecurrence(const Curve&curve,const Ring&r,const Poly&px):e(curve),ring(r),x(r.reduce(px)),w4(e.p) {
        Poly x2=ring.mul(x,x),x3=ring.mul(x2,x),x4=ring.mul(x2,x2),x6=ring.mul(x3,x3);
        Poly h=plus(plus(x3,scale(x,e.a)),ring.c(e.b));
        Poly w=scale(h,4);w4=ring.mul(w,w);
        memo.emplace(0,ring.c(0));memo.emplace(1,ring.c(1));memo.emplace(2,ring.c(1));
        Poly a2=ring.c(mulw(e.a,e.a,e.p));
        Poly psi3=minus(plus(plus(scale(x4,3),scale(x2,mulw(6,e.a,e.p))),scale(x,mulw(12,e.b,e.p))),a2);
        memo.emplace(3,ring.reduce(psi3));
        Word a3=mulw(mulw(e.a,e.a,e.p),e.a,e.p),b2=mulw(e.b,e.b,e.p);
        Poly inside=plus(plus(x6,scale(x4,mulw(5,e.a,e.p))),scale(x3,mulw(20,e.b,e.p)));
        inside=minus(inside,scale(x2,mulw(5,mulw(e.a,e.a,e.p),e.p)));
        inside=minus(inside,scale(x,mulw(4,mulw(e.a,e.b,e.p),e.p)));
        inside=minus(inside,ring.c(addw(mulw(8,b2,e.p),a3,e.p)));
        memo.emplace(4,ring.reduce(scale(inside,2)));
    }
    const Poly& normalized(Word n) {
        auto hit=memo.find(n);if(hit!=memo.end()) return hit->second;
        Word k=n/2;Poly out(e.p);
        if(n&1) {
            const Poly &a=normalized(k+2),&b=normalized(k),&c=normalized(k-1),&d=normalized(k+1);
            Poly t1=ring.mul(a,ring.mul(ring.mul(b,b),b));
            Poly t2=ring.mul(c,ring.mul(ring.mul(d,d),d));
            if(k&1) t2=ring.mul(w4,t2);else t1=ring.mul(w4,t1);
            out=minus(t1,t2);
        } else {
            const Poly &a=normalized(k),&b=normalized(k+2),&c=normalized(k-1),&d=normalized(k-2),&f=normalized(k+1);
            out=ring.mul(a,minus(ring.mul(b,ring.mul(c,c)),ring.mul(d,ring.mul(f,f))));
        }
        count(ring.stats,"division.normalized_states");
        return memo.emplace(n,ring.reduce(out)).first->second;
    }
};
Poly division_at(const Curve&e,Word n,const Poly&x,const Poly&y,const Poly&u,Stats*s) {
    Ring ring(u,s);DivisionRecurrence recurrence(e,ring,x);Poly out=recurrence.normalized(n);
    if(!(n&1)) out=ring.mul(scale(ring.reduce(y),2),out);
    count(s,"division.evaluations");return out;
}

Word divisor_degree(const Divisor& d) {
    Word n=d.infinity?1:0;
    for(const auto& c:d.components) if(!c.u.empty()) n+=c.u.size()-1;
    return n;
}
void append_divisor(Divisor& target,const Divisor& add) {
    if(target.infinity && add.infinity) throw std::logic_error("duplicate divisor infinity");
    target.infinity=target.infinity||add.infinity;
    target.components.insert(target.components.end(),add.components.begin(),add.components.end());
}
Divisor singleton(Word p,Point q) {
    Divisor d;if(q.infinity)d.infinity=true;
    else d.components.push_back({{q.x?p-q.x:0,1},{q.y}});
    return d;
}
bool remove_point(Divisor& d,const Curve&e,Point q) {
    if(q.infinity) { if(!d.infinity)return false;d.infinity=false;return true; }
    for(auto it=d.components.begin();it!=d.components.end();++it) {
        Poly u(e.p,it->u),v(e.p,it->v);
        if(u.at(q.x)!=0 || v.at(q.x)!=q.y)continue;
        Poly reduced=quotient_exact(u,Poly(e.p,Coefficients{q.x?e.p-q.x:0,1}));
        if(reduced.degree()==0)d.components.erase(it);
        else {it->u=reduced.coefficients();it->v=remainder(v,reduced).coefficients();}
        return true;
    }
    return false;
}
using Classes=std::map<Point,Divisor>;
std::vector<Point> representatives(const Curve&e,Word ell,unsigned m,const GroupBasis&b,Stats*s) {
    unsigned nu1=valuation(b.first_order,ell),nu2=valuation(b.second_order,ell);
    Word n1=nu1>=m?ell:1,n2=nu2>=m?ell:1;
    Point t1,t2;
    if(nu1>=m)t1=point_mul(e,exact_power(ell,nu1-m),b.first,s);
    if(nu2>=m)t2=point_mul(e,exact_power(ell,nu2-m),b.second,s);
    std::vector<Point> result;Point q1;
    for(Word i=0;i<n1;++i) {
        Point q=q1;
        for(Word j=0;j<n2;++j) { result.push_back(q);q=point_add(e,q,t2,s);count(s,"primary.representatives"); }
        q1=point_add(e,q1,t1,s);
    }
    return result;
}
Classes at_level(const Curve&e,Word index,const std::vector<Point>&reps,const Divisor&d,Stats*s) {
    Classes result;
    if(d.infinity)throw std::logic_error("infinity escaped representative extraction");
    for(const auto&component:d.components) {
        Poly u(e.p,component.u),v(e.p,component.v),X=xpoly(e.p);Ring ring(u,s);
        for(Point q:reps) {
            Poly px=ring.reduce(X),py=ring.reduce(v);
            if(!q.infinity) {
                Poly slope=ring.mul(plus(v,ring.c(q.y)),ring.inverse(minus(X,ring.c(q.x))));
                px=ring.reduce(minus(minus(ring.mul(slope,slope),X),ring.c(q.x)));
                // For Q=-P the group formula subtracts yQ=-yP.
                py=plus(ring.c(q.y),ring.mul(slope,minus(ring.c(q.x),px)));
            }
            Poly torsion=division_at(e,index,px,py,u,s);
            Poly part=gcd(u,torsion,s);count(s,"primary.membership_tests");
            if(part.degree()>0)result[q].components.push_back({part.coefficients(),remainder(v,part).coefficients()});
        }
    }
    return result;
}
Classes primary(const Curve&e,Word ell,unsigned m,Word nell,const GroupBasis&b,const Divisor&d,
                Word beta,TranslationMethod method,Stats*s) {
    count(s,"primary.recursion_calls");
    if(!divisor_degree(d))return {};
    if(!m)return {{Point{},d}};
    std::vector<Point> reps=representatives(e,ell,m,b,s);
    Divisor remaining=d;std::vector<Point> exceptional;
    for(Point q:reps) for(unsigned sign=0;sign<2;++sign) {
        Point actual=sign?point_neg(e,q):q;
        if(remove_point(remaining,e,actual)) {
            count(s,"primary.exceptional_representatives");
            exceptional.push_back(actual);
        }
    }
    if(!exceptional.empty()) {
        // Algorithm 5's repeated one-point extraction flattened into one
        // pass over known representatives; no field/group scan is added.
        Classes result=primary(e,ell,m,nell,b,remaining,beta,method,s);
        for(Point actual:exceptional)append_divisor(result[actual],singleton(e.p,actual));
        return result;
    }
    Word index=nell*exact_power(ell,m-1);
    Classes level=at_level(e,index,reps,d,s),result;
    Word accounted=0;
    for(const auto& item:level) {
        Point q=item.first;const Divisor& part=item.second;accounted+=divisor_degree(part);
        Divisor shifted=translate(e,part,point_neg(e,q),method,beta,s);
        Classes lower=primary(e,ell,m-1,nell,b,shifted,beta,method,s);
        for(const auto& entry:lower) {
            Point key=point_add(e,q,entry.first,s);
            append_divisor(result[key],translate(e,entry.second,q,method,beta,s));
        }
    }
    if(accounted!=divisor_degree(d))throw std::logic_error("Sylow classes did not partition divisor");
    return result;
}
std::vector<Poly> separate_curve(const Curve&e,Word n,const std::vector<PrimePower>&fs,
                              const GroupBasis&basis,const Poly&u,const Poly&v,
                              Word beta,TranslationMethod method,Stats*s) {
    std::vector<Poly> factors;if(u.degree()>0)factors.push_back(u);
    for(const auto&f:fs) {
        Word nell=n/exact_power(f.prime,f.exponent);
        GroupBasis sylow{point_mul(e,nell,basis.first,s),point_mul(e,nell,basis.second,s),
            basis.first_order/std::gcd(basis.first_order,nell),basis.second_order/std::gcd(basis.second_order,nell)};
        unsigned m=std::max(valuation(basis.first_order,f.prime),valuation(basis.second_order,f.prime));
        std::vector<Poly> next;
        for(const Poly& factor:factors) {
            Divisor d;d.components.push_back({factor.coefficients(),remainder(v,factor).coefficients()});
            Classes classes=primary(e,f.prime,m,nell,sylow,d,beta,method,s);
            for(const auto& entry:classes) {
                if(entry.second.infinity)throw std::logic_error("original finite divisor acquired infinity");
                for(const auto& component:entry.second.components)if(component.u.size()>1) next.emplace_back(e.p,component.u);
            }
        }
        factors=std::move(next);count(s,"root.primary_prime_stages");
    }
    return factors;
}
} // namespace

void validate_datum(const Datum&d,Stats*s) {
    const Word p=d.curve.p;count(s,"datum.validation_calls");count(s,"datum.primality_checks");
    if(p<=3 || p>=(Word{1}<<63) || !n_is_prime(p) || d.twist.p!=p)throw std::invalid_argument("datum requires one prime field 3<p<2^63");
    if(d.curve.a>=p || d.curve.b>=p || d.twist.a>=p || d.twist.b>=p || !d.beta || d.beta>=p)
        throw std::invalid_argument("noncanonical datum field element");
    if(poww(d.beta,(p-1)/2,p)!=p-1)throw std::invalid_argument("datum beta is not a nonsquare");
    Word disc=addw(mulw(4,mulw(mulw(d.curve.a,d.curve.a,p),d.curve.a,p),p),mulw(27,mulw(d.curve.b,d.curve.b,p),p),p);
    if(!disc)throw std::invalid_argument("singular auxiliary curve");
    Word ib=invw(d.beta,p);
    if(d.twist.a!=mulw(d.curve.a,mulw(ib,ib,p),p) || d.twist.b!=mulw(d.curve.b,mulw(mulw(ib,ib,p),ib,p),p))
        throw std::invalid_argument("incorrect quadratic twist");
    if(static_cast<__uint128_t>(d.order)+d.twist_order!=2*(static_cast<__uint128_t>(p)+1))
        throw std::invalid_argument("curve/twist order sum mismatch");
    for(Word n:{d.order,d.twist_order}) {
        __int128 diff=static_cast<__int128>(n)-static_cast<__int128>(p)-1;
        if(diff*diff>4*static_cast<__int128>(p))throw std::invalid_argument("order outside Hasse interval");
    }
    check_factorization(d.order,d.factors,s);check_factorization(d.twist_order,d.twist_factors,s);
    check_basis(d.curve,d.order,d.factors,d.basis,s);check_basis(d.twist,d.twist_order,d.twist_factors,d.twist_basis,s);
    // The two injected subgroups have orders N,Nt, while actual twisted
    // curve orders sum to N+Nt. Thus both injections are surjections.
}
PreparedDatum::PreparedDatum(const Datum&d,Stats*s):datum_(d) { validate_datum(datum_,s); }

Embedding finite_field_embedding(const Curve&e,const Coefficients&f,Word beta,Stats*s) {
    Poly input(e.p,f),h=curve_polynomial(e);
    if(input.degree()<0)throw std::invalid_argument("zero input polynomial");
    if(!beta || beta>=e.p || poww(beta,(e.p-1)/2,e.p)!=e.p-1)throw std::invalid_argument("embedding beta is not a nonsquare");
    if(nmod_poly_get_coeff_ui(input.value,input.degree())!=1)throw std::invalid_argument("embedding input is not monic");
    if(input.degree()==0)return {{1},{1},{},{}};
    if(gcd(input,h,s).degree()>0)throw std::invalid_argument("embedding input meets two-torsion");
    unsigned full=valuation(e.p-1,2);Word odd=(e.p-1)/exact_power(2,full),gamma=odd==1?0:(odd+1)/2;
    Pieces pieces=twoadic(h,input,beta,full,full,s);
    auto a=crt_embedding(pieces,h,beta,gamma,0,s),b=crt_embedding(pieces,h,beta,gamma,1,s);
    return {a.first.coefficients(),b.first.coefficients(),a.second.coefficients(),b.second.coefficients()};
}
Coefficients division_polynomial_mod(const Curve&e,Word n,const Coefficients&x,const Coefficients&y,
                                     const Coefficients&mod,Stats*s) {
    Poly u(e.p,mod),px(e.p,x),py(e.p,y);return division_at(e,n,px,py,u,s).coefficients();
}
std::vector<Word> root_find(const PreparedDatum&prepared,const Coefficients&f,TranslationMethod method,Stats*s) {
    const Datum&d=prepared.get();Word p=d.curve.p;Poly input(p,f);count(s,"root.calls");
    if(input.degree()<0 || nmod_poly_get_coeff_ui(input.value,input.degree())!=1)
        throw std::invalid_argument("input must be nonzero monic");
    if(input.degree()==0)return {};
    const auto expected_degree=input.degree();
    Poly derivative(p);nmod_poly_derivative(derivative.value,input.value);
    if(gcd(input,derivative,s).degree()>0)throw std::invalid_argument("input is not squarefree");
    Ring input_ring(input,s);Poly X=xpoly(p);
    if(!poly_equal(input_ring.power(X,p),input_ring.reduce(X)))throw std::invalid_argument("input is not split over the field");
    Poly common=gcd(input,curve_polynomial(d.curve),s);std::set<Word> roots;
    if(common.degree()>0) {
        Point t1,t2;
        if(d.basis.first_order%2==0)t1=point_mul(d.curve,d.basis.first_order/2,d.basis.first,s);
        if(d.basis.second_order%2==0)t2=point_mul(d.curve,d.basis.second_order/2,d.basis.second,s);
        for(Point q:{t1,t2,point_add(d.curve,t1,t2,s)})if(!q.infinity && common.at(q.x)==0)roots.insert(q.x);
        if(roots.size()!=static_cast<std::size_t>(common.degree()))throw std::logic_error("basis two-torsion extraction incomplete");
        count(s,"root.two_torsion_roots",roots.size());input=quotient_exact(input,common);
    }
    if(input.degree()>0) {
        Embedding lift=finite_field_embedding(d.curve,input.coefficients(),d.beta,s);
        Poly u0(p,lift.u0),v0(p,lift.v0);
        auto factors=separate_curve(d.curve,d.order,d.factors,d.basis,u0,v0,d.beta,method,s);
        for(const Poly&u:factors) {
            if(u.degree()!=1)throw std::logic_error("curve root factor is not linear");
            roots.insert(subw(0,nmod_poly_get_coeff_ui(u.value,0),p));
        }
        Coefficients uc=lift.u1,vc=lift.v1;Word power=1;
        for(Word&c:uc) {c=mulw(c,power,p);power=mulw(power,d.beta,p);}
        Word leading=uc.empty()?1:uc.back(),inverse_leading=invw(leading,p);
        for(Word&c:uc)c=mulw(c,inverse_leading,p);
        power=invw(d.beta,p);
        for(Word&c:vc) {c=mulw(c,power,p);power=mulw(power,d.beta,p);}
        Poly ut(p,uc),vt(p,vc);
        factors=separate_curve(d.twist,d.twist_order,d.twist_factors,d.twist_basis,ut,vt,d.beta,method,s);
        for(const Poly&u:factors) {
            if(u.degree()!=1)throw std::logic_error("twist root factor is not linear");
            roots.insert(mulw(d.beta,subw(0,nmod_poly_get_coeff_ui(u.value,0),p),p));
        }
    }
    if(roots.size()!=static_cast<std::size_t>(expected_degree))throw std::logic_error("root count mismatch");
    Poly original(p,f);for(Word x:roots)if(original.at(x))throw std::logic_error("reported value is not a root");
    return {roots.begin(),roots.end()};
}
Datum paper_f83_datum() {
    Datum d;d.beta=2;d.curve={83,31,21};d.twist={83,70,13};d.order=72;d.twist_order=96;
    d.factors={{2,3},{3,2}};d.twist_factors={{2,5},{3,1}};
    d.basis={{32,0,false},{59,71,false},2,36};
    d.twist_basis={{16,0,false},{6,20,false},2,48};return d;
}
} // namespace paper_ec
