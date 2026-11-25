// Separate recorded-divisor checker. Only JSON transport is shared.
#include "typed_json.hpp"
#include <openssl/bn.h>
#include <algorithm>
#include <chrono>
#include <iostream>
#include <memory>
#include <tuple>

using BN = std::unique_ptr<BIGNUM, decltype(&BN_free)>;
const std::string prime_text = "6277101735386680763835789423207666416083908700390324961279";
const std::string b_text = "2455155546008943817740293915197451784769108058161191238065";
BN number(const std::string& s) {
    BIGNUM* raw = nullptr;
    require(BN_dec2bn(&raw, s.c_str()) == int(s.size()), "decimal BIGNUM");
    return BN(raw, BN_free);
}
const BIGNUM* prime() { static BN p = number(prime_text); return p.get(); }
BN_CTX* context() {
    static std::unique_ptr<BN_CTX, decltype(&BN_CTX_free)> c(BN_CTX_new(), BN_CTX_free);
    require(bool(c), "BN context"); return c.get();
}
struct F {
    BN n;
    F(long x = 0): F(std::to_string(x)) {}
    explicit F(const std::string& s): n(number(s)) {
        require(BN_nnmod(n.get(), n.get(), prime(), context()) == 1, "field reduction");
    }
    F(const F& a): n(BN_dup(a.n.get()), BN_free) { require(bool(n), "BN copy"); }
    F& operator=(const F& a) { if (this != &a) { F b(a); n.swap(b.n); } return *this; }
    F(F&&) = default; F& operator=(F&&) = default;
    bool operator==(const F& a) const { return BN_cmp(n.get(), a.n.get()) == 0; }
    bool operator!=(const F& a) const { return !(*this == a); }
    F operator+(const F& a) const {
        F r; require(BN_mod_add(r.n.get(), n.get(), a.n.get(), prime(), context()) == 1, "BN add"); return r;
    }
    F operator-(const F& a) const {
        F r; require(BN_mod_sub(r.n.get(), n.get(), a.n.get(), prime(), context()) == 1, "BN subtract"); return r;
    }
    F operator-() const { return F(0) - *this; }
    F operator*(const F& a) const {
        F r; require(BN_mod_mul(r.n.get(), n.get(), a.n.get(), prime(), context()) == 1, "BN multiply"); return r;
    }
    F inverse() const {
        require(!BN_is_zero(n.get()), "zero inverse"); F r;
        require(BN_mod_inverse(r.n.get(), n.get(), prime(), context()) != nullptr, "BN inverse"); return r;
    }
    F operator/(const F& a) const { return *this * a.inverse(); }
};
F coefficient(const Json& j) {
    require(j.kind == JsonKind::string || j.kind == JsonKind::number, "coefficient token type");
    require(!j.scalar.empty() && (j.scalar == "0" || j.scalar[0] != '0'), "canonical decimal");
    for (char c: j.scalar) require(c >= '0' && c <= '9', "unsigned decimal");
    BN n = number(j.scalar); require(BN_cmp(n.get(), prime()) < 0, "coefficient outside field");
    return F(j.scalar);
}
struct Poly {
    std::vector<F> c;
    Poly() = default;
    Poly(std::initializer_list<F> a): c(a) { trim(); }
    explicit Poly(std::vector<F> a): c(std::move(a)) { trim(); }
    void trim() { while (!c.empty() && c.back() == F(0)) c.pop_back(); }
    int degree() const { return int(c.size()) - 1; }
    F at(int i) const { return i >= 0 && i <= degree() ? c[size_t(i)] : F(0); }
    bool operator==(const Poly& a) const { return c == a.c; }
    Poly operator+(const Poly& a) const {
        std::vector<F> r(std::max(c.size(), a.c.size()));
        for (size_t i = 0; i < r.size(); ++i) r[i] = at(int(i)) + a.at(int(i));
        return Poly(std::move(r));
    }
    Poly operator-() const { auto r = c; for (auto& x: r) x = -x; return Poly(std::move(r)); }
    Poly operator-(const Poly& a) const { return *this + (-a); }
    Poly scale(const F& a) const { auto r = c; for (auto& x: r) x = x*a; return Poly(std::move(r)); }
    Poly operator*(const Poly& a) const {
        if (c.empty() || a.c.empty()) return {};
        std::vector<F> r(c.size() + a.c.size() - 1);
        for (size_t i = 0; i < c.size(); ++i)
            for (size_t k = 0; k < a.c.size(); ++k) r[i+k] = r[i+k] + c[i]*a.c[k];
        return Poly(std::move(r));
    }
    std::pair<Poly,Poly> divide(const Poly& d) const {
        require(!d.c.empty(), "polynomial zero divisor"); Poly r = *this;
        std::vector<F> q(size_t(std::max(0, degree() - d.degree() + 1)));
        F inverse = d.c.back().inverse();
        while (!r.c.empty() && r.degree() >= d.degree()) {
            int shift = r.degree() - d.degree(); F t = r.c.back()*inverse;
            q[size_t(shift)] = t;
            for (int i = 0; i <= d.degree(); ++i)
                r.c[size_t(i+shift)] = r.c[size_t(i+shift)] - t*d.c[size_t(i)];
            r.trim();
        }
        return {Poly(std::move(q)), r};
    }
    Poly operator%(const Poly& d) const { return divide(d).second; }
    Poly exact(const Poly& d) const {
        auto qr = divide(d); require(qr.second.c.empty(), "non-exact quotient"); return qr.first;
    }
    Poly monic() const { require(!c.empty(), "zero monic polynomial"); return scale(c.back().inverse()); }
};
Poly read_poly(const Json& j) {
    require(j.kind == JsonKind::array && j.arr.size() <= 32, "bounded polynomial array");
    std::vector<F> r; for (auto& x: j.arr) r.push_back(coefficient(x));
    require(r.empty() || r.back() != F(0), "canonical polynomial tail"); return Poly(std::move(r));
}
Poly power(Poly a, int n) { Poly r{F(1)}; for (int i=0; i<n; ++i) r=r*a; return r; }
Poly power_mod(Poly a, int n, const Poly& m) {
    Poly r{F(1)}; for (int i=0; i<n; ++i) r=(r*a)%m; return r;
}
std::tuple<Poly,Poly,Poly> bezout(Poly a, Poly b) {
    Poly s{F(1)}, t, ns, nt{F(1)};
    while (!b.c.empty()) {
        auto qr = a.divide(b); Poly ss=s-qr.first*ns, tt=t-qr.first*nt;
        a=b; b=qr.second; s=ns; t=nt; ns=ss; nt=tt;
    }
    require(!a.c.empty(), "gcd(0,0)"); F unit=a.c.back().inverse();
    return {a.scale(unit),s.scale(unit),t.scale(unit)};
}
Poly derivative(const Poly& p) {
    std::vector<F> c; for (int i=1; i<=p.degree(); ++i) c.push_back(p.at(i)*F(i));
    return Poly(std::move(c));
}
Poly cover() {
    Poly t{F(0),F(1)}, tm=t*t-Poly{F(1)}, td=t*t+Poly{F(7)};
    Poly h=tm*(power(td,3)-(td*tm*tm).scale(F(3))+power(tm,3).scale(F(b_text)));
    // X^8 h(1 + 1/(c X)): expand exactly rather than import producer coefficients.
    Poly numerator; const F c(1024);
    for (int i=0; i<=h.degree(); ++i) {
        long choose=1; F inv_c_power(1);
        for (int k=0; k<=i; ++k) {
            std::vector<F> term(size_t(9-k)); term.back()=h.at(i)*F(choose)*inv_c_power;
            numerator=numerator+Poly(std::move(term));
            if (k<i) { choose=choose*(i-k)/(k+1); inv_c_power=inv_c_power/c; }
        }
    }
    require(numerator.degree()==7 && numerator.c.back()==F(1), "normalized degree-seven cover");
    require(std::get<0>(bezout(numerator,derivative(numerator))).degree()==0, "smooth source");
    return numerator;
}
struct Divisor {
    Poly u{F(1)},v;
    bool operator==(const Divisor& a) const { return u==a.u && v==a.v; }
};
void valid(const Divisor& d, const Poly& h) {
    require(d.u.degree()>=0 && d.u.degree()<=3 && d.u.c.back()==F(1), "reduced monic support");
    require(d.v.degree()<d.u.degree(), "reduced ordinate");
    require(((d.v*d.v-h)%d.u).c.empty(), "source curve incidence");
}
Divisor negate(const Divisor& d) { return {d.u,(-d.v)%d.u}; }
Divisor add(const Divisor& a, const Divisor& b, const Poly& h) {
    valid(a,h); valid(b,h);
    auto [g,e,f]=bezout(a.u,b.u); auto [s,r,t]=bezout(g,a.v+b.v);
    require((r*e*a.u+r*f*b.u+t*(a.v+b.v))==s, "three-polynomial Bezout certificate");
    Poly u=(a.u*b.u).exact(s*s);
    Poly v=(r*e*a.u*b.v+r*f*b.u*a.v+t*(a.v*b.v+h)).exact(s)%u;
    while (u.degree()>3) {
        Poly next=(v*v-h).exact(u).monic();
        require(next.degree()<u.degree(), "strict reduction degree"); v=(-v)%next; u=next;
    }
    Divisor d{u.monic(),v}; valid(d,h); return d;
}
Divisor pull(const Json& p, const Poly& h) {
    require(p.at("infinity").kind==JsonKind::boolean, "point infinity flag");
    if (p.at("infinity").scalar=="true") return {};
    F x=coefficient(p.at("x")), y=coefficient(p.at("y"));
    require(y*y==x*x*x-F(3)*x+F(b_text), "elliptic point incidence");
    F c(1024), w=x-F(1);
    Poly u{-w/(F(8)*c*c),-F(2)*w/(F(8)*c),F(1)};
    Poly ordinate=power(Poly{F(1),F(2)*c},2).scale(y/(c*c*c*c))%u;
    Divisor fiber{u,ordinate}, anchor{Poly{F(1)/(F(2)*c),F(1)},Poly{}};
    valid(fiber,h); valid(anchor,h); require(add(anchor,anchor,h)==Divisor{}, "ramified anchor order two");
    return add(fiber,negate(anchor),h);
}
void cycle(const Json& j, const Json& model) {
    Poly u=read_poly(j.at("support")), x=read_poly(j.at("x")), y=read_poly(j.at("y"));
    require(j.at("dimension").integer()==9 && u.degree()==9 && u.c.back()==F(1), "full length-nine cycle");
    require(x.degree()<9 && y.degree()<9, "reduced cycle coordinates");
    require((x+y.scale(F(j.at("coordinate_lambda").integer())))%u==Poly{F(0),F(1)}, "separating coordinate");
    Poly value;
    for (const auto& term: model.at("terms").arr) {
        const auto& e=term.at("exponents").arr; require(e.size()==3, "homogeneous quartic exponent triple");
        int a=e[0].integer(), b=e[1].integer(), c=e[2].integer();
        require(a>=0 && b>=0 && c>=0 && a+b+c==4, "quartic degree");
        value=(value+(power_mod(x,a,u)*power_mod(y,b,u)).scale(coefficient(term.at("coefficient"))))%u;
    }
    require(value.c.empty(), "full quotient-algebra quartic incidence");
}
template<class Fn> void rejects(Fn fn) {
    bool rejected=false; try { fn(); } catch (const std::exception&) { rejected=true; }
    require(rejected, "mutation accepted");
}
void panel(const Json& input) {
    const auto& rows=input.at("fixtures").arr; require(rows.size()==6, "complete fixed panel");
    require(rows[0].at("point").at("infinity").scalar=="true", "identity first");
    for (size_t i=1;i<6;++i) require(rows[i].at("point").at("infinity").scalar=="false", "nonzero fixture");
    auto x=[&](size_t i){return coefficient(rows[i].at("point").at("x"));};
    auto y=[&](size_t i){return coefficient(rows[i].at("point").at("y"));};
    require(x(1)==x(2) && y(1)==-y(2), "G and -G");
    require(x(3)==x(5) && y(3)==-y(5), "exceptional -2G binding");
    F s=(F(3)*x(1)*x(1)-F(3))/(F(2)*y(1)); F xx=s*s-F(2)*x(1);
    require(x(3)==xx && y(3)==s*(x(1)-xx)-y(1), "2G binding");
}
