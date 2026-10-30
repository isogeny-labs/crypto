#include "io.hpp"
#include <gmpxx.h>
#include <algorithm>
#include <iostream>
#include <map>
#include <set>
using Text = std::vector<std::string>;
static mpz_class number(const std::string& s) {
    mpz_class value;
    require(value.set_str(s, 0) == 0, "integer parse");
    return value;
}
static mpz_class mod(mpz_class x, const mpz_class& p) {
    mpz_mod(x.get_mpz_t(), x.get_mpz_t(), p.get_mpz_t());
    return x;
}
struct Point { mpz_class x, y; bool infinity = false; };
static Point add(const Point& u, const Point& v, const mpz_class& p) {
    if (u.infinity) return v;
    if (v.infinity) return u;
    if (u.x == v.x && mod(u.y + v.y, p) == 0) return {0, 0, true};
    mpz_class top, bottom, inverse;
    if (u.x == v.x) { top = 3*u.x*u.x-3; bottom = 2*u.y; }
    else { top = v.y-u.y; bottom = v.x-u.x; }
    bottom = mod(bottom, p);
    require(mpz_invert(inverse.get_mpz_t(), bottom.get_mpz_t(), p.get_mpz_t()) != 0,
            "noninvertible point denominator");
    mpz_class slope = mod(top*inverse, p), x = mod(slope*slope-u.x-v.x, p);
    return {x, mod(slope*(u.x-x)-u.y, p), false};
}
static Point mul(Point q, mpz_class n, const mpz_class& p) {
    Point result{0, 0, true};
    while (n > 0) {
        if (mpz_odd_p(n.get_mpz_t())) result = add(result, q, p);
        q = add(q, q, p);
        n >>= 1;
    }
    return result;
}
struct Branch {
    Text c;
    mpz_class p, b, n, x, y, disc;
    std::vector<int> s, r;
};
static std::string intervals(const std::vector<int>& good) {
    std::ostringstream out;
    out << '[';
    bool comma = false;
    for (size_t i = 0; i < good.size();) {
        size_t j = i;
        while (j+1 < good.size() && good[j+1] == good[j]+1) ++j;
        if (comma) out << ',';
        comma = true;
        out << '[' << good[i] << ',' << good[j] << ']';
        i = j+1;
    }
    out << ']';
    return out.str();
}
static void self_test() {
    mpz_class p = 17;
    Point g{0, 1, false};
    auto q = mul(g, 2, p);
    require(q.x == 15 && q.y == 13 && !q.infinity, "known doubling");
    require(mul(g, 10, p).infinity && mul(g, 5, p).y == 0, "known order10");
    require(intervals({691, 692, 769}) == "[[691,692],[769,769]]", "interval control");
    std::cout << "PASS known F17 point/order and interval controls\n";
}
int main(int argc, char** argv) {
try {
    if (argc == 2 && std::string(argv[1]) == "--self-test") { self_test(); return 0; }
    require(argc == 5 && std::string(argv[4]) == "ACK_LOCAL_EXACT_REANALYSIS",
            "producer INPUTS PRIOR21 FRESH_OUTPUT ACK_LOCAL_EXACT_REANALYSIS");
    auto input = table(argv[1], 14), prior = table(argv[2], 9);
    require(input.size() == 82 && prior.size() == 21, "full input inventories");
    std::vector<int> primes;
    for (int n = 2; n <= 5000; ++n) {
        bool prime = true;
        for (int d = 2; d <= n/d; ++d) if (n%d == 0) { prime = false; break; }
        if (prime) primes.push_back(n);
    }
    require(primes.size() == 669, "prime inventory");
    std::vector<Branch> branches;
    std::map<std::string, size_t> chosen;
    std::set<std::string> ids;
    std::ostringstream characters;
    characters << "branch\tell\tcharacter\tstrict\tramified\tinclusive\n";
    for (const auto& c : input) {
        require(ids.insert(c[0]).second, "duplicate branch");
        Branch a{c, number(c[7]), number(c[8]), number(c[9]), number(c[10]), number(c[11]), 0, {}, {}};
        require(a.p > 5000 && a.n > 0 && a.b >= 0 && a.b < a.p, "field/coefficient bounds");
        require(a.x >= 0 && a.x < a.p && a.y >= 0 && a.y < a.p, "point bounds");
        require(mod(a.y*a.y-a.x*a.x*a.x+3*a.x-a.b, a.p) == 0, "point equation");
        require(mod(27*a.b*a.b-108, a.p) != 0, "nonsingular");
        mpz_class trace = a.p+1-a.n;
        a.disc = trace*trace-4*a.p;
        require(a.disc < 0 && a.n*a.n > 16*a.p, "Hasse/uniqueness");
        require(mul({a.x, a.y, false}, a.n, a.p).infinity, "order-point certificate");
        if (chosen.count(c[1])) {
            const auto& before = branches[chosen.at(c[1])];
            require(before.disc == a.disc && before.c[2] == c[2] && before.c[3] == c[3]
                    && before.c[4] == c[4] && before.c[6] == c[6], "branch/seed invariance");
            if (c[5] == "canonical") chosen[c[1]] = branches.size();
        } else chosen[c[1]] = branches.size();
        int s = 0, r = 0;
        for (int ell : primes) {
            if (ell == 2) {
                characters << c[0] << "\t2\tNA_ell2\t0\t0\t0\n";
            } else {
                std::vector<bool> squares(static_cast<size_t>(ell));
                for (int z = 1; z < ell; ++z) squares[static_cast<size_t>(z*z%ell)] = true;
                auto rem = mpz_fdiv_ui(a.disc.get_mpz_t(), static_cast<unsigned long>(ell));
                int ch = rem == 0 ? 0 : squares[rem] ? 1 : -1;
                s += ch == 1; r += ch == 0;
                characters << c[0] << '\t' << ell << '\t' << ch << '\t' << s << '\t' << r << '\t' << s+r << '\n';
            }
            a.s.push_back(s); a.r.push_back(r);
        }
        branches.push_back(std::move(a));
    }
    require(chosen.size() == 69, "all69 seed positions");
    std::vector<size_t> seeds;
    for (size_t i = 0; i < branches.size(); ++i)
        if (chosen.at(branches[i].c[1]) == i) seeds.push_back(i);
    int pubs = 0, narrow = 0;
    for (size_t i : seeds) { pubs += branches[i].c[3] == "published"; narrow += branches[i].c[6] == "1"; }
    require(pubs == 10 && narrow == 21, "seed label inventory");
    for (const auto& old : prior) {
        require(chosen.count(old[0]), "prior identity missing");
        const auto& a = branches[chosen.at(old[0])];
        require(a.c[6] == "1" && a.p == number(old[5]) && a.n == number(old[7]), "prior p/n identity");
    }
    auto col = [&](int cutoff) { return static_cast<size_t>(std::upper_bound(primes.begin(), primes.end(), cutoff)-primes.begin()-1); };
    auto count = [&](size_t i, size_t j, bool inclusive) { return branches[i].s[j] + (inclusive ? branches[i].r[j] : 0); };
    auto margins = [&](size_t j, bool inclusive, bool only_narrow) {
        std::vector<int> result;
        for (int bits : {192,224,239,256,384}) {
            if (only_narrow && bits == 384) continue;
            int hi = -1, lo = 10000;
            for (size_t i : seeds) {
                const auto& a = branches[i];
                if (std::stoi(a.c[2]) != bits || (only_narrow && a.c[6] != "1")) continue;
                if (a.c[3] == "published") hi = std::max(hi, count(i,j,inclusive));
                else lo = std::min(lo, count(i,j,inclusive));
            }
            require(hi >= 0 && lo < 10000, "paired field inventory");
            result.push_back(lo-hi);
        }
        return result;
    };
    auto fit = [](const std::vector<int>& m) { return *std::min_element(m.begin(), m.end()) > 0; };
    std::ostringstream profiles, cuts, endpoints, violations;
    profiles << "seed_id\tbits\trole\tcategory\tcutoff\tstrict\tramified\tinclusive\n";
    cuts << "cutoff\tnarrow_strict\tnarrow_inclusive\tbroader_strict\tbroader_inclusive\tmargin192\tmargin224\tmargin239\tmargin256\tmargin384\n";
    std::vector<int> ns, ni, bs, bi;
    for (int cutoff = 100; cutoff <= 5000; ++cutoff) {
        size_t j = col(cutoff);
        auto m = margins(j,false,false);
        bool a = fit(margins(j,false,true)), b = fit(margins(j,true,true)),
             c = fit(m), d = fit(margins(j,true,false));
        if (a) ns.push_back(cutoff);
        if (b) ni.push_back(cutoff);
        if (c) bs.push_back(cutoff);
        if (d) bi.push_back(cutoff);
        cuts << cutoff << '\t' << a << '\t' << b << '\t' << c << '\t' << d;
        for (int v : m) cuts << '\t' << v;
        cuts << '\n';
        for (size_t i : seeds) {
            const auto& a0 = branches[i];
            tsv(profiles,{a0.c[1],a0.c[2],a0.c[3],a0.c[4],std::to_string(cutoff),
                          std::to_string(a0.s[j]),std::to_string(a0.r[j]),std::to_string(a0.s[j]+a0.r[j])});
        }
    }
    require(intervals(ns) == "[[691,718],[769,772]]" && ni.empty(), "old21 reproduction control");
    endpoints << "branch\tseed_id\tbits\trole\tcategory\tside\tselected\tstrict691\tramified691\tinclusive691\tstrict5000\tramified5000\tinclusive5000\tdelta\n";
    for (size_t i = 0; i < branches.size(); ++i) {
        const auto& a = branches[i]; size_t j = col(691), k = col(5000);
        tsv(endpoints,{a.c[0],a.c[1],a.c[2],a.c[3],a.c[4],a.c[5],chosen.at(a.c[1]) == i ? "1" : "0",
                       std::to_string(a.s[j]),std::to_string(a.r[j]),std::to_string(a.s[j]+a.r[j]),
                       std::to_string(a.s[k]),std::to_string(a.r[k]),std::to_string(a.s[k]+a.r[k]),a.disc.get_str()});
    }
    violations << "seed_id\tbits\tcategory\tstrict691\tmax_published691\tmargin\n";
    int bad = 0;
    for (size_t i : seeds) if (branches[i].c[3] != "published") {
        const auto& a = branches[i]; int hi = -1;
        for (size_t k : seeds) if (branches[k].c[2] == a.c[2] && branches[k].c[3] == "published")
            hi = std::max(hi, branches[k].s[col(691)]);
        require(hi >= 0, "published comparison");
        int v = a.s[col(691)];
        if (v <= hi) { ++bad; tsv(violations,{a.c[1],a.c[2],a.c[4],std::to_string(v),std::to_string(hi),std::to_string(v-hi)}); }
    }
    std::ostringstream summary;
    summary << "{\"kind\":\"broader-recorded-census/v1\",\"branches\":82,\"seed_positions\":69,\"published_positions\":10,\"unpublished_positions\":59,\"integer_cutoffs\":4901,\"character_cells\":54776,\"profile_rows\":338169,\"branch_discriminant_invariance\":true,\"narrow_strict_intervals\":" << intervals(ns)
            << ",\"broader_strict_intervals\":" << intervals(bs) << ",\"broader_inclusive_intervals\":" << intervals(bi)
            << ",\"broader_strict_success_cutoffs\":" << bs.size() << ",\"broader_inclusive_success_cutoffs\":" << bi.size()
            << ",\"strict691_violating_unpublished_positions\":" << bad << ",\"historical_use_established\":false,\"ECDLP_advantage_established\":false}\n";
    fs::path out = argv[3]; require(!fs::exists(out), "fresh output"); fs::create_directories(out);
    put(out/"characters.tsv",characters.str()); put(out/"profiles.tsv",profiles.str());
    put(out/"cutoffs.tsv",cuts.str()); put(out/"branches.tsv",endpoints.str());
    put(out/"at691-violations.tsv",violations.str()); put(out/"summary.json",summary.str());
    std::cout << "Completed broader recorded-census extraction; separate verification pending\n";
    return 0;
} catch (const std::exception& error) { std::cerr << "FAIL: " << error.what() << '\n'; return 1; }
}
