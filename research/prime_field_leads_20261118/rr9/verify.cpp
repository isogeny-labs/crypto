#include <algorithm>
#include <array>
#include <cassert>
#include <chrono>
#include <cstdint>
#include <fstream>
#include <iostream>
#include <map>
#include <stdexcept>
#include <string>
#include <vector>

namespace prior {
#define main prior_rr9_verifier
#include "../../prime_field_followup_20261117/rr9/prior_verify.cpp"
#undef main
}

using prior::Int;
using prior::Point;
using prior::add;
using prior::base_points;
using prior::curve_a;
using prior::curve_b;
using prior::infinity;
using prior::mod;
using prior::negate;
using prior::p;
using prior::power;
using prior::xs;

using Exp=std::uint64_t;
using Poly=std::map<Exp,Int>;
constexpr std::array<const char*,7> variables{"a0","a1","a2","a3","v0","v1","v2"};

void require(bool ok, const char* message) {
    if (!ok) throw std::runtime_error(message);
}

void accumulate(Poly& dst, Exp monomial, Int value) {
    Int& coefficient=dst[monomial];
    coefficient=mod(coefficient+value);
    if (!coefficient) dst.erase(monomial);
}

Poly constant(Int value) {
    Poly out;
    accumulate(out,0,value);
    return out;
}

Poly variable(unsigned j) {
    return {{Exp{1}<<(8*j),1}};
}

void scaled_add(Poly& dst, const Poly& src, Int scale) {
    scale=mod(scale);
    for (auto [exp,coefficient] : src) accumulate(dst,exp,scale*coefficient);
}

Poly multiply(const Poly& left, const Poly& right) {
    Poly out;
    for (auto [e,x] : left) for (auto [f,y] : right) {
        for (unsigned i=0; i<variables.size(); ++i)
            require(((e>>(8*i))&255)+((f>>(8*i))&255)<256,"exponent overflow");
        accumulate(out,e+f,x*y);
    }
    return out;
}

Poly read_poly(const std::string& text) {
    Poly out;
    for (std::size_t start=0; start<text.size();) {
        Int coefficient=1;
        if (text[start]=='-') { coefficient=-1; ++start; }
        else if (text[start]=='+') ++start;
        auto end=std::min(text.size(),text.find_first_of("+-",start));
        Exp exp=0;
        for (std::size_t pos=start; pos<end;) {
            auto stop=std::min(end,text.find('*',pos));
            auto factor=text.substr(pos,stop-pos);
            require(!factor.empty(),"empty polynomial factor");
            if (factor[0]>='0' && factor[0]<='9') {
                std::size_t consumed=0;
                Int value=std::stoll(factor,&consumed);
                require(consumed==factor.size() && value<p,"malformed field coefficient");
                coefficient=mod(coefficient*value);
            } else {
                auto id=std::find(variables.begin(),variables.end(),factor.substr(0,2));
                require(id!=variables.end(),"unknown polynomial variable");
                unsigned degree=1;
                if (factor.size()!=2) {
                    require(factor[2]=='^',"malformed polynomial exponent");
                    std::size_t consumed=0;
                    auto literal=factor.substr(3);
                    degree=std::stoul(literal,&consumed);
                    require(consumed==literal.size(),"malformed exponent");
                }
                unsigned shift=8*(id-variables.begin());
                require(degree>0 && degree+((exp>>shift)&255)<256,"invalid polynomial exponent");
                exp+=Exp{degree}<<shift;
            }
            pos=stop+1;
        }
        accumulate(out,exp,coefficient);
        start=end;
    }
    return out;
}
using Coefficients = std::vector<Poly>;

Coefficients convolve(const Coefficients& left, const Coefficients& right) {
    Coefficients out(left.size() + right.size() - 1);
    for (std::size_t i = 0; i < left.size(); ++i)
        for (std::size_t j = 0; j < right.size(); ++j)
            scaled_add(out[i+j], multiply(left[i], right[j]), 1);
    return out;
}

