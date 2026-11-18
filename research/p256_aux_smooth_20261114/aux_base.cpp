// Standalone native construction / system exporter, not a performance result.
// Build: c++ -std=c++20 -O2 -Wall -Wextra -Werror aux_base.cpp -lgmpxx -lgmp -lcrypto
#include <gmpxx.h>
#include <openssl/bn.h>
#include <openssl/ec.h>
#include <openssl/obj_mac.h>
#include <openssl/sha.h>

#include <algorithm>
#include <array>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <memory>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
#include <unistd.h>

using Z = mpz_class;
const Z P("FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF", 16);
const Z B("5AC635D8AA3A93E7B3EBBD55769886BC651D06B0CC53B0F63BCE3C3E27D2604B", 16);
const Z N("FFFFFFFF00000000FFFFFFFFFFFFFFFFBCE6FAADA7179E84F3B9CAC2FC632551", 16);
const std::string SLUG = "icv1-fp256-t89188191154553853111372247798585809583-f188c491";
const std::string ICV1 = "ICV1:fp-115792089210356248762697446949407573530086143415290314195533631308867097853951:89188191154553853111372247798585809583:115792089210356248762697446949407573529996955224135760342422259061068512044369:7958909377132088453074743217357398615041065282494610304372115906626967530147:unk:unk:r:f188c4914bb7";
const std::string FAMILY = "aligned-auxiliary-2-isogeny";
const std::string SEED = "p256-aux-smooth-20261114/v1";
const std::string KEY_CODEC = "prime-affine-x-plus-one-shift-sign/be33/v1";

void require(bool ok, const std::string& message) {
    if (!ok) throw std::runtime_error(message);
}
struct Counters {
    uint64_t reductions=0,inversions=0,powm=0,addition_calls=0,doubling_calls=0;
} counters;
Z mod(const Z& a) { ++counters.reductions; Z r; mpz_mod(r.get_mpz_t(), a.get_mpz_t(), P.get_mpz_t()); return r; }
Z inv(const Z& a) {
    ++counters.inversions;
    Z r; require(mpz_invert(r.get_mpz_t(), a.get_mpz_t(), P.get_mpz_t()) != 0,
                 "zero/nonunit denominator"); return r;
}
Z power(const Z& a, const Z& e) {
    ++counters.powm;
    Z r; mpz_powm(r.get_mpz_t(), a.get_mpz_t(), e.get_mpz_t(), P.get_mpz_t()); return r;
}
bool square_root(const Z& a, Z& y) {
    y = power(mod(a), Z((P + 1) / 4));
    if (mod(y*y) != mod(a)) return false;
    if (y > P-y) y = P-y;
    return true;
}
std::string hash(const std::string& s) {
    std::array<unsigned char, SHA256_DIGEST_LENGTH> digest{};
    SHA256(reinterpret_cast<const unsigned char*>(s.data()), s.size(), digest.data());
    static const char hex[] = "0123456789abcdef";
    std::string out;
    for (auto v : digest) { out += hex[v >> 4]; out += hex[v & 15]; }
    return out;
}
// Only controlled ASCII strings enter this codec. This is sorted-key compact
// JSON, matching ecbench::canonical, not an alternative identity convention.
std::string jquote(const std::string& s) {
    std::string r = "\"";
    for (unsigned char c : s) {
        require(c >= 32 && c < 127, "non-ASCII/control string in native codec");
        if (c == '\\' || c == '"') r += '\\';
        r += static_cast<char>(c);
    }
    return r + '"';
}
using Object = std::map<std::string, std::string>;
std::string object(const Object& v) {
    std::string r = "{"; bool first = true;
    for (const auto& [k,x] : v) {
        if (!first) r += ',';
        first = false; r += jquote(k) + ':' + x;
    }
    return r + '}';
}
std::string array(const std::vector<std::string>& v) {
    std::string r = "[";
    for (size_t i=0; i<v.size(); ++i) { if (i) r += ','; r += v[i]; }
    return r + ']';
}
std::string dec(const Z& n) { return jquote(n.get_str()); }
std::string hx(const Z& n) { return jquote("0x" + n.get_str(16)); }
std::string num(size_t n) { return std::to_string(n); }
void write(const std::filesystem::path& path, const std::string& data) {
    std::filesystem::create_directories(path.parent_path());
    std::ofstream f(path, std::ios::binary); f << data << '\n';
    require(f.good(), "write failed: " + path.string());
}
std::string read(const std::filesystem::path& path) {
    std::ifstream f(path, std::ios::binary);
    require(f.good(), "read failed: " + path.string());
    return std::string(std::istreambuf_iterator<char>(f), {});
}

struct Point {
    Z x=0, y=0; bool infinity=true;
    bool operator==(const Point&) const = default;
};
Point point(const Z& x, const Z& y) { return {mod(x), mod(y), false}; }
std::string point_json(const Point& q) {
    return object({{"infinity", q.infinity ? "true":"false"}, {"x",dec(q.x)}, {"y",dec(q.y)}});
}
std::string point_key(const Point& q) {
    return q.infinity ? "infinity" : q.x.get_str(16) + ":" + q.y.get_str(16);
}
struct Curve {
    Z a, b;
    bool on(const Point& q) const {
        return q.infinity ? q.x==0 && q.y==0 : mod(q.y*q.y-q.x*q.x*q.x-a*q.x-b)==0;
    }
    Point neg(const Point& q) const { return q.infinity ? q : point(q.x,-q.y); }
    Point add(const Point& q, const Point& r) const {
        ++counters.addition_calls;
        if(!q.infinity && q==r) ++counters.doubling_calls;
        require(on(q) && on(r), "off-curve addition input");
        if (q.infinity) return r;
        if (r.infinity) return q;
        if (q.x==r.x && mod(q.y+r.y)==0) return {};
        Z slope = q.x==r.x ? mod((3*q.x*q.x+a)*inv(2*q.y))
                              : mod((r.y-q.y)*inv(r.x-q.x));
        Z x=mod(slope*slope-q.x-r.x);
        Point out=point(x,slope*(q.x-x)-q.y);
        require(on(out), "off-curve addition output"); return out;
    }
    Point mul(Z n, Point q) const {
        require(n>=0, "negative scalar"); Point r;
        while(n>0) { if(mpz_odd_p(n.get_mpz_t())) r=add(r,q); n>>=1; if(n>0) q=add(q,q); }
        return r;
    }
};
const Curve TARGET{P-3,B}, AUX{P-3,0};
struct Step {
    Curve source, target; Z r, delta;
    Point apply(const Point& q) const {
        require(source.on(q), "isogeny source point");
        if(q.infinity || q.x==r) return {};
        Z t=inv(q.x-r);
        Point out=point(q.x+delta*t, q.y*(1-delta*t*t));
        require(target.on(out), "isogeny codomain point"); return out;
    }
};
Step step(const Curve& c, const Point& t) {
    require(c.on(t) && !t.infinity && t.y==0, "kernel is not nonzero rational 2-torsion");
    Z d=mod(3*t.x*t.x+c.a); require(d!=0, "singular kernel derivative");
    Curve next{mod(c.a-5*d),mod(c.b-7*t.x*d)};
    require(mod(4*next.a*next.a*next.a+27*next.b*next.b)!=0, "singular codomain");
    return {c,next,t.x,d};
}

