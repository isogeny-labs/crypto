// Independent token-scanning replay of recorded counts, not prime arithmetic.
#include <algorithm>
#include <array>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
namespace fs = std::filesystem;
using V = std::array<long, 10>;
static void check(bool b, const std::string& s) { if (!b) throw std::runtime_error(s); }
static std::string load(const fs::path& p) { std::ifstream f(p); check(bool(f), "missing " + p.string()); return {std::istreambuf_iterator<char>(f), {}}; }
static std::vector<std::string> cells(const std::string& s) { std::vector<std::string> v; std::istringstream f(s); std::string x; while (std::getline(f, x, '\t')) v.push_back(x); return v; }
static bool begins(const std::string& s, const std::string& prefix) { return s.compare(0, prefix.size(), prefix) == 0; }
static std::string quoted(const std::string& s) { check(s.find_first_of("\"\\\n\r\t") == std::string::npos, "unsafe string"); return '"' + s + '"'; }
static std::string strings(const std::vector<std::string>& v) { std::string s = "["; for (const auto& x : v) { if (s.size() > 1) s += ','; s += quoted(x); } return s + ']'; }
static std::map<long, long> ranks(const std::string& text, long expected) {
    std::istringstream f(text); std::string line; std::map<long, long> out; long rank = 0;
    while (std::getline(f, line)) {
        if (line.empty()) continue;
        check(begins(line, "((") && line.back() == ')', "catalog framing"); const auto end = line.find(')'); check(end != std::string::npos, "factor delimiter");
        std::istringstream tail(line.substr(end + 1)); long D, h; check(bool(tail >> D >> h), "catalog D/h");
        check(out.emplace(D, ++rank).second, "repeated catalog D");
    }
    check(rank == expected && out.at(0) == 1, "catalog count"); return out;
}
static V extract(const std::string& text, const std::string& root, const std::map<long, long>& rank) {
    const std::string start = "% Starting phase 1: building the sequence of primes\n", stop = "% Starting phase 2: proving\n";
    const auto a = text.find(start), b = text.find(stop); check(a != std::string::npos && b != std::string::npos && a < b, "phase framing");
    check(text.find(start, a + 1) == std::string::npos && text.find(stop, b + 1) == std::string::npos, "duplicate phases");
    const std::string final = "\nThis number is prime\n"; const auto done = text.find(final); check(done != std::string::npos && text.find(final, done + 1) == std::string::npos, "prime completion");
    std::istringstream f(text.substr(a + start.size(), b - a - start.size())); std::string line, last_n; V v{};
    long level = -1, last_rank = 0, chosen = 0, forms = 0; bool have_choice = false;
    auto finish = [&]() {
        if (level < 0) return;
        check(have_choice && forms > 0, "incomplete level"); if (!level) { v[0] = last_rank; v[3] = forms; }
        v[1] = std::max(v[1], last_rank); v[2] += last_rank; v[4] += forms; ++v[5]; v[6] += chosen > 1;
    };
    while (std::getline(f, line)) {
        if (begins(line, "% N_")) {
            finish(); const auto equal = line.find('='); check(equal != std::string::npos, "N delimiter");
            check(std::stol(line.substr(4, equal - 4)) == level + 1, "depth inventory"); ++level;
            const auto n = line.substr(equal + 1); check(!n.empty() && n[0] != '0' && n.find_first_not_of("0123456789") == std::string::npos, "chain digits");
            if (!level) check(n == root, "root binding");
            else check(n.size() < last_n.size() || (n.size() == last_n.size() && n < last_n), "chain descent");
            last_n = n; last_rank = 0; forms = 0; have_choice = false;
        } else if (begins(line, "% next D is ")) {
            check(level >= 0 && !have_choice, "form placement"); const long D = std::stol(line.substr(12)); const long r = rank.at(D);
            check(r > last_rank, "nonincreasing catalog scan"); last_rank = r; ++forms;
        } else if (begins(line, "% D[[")) {
            const auto close = line.find("]]=" ); check(close != std::string::npos, "route delimiter");
            check(std::stol(line.substr(5, close - 5)) == level && !have_choice, "route inventory"); chosen = std::stol(line.substr(close + 3));
            check(chosen == -1 || chosen == 1 || chosen > 1, "route value"); check(rank.at(chosen <= 1 ? 0 : chosen) == last_rank, "route rank"); have_choice = true;
        } else if (begins(line, "% Factor=")) ++v[7];
        else if (begins(line, "% itmax=")) ++v[8];
        else check(!begins(line, "%!% Forced to retry") && !begins(line, "%!% No next D"), "catalog count censored");
    }
    finish(); check(v[5] > 0, "empty chain"); return v;
}
struct Record { std::string label, role, admission; int field; V values; };
static void controls() {
    const auto catalog = ranks("((0) 0 0)\n((3) 3 -1)\n((4) 4 -1)\n", 3);
    const std::string t = "% Starting phase 1: building the sequence of primes\n% N_0=1009\n% next D is 0\n% next D is 4\n% D[[0]]=4\n% Factor= 2^1\n% N_1=101\n% next D is 0\n% D[[1]]=-1\n% Starting phase 2: proving\nThis number is prime\n";
    check(extract(t, "1009", catalog) == V{3, 3, 4, 2, 3, 2, 1, 1, 0, 0}, "direct-scanner synthetic counts");
    bool refused = false; try { (void)extract(t, "1013", catalog); } catch (...) { refused = true; } check(refused, "direct-scanner wrong root");
    std::cout << "PASS: independent direct scanner exact counts and root control\n";
}
int main(int argc, char** argv) { try {
    if (argc == 2 && std::string(argv[1]) == "--self-test") { controls(); return 0; }
    check(argc == 5, "usage: verify STUDY SAVED_RUN PROFILE_TSV FRESH_OUTPUT"); const fs::path study = argv[1], saved = argv[2], out = argv[4]; check(!fs::exists(out), "output exists");
    const auto catalog = ranks(load(saved / "runtime/Data/ecpp.DfD"), 1684);
    std::istringstream input(load(study / "inputs/cohort.tsv")), table(load(argv[3])), admissions(load(study / "inputs/admission.tsv")); std::string l, r, z;
    std::getline(input, l); std::getline(admissions, z); std::getline(table, r); const auto header = cells(r); check(header.size() == 14, "profile width");
    std::vector<Record> records; long checked = 0, pubs = 0;
    while (std::getline(input, l)) {
        check(bool(std::getline(table, r)) && bool(std::getline(admissions, z)), "missing row"); const auto a = cells(l), b = cells(r), c = cells(z);
        check(a.size() == 5 && b.size() == 14 && c.size() == 7, "row grammar");
        check(a[0] == "curve-" + std::to_string(records.size()) && a[1] == b[0] && a[2] == b[1] && a[3] == b[2] && a[1] == c[0] && b[3] == c[6], "identity/status binding");
        check(a[3] == "published" || a[3] == "skipped", "role"); pubs += a[3] == "published";
        const auto dir = saved / "cases" / a[0]; check(load(dir / "input") == a[4] + "\n" && load(dir / "prover.status") == "0\n", "packet binding"); const V actual = extract(load(dir / "prover.stdout"), a[4], catalog);
        for (std::size_t k = 0; k < 10; ++k) { check(b[k + 4] != "UNKNOWN" && std::stol(b[k + 4]) == actual[k], "count mismatch " + a[1] + " " + header[k + 4]); ++checked; }
        records.push_back({a[1], a[3], c[6], std::stoi(a[2]), actual});
    }
    check(records.size() == 21 && pubs == 10 && !std::getline(table, r) && !std::getline(admissions, z), "complete table inventory");
    fs::create_directories(out); std::ofstream json(out / "verification.json"), witnesses(out / "witnesses.tsv"); check(bool(json) && bool(witnesses), "output");
    json << "{\"kind\":\"native-direct-saved-work-verification/v1\",\"rows\":21,\"cells_checked\":" << checked << ",\"raw_reextraction_passed\":true,\"envelopes\":[";
    witnesses << "feature\tscope\torientation\tfield\tpublished_witness\tbound\tretained_skip\tskip_value\n"; bool first = true;
    for (std::size_t k = 0; k < 10; ++k) for (bool field : {false, true}) for (bool upper : {true, false}) {
        std::map<int, const Record*> extremes;
        for (const auto& v : records) if (v.role == "published") { const int key = field ? v.field : 0; if (!extremes.count(key) || (upper ? v.values[k] > extremes.at(key)->values[k] : v.values[k] < extremes.at(key)->values[k])) extremes[key] = &v; }
        std::vector<std::string> retained;
        for (const auto& v : records) if (v.role == "skipped") { const int key = field ? v.field : 0; const auto& pub = *extremes.at(key); if (upper ? v.values[k] <= pub.values[k] : v.values[k] >= pub.values[k]) { retained.push_back(v.label); witnesses << header[k + 4] << '\t' << (field ? "per_field" : "global") << '\t' << (upper ? "upper" : "lower") << '\t' << key << '\t' << pub.label << '\t' << pub.values[k] << '\t' << v.label << '\t' << v.values[k] << '\n'; } }
        if (!first) { json << ','; } first = false;
        json << "{\"feature\":" << quoted(header[k + 4]) << ",\"scope\":" << quoted(field ? "per_field" : "global") << ",\"orientation\":" << quoted(upper ? "upper" : "lower") << ",\"thresholds\":{";
        bool tf = true; for (const auto& [key, pub] : extremes) { if (!tf) { json << ','; } tf = false; json << quoted(std::to_string(key)) << ':' << pub->values[k]; }
        json << "},\"published_kept\":10,\"skipped_kept\":" << retained.size() << ",\"retained_skips\":" << strings(retained) << ",\"separates\":" << (retained.empty() ? "true" : "false") << '}';
    }
    json << "],\"same_field_dominance_pairs\":["; first = true;
    for (const auto& skip : records) if (skip.role == "skipped") for (const auto& pub : records) if (pub.role == "published" && pub.field == skip.field) {
        bool below = true; for (std::size_t k = 0; k < 10; ++k) below = below && skip.values[k] <= pub.values[k];
        if (below) { if (!first) { json << ','; } first = false; json << "{\"skipped\":" << quoted(skip.label) << ",\"published\":" << quoted(pub.label) << ",\"field\":" << skip.field << '}'; }
    }
    json << "],\"posthoc_dominance_diagnostic\":true,\"independent_research_review\":false,\"historical_use_established\":false}\n";
    std::cout << "PASS:21 exact identities,210 independently re-extracted cells,40 recomputed envelopes and retained witnesses\n"; return 0;
} catch (const std::exception& e) { std::cerr << e.what() << '\n'; return 1; } }