Coefficients norm(Point residual, const Coefficients& chart_a, const Coefficients& chart_v) {
    auto [r,s] = residual;
    Coefficients out = convolve({constant(mod(r*r + curve_a)), constant(r), constant(1)},
                                convolve(chart_v, chart_v));
    auto xaa = convolve({constant(mod(-r)), constant(1)}, convolve(chart_a, chart_a));
    auto av = convolve(chart_a, chart_v);
    for (std::size_t i = 0; i < xaa.size(); ++i) scaled_add(out[i], xaa[i], -1);
    for (std::size_t i = 0; i < av.size(); ++i) scaled_add(out[i], av[i], -2*s);
    Coefficients fixed{constant(1)};
    for (Int x : xs) fixed = convolve(fixed, {constant(mod(-x)), constant(1)});
    for (std::size_t i = 0; i < fixed.size(); ++i) scaled_add(out[i], fixed[i], -1);
    require(out.back().empty(), "norm is not monic");
    out.pop_back();
    return out;
}

struct Ideal {
    Coefficients equations;
    std::array<Poly,3> reconstruction;
};

Ideal build(Point residual, bool reduced) {
    Coefficients ca{variable(0),variable(1),variable(2),variable(3)};
    Coefficients cv{variable(4),variable(5),variable(6),constant(1)};
    if (!reduced) return {norm(residual,ca,cv),{}};

    for (int j = 2; j >= 0; --j) {
        auto polynomials = norm(residual,ca,cv);
        auto pivot = polynomials[5+j];
        scaled_add(pivot, variable(4+j), -2);
        for (const auto& [monomial, coeff] : pivot) {
            require((monomial >> 32) == 0 && coeff != 0, "pivot is not 2*v_j plus a polynomial in A");
        }
        Poly replacement;
        scaled_add(replacement,pivot,(p-1)/2);
        cv[j] = std::move(replacement);
    }
    auto equations = norm(residual,ca,cv);
    for (std::size_t i = 5; i < equations.size(); ++i)
        require(equations[i].empty(), "reconstruction does not zero the eliminated equations");
    equations.resize(5);
    return {std::move(equations),{cv[0],cv[1],cv[2]}};
}

Int evaluate(const Poly& poly, const std::array<Int,7>& assignments) {
    Int result = 0;
    for (auto [exp, coeff] : poly) {
        for (unsigned j = 0; j < assignments.size(); ++j)
            coeff = mod(coeff * power(assignments[j], (exp >> (8*j)) & 255));
        result = mod(result + coeff);
    }
    return result;
}

void replay_point(Point residual, const std::array<Int,7>& assignments) {
    auto full = build(residual,false);
    for (const auto& equation : full.equations)
        require(evaluate(equation,assignments) == 0,"positive violates a full equation");
    Point sum = infinity;
    for (auto point : base_points()) {
        Int x=point[0], aa=0, vv=1;
        for (int i = 3; i >= 0; --i) aa=mod(aa*x+assignments[i]);
        for (int i = 6; i >= 4; --i) vv=mod(vv*x+assignments[i]);
        auto test=[&](Int y) { return mod(mod(x-residual[0])*aa+mod(y+residual[1])*vv)==0; };
        bool plus=test(point[1]), minus=test(mod(-point[1]));
        require(plus != minus,"positive has ambiguous or absent root sign");
        sum = add(sum,plus?point:negate(point));
    }
    require(sum == residual,"positive did not replay on the curve");
}