// OpenSSL is a second arithmetic implementation, not a signed PCC verifier.
struct OpenSSLCheck {
    using Group=std::unique_ptr<EC_GROUP,decltype(&EC_GROUP_free)>;
    using EP=std::unique_ptr<EC_POINT,decltype(&EC_POINT_free)>;
    using BN=std::unique_ptr<BIGNUM,decltype(&BN_free)>;
    std::unique_ptr<BN_CTX,decltype(&BN_CTX_free)> ctx{BN_CTX_new(),BN_CTX_free};
    Group group{nullptr,EC_GROUP_free};
    static BN bn(const Z& n) {
        BIGNUM* raw=nullptr; require(BN_hex2bn(&raw,n.get_str(16).c_str())!=0,"BN conversion");
        return BN(raw,BN_free);
    }
    explicit OpenSSLCheck(const Curve& c) {
        auto p=bn(P), a=bn(c.a), b=bn(c.b);
        group.reset(EC_GROUP_new_curve_GFp(p.get(),a.get(),b.get(),ctx.get()));
        require(group && ctx,"OpenSSL curve allocation");
    }
    EP encode(const Point& q) {
        EP r(EC_POINT_new(group.get()),EC_POINT_free); require(bool(r),"EC point allocation");
        if(q.infinity) require(EC_POINT_set_to_infinity(group.get(),r.get())==1,"EC infinity");
        else {
            auto x=bn(q.x), y=bn(q.y);
            require(EC_POINT_set_affine_coordinates(group.get(),r.get(),x.get(),y.get(),ctx.get())==1,
                    "OpenSSL rejected point");
        }
        return r;
    }
    void equal(const EC_POINT* q, const Point& r) {
        auto e=encode(r);
        require(EC_POINT_cmp(group.get(),q,e.get(),ctx.get())==0,"GMP/OpenSSL point disagreement");
    }
    void mul(const Point& q, const Z& n, const Point& expected) {
        auto e=encode(q); EP out(EC_POINT_new(group.get()),EC_POINT_free); auto s=bn(n);
        require(EC_POINT_mul(group.get(),out.get(),nullptr,e.get(),s.get(),ctx.get())==1,"EC mul");
        equal(out.get(),expected);
    }
    void add(const Point& q, const Point& r, const Point& expected) {
        auto e=encode(q), f=encode(r); EP out(EC_POINT_new(group.get()),EC_POINT_free);
        require(EC_POINT_add(group.get(),out.get(),e.get(),f.get(),ctx.get())==1,"EC add");
        equal(out.get(),expected);
    }
};

// Sparse exact polynomials. Names are part of the exported ring; there is no
// u64 field truncation, floating arithmetic or silent Groebner pair cap.
using Monomial=std::map<std::string,unsigned>;
struct Poly {
    std::map<Monomial,Z> terms;
    Poly(long n=0) { Z c=mod(Z(n)); if(c!=0) terms[{}]=c; }
    explicit Poly(const Z& n) { Z c=mod(n); if(c!=0) terms[{}]=c; }
    static Poly var(const std::string& s) { Poly p; p.terms[{{s,1}}]=1; return p; }
    Poly& operator+=(const Poly& b) {
        for(const auto& [m,c]:b.terms) { Z v=mod(terms[m]+c); if(v==0) terms.erase(m); else terms[m]=v; }
        return *this;
    }
    Poly operator-() const { Poly r; for(const auto& [m,c]:terms) r.terms[m]=mod(-c); return r; }
    unsigned degree() const {
        unsigned d=0; for(const auto& [m,c]:terms) { (void)c; unsigned v=0;
            for(const auto& [s,e]:m) { (void)s; v+=e; } d=std::max(d,v); } return d;
    }
    Z eval(const std::map<std::string,Z>& values) const {
        Z r=0;
        for(const auto& [m,c]:terms) { Z v=c;
            for(const auto& [s,e]:m) { auto it=values.find(s); require(it!=values.end(),"missing witness "+s);
                v=mod(v*power(it->second,Z(e))); } r=mod(r+v); } return r;
    }
    std::string json() const {
        std::vector<std::string> ts;
        for(const auto& [m,c]:terms) { Object es; for(const auto& [s,e]:m) es[s]=num(e);
            ts.push_back(object({{"coefficient",dec(c)},{"powers",object(es)}})); }
        return array(ts);
    }
};
Poly operator+(Poly a,const Poly& b) { return a+=b; }
Poly operator-(Poly a,const Poly& b) { return a+=-b; }
Poly operator*(const Poly& a,const Poly& b) {
    Poly r; for(const auto& [m,c]:a.terms) for(const auto& [n,d]:b.terms) {
        Monomial v=m; for(const auto& [s,e]:n) v[s]+=e;
        Z t=mod(r.terms[v]+c*d); if(t==0) r.terms.erase(v); else r.terms[v]=t;
    } return r;
}
Poly sq(const Poly& a) { return a*a; }
Poly cube(const Poly& a) { return a*a*a; }
void check_map_identity(const Step& s) {
    Poly x=Poly::var("x"), r(s.r), d(s.delta), t=x-r, v=x*t+d;
    Poly lhs=sq(sq(t)-d)*(cube(x)+Poly(s.source.a)*x+Poly(s.source.b));
    Poly rhs=t*cube(v)+Poly(s.target.a)*v*cube(t)+Poly(s.target.b)*sq(sq(t));
    require((lhs-rhs).terms.empty(),"cleared-denominator isogeny identity failed");
}

