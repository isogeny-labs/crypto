// Repository identity.certificate/v1 companion. Fixed modulus is 2^61-1.
// This sample certificate does not replace exact P256 coefficient checks.
#define main frozen_auxiliary_main
#include "../aux_base.cpp"
#undef main
uint64_t splitmix(uint64_t& state) {
    uint64_t z=(state+=UINT64_C(0x9e3779b97f4a7c15));
    z=(z^(z>>30))*UINT64_C(0xbf58476d1ce4e5b9);
    z=(z^(z>>27))*UINT64_C(0x94d049bb133111eb); return z^(z>>31);
}
std::string variable(const std::string& name) { return object({{"op",jquote("var")},{"args",jquote(name)}}); }
std::string negate(const std::string& a) { return object({{"op",jquote("neg")},{"args",a}}); }
std::string sum(const std::vector<std::string>& a) { return object({{"op",jquote("add")},{"args",array(a)}}); }
std::string multiply(const std::vector<std::string>& a) { return object({{"op",jquote("mul")},{"args",array(a)}}); }
int main(int argc,char** argv) {
    try {
        require(argc==2 && !std::filesystem::exists(argv[1]),"usage: identity FRESH_FILE");
        const auto x=variable("x"),n=variable("next"),r=variable("r"),d=variable("delta");
        const auto lhs=sum({multiply({sum({n,negate(x)}),sum({x,negate(r)})}),negate(d)});
        const auto rhs=sum({multiply({n,x}),negate(multiply({n,r})),negate(multiply({x,x})),multiply({r,x}),negate(d)});
        const auto statement=object({{"name",jquote("Cleared isogeny fibre quadratic expansion")},
          {"source",jquote("goal_dreg_half_20261117/PROOF.md; universal distributive identity")},
          {"variables",array({jquote("x"),jquote("next"),jquote("r"),jquote("delta")})},
          {"degree_bound","2"},{"lhs",lhs},{"rhs",rhs}});
        const std::string id="IDC1h"+hash(statement).substr(0,16);
        std::string stream=id+std::string(8,'\0')+"cert", seed_hash=hash(stream); uint64_t state=0;
        for(unsigned i=0;i<8;++i) state|=std::stoull(seed_hash.substr(2*i,2),nullptr,16)<<(8*i);
        const uint64_t modulus=UINT64_C(2305843009213693951); const Z q=Z("2305843009213693951");
        auto f=[&](const Z& z) { Z a=z%q; if(a<0) a+=q; return a; };
        std::string values;
        for(unsigned i=0;i<32;++i) {
            Z xv=Z(std::to_string(splitmix(state)%modulus)),nv=Z(std::to_string(splitmix(state)%modulus));
            Z rv=Z(std::to_string(splitmix(state)%modulus)),dv=Z(std::to_string(splitmix(state)%modulus));
            Z l=f((nv-xv)*(xv-rv)-dv), rr=f(nv*xv-nv*rv-xv*xv+rv*xv-dv);
            require(l==rr,"identity failed derived point");
            uint64_t v=std::stoull(l.get_str());
            for(unsigned k=0;k<8;++k) values.push_back(static_cast<char>((v>>(8*k))&255));
        }
        Poly xp=Poly::var("x"),np=Poly::var("next"),rp=Poly::var("r"),dp=Poly::var("delta");
        require(((np-xp)*(xp-rp)-dp-(np*xp-np*rp-sq(xp)+rp*xp-dp)).terms.empty(),"exact P256 coefficient identity");
        write(argv[1],object({{"schema",jquote("identity.certificate/v1")},{"id",jquote(id)},
          {"statement",statement},{"modulus",std::to_string(modulus)},{"points","32"},{"seed","0"},
          {"value_digest",jquote(hash(values))},{"proof","null"},
          {"independence",jquote("Native producer issuer; official Rust checker not run (no Rust toolchain); exact P256 coefficient check is separate, no signed or Lean standing")}}));
        std::cout<<id<<'\n'; return 0;
    } catch(const std::exception& e) { std::cerr<<"identity: "<<e.what()<<'\n'; return 1; }
}
