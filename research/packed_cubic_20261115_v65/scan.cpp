// Exact GF(2^83) prefix-domain norm-family scan. No Groebner basis is used.
// Field/header cross-checks are independent of this scanner. Accepted cubics
// require charged factor/lift/group replay by the Python worker. Any cap or
// deadline returns unresolved, never a partial empty certificate.
#include "../k0_zero_cubic_20261115_v43/lab_variant_02/native_gf83.h"
#include <array>
#include <chrono>
#include <cstdint>
#include <iostream>
#include <stdexcept>
#include <string>
#include <vector>
#include <memory>

using E = gf83::Element;
static constexpr E ORDER = E(1) << 83;
static constexpr E MODULUS = ORDER | (E(1) << 45) | 7;
static bool force_portable = false;
static bool use_constant_gray = false;
static E curve_A=0,curve_B=1;
static uint64_t inverse_calls=0,inverse_power_fallbacks=0;
static E mul(E a, E b) { return force_portable ? gf83::mul_portable(a,b) : gf83::mul(a,b); }
static E square(E a) { return force_portable ? gf83::square_portable(a) : gf83::square(a); }
static E xtime(E a) { a <<= 1; return a & ORDER ? a ^ MODULUS : a; }

static std::string hex(E value) {
    if (!value) return "0x0";
    std::string out;
    while (value) { out.push_back("0123456789abcdef"[unsigned(value & 15)]); value >>= 4; }
    std::string result="0x";
    for(auto it=out.rbegin();it!=out.rend();++it) result.push_back(*it);
    return result;
}
static E parse_element(const std::string& input) {
    if (input.size()<3 || input.substr(0,2)!="0x") throw std::runtime_error("field elements require 0x hexadecimal");
    E value=0;
    for(size_t i=2;i<input.size();++i) {
        char c=input[i]; unsigned digit;
        if(c>='0'&&c<='9') digit=c-'0';
        else if(c>='a'&&c<='f') digit=c-'a'+10;
        else if(c>='A'&&c<='F') digit=c-'A'+10;
        else throw std::runtime_error("invalid hexadecimal field element");
        if(value >= (ORDER>>4)) throw std::runtime_error("field element out of range");
        value=(value<<4)|digit;
    }
    if(value>=ORDER) throw std::runtime_error("field element out of range");
    return value;
}
static int bit_index(E a) {
    uint64_t hi=uint64_t(a>>64);
    return hi ? 127-__builtin_clzll(hi) : 63-__builtin_clzll(uint64_t(a));
}
struct Row {
    E lo=0, hi=0;
    bool nonzero() const { return lo || hi; }
    int leading() const { return hi ? 83+bit_index(hi) : bit_index(lo); }
    Row& operator^=(const Row& rhs) { lo^=rhs.lo; hi^=rhs.hi; return *this; }
};
struct Family {
    bool consistent=true;
    int rank=0;
    E particular=0;
    std::vector<E> kernel;
    int reduced_rank=-1,coordinate_count=83;
};
#include "packed.inc"
// Exact Gold-polarization quotient. For p1=e1 in V, p3 is in e1^3+C;
// Newton's identity makes the shared condition e1*e2+e3 in C.
// Low quotient coordinates are packed through fixed byte tables.
struct GoldProjection {
    int width=0,rank=0,free_count=0,bytes=0;
    std::array<E,83> pivots{};
    std::array<std::array<uint8_t,256>,11> tables{};
    explicit GoldProjection(int dimension):width(dimension?3*dimension-2:0) {
        for(int i=0;i<dimension;++i)for(int j=i+1;j<dimension;++j) {
            E value=(E(1)<<(2*i+j))^(E(1)<<(i+2*j));
            while(value) {int p=bit_index(value);if(pivots[p])value^=pivots[p];else{pivots[p]=value;++rank;break;}}
        }
        std::vector<int> free;
        for(int i=0;i<width;++i)if(!pivots[i])free.push_back(i);
        free_count=int(free.size());
        if(free_count>8)throw std::runtime_error("Gold quotient packing exceeds byte capacity");
        std::array<uint8_t,83> unit_projection{};
        for(int i=0;i<width;++i) {
            E value=E(1)<<i;
            for(int p=width-1;p>=0;--p)if(pivots[p]&&((value>>p)&1))value^=pivots[p];
            uint8_t packed=0;
            for(size_t k=0;k<free.size();++k)if((value>>free[k])&1)packed|=uint8_t(1)<<k;
            unit_projection[i]=packed;
        }
        bytes=(width+7)/8;
        for(int i=0;i<bytes;++i)for(int value=0;value<256;++value) {
            uint8_t packed=0;
            for(int bit=0;bit<8;++bit)if((value>>bit)&1)if(8*i+bit<width)packed^=unit_projection[8*i+bit];
            tables[i][value]=packed;
        }
    }
    E project(E value)const {
        E result=(value>>width)<<free_count;uint8_t low=0;
        for(int i=0;i<bytes;++i)low^=tables[i][unsigned((value>>(8*i))&255)];
        return result^low;
    }
};
static Family solve(const std::array<Row,83>& columns, Row rhs,int variable_count=83) {
    std::array<Row,166> pivots{};
    std::array<E,166> coordinates{};
    std::array<bool,166> occupied{};
    Family result;
    for(int index=0;index<variable_count;++index) {
        Row value=columns[index]; E coordinate=E(1)<<index;
        while(value.nonzero()) {
            int p=value.leading();
            if(occupied[p]) { value^=pivots[p]; coordinate^=coordinates[p]; }
            else { occupied[p]=true; pivots[p]=value; coordinates[p]=coordinate; ++result.rank; break; }
        }
        if(!value.nonzero()) result.kernel.push_back(coordinate);
    }
    while(rhs.nonzero()) {
        int p=rhs.leading();
        if(!occupied[p]) { result.consistent=false; return result; }
        rhs^=pivots[p]; result.particular^=coordinates[p];
    }
    return result;
}
// HT is a fixed linear map for this certified odd-degree field. Build the
// entire map once; each application then uses11 byte lookups and XORs.
struct HalfTrace {
    std::array<std::array<E,256>,11> tables{};
    E trace_mask=0;
    HalfTrace() {
        std::array<E,83> columns{};
        for(int i=0;i<83;++i) {
            E value=E(1)<<i,half=0,power=value;
            for(int j=0;j<42;++j){half^=power;power=square(square(power));}
            columns[i]=half;
            E trace=0;power=value;
            for(int j=0;j<83;++j){trace^=power;power=square(power);}
            if(trace>1)throw std::runtime_error("absolute trace was not binary");
            if(trace)trace_mask|=E(1)<<i;
            if((square(half)^half)!=(value^trace))throw std::runtime_error("half-trace identity failed");
        }
        // This mask is independently checked in Python, and verified at
        // startup by all83 basis traces before its cheap evaluation is used.
        if(trace_mask!=(E(1)|(E(1)<<81)))throw std::runtime_error("unexpected GF83 trace mask");
        for(int i=0;i<11;++i)for(int value=0;value<256;++value) {
            E out=0;for(int bit=0;bit<8;++bit)if((value>>bit)&1)if(8*i+bit<83)out^=columns[8*i+bit];
            tables[i][value]=out;
        }
    }
    E apply(E value)const {E out=0;for(int i=0;i<11;++i)out^=tables[i][unsigned((value>>(8*i))&255)];return out;}
    bool trace(E value)const {return ((value^(value>>81))&1)!=0;}
};
static E inverse_power(E value) {
    // Itoh-style addition chain:82 squarings and8 multiplications.
    E f2=mul(square(value),value),f4=f2;
    for(int i=0;i<2;++i)f4=square(f4);f4=mul(f4,f2);
    E f8=f4;for(int i=0;i<4;++i)f8=square(f8);f8=mul(f8,f4);
    E f16=f8;for(int i=0;i<8;++i)f16=square(f16);f16=mul(f16,f8);
    E f32=f16;for(int i=0;i<16;++i)f32=square(f32);f32=mul(f32,f16);
    E f64=f32;for(int i=0;i<32;++i)f64=square(f64);f64=mul(f64,f32);
    E f80=f64;for(int i=0;i<16;++i)f80=square(f80);f80=mul(f80,f16);
    return square(mul(square(square(f80)),f2));
}
static E inverse(E value) {
    if(!value)throw std::runtime_error("inverse of zero");
    ++inverse_calls;
    E u=value,v=MODULUS,left=1,right=0;
    while(u!=1) {
        int shift=bit_index(u)-bit_index(v);
        if(shift<0){std::swap(u,v);std::swap(left,right);shift=-shift;}
        // Preserve exactness even if an unusually large coefficient shift
        // would overflow128 bits: use a bounded exact field-power fallback.
        if(shift&&right>>(128-shift)){++inverse_power_fallbacks;E result=inverse_power(value);if(mul(value,result)!=1)throw std::runtime_error("inverse fallback failed");return result;}
        u^=v<<shift;left^=right<<shift;
    }
    E result=gf83::reduce_product(left,0);
    if(mul(value,result)!=1)throw std::runtime_error("extended Euclid inverse failed");
    return result;
}
static Family artin_schreier_family(E b,E e1,E e20,E e30,E z,int w2,const GoldProjection& gold,const HalfTrace& ht) {
    E inv=square(inverse(b)),b2=square(b),a0=mul(e20,inv),c0=mul(b,ht.apply(a0));
    bool trace0=ht.trace(a0);E e1b2=mul(e1,b2);
    // Use ACTUAL c-image coefficients, including the trace correction for
    // individual trace1 basis columns. The trace row is kept explicitly.
    E moment0=(trace0?e1b2:0)^mul(z,square(c0))^e30;
    std::array<Row,83> columns{};std::array<E,83> images{};
    E ai=inv,e1unit=e1;
    for(int i=0;i<w2;++i) {
        bool tr=ht.trace(ai);E image=mul(b,ht.apply(ai));images[i]=image;
        columns[i]={tr?E(1):E(0),gold.project(e1unit^(tr?e1b2:0)^mul(z,square(image)))};
        ai=xtime(ai);e1unit=xtime(e1unit);
    }
    images[w2]=b;columns[w2]={0,gold.project(mul(z,b2))};
    int count=w2+1;Family reduced=solve(columns,{trace0?E(1):E(0),gold.project(moment0)},count);
    auto image_of=[&](E coordinates){E out=0;while(coordinates){int i=bit_index(coordinates);out^=images[i];coordinates^=E(1)<<i;}return out;};
    Family result;result.consistent=reduced.consistent;result.particular=c0^image_of(reduced.particular);
    for(E coordinates:reduced.kernel){E mapped=image_of(coordinates);if(!mapped)throw std::runtime_error("AS homogeneous coordinate map was not injective");result.kernel.push_back(mapped);}
    result.rank=83-int(result.kernel.size());result.reduced_rank=reduced.rank;result.coordinate_count=count;
    return result;
}
#include "block.inc"
struct Cubic {
    E e1,e2,e3,x4a,x4b,x4c;
    Cubic(E a,E b,E c):e1(a),e2(b),e3(c),x4a(mul(a,c)),x4b(c^mul(a,b)),x4c(b^square(a)){}
};
using Remainder=std::array<E,3>;
static Remainder square_mod(const Remainder& r,const Cubic& h) {
    E top=square(r[2]);
    return {square(r[0])^mul(top,h.x4a),mul(top,h.x4b),square(r[1])^mul(top,h.x4c)};
}
static std::vector<E> domain_scalars(int dimension) {
    std::vector<E> coefficients{1},scalars;
    for(int i=0;i<dimension;++i) {
        E value=0,power=E(1)<<i;
        for(E c:coefficients) { value^=mul(c,power); power=square(power); }
        scalars.push_back(value);
        std::vector<E> next(coefficients.size()+1,0);
        for(size_t j=0;j<coefficients.size();++j) { next[j]^=mul(value,coefficients[j]); next[j+1]^=square(coefficients[j]); }
        coefficients=std::move(next);
    }
    return scalars;
}
static bool supported(const Cubic& h,const std::vector<E>& scalars) {
    Remainder value{0,1,0};
    for(E scalar:scalars) {
        Remainder next=square_mod(value,h);
        for(int j=0;j<3;++j) next[j]^=mul(scalar,value[j]);
        value=next;
    }
    // Nilpotence exponent4 preserves every multiplicity up to degree3.
    value=square_mod(square_mod(value,h),h);
    return !(value[0]||value[1]||value[2]);
}
struct Accepted { E b,c,e1,e2,e3; };
struct FamilyRecord {E b;Family family;};
struct Stats {
    uint64_t outer_size=0,outer_visited=0,inner_inconsistent=0,candidates_planned=0,candidates_visited=0;
    int outer_rank=0,outer_nullity=0,refused_nullity=-1;
    int gold_rank=0,gold_joint_rank=0;
    uint64_t as_reduced_families=0,zero_b_fallback_families=0;
    std::array<uint64_t,84> reduced_rank_hist{},coordinate_count_hist{};
    std::vector<FamilyRecord> family_records;
    std::array<uint64_t,84> rank_hist{},nullity_hist{};
};
static void histogram(const std::array<uint64_t,84>& values) {
    std::cout<<"{"; bool first=true;
    for(size_t i=0;i<values.size();++i) if(values[i]) { if(!first)std::cout<<","; first=false; std::cout<<"\""<<i<<"\":"<<values[i]; }
    std::cout<<"}";
}
static void output(bool complete,const std::string& reason,E z,E y,int dimension,
                   const Stats& s,const std::vector<Accepted>& accepted) {
    if(profile_enabled)profile_total_ns=std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::steady_clock::now()-profile_start).count();
    std::cout<<"{\"schema\":\"binary_triple_native_scan_v5_general_AB\",\"status\":\""<<(complete?"complete":"unresolved")
             <<"\",\"complete\":"<<(complete?"true":"false")<<",\"reason\":\""<<reason
             <<"\",\"field_degree\":83,\"field_modulus_hex\":\""<<hex(MODULUS)
             <<"\",\"target\":[\""<<hex(z)<<"\",\""<<hex(y)<<"\"],\"source_dimension\":"<<dimension
             <<",\"backend\":\""<<(force_portable?"portable":"runtime_dispatch")
             <<"\",\"hardware_pclmul_available\":"<<(gf83::pclmul_available()?"true":"false")
             <<",\"stats\":{\"outer_cardinality_exact\":"<<s.outer_size<<",\"outer_rank\":"<<s.outer_rank
             <<",\"outer_nullity\":"<<s.outer_nullity<<",\"outer_visited\":"<<s.outer_visited
             <<",\"inner_inconsistent\":"<<s.inner_inconsistent<<",\"candidates_planned\":"<<s.candidates_planned
             <<",\"candidates_visited\":"<<s.candidates_visited<<",\"refused_nullity\":"<<s.refused_nullity
             <<",\"gold_polarization_rank\":"<<s.gold_rank<<",\"joint_moment_rank\":"<<s.gold_joint_rank
             <<",\"as_reduced_families\":"<<s.as_reduced_families<<",\"zero_b_fallback_families\":"<<s.zero_b_fallback_families
             <<",\"field_inverse_calls\":"<<inverse_calls<<",\"inverse_power_fallbacks\":"<<inverse_power_fallbacks
             <<",\"packed_calls\":"<<packed_calls<<",\"wide_inner_calls\":"<<wide_calls<<",\"verified_packed_calls\":"<<verified_packed_calls
             <<",\"profile_setup_ns\":"<<profile_setup_ns<<",\"profile_fiber_ns\":"<<profile_fiber_ns<<",\"profile_support_ns\":"<<profile_support_ns<<",\"profile_total_ns\":"<<profile_total_ns
             <<",\"inner_rank_histogram\":"; histogram(s.rank_hist);
    std::cout<<",\"inner_nullity_histogram\":";histogram(s.nullity_hist);
    std::cout<<",\"reduced_rank_histogram\":";histogram(s.reduced_rank_hist);
    std::cout<<",\"coordinate_count_histogram\":";histogram(s.coordinate_count_hist);
    std::cout<<"},\"supported_norm_count"<<(complete?"_exact":"_lower_bound")<<"\":"<<accepted.size()<<",\"accepted_norms\":[";
    bool first=true;
    for(const auto& a:accepted) {
        if(!first)std::cout<<",";first=false;
        std::cout<<"{\"b\":\""<<hex(a.b)<<"\",\"c\":\""<<hex(a.c)<<"\",\"cubic\":[\""<<hex(a.e3)
                 <<"\",\""<<hex(a.e2)<<"\",\""<<hex(a.e1)<<"\",\"0x1\"]}";
    }
    std::cout<<"],\"conditional_family_records\":[";first=true;
    for(const auto& rec:s.family_records) {
        if(!first)std::cout<<",";first=false;
        std::cout<<"{\"b\":\""<<hex(rec.b)<<"\",\"rank\":"<<rec.family.rank
                 <<",\"nullity\":"<<rec.family.kernel.size()<<",\"consistent\":"<<(rec.family.consistent?"true":"false")
                 <<",\"reduced_rank\":"<<rec.family.reduced_rank<<",\"coordinate_count\":"<<rec.family.coordinate_count
                 <<",\"particular\":\""<<hex(rec.family.particular)<<"\",\"kernel\":[";
        bool innerfirst=true;for(E k:rec.family.kernel){if(!innerfirst)std::cout<<",";innerfirst=false;std::cout<<"\""<<hex(k)<<"\"";}
        std::cout<<"]}";
    }
    std::cout<<"],\"pruning\":\"joint_Gold_moment_AS_coordinates\",\"half_trace_mask\":\"0x200000000000000000001\",\"half_trace_table_bytes\":45056,\"half_trace_basis_columns_verified\":83,\"contract\":\"K_rational_norm_family_support_scan_requires_point_replay\"}\n";
}
int main(int argc,char** argv) {
    E z=0,y=0;int dimension=18,max_nullity=20;
    uint64_t max_outer=1u<<19,max_candidates=1u<<23,max_accepted=10000;
    uint64_t max_outer_visits=UINT64_MAX,record_families=0;
    double max_seconds=30;
    try {
        bool got_z=false,got_y=false;
        for(int i=1;i<argc;++i) {
            std::string option=argv[i];
            if(option=="--packed") {use_packed=true;continue;}
            if(option=="--verify-packed") {verify_packed=true;continue;}
            if(option=="--profile") {profile_enabled=true;continue;}
            if(option=="--constant-gray") {use_constant_gray=true;continue;}
            if(option=="--portable") { force_portable=true;continue; }
            if(i+1==argc) throw std::runtime_error("missing option value");
            std::string value=argv[++i];
            if(option=="--z") {z=parse_element(value);got_z=true;}
            else if(option=="--A") {curve_A=parse_element(value);}
            else if(option=="--B") {curve_B=parse_element(value);}
            else if(option=="--y") {y=parse_element(value);got_y=true;}
            else if(option=="--dimension") dimension=std::stoi(value);
            else if(option=="--max-nullity") max_nullity=std::stoi(value);
            else if(option=="--max-outer") max_outer=std::stoull(value);
            else if(option=="--max-candidates") max_candidates=std::stoull(value);
            else if(option=="--max-accepted") max_accepted=std::stoull(value);
            else if(option=="--max-outer-visits") max_outer_visits=std::stoull(value);
            else if(option=="--record-families") record_families=std::stoull(value);
            else if(option=="--max-seconds") max_seconds=std::stod(value);
            else throw std::runtime_error("unrecognized option");
        }
        if(!got_z||!got_y||dimension<0||dimension>28||max_nullity<0||max_nullity>62||!(max_seconds>0))
            throw std::runtime_error("unsupported or missing scan configuration");
        if(!curve_B)throw std::runtime_error("singular B0 curve");
        if(square(y)^mul(z,y)^mul(square(z),z)^mul(curve_A,square(z))^curve_B) throw std::runtime_error("target is not on declared A B1 curve");
        auto start=std::chrono::steady_clock::now();profile_start=start;
        auto expired=[&](){return std::chrono::duration<double>(std::chrono::steady_clock::now()-start).count()>=max_seconds;};
        Stats stats;std::vector<Accepted> accepted;GoldProjection gold(dimension);HalfTrace ht;
        stats.gold_rank=gold.rank;stats.gold_joint_rank=dimension+gold.rank;
        std::array<Row,83> outer_columns{};
        std::array<E,83> squares{},z_squares{};
        for(int i=0;i<83;++i) {
            E unit=E(1)<<i;squares[i]=square(unit);z_squares[i]=mul(z,squares[i]);
            outer_columns[i]={(squares[i]^unit)>>dimension,0};
        }
        Family outer=solve(outer_columns,{z>>dimension,0});
        stats.outer_rank=outer.rank;stats.outer_nullity=int(outer.kernel.size());
        stats.outer_size=outer.consistent?(uint64_t(1)<<outer.kernel.size()):0;
        if(stats.outer_size>max_outer) {output(false,"outer family cap",z,y,dimension,stats,accepted);return 3;}
        std::vector<E> scalars=domain_scalars(dimension);
        int w2=dimension?2*dimension-1:0,w3=dimension?3*dimension-2:0;
        E yt=y^z,z2=square(z),z3=mul(z2,z),qconstant=yt^mul(curve_A,z)^z2,b=outer.particular;
        std::unique_ptr<ConstantGray> constant; if(use_constant_gray&&outer.consistent)constant=std::make_unique<ConstantGray>(outer,z,qconstant,curve_A,w2,w3,gold);
        if(profile_enabled)profile_setup_ns=std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::steady_clock::now()-start).count();
        for(uint64_t outer_index=0;outer_index<stats.outer_size;++outer_index) {
            if(outer_index) b^=outer.kernel[__builtin_ctzll(outer_index)];
            if(expired()) {output(false,"deadline before next outer family",z,y,dimension,stats,accepted);return 3;}
            if(stats.outer_visited>=max_outer_visits){output(false,"outer visit cap before next family",z,y,dimension,stats,accepted);return 3;}
            ++stats.outer_visited;
            E b2=square(b),e1=b2^b^z,e20=mul(curve_A,b2)^mul(z,e1),e30=mul(b2,qconstant)^z3;
            std::chrono::steady_clock::time_point fiber_begin;if(profile_enabled)fiber_begin=std::chrono::steady_clock::now();
            Family inner;
            if(use_constant_gray) {inner=constant->next(outer_index);}
            else if(b) {inner=artin_schreier_family(b,e1,e20,e30,z,w2,gold,ht);++stats.as_reduced_families;}
            else {
                std::array<Row,83> columns{};E moment_square=e1^z;
                for(int i=0;i<83;++i){columns[i]={squares[i]>>w2,gold.project(moment_square)};moment_square=xtime(xtime(moment_square));}
                inner=solve(columns,{e20>>w2,gold.project(mul(e1,e20)^e30)});
                inner.reduced_rank=inner.rank;inner.coordinate_count=83;++stats.zero_b_fallback_families;
            }
            if(profile_enabled)profile_fiber_ns+=std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::steady_clock::now()-fiber_begin).count();
            ++stats.reduced_rank_hist[inner.reduced_rank];++stats.coordinate_count_hist[inner.coordinate_count];
            if(stats.family_records.size()<record_families)stats.family_records.push_back({b,inner});
            ++stats.rank_hist[inner.rank];++stats.nullity_hist[inner.kernel.size()];
            if(!inner.consistent) {++stats.inner_inconsistent;continue;}
            if(int(inner.kernel.size())>max_nullity) {stats.refused_nullity=int(inner.kernel.size());output(false,"inner nullity cap before enumeration",z,y,dimension,stats,accepted);return 3;}
            uint64_t count=uint64_t(1)<<inner.kernel.size();
            if(count>max_candidates-stats.candidates_planned) {output(false,"candidate cap before enumeration",z,y,dimension,stats,accepted);return 3;}
            stats.candidates_planned+=count;E c=inner.particular;
            for(uint64_t index=0;index<count;++index) {
                if(index)c^=inner.kernel[__builtin_ctzll(index)];
                if((index&255)==0&&expired()) {output(false,"deadline inside conditional family",z,y,dimension,stats,accepted);return 3;}
                ++stats.candidates_visited;
                E c2=square(c),e2=c2^mul(b,c)^e20,e3=mul(z,c2)^e30;
                E constant=z2^mul(c,z)^mul(b,yt);
                if(square(constant)^mul(curve_B,b2)^mul(z,e3)) throw std::runtime_error("literal norm constant remainder mismatch");
                bool ok;if(profile_enabled){auto t=std::chrono::steady_clock::now();ok=supported(Cubic(e1,e2,e3),scalars);profile_support_ns+=std::chrono::duration_cast<std::chrono::nanoseconds>(std::chrono::steady_clock::now()-t).count();}else ok=supported(Cubic(e1,e2,e3),scalars);if(!ok)continue;
                if(accepted.size()>=max_accepted) {output(false,"accepted norm cap requires replay",z,y,dimension,stats,accepted);return 3;}
                accepted.push_back({b,c,e1,e2,e3});
            }
        }
        output(true,"",z,y,dimension,stats,accepted);return 0;
    } catch(const std::exception& error) {
        std::cerr<<error.what()<<"\n";return 2;
    }
}