struct Column { Point target, auxiliary; };
struct Base {
    unsigned depth; Point kernel96,kernel,coset,terminal;
    std::vector<Step> chain; std::vector<Column> columns;
    std::vector<std::string> trials; std::string identity,fb_id,dump,certificate;
};
Point trial_point(const Curve& c,const std::string& domain,size_t i) {
    Z x=mod(Z(hash(SEED+"|"+domain+"|"+num(i)),16)), y;
    if(!square_root(x*x*x+c.a*x+c.b,y)) return {};
    return point(x,y);
}
std::string model(const Curve& c) {
    return object({{"a",dec(c.a)},{"b",dec(c.b)},{"field",jquote("fp-"+P.get_str())},
                   {"form",jquote("y^2=x^3+a*x+b")},{"p",dec(P)},{"v",jquote("1")}});
}
std::string curve_facts() {
    return object({{"automorphisms_available","2"},{"cofactor",jquote("1")},
      {"construction",jquote("CurveParams::p256()")},{"curve_uid",jquote("urn:ec-record:1:sha256:0523b774e0666cf37ff4dc9cf452eae024192a7f89bdad10ca226d6e64d4dd70")},
      {"ec1",jquote("EC1P256Cp256h0523b774e066")},{"family",jquote("prime")},
      {"field_bits","256"},{"field_degree","null"},
      {"generator",array({hx(Z("6b17d1f2e12c4247f8bce6e563a440f277037d812deb33a0f4a13945d898c296",16)),
                           hx(Z("4fe342e2fe1a7f9b8ee7eb4a7c0f9e162bce33576b315ececbb6406837bf51f5",16))})},
      {"group_order",dec(N)},{"icv1",jquote(ICV1)},{"r",dec(N)},{"registered","true"},{"slug",jquote(SLUG)}});
}
Base build(unsigned depth) {
    Counters before=counters;
    require(depth>=1 && depth<=18,"depth must be 1..18; larger needs a new materialization allowance");
    require(mod(P)==0 && P%4==3 && (P+1)%(Z(1)<<96)==0 && ((P+1)>>96)%2==1,"prime/order shape");
    Base b{}; b.depth=depth;
    Z odd=(P+1)>>96, two96=Z(1)<<96;
    for(size_t i=0;;++i) {
        require(i<4096,"kernel search cap"); Point q=trial_point(AUX,"kernel",i);
        if(q.infinity) { b.trials.push_back(object({{"domain",jquote("kernel")},{"index",num(i)},{"status",jquote("nonsquare")}})); continue; }
        Point k=AUX.mul(odd,q);
        bool good=AUX.mul(two96,k).infinity && !AUX.mul(two96>>1,k).infinity;
        b.trials.push_back(object({{"domain",jquote("kernel")},{"index",num(i)},
                                  {"status",jquote(good?"accepted":"lower_order")}}));
        if(good) { b.kernel96=k; break; }
    }
    b.kernel=AUX.mul(Z(1)<<(96-depth),b.kernel96);
    require(AUX.mul(Z(1)<<depth,b.kernel).infinity && !AUX.mul(Z(1)<<(depth-1),b.kernel).infinity,"kernel exact order");
    Point k=b.kernel; Curve c=AUX;
    for(unsigned i=0;i<depth;++i) {
        Step s=step(c,c.mul(Z(1)<<(depth-i-1),k)); check_map_identity(s);
        b.chain.push_back(s); k=s.apply(k); c=s.target;
    }
    require(k.infinity,"tower kernel not annihilated");
    for(size_t i=0;;++i) {
        require(i<4096,"coset search cap"); Point r=trial_point(AUX,"coset-depth-"+num(depth),i);
        Point t=r; for(const auto& s:b.chain) t=s.apply(t);
        std::string status=r.infinity?"nonsquare":t.infinity?"kernel":t.y==0?"terminal_2torsion":"accepted";
        std::vector<Column> columns; std::set<Z> xs;
        if(status=="accepted") {
            Point q=r;
            for(size_t j=0;j<(size_t(1)<<depth);++j) {
                require(!q.infinity && AUX.on(q) && xs.insert(q.x).second,"coset abscissae not distinct");
                Point mapped=q; for(const auto& s:b.chain) mapped=s.apply(mapped);
                require(mapped==t,"coset does not map to exact terminal point");
                Z y; if(square_root(q.x*q.x*q.x-3*q.x+B,y)) {
                    require(y!=0,"target 2-torsion incompatible with prime order");
                    columns.push_back({point(q.x,y),q});
                }
                q=AUX.add(q,b.kernel);
            }
            require(q==r,"coset orbit did not close");
            if(columns.empty()) status="empty_target_lift";
        }
        b.trials.push_back(object({{"domain",jquote("coset")},{"index",num(i)},{"status",jquote(status)}}));
        if(status=="accepted") { b.coset=r; b.terminal=t; b.columns=std::move(columns); break; }
    }
    std::sort(b.columns.begin(),b.columns.end(),[](const Column& q,const Column& r){ return q.target.x<r.target.x; });
    std::string keys; std::vector<std::string> rows;
    for(size_t i=0;i<b.columns.size();++i) {
        const Point& q=b.columns[i].target;
        for(unsigned sign=0;sign<2;++sign) {
            Z key=((q.x+1)<<1)+sign; std::array<unsigned char,33> raw{}; size_t count=0;
            std::array<unsigned char,33> tmp{};
            mpz_export(tmp.data(),&count,1,1,1,0,key.get_mpz_t()); require(count<=33,"point key size");
            std::copy(tmp.begin(),tmp.begin()+static_cast<std::ptrdiff_t>(count),raw.end()-static_cast<std::ptrdiff_t>(count));
            keys.append(reinterpret_cast<const char*>(raw.data()),raw.size());
            Z y=sign ? Z(P-q.y):q.y;
            rows.push_back(object({{"coef",dec(sign?Z(N-1):Z(1))},{"col",num(i)},{"x",hx(q.x)},{"y",hx(y)}}));
        }
    }
    Object params{{"aux_model_sha256",jquote(hash(model(AUX)))},{"coset_x",hx(b.coset.x)},
      {"coset_y",hx(b.coset.y)},{"depth",jquote(num(depth))},{"kernel_x",hx(b.kernel.x)},
      {"kernel_y",hx(b.kernel.y)},{"point_key_encoding",jquote(KEY_CODEC)},
      {"seed",jquote(SEED)},{"terminal_x",hx(b.terminal.x)}};
    b.identity=object({{"columns",num(b.columns.size())},{"curve",jquote(SLUG)},
      {"family",jquote(FAMILY)},{"params",object(params)},{"points_sha256",jquote(hash(keys))},
      {"schema",jquote("ecbench.factor_base/v1")},{"signed_points",num(rows.size())}});
    std::string fb_sha=hash(b.identity); b.fb_id="FB1h"+fb_sha.substr(0,12);
    Object facts{{"abscissae",num(b.columns.size())},{"columns",num(b.columns.size())},
      {"description",jquote("Aligned auxiliary rational 2-isogeny coset; negation folded into columns")},
      {"dimension","null"},{"family",jquote(FAMILY)},{"fb_id",jquote(b.fb_id)},
      {"fb_sha256",jquote(fb_sha)},{"params",object(params)},{"points_sha256",jquote(hash(keys))},
      {"signed_points",num(rows.size())}};
    // Zeroes below count target-group additions/doublings (none), not auxiliary
    // work. Exact GMP API call counts including build-time checks are separate.
    b.dump=object({{"build_adds","0"},{"build_doubles","0"},{"curve",curve_facts()},
                  {"factor_base",object(facts)},{"points",array(rows)},{"schema",jquote("ecbench.factor_base_dump/v1-wide")}});
    std::vector<std::string> steps;
    for(const auto& s:b.chain) steps.push_back(object({{"a",dec(s.source.a)},{"b",dec(s.source.b)},
      {"codomain_a",dec(s.target.a)},{"codomain_b",dec(s.target.b)},{"delta",dec(s.delta)},{"r",dec(s.r)}}));
    std::string aux_hash=hash(model(AUX));
    std::string work=object({{"addition_calls_including_identity",num(counters.addition_calls-before.addition_calls)},
      {"doubling_calls_subset",num(counters.doubling_calls-before.doubling_calls)},
      {"modular_exponentiation_calls",num(counters.powm-before.powm)},
      {"modular_inversion_calls",num(counters.inversions-before.inversions)},
      {"modular_reduction_calls",num(counters.reductions-before.reductions)},
      {"scope",jquote("auxiliary construction and internal checks; API calls, not field-equivalent operations")}});
    b.certificate=object({{"auxiliary_icv1",jquote("ICV1:fp-"+P.get_str()+":0:"+Z(P+1).get_str()+":1728:unk:unk:r:"+aux_hash.substr(0,12))},
      {"auxiliary_model",model(AUX)},{"auxiliary_slug",jquote("icv1-fp256-t0-"+aux_hash.substr(0,8))},
      {"auxiliary_order",dec(P+1)},{"auxiliary_order_basis",jquote("j1728-character-pairing-p=3mod4")},
      {"chain",array(steps)},{"construction_counters",work},{"coset",point_json(b.coset)},{"depth",num(depth)},
      {"dump_sha256",jquote(hash(b.dump+"\n"))},{"factor_base_id",jquote(b.fb_id)},
      {"group_order_is_asserted_smooth","false"},{"identity_preimage",b.identity},
      {"kernel",point_json(b.kernel)},{"kernel96",point_json(b.kernel96)},
      {"odd_cofactor",dec(odd)},{"p",dec(P)},{"schema",jquote("p256.aligned_auxiliary_certificate/v1")},
      {"seed",jquote(SEED)},{"smooth_subgroup_order",dec(two96)},{"terminal",point_json(b.terminal)},
      {"trials",array(b.trials)},{"verification_standing",jquote("producer-unverified")}});
    return b;
}

