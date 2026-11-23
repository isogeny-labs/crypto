// Audit saved public answers and summary arithmetic, without solver replay.
// Requires nlohmann-json 3.11.3 and OpenSSL libcrypto; see AUDIT.md.
#include <nlohmann/json.hpp>
#include <openssl/evp.h>
#include <algorithm>
#include <cmath>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <numeric>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
using json = nlohmann::json;
using u64 = uint64_t;
namespace fs = std::filesystem;

static void require(bool ok, const std::string& message) {
    if (!ok) throw std::runtime_error(message);
}
static std::string read(const fs::path& path) {
    std::ifstream in(path, std::ios::binary);
    require(bool(in), "cannot read " + path.string());
    std::ostringstream out; out << in.rdbuf(); return out.str();
}
static std::string sha256(const std::string& data) {
    unsigned char digest[EVP_MAX_MD_SIZE]; unsigned size = 0;
    require(EVP_Digest(data.data(), data.size(), digest, &size, EVP_sha256(), nullptr) == 1,
            "SHA-256 failed");
    std::ostringstream out; out << std::hex << std::setfill('0');
    for (unsigned i = 0; i < size; ++i) out << std::setw(2) << unsigned(digest[i]);
    return out.str();
}
static bool prime(u64 n) {
    if (n < 2) return false;
    for (u64 d = 2; d <= n/d; ++d) if (n%d == 0) return false;
    return true;
}
struct Point {
    u64 x = 0, y = 0; bool inf = true;
    bool operator==(const Point& r) const {
        return inf == r.inf && (inf || (x == r.x && y == r.y));
    }
};
struct Curve {
    u64 p, a, b;
    u64 mul(u64 x, u64 y) const { return (x*y)%p; }
    u64 power(u64 x, u64 n) const {
        u64 r = 1;
        while (n) { if (n&1) r = mul(r,x); x = mul(x,x); n >>= 1; }
        return r;
    }
    bool on(Point q) const {
        return q.inf || (q.x < p && q.y < p &&
               mul(q.y,q.y) == (mul(mul(q.x,q.x),q.x)+mul(a,q.x)+b)%p);
    }
    Point plus(Point q, Point r) const {
        if (q.inf) return r;
        if (r.inf) return q;
        if (q.x == r.x && (q.y+r.y)%p == 0) return {};
        u64 numerator, denominator;
        if (q == r) { numerator = (3*mul(q.x,q.x)+a)%p; denominator = 2*q.y%p; }
        else { numerator = (r.y+p-q.y)%p; denominator = (r.x+p-q.x)%p; }
        require(denominator != 0, "zero denominator");
        u64 slope = mul(numerator,power(denominator,p-2));
        u64 x = (mul(slope,slope)+2*p-q.x-r.x)%p;
        return {x,(mul(slope,(q.x+p-x)%p)+p-q.y)%p,false};
    }
    Point times(u64 k, Point q) const {
        Point r;
        // Do not reduce k modulo the asserted group order.
        while (k) { if (k&1) r = plus(r,q); q = plus(q,q); k >>= 1; }
        return r;
    }
};
static Point point(const json& j) {
    require(j.is_array() && j.size() == 2 && j[0].is_number_unsigned() &&
            j[1].is_number_unsigned(), "point encoding");
    return {j[0].get<u64>(),j[1].get<u64>(),false};
}
static u64 number(const json& j, const std::string& key) {
    const auto& value = j.at(key);
    require(value.is_number_unsigned(), "nonnegative integer required: " + key);
    return value.get<u64>();
}
static std::string fraction(u64 numerator, u64 denominator) {
    require(denominator != 0, "zero ratio denominator");
    u64 d = std::gcd(numerator,denominator);
    return std::to_string(numerator/d)+"/"+std::to_string(denominator/d);
}
int main(int argc, char** argv) {
    try {
        require(argc == 2, "usage: audit_recorded STUDY_DIRECTORY");
        fs::path root = argv[1];
        auto raw = read(root/"RUNS.jsonl");
        auto summary = json::parse(read(root/"RESULTS.json"));
        require(sha256(raw) == summary.at("runs_sha256"), "raw digest");
        require(sha256(read(root/"prime31_ic.cpp")) == summary.at("source_sha256"), "source digest");
        // Fix the arithmetic domain before multiplying: all products fit u64.
        require(number(summary,"field_prime") == 2147483647ULL &&
                number(summary,"prime_order") == 2147446831ULL, "unexpected audit domain");
        Curve curve{number(summary,"field_prime"),number(summary,"curve_a"),number(summary,"curve_b")};
        u64 n = number(summary,"prime_order");
        require(curve.a < curve.p && curve.b < curve.p, "curve coefficient range");
        require(prime(curve.p) && prime(n), "primality");
        u64 disc = (4*curve.mul(curve.mul(curve.a,curve.a),curve.a)+
                    27*curve.mul(curve.b,curve.b))%curve.p;
        require(disc != 0, "singular curve");
        Point g = point(summary.at("generator"));
        require(curve.on(g) && curve.times(n,g).inf, "generator order");
        u64 root4p = u64(std::sqrt(4.0L*curve.p));
        while ((root4p+1)*(root4p+1) <= 4*curve.p) ++root4p;
        while (root4p*root4p > 4*curve.p) --root4p;
        require(2*n > curve.p+1+root4p, "Hasse uniqueness");
        std::set<std::pair<u64,u64>> expected, seen, targets;
        for (const auto& size : summary.at("factor_base_sizes"))
            for (const auto& seed : summary.at("seeds"))
                require(expected.emplace(size.get<u64>(),seed.get<u64>()).second, "duplicate expected cell");
        std::vector<json> rows;
        std::istringstream input(raw); std::string line;
        while (std::getline(input,line)) {
            require(!line.empty(), "empty record");
            auto r = json::parse(line);
            auto size = number(r,"factor_base"), seed = number(r,"seed");
            require(seen.emplace(size,seed).second, "duplicate record");
            require(r.at("status") == "complete", "incomplete record");
            require(number(r,"p") == curve.p && number(r,"a") == curve.a &&
                    number(r,"b") == curve.b && number(r,"subgroup_order") == n &&
                    point(r.at("generator")) == g, "curve mismatch");
            auto rank = number(r,"independent_rank"), dependent = number(r,"dependent_relations");
            auto positive = number(r,"positive_relations"), trials = number(r,"trials");
            require(rank == size, "recorded rank assertion");
            require(dependent <= positive && positive-dependent == rank && positive <= trials,
                    "relation accounting");
            u64 d = number(r,"verified_log");
            require(d < n && d == number(r,"secret"), "scalar range or known answer");
            Point q = point(r.at("target"));
            require(curve.on(q) && curve.times(d,g) == q, "public answer");
            targets.emplace(q.x,q.y); rows.push_back(std::move(r));
        }
        require(seen == expected, "panel coverage");
        json output = {{"standing","[pcc:unverified]"},
                       {"scope","recorded-answer and aggregation audit; no execution replay"},
                       {"rows_checked",rows.size()},{"distinct_targets",targets.size()},
                       {"curve_order_certificate","prime divisor plus Hasse uniqueness"}};
        for (const auto& size : summary.at("factor_base_sizes")) {
            auto size_string = std::to_string(size.get<u64>());
            std::vector<json> panel;
            for (const auto& r : rows) if (r.at("factor_base") == size) panel.push_back(r);
            const auto& recorded = summary.at("data").at(size_string);
            unsigned checked = 0;
            for (auto it = recorded.begin(); it != recorded.end(); ++it) {
                if (!it.value().is_object()) continue;
                std::vector<u64> values;
                for (const auto& r : panel) values.push_back(number(r,it.key()));
                long double total = std::accumulate(values.begin(),values.end(),0.0L);
                require(number(it.value(),"minimum") == *std::min_element(values.begin(),values.end()) &&
                        number(it.value(),"maximum") == *std::max_element(values.begin(),values.end()), it.key());
                require(std::abs(it.value().at("mean").get<long double>()-total/values.size()) <= .000501L,
                        it.key()+" mean");
                checked += 3;
            }
            json exact;
            for (std::string control : {"bsgs","rho"}) {
                u64 total_ic = 0, total_control = 0; long double sum_ratios = 0;
                for (const auto& r : panel) {
                    auto ic = number(r,"ic_additions"), c = number(r,control+"_additions");
                    require(c != 0 && UINT64_MAX-total_ic >= ic && UINT64_MAX-total_control >= c,
                            "ratio overflow or zero denominator");
                    total_ic += ic; total_control += c; sum_ratios += (long double)ic/c;
                }
                // Admit only the error implied by the producer's 3-decimal mean rounding.
                auto observed = recorded.at("ratio_of_mean_adds_ic_to_"+control).get<long double>();
                require(std::abs(observed-(long double)total_ic/total_control) <=
                        .000501L*panel.size()/total_control, "ratio of means");
                auto per_target = recorded.at("mean_of_per_target_ratio_ic_to_"+control).get<long double>();
                require(std::abs(per_target-sum_ratios/panel.size()) <=
                        1e-12L*std::max(1.0L,std::abs(per_target)), "mean of ratios");
                exact[control] = fraction(total_ic,total_control); checked += 2;
            }
            output["aggregates"][size_string] = {{"summary_fields_checked",checked},
                                                  {"exact_ratios_of_sum_additions",exact}};
        }
        std::cout << output.dump() << '\n';
    } catch (const std::exception& e) {
        std::cerr << "AUDIT ERROR: " << e.what() << '\n'; return 1;
    }
}
