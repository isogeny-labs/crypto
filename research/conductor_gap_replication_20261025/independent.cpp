// Separately written prime-field arithmetic and statistics; no crypto_lib.
// C++ deliberately provides a second implementation of the Rust checks.
#include <algorithm>
#include <array>
#include <cmath>
#include <cstdint>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <random>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <tuple>
#include <vector>
using U = uint64_t;
U mm(U a, U b, U p) { return U((__uint128_t)a * b % p); }
U pw(U a, U k, U p) {
    U z = 1;
    for (; k; k >>= 1, a = mm(a, a, p)) if (k & 1) z = mm(z, a, p);
    return z;
}
void require(bool b, const char* why) { if (!b) throw std::runtime_error(why); }
bool prime(U n) {
    if (n < 2) return false;
    for (U q : {2,3,5,7,11,13,17,19,23,29,31,37}) if (n % q == 0) return n == q;
    U d = n - 1; int s = 0;
    while (!(d & 1)) { d >>= 1; ++s; }
    for (U a : {2,325,9375,28178,450775,9780504,1795265022}) {
        if (a % n == 0) continue;
        U x = pw(a % n, d, n);
        if (x == 1 || x == n - 1) continue;
        bool ok = false;
        for (int j = 1; j < s; ++j) { x = mm(x,x,n); if (x == n-1) { ok = true; break; } }
        if (!ok) return false;
    }
    return true;
}
struct P {
    U x = 0, y = 0; bool inf = true;
    bool operator==(const P& b) const { return inf == b.inf && (inf || (x == b.x && y == b.y)); }
};
struct C {
    U p, a, b;
    bool on(P z) const { return z.inf || mm(z.y,z.y,p) == (mm(mm(z.x,z.x,p),z.x,p)+mm(a,z.x,p)+b)%p; }
    P add(P x, P y) const {
        if (x.inf) return y;
        if (y.inf) return x;
        if (x.x == y.x && (x.y+y.y)%p == 0) return {};
        U numerator = x == y ? (3*mm(x.x,x.x,p)+a)%p : (y.y+p-x.y)%p;
        U denominator = x == y ? 2*x.y%p : (y.x+p-x.x)%p;
        U s = mm(numerator,pw(denominator,p-2,p),p);
        U z = (mm(s,s,p)+2*p-x.x-y.x)%p;
        return {z,(mm(s,(x.x+p-z)%p,p)+p-x.y)%p,false};
    }
    P mul(P x, U k) const {
        P z;
        for (; k; k >>= 1, x = add(x,x)) if (k&1) z = add(z,x);
        return z;
    }
};
U integer(const std::string& s) {
    require(!s.empty() && s[0] != '-', "unsigned integer required");
    size_t end = 0;
    int base = s.rfind("0x",0) == 0 || s.rfind("0X",0) == 0 ? 16 : 10;
    U z = std::stoull(s,&end,base);
    require(end == s.size(), "malformed integer");
    return z;
}
std::vector<std::string> fields(const std::string& s) {
    std::vector<std::string> v; std::stringstream ss(s); std::string x;
    while (std::getline(ss,x,',')) v.push_back(x);
    return v;
}
struct Agg {
    long double sum = 0; U n = 0;
    void add(double x) { sum += x; ++n; }
    double mean() const { require(n > 0,"empty cell"); return double(sum/n); }
};
using Cell = std::tuple<int,int,int>;
using Control = std::tuple<int,std::string,int>;
using BCell = std::tuple<int,int,int,std::string>;
using Workload = std::array<U,10>;
double quant(std::vector<double> v, double q) {
    std::sort(v.begin(),v.end()); return v[std::llround((v.size()-1)*q)];
}
void interval(const std::vector<double>& logs, U seed) {
    require(logs.size() == 8,"eight classes required");
    std::mt19937_64 rng(seed); std::uniform_int_distribution<int> pick(0,7);
    std::vector<double> bs;
    for (int k=0;k<10000;++k) {
        double z=0; for (int j=0;j<8;++j) z += logs[pick(rng)];
        bs.push_back(std::exp(z/8));
    }
    double z=0; for (double x:logs) z += x;
    std::cout << "\"geometric_mean\":" << std::exp(z/8);
    const std::array<std::pair<const char*,double>,3> levels{{{"ci90",.05},{"ci95",.025},{"ci98",.01}}};
    for (auto a:levels) std::cout << ",\"" << a.first << "\":[" << quant(bs,a.second) << "," << quant(bs,1-a.second) << "]";
}
int main(int argc, char** argv) { try {
    require(argc == 3,"A.csv B.csv required");
    std::map<Cell,Agg> cells;
    std::map<Control,Agg> controls;
    std::map<BCell,Agg> b_cells;
    std::map<Workload,double> primary, aa;
    std::set<std::tuple<U,U,U,U,U,U>> checked;
    std::map<std::string,U> statuses;
    U rows=0, verified=0, mutations=0;
    std::array<U,2> panel_rows{};
    for (int panel=0;panel<2;++panel) {
        std::ifstream file(argv[panel+1]); require(bool(file),"CSV unreadable");
        std::string line; std::getline(file,line);
        while (std::getline(file,line)) {
            auto v=fields(line); require(v.size()==18,"CSV column count");
            ++rows; ++panel_rows[panel]; ++statuses[v[14]];
            C c{integer(v[5]),integer(v[6]),integer(v[7])}; U r=integer(v[8]);
            require(c.p < (U(1)<<40),"checker scope is the bounded study fixtures");
            P g{integer(v[9]),integer(v[10]),false}, q{integer(v[11]),integer(v[12]),false};
            require(c.a<c.p && c.b<c.p && g.x<c.p && g.y<c.p && q.x<c.p && q.y<c.p,"noncanonical coordinate");
            require(c.on(g) && c.on(q),"point off curve");
            auto key=std::make_tuple(c.p,c.a,c.b,r,g.x,g.y);
            if (checked.insert(key).second) {
                require(prime(c.p) && prime(r),"composite parameter");
                require((4*mm(mm(c.a,c.a,c.p),c.a,c.p)+27*mm(c.b,c.b,c.p))%c.p != 0,"singular curve");
                require(c.mul(g,r).inf,"wrong subgroup order");
            }
            if (v[14] != "verified") continue;
            ++verified; U k=integer(v[13]);
            require(k<r && c.mul(g,k)==q,"scalar recovery mismatch");
            require(!(c.mul(g,(k+1)%r)==q),"mutated scalar accepted"); ++mutations;
            double s=std::stod(v[15]), gae=std::stod(v[16]);
            require(std::isfinite(s) && std::isfinite(gae),"nonfinite cost");
            require(std::abs(s-gae/std::sqrt(double(r))) < 1e-10*std::max(1.,std::abs(s)),"S conversion mismatch");
            int ci=std::stoi(v[0]), volcano=std::stoi(v[1]), level=std::stoi(v[2]), model=std::stoi(v[3]);
            if (panel==0) {
                Workload w{U(ci),c.p,c.a,c.b,r,g.x,g.y,q.x,q.y,integer(v[17])};
                if (v[4]=="rho-neg") { cells[{ci,level,volcano}].add(s); require(primary.emplace(w,s).second,"duplicate primary"); }
                if (v[4]=="rho-neg-aa" || v[4]=="rho-plain") controls[{ci,v[4],model}].add(s);
                if (v[4]=="rho-neg-aa" && model==0) require(aa.emplace(w,s).second,"duplicate AA");
            } else b_cells[{ci,volcano,model,v[4]}].add(s);
        }
    }
    require(panel_rows[0]==41856 && panel_rows[1]==1664,"planned panel count mismatch");
    require(statuses.size()==1 && statuses["verified"]==rows,"incomplete or failed panel");
    std::array<long double,8> primary_sum{}, aa_sum{};
    std::array<U,8> aa_n{};
    for (const auto& [w,s]:primary) {
        auto it=aa.find(w); if (it==aa.end()) continue;
        primary_sum[w[0]]+=s; aa_sum[w[0]]+=it->second; ++aa_n[w[0]];
    }
    std::vector<double> positive, aa_logs, coordinate, overall(8,0.);
    for (int ci=0;ci<8;++ci) {
        require(aa_n[ci]==160,"AA pairing count");
        auto mean=[&](const char* arm,int model){return controls.at({ci,arm,model}).mean();};
        double n0=mean("rho-neg-aa",0),n1=mean("rho-neg-aa",1);
        double p0=mean("rho-plain",0),p1=mean("rho-plain",1);
        positive.push_back(std::log((p0+p1)/(n0+n1)));
        coordinate.push_back(std::log(n1/n0));
        aa_logs.push_back(std::log(double(aa_sum[ci]/primary_sum[ci])));
    }
    std::cout << std::setprecision(17)
        << "{\"schema\":\"conductor-gap.independent/v1\",\"implementation\":\"standalone C++20; no shared arithmetic or analyzer\","
        << "\"host_independence\":\"caller provenance required\",\"rows\":" << rows
        << ",\"verified\":" << verified << ",\"unique_curve_generators\":" << checked.size()
        << ",\"mutated_scalar_rejections\":" << mutations
        << ",\"bootstrap\":\"mt19937_64,10000 whole-class draws; seeds 2026102522+level,2528 overall,2527 positive,2529 AA,2530 coordinate\",\"levels\":[";
    for (int level=0;level<5;++level) {
        if (level) std::cout << ",";
        std::vector<double> logs;
        std::cout << "{\"level\":" << level << ",\"class_ratios\":[";
        for (int ci=0;ci<8;++ci) {
            for (int v=0;v<4;++v) require(cells.at({ci,level,v}).n==192,"level effort mismatch");
            double floors=0; for (int v=1;v<4;++v) floors+=cells.at({ci,level,v}).mean()/3;
            double ratio=floors/cells.at({ci,level,0}).mean();
            logs.push_back(std::log(ratio)); overall[ci]+=std::log(ratio)/5;
            if (ci) { std::cout << ","; }
            std::cout << ratio;
        }
        std::cout << "],"; interval(logs,2026102522+level); std::cout << "}";
    }
    std::cout << "],\"equal_level_secondary\":{"; interval(overall,2026102528);
    std::cout << "},\"positive_control\":{"; interval(positive,2026102527);
    std::cout << "},\"aa\":{"; interval(aa_logs,2026102529);
    std::cout << "},\"coordinate\":{"; interval(coordinate,2026102530);
    std::cout << "},\"matched_aa_pairs\":1280,\"b_cells\":[";
    require(b_cells.size()==104,"B cell coverage"); bool first=true;
    for (const auto& [key,x]:b_cells) {
        require(x.n==16,"B targets per cell");
        if (!first) { std::cout << ","; }
        first=false;
        std::cout << "{\"class\":" << std::get<0>(key) << ",\"volcano\":" << std::get<1>(key)
            << ",\"model\":" << std::get<2>(key) << ",\"arm\":\"" << std::get<3>(key)
            << "\",\"verified\":" << x.n << ",\"mean_s\":" << x.mean() << "}";
    }
    std::cout << "],\"verdict\":\"all arithmetic checks passed; headlines independently recomputed; bootstrap is an independent Monte Carlo check\"}\n";
    return 0;
} catch (const std::exception& e) { std::cerr << e.what() << "\n"; return 1; } }