int run(int argc, char** argv) {
    require(argc == 5 || argc == 6,"usage: verify output x y full|reduced [mutate]");
    Point residual{std::stoll(argv[2]),std::stoll(argv[3])};
    auto [r,s] = residual;
    require(0<=r && r<p && 0<=s && s<p,"noncanonical residual");
    require(mod(s*s)==mod(mod(r*r)*r+mod(curve_a*r)+curve_b),"off-curve residual");
    std::string variant=argv[4];
    require(variant=="reduced" || variant=="full","unknown variant");
    bool reduced=variant=="reduced";
    auto ideal=build(residual,reduced);
    std::ifstream in(argv[1]);
    require(bool(in),"missing solver output");
    std::array<bool,8> witnessed{};
    std::array<bool,7> assigned{};
    std::array<bool,3> reconstructed{};
    std::array<Int,7> values{};
    Poly replay;
    bool unit=false, point=false;
    std::size_t terms=0, printed_terms=0;
    unsigned degree=0, printed_degree=0;
    for (std::string line; std::getline(in,line); ) {
        require(line.find("? error") == std::string::npos,"solver reported an error");
        if (line == "UNIT") unit=true;
        if (line == "POINT") point=true;
        if (line.starts_with("VALUE")) {
            unsigned i=line.at(5)-'1';
            require(i < (reduced?4:7) && !assigned[i],"invalid or repeated value");
            assigned[i]=true;
            values[i]=mod(std::stoll(line.substr(7)));
        }
        if (line.starts_with("W") && line.size()>3 && line[2]==' ') {
            unsigned i=line[1]-'1';
            require(i<ideal.equations.size() && !witnessed[i],"invalid or repeated multiplier");
            witnessed[i]=true;
            auto w=read_poly(line.substr(3));
            if (argc==6 && i==0) scaled_add(w,constant(1),1);
            for (auto [e,c] : w) {
                require(!reduced || (e>>32)==0,"reduced certificate uses eliminated variables");
                unsigned term_degree=0;
                for (unsigned j=0; j<7; ++j) term_degree+=(e>>(8*j))&255;
                degree=std::max(degree,term_degree);
            }
            terms+=w.size();
            scaled_add(replay,multiply(w,ideal.equations[i]),1);
        }
        if (line.starts_with("R") && line.size()>3 && line[2]==' ') {
            unsigned i=line[1]-'0';
            require(i<3 && !reconstructed[i] && reduced,"invalid reconstruction record");
            reconstructed[i]=true;
            require(read_poly(line.substr(3))==ideal.reconstruction[i],"reconstruction polynomial differs");
        }
        if (line.starts_with("witness_terms ")) printed_terms=std::stoul(line.substr(14));
        if (line.starts_with("witness_degree ")) printed_degree=std::stoul(line.substr(15));
    }
    if (reduced) require(std::all_of(reconstructed.begin(),reconstructed.end(),[](bool b){return b;}),
                         "missing reconstruction records");
    require(unit!=point,"missing or conflicting solver conclusion");
    if (unit) {
        if (std::none_of(witnessed.begin(),witnessed.end(),[](bool b){return b;})) {
            require(argc==5,"no identity to mutate");
            std::cout << "decision only\n";
            return 0;
        }
        require(std::all_of(witnessed.begin(),witnessed.begin()+ideal.equations.size(),[](bool b){return b;}),
                "missing multipliers");
        require(replay==constant(1),"identity does not equal one");
        require(terms==printed_terms && degree==printed_degree,"identity count differs");
        std::cout << "identity_REPLAY terms=" << terms << " degree=" << degree << '\n';
    } else {
        require(std::all_of(assigned.begin(),assigned.begin()+(reduced?4:7),[](bool b){return b;}),
                "missing point coefficients");
        if (reduced) for (unsigned j=0;j<3;++j) values[4+j]=evaluate(ideal.reconstruction[j],values);
        replay_point(residual,values);
        std::cout << "point_REPLAY";
        for (Int c:values) std::cout << ' ' << c;
        std::cout << '\n';
    }
    return 0;
}

int main(int argc, char** argv) {
    try { return run(argc,argv); }
    catch (const std::exception& error) { std::cerr << error.what() << '\n'; return 1; }
}