struct System {
    std::vector<std::pair<std::string,Poly>> equations;
    std::map<std::string,Z> witness;
    Poly variable(const std::string& name,const Z& value) { witness[name]=mod(value); return Poly::var(name); }
    void eq(const std::string& name,const Poly& p) { equations.emplace_back(name,p); }
    void check() const { for(const auto& [name,p]:equations) require(p.eval(witness)==0,"nonzero witness equation: "+name); }
};
struct SymbolicPoint { Poly x,y,z; Point value; };
SymbolicPoint fixed(const Point& q) { return {Poly(q.x),Poly(q.y),Poly(q.infinity?0:1),q}; }
SymbolicPoint intermediate(System& s,const std::string& prefix,const Point& value) {
    Poly x=s.variable(prefix+"_x",value.x), y=s.variable(prefix+"_y",value.y),
         z=s.variable(prefix+"_z",Z(value.infinity?0:1));
    s.eq(prefix+"_finite_bit",z*(z-Poly(1)));
    s.eq(prefix+"_infinity_x",(Poly(1)-z)*x);
    s.eq(prefix+"_infinity_y",(Poly(1)-z)*y);
    s.eq(prefix+"_curve",sq(y)-cube(x)+Poly(3)*x-Poly(B)*z);
    return {x,y,z,value};
}
void addition(System& s,const std::string& prefix,const SymbolicPoint& a,
              const SymbolicPoint& b,const SymbolicPoint& c) {
    const Point &q=a.value,&r=b.value;
    // Unplanted exports have no witness. Their placeholder assignment is not
    // checked; planted systems and all chart unit cases call System::check.
    std::string active=q.infinity?"l":r.infinity?"r":q.x!=r.x?"g":mod(q.y+r.y)==0?"i":"d";
    std::map<std::string,Poly> flags; Poly sum;
    for(const std::string name:{"g","d","i","l","r"}) {
        Poly e=s.variable(prefix+"_"+name,Z(active==name?1:0));
        s.eq(prefix+"_"+name+"_bit",e*(e-Poly(1))); flags[name]=e; sum=sum+e;
    }
    s.eq(prefix+"_one_chart",sum-Poly(1));
    auto gate=[&](const std::string& branch,const std::string& name,const Poly& p) {
        s.eq(prefix+"_"+branch+"_"+name,flags.at(branch)*p);
    };
    gate("l","left_infinity",a.z);
    gate("r","right_infinity",b.z); gate("r","left_finite",a.z-Poly(1));
    for(const std::string branch:{"l","r"}) {
        const auto& from=branch=="l"?b:a;
        gate(branch,"copy_x",c.x-from.x); gate(branch,"copy_y",c.y-from.y); gate(branch,"copy_z",c.z-from.z);
    }
    for(const std::string branch:{"g","d","i"}) {
        gate(branch,"left_finite",a.z-Poly(1)); gate(branch,"right_finite",b.z-Poly(1));
    }
    gate("i","same_x",a.x-b.x); gate("i","opposite_y",a.y+b.y); gate("i","output_infinity",c.z);
    gate("d","same_x",a.x-b.x); gate("d","same_y",a.y-b.y);
    for(const std::string branch:{"g","d"}) {
        Z t=0,l=0;
        if(active==branch) {
            t=branch=="g"?inv(r.x-q.x):inv(2*q.y);
            l=branch=="g"?mod((r.y-q.y)*t):mod((3*q.x*q.x-3)*t);
        }
        Poly inverse=s.variable(prefix+"_"+branch+"_inverse",t), slope=s.variable(prefix+"_"+branch+"_slope",l);
        // Inactive auxiliaries must not introduce positive-dimensional fibres.
        s.eq(prefix+"_"+branch+"_inactive_inverse",(Poly(1)-flags.at(branch))*inverse);
        s.eq(prefix+"_"+branch+"_inactive_slope",(Poly(1)-flags.at(branch))*slope);
        Poly denominator=branch=="g"?b.x-a.x:Poly(2)*a.y;
        Poly numerator=branch=="g"?b.y-a.y:Poly(3)*sq(a.x)-Poly(3);
        gate(branch,"denominator",denominator*inverse-Poly(1));
        // The nonzero witness already certifies denominator != 0. Clear the
        // slope denominator directly: this keeps the doubling chart cubic
        // after gating instead of multiplying selector*numerator*inverse.
        gate(branch,"slope",denominator*slope-numerator);
        gate(branch,"output_finite",c.z-Poly(1));
        gate(branch,"output_x",c.x-sq(slope)+a.x+b.x);
        gate(branch,"output_y",c.y-slope*(a.x-c.x)+a.y);
    }
}
std::vector<Point> signed_points(const Base& b) {
    std::vector<Point> out;
    for(const auto& c:b.columns) { out.push_back(c.target); out.push_back(TARGET.neg(c.target)); }
    return out;
}
std::vector<size_t> planted_indices(const Base& b,unsigned m) {
    std::vector<size_t> out; size_t size=2*b.columns.size();
    for(unsigned j=0;j<m;++j) {
        Z v(hash(SEED+"|"+b.fb_id+"|planted|m="+num(m)+"|replicate=0|column="+num(j)),16);
        out.push_back(mpz_fdiv_ui(v.get_mpz_t(),size));
    } return out;
}
Point replay(const std::vector<Point>& domain,const std::vector<size_t>& indices) {
    Point sum; for(size_t i:indices) { require(i<domain.size(),"invalid signed index"); sum=TARGET.add(sum,domain[i]); } return sum;
}
Point natural_target(const Base& b,unsigned m) {
    for(size_t i=0;i<4096;++i) {
        Point q=trial_point(TARGET,b.fb_id+"|unplanted|m="+num(m)+"|replicate=0",i);
        if(!q.infinity) return q;
    } throw std::runtime_error("target selection exhausted");
}
System relation_system(const Base& base,unsigned m,const std::string& arm,const Point& target,
                       const std::vector<size_t>& indices) {
    require(m>=2 && m<=7 && indices.size()==m,"relation arity");
    require(arm=="dense" || arm=="tower" || arm=="paired" || arm=="oriented","unknown arm");
    System s; auto domain=signed_points(base); std::vector<SymbolicPoint> points;
    for(unsigned j=0;j<m;++j) {
        Point q=domain.at(indices[j]); std::string prefix="p"+num(j);
        Poly x=s.variable(prefix+"_x",q.x), y=s.variable(prefix+"_y",q.y);
        Poly auxiliary_w; Z auxiliary_w_value=0;
        if(arm=="paired" || arm=="oriented") {
            Z w;
            if(arm=="oriented") w=base.columns.at(indices[j]/2).auxiliary.y;
            else require(square_root(q.x*q.x*q.x-3*q.x,w),"auxiliary ordinate missing");
            Poly u=s.variable(prefix+"_u",mod(q.y+w)), v=s.variable(prefix+"_v",mod(q.y-w));
            auxiliary_w=(u-v)*Poly(inv(2)); auxiliary_w_value=w;
            s.eq(prefix+"_paired_norm",u*v-Poly(B));
            s.eq(prefix+"_aux_curve",Poly(4)*(cube(x)-Poly(3)*x)-sq(u-v));
            s.eq(prefix+"_target_ordinate",Poly(2)*y-u-v);
        } else s.eq(prefix+"_target_curve",sq(y)-cube(x)+Poly(3)*x-Poly(B));
        if(arm=="dense") {
            Poly membership(1); for(const auto& c:base.columns) membership=membership*(x-Poly(c.target.x));
            s.eq(prefix+"_membership",membership);
        } else {
            Poly current=x; Z value=q.x;
            for(unsigned k=0;k<base.depth;++k) {
                const auto& t=base.chain[k];
                Z next_value=mod(value+t.delta*inv(value-t.r));
                Poly next=k+1==base.depth ? Poly(base.terminal.x) : s.variable(prefix+"_fibre_"+num(k+1),next_value);
                s.eq(prefix+"_fibre_step_"+num(k),(next-current)*(current-Poly(t.r))-Poly(t.delta));
                if(arm=="oriented") {
                    Z denominator=inv(value-t.r);
                    Z next_w_value=mod(auxiliary_w_value*(1-t.delta*denominator*denominator));
                    Poly next_w=k+1==base.depth?Poly(base.terminal.y):s.variable(prefix+"_ordinate_"+num(k+1),next_w_value);
                    Poly square_denominator=sq(current-Poly(t.r));
                    s.eq(prefix+"_oriented_step_"+num(k),next_w*square_denominator-auxiliary_w*(square_denominator-Poly(t.delta)));
                    auxiliary_w=next_w; auxiliary_w_value=next_w_value;
                }
                current=next; value=next_value;
            }
            require(value==base.terminal.x,"membership witness terminal");
            if(arm=="oriented") require(auxiliary_w_value==base.terminal.y,"oriented membership witness terminal");
        }
        points.push_back({x,y,Poly(1),q});
    }
    SymbolicPoint sum=points[0];
    for(unsigned j=1;j<m;++j) {
        Point actual=TARGET.add(sum.value,points[j].value);
        SymbolicPoint out=j+1==m?fixed(target):intermediate(s,"sum"+num(j),actual);
        addition(s,"add"+num(j),sum,points[j],out); sum=out;
    }
    return s;
}
std::string system_json(const Base& b,unsigned m,const std::string& arm,const std::string& kind,
                        const Point& target,const System& s) {
    unsigned maximum=0; std::vector<std::string> eqs,vars; Object witness;
    for(const auto& [name,p]:s.equations) { maximum=std::max(maximum,p.degree());
        eqs.push_back(object({{"name",jquote(name)},{"terms",p.json()}})); }
    for(const auto& [name,value]:s.witness) { vars.push_back(jquote(name)); if(kind=="planted") witness[name]=dec(value); }
    return object({{"addition_coverage",jquote("complete-five-chart-with-canonical-infinity")},
      {"arm",jquote(arm)},{"auxiliary_lift_multiplicity_per_point",arm=="paired"?"2":"1"},
      {"curve",jquote(SLUG)},{"equations",array(eqs)},
      {"factor_base",jquote(b.fb_id)},{"instance_kind",jquote(kind)},{"m",num(m)},
      {"maximum_input_degree",num(maximum)},{"modulus",dec(P)},
      {"monomial_order",jquote("not-frozen-for-solving")},{"schema",jquote("p256.relation_system/v1")},
      {"solution_domain",jquote("Fp-rational; extension-field outputs require rejection")},
      {"structured_degree_of_regularity","null"},{"target",point_json(target)},
      {"variables",array(vars)},{"witness",object(witness)}});
}
Poly semaev3(const Poly& x,const Poly& y,const Poly& z) {
    return sq(x-y)*sq(z)-Poly(2)*((x+y)*(x*y-Poly(3))+Poly(2)*Poly(B))*z
            +sq(x*y+Poly(3))-Poly(4)*Poly(B)*(x+y);
}
std::string semaev_json(const Base& b,unsigned m,const Point& target) {
    System s; std::vector<Poly> xs;
    for(unsigned j=0;j<m;++j) {
        std::string name="x"+num(j); Poly x=s.variable(name,0),cur=x; xs.push_back(x);
        for(unsigned k=0;k<b.depth;++k) {
            const auto& t=b.chain[k]; Poly next=k+1==b.depth?Poly(b.terminal.x):s.variable(name+"_f"+num(k+1),0);
            s.eq(name+"_fibre_"+num(k),(next-cur)*(cur-Poly(t.r))-Poly(t.delta)); cur=next;
        }
    }
    Poly cur=xs[0];
    for(unsigned j=1;j<m;++j) {
        Poly next=j+1==m?Poly(target.x):s.variable("sum_x"+num(j),0);
        s.eq("semaev_"+num(j),semaev3(cur,xs[j],next)); cur=next;
    }
    std::vector<std::string> eqs,vars;
    for(const auto& [name,p]:s.equations) eqs.push_back(object({{"name",jquote(name)},{"terms",p.json()}}));
    for(const auto& [name,value]:s.witness) { (void)value; vars.push_back(jquote(name)); }
    return object({{"complete_coverage","false"},{"equations",array(eqs)},{"factor_base",jquote(b.fb_id)},
      {"limitation",jquote("Intermediate/target infinity excluded; sign and target lift replay required")},
      {"m",num(m)},{"modulus",dec(P)},{"schema",jquote("p256.semaev_affine_chart/v1")},
      {"structured_degree_of_regularity","null"},{"target",point_json(target)},{"variables",array(vars)}});
}

using Tuple=std::vector<size_t>;
struct Entry { Point sum; Tuple tuple; };
void enumerate(const std::vector<Point>& domain,unsigned n,Point sum,Tuple& prefix,std::vector<Entry>& out) {
    if(n==0) { out.push_back({sum,prefix}); return; }
    for(size_t i=0;i<domain.size();++i) { prefix.push_back(i); enumerate(domain,n-1,TARGET.add(sum,domain[i]),prefix,out); prefix.pop_back(); }
}
std::vector<Tuple> exhaustive(const std::vector<Point>& domain,unsigned m,const Point& target) {
    std::vector<Entry> entries; Tuple tmp; enumerate(domain,m,{},tmp,entries);
    std::vector<Tuple> out; for(const auto& e:entries) if(e.sum==target) out.push_back(e.tuple);
    return out;
}
std::vector<Tuple> mitm(const std::vector<Point>& domain,unsigned m,const Point& target) {
    std::vector<Entry> left,right; Tuple tmp;
    enumerate(domain,m/2,{},tmp,left); enumerate(domain,m-m/2,{},tmp,right);
    std::map<std::string,std::vector<Tuple>> table;
    for(const auto& e:left) table[point_key(e.sum)].push_back(e.tuple);
    std::vector<Tuple> out;
    for(const auto& r:right) {
        auto it=table.find(point_key(TARGET.add(target,TARGET.neg(r.sum))));
        if(it==table.end()) continue;
        for(const auto& l:it->second) { Tuple t=l; t.insert(t.end(),r.tuple.begin(),r.tuple.end());
            require(replay(domain,t)==target,"MITM false positive"); out.push_back(std::move(t)); }
    }
    std::sort(out.begin(),out.end()); return out;
}
template<class F> void rejects(F f,const std::string& name) {
    bool rejected=false; try { f(); } catch(const std::exception&) { rejected=true; }
    require(rejected,"negative test accepted: "+name);
}
void crosscheck(const Base& b) {
    OpenSSLCheck aux(AUX), target(TARGET);
    aux.mul(b.kernel96,Z(1)<<96,{});
    aux.mul(b.kernel96,Z(1)<<95,AUX.mul(Z(1)<<95,b.kernel96));
    for(const auto& c:b.columns) {
        target.mul(c.target,N,{});
        aux.add(c.auxiliary,b.kernel,AUX.add(c.auxiliary,b.kernel));
        require(mod(c.target.y*c.target.y-c.auxiliary.y*c.auxiliary.y)==B,"aligned norm identity");
        require(TARGET.on(c.target),"target lift not on curve");
    }
    for(const auto& t:b.chain) {
        check_map_identity(t); Point r;
        for(size_t i=0;r.infinity;++i) r=trial_point(t.source,"map-crosscheck",i);
        OpenSSLCheck source(t.source), dest(t.target);
        Point double_r=t.source.add(r,r),mapped=t.apply(r),mapped_double=t.apply(double_r);
        source.add(r,r,double_r); dest.add(mapped,mapped,mapped_double);
        require(t.apply(point(t.r,0)).infinity,"kernel exception did not map infinity");
        require(t.apply({}).infinity,"infinity exception did not map infinity");
        Step bad=t; bad.delta=mod(bad.delta+1);
        rejects([&]{check_map_identity(bad);},"corrupt isogeny");
    }
    rejects([&]{(void)inv(0);},"zero denominator");
    rejects([&]{(void)step(AUX,b.coset);},"wrong kernel");
    rejects([&]{require(AUX.mul(Z(1)<<(b.depth-1),b.kernel).infinity,"lower-order lie");},"wrong order claim");
}
void emit(const Base& b,const std::filesystem::path& out) {
    require(b.depth<=8,"dense system materialization limited to d<=8; use base for larger construction-only fixtures");
    write(out/"factor_base.json",b.dump); write(out/"certificate.json",b.certificate);
    write(out/"identity.json",b.identity);
    std::vector<std::string> artifacts;
    auto put=[&](const std::string& name,const std::string& data) {
        write(out/name,data); artifacts.push_back(object({{"path",jquote(name)},{"sha256",jquote(hash(data+"\n"))}}));
    };
    auto domain=signed_points(b);
    for(unsigned m=2;m<=7;++m) {
        auto indices=planted_indices(b,m); Point planted=replay(domain,indices), natural=natural_target(b,m);
        OpenSSLCheck target(TARGET);
        Point accumulator;
        for(size_t i:indices) { Point next=TARGET.add(accumulator,domain[i]); target.add(accumulator,domain[i],next); accumulator=next; }
        require(accumulator==planted,"planted exact replay");
        std::vector<std::string> signed_indices; for(size_t i:indices) signed_indices.push_back(num(i));
        put("m"+num(m)+"-instance.json",object({{"factor_base",jquote(b.fb_id)},{"m",num(m)},
            {"planted_indices",array(signed_indices)},{"planted_target",point_json(planted)},
            {"unplanted_target",point_json(natural)},{"unplanted_relations_status",jquote("not-searched")}}));
        for(const std::string arm:{"dense","tower","paired","oriented"}) {
            System planted_system=relation_system(b,m,arm,planted,indices); planted_system.check();
            put("m"+num(m)+"-"+arm+"-planted.json",system_json(b,m,arm,"planted",planted,planted_system));
            System natural_system=relation_system(b,m,arm,natural,indices);
            put("m"+num(m)+"-"+arm+"-unplanted.json",system_json(b,m,arm,"unplanted",natural,natural_system));
        }
        put("m"+num(m)+"-semaev-affine-chart.json",semaev_json(b,m,natural));
    }
    for(const std::string name:{"factor_base.json","certificate.json","identity.json"})
        artifacts.push_back(object({{"path",jquote(name)},{"sha256",jquote(hash(read(out/name)))}}));
    write(out/"manifest.json",object({{"artifacts",array(artifacts)},{"depth",num(b.depth)},
      {"factor_base",jquote(b.fb_id)},{"schema",jquote("p256.auxiliary_artifact_manifest/v1")}}));
}
void selftest(const std::filesystem::path& out) {
    require(hash("")=="e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855","SHA256 KAT");
    require(hash(model(TARGET)).substr(0,12)=="f188c4914bb7","registered ICV1 codec mismatch");
    std::vector<std::string> checks;
    for(unsigned d=1;d<=6;++d) {
        Base b=build(d); crosscheck(b);
        if(d<=2) {
            auto domain=signed_points(b);
            require(domain.size()<=(size_t(1)<<(d+1)),"exhaustive domain exceeds protocol");
            unsigned max_m=d==1?7:4;
            for(unsigned m=2;m<=max_m;++m) for(const std::string kind:{"planted","unplanted"}) {
                Point target=kind=="planted"?replay(domain,planted_indices(b,m)):natural_target(b,m);
                auto all=exhaustive(domain,m,target), joined=mitm(domain,m,target);
                require(all==joined,"exhaustive/MITM false positive or false negative");
                OpenSSLCheck openssl(TARGET);
                for(const auto& t:all) {
                    Point acc;
                    for(size_t index:t) { Point next=TARGET.add(acc,domain[index]); openssl.add(acc,domain[index],next); acc=next; }
                    require(acc==target,"reported relation exact replay");
                    for(const std::string arm:{"dense","tower","paired","oriented"}) {
                        System s=relation_system(b,m,arm,target,t); s.check();
                        if(m==2 && (arm=="paired" || arm=="oriented")) {
                            std::swap(s.witness.at("p0_u"),s.witness.at("p0_v"));
                            for(auto& [name,value]:s.witness)
                                if(name.starts_with("p0_ordinate_")) value=mod(-value);
                            if(arm=="paired") s.check();
                            else rejects([&]{s.check();},"reversed auxiliary ordinate accepted");
                            s=relation_system(b,m,arm,target,t);
                        }
                        if(m==2) {
                            s.witness["p0_y"]=mod(s.witness.at("p0_y")+1);
                            rejects([&]{s.check();},"corrupt relation witness");
                        }
                    }
                }
                checks.push_back(object({{"depth",num(d)},{"kind",jquote(kind)},{"m",num(m)},
                  {"ordered_relations",num(all.size())},{"signed_domain",num(domain.size())}}));
            }
        }
        if(d==4) {
            emit(b,out/"fixture-depth4"); Base again=build(4);
            require(b.dump==again.dump && b.certificate==again.certificate,"nondeterministic rebuild");
            std::string corrupt=b.dump; corrupt[corrupt.size()/2]^=1;
            rejects([&]{require(corrupt==again.dump,"dump differs from rebuild");},"dump corruption");
        }
        std::cout<<"depth="<<d<<" columns="<<b.columns.size()<<" id="<<b.fb_id<<" native_checks=pass\n";
    }
    // Force every exceptional chart rather than rely on random planted tuples.
    Base b=build(1); auto dom=signed_points(b); Point q=dom[0], nq=TARGET.neg(q);
    const std::vector<std::pair<Point,Point>> chart_cases{{{},{}},{{},q},{q,{}},{q,nq},{q,q},{q,TARGET.add(q,q)}};
    for(size_t i=0;i<chart_cases.size();++i) {
        const auto& [a,c]=chart_cases[i]; System s;
        auto left=intermediate(s,"left",a),right=intermediate(s,"right",c),
             result=intermediate(s,"result",TARGET.add(a,c));
        addition(s,"edge",left,right,result); s.check();
        s.witness["result_x"]=mod(s.witness.at("result_x")+1);
        rejects([&]{s.check();},"corrupt addition result");
    }
    write(out/"selftest.json",object({{"complete_domain_comparisons",array(checks)},
      {"exceptional_chart_cases",num(chart_cases.size())},{"native_arithmetic_crosscheck",jquote("GMP/OpenSSL")},
      {"performance_experiment","false"},{"schema",jquote("p256.auxiliary_selftest/v1")},
      {"structured_degree_of_regularity","null"},{"verification_standing",jquote("producer-unverified")}}));
    std::cout<<"complete-reference comparisons, all addition charts, corrupted maps/orders/dumps: pass\n";
}
int main(int argc,char** argv) {
    try {
        require(argc>=3,"usage: aux_base selftest OUT | build DEPTH OUT | base DEPTH OUT | check DEPTH OUT");
        std::string mode=argv[1];
        if(mode=="selftest") { require(argc==3,"selftest arity"); selftest(argv[2]); }
        else {
            require(argc==4 && (mode=="build" || mode=="base" || mode=="check"),"build/base/check arity");
            std::string ds=argv[2]; require(!ds.empty() && ds.find_first_not_of("0123456789")==std::string::npos,"depth syntax");
            unsigned depth=static_cast<unsigned>(std::stoul(ds)); require(ds==num(depth),"depth integer overflow/noncanonical");
            Base b=build(depth);
            if(mode=="base") {
                write(std::filesystem::path(argv[3])/"factor_base.json",b.dump);
                write(std::filesystem::path(argv[3])/"certificate.json",b.certificate);
                write(std::filesystem::path(argv[3])/"identity.json",b.identity);
            }
            else if(mode=="build") emit(b,argv[3]);
            else {
                auto tmp=std::filesystem::temp_directory_path()/("aux-replay-"+hash(std::filesystem::absolute(argv[3]).string()).substr(0,16)+"-"+num(static_cast<size_t>(getpid())));
                require(!std::filesystem::exists(tmp),"replay temporary directory already exists");
                struct Cleanup {
                    std::filesystem::path path;
                    ~Cleanup() { std::error_code error; std::filesystem::remove_all(path,error); }
                } cleanup{tmp};
                emit(b,tmp);
                size_t expected=static_cast<size_t>(std::distance(std::filesystem::directory_iterator(tmp),std::filesystem::directory_iterator{}));
                size_t actual=static_cast<size_t>(std::distance(std::filesystem::directory_iterator(argv[3]),std::filesystem::directory_iterator{}));
                require(actual==expected,"replay directory has missing or additional files");
                for(const auto& e:std::filesystem::directory_iterator(tmp))
                    require(read(e.path())==read(std::filesystem::path(argv[3])/e.path().filename()),"byte replay mismatch: "+e.path().filename().string());
            }
            std::cout<<b.fb_id<<" columns="<<b.columns.size()<<" depth="<<depth<<" mode="<<mode<<"\n";
        }
        return 0;
    } catch(const std::exception& e) { std::cerr<<"aux_base: "<<e.what()<<'\n'; return 1; }
}
