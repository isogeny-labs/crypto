// Count source-defined messages in saved ECPP output; no primality arithmetic.
#include <algorithm>
#include <array>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <limits>
#include <map>
#include <optional>
#include <regex>
#include <set>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

namespace fs = std::filesystem;
using Count = std::optional<long>;
constexpr std::size_t NF = 10;
const std::array<std::string, NF> names = {"root_catalog_records", "peak_catalog_records", "total_catalog_records", "root_returned_forms", "total_returned_forms", "chain_levels", "elliptic_levels", "factor_records", "factor_parameter_records", "forced_retries"};
static void need(bool b, const std::string& s) { if (!b) throw std::runtime_error(s); }
static std::string read(const fs::path& p) {
    std::ifstream f(p, std::ios::binary); need(bool(f), "missing " + p.string());
    std::string s{std::istreambuf_iterator<char>(f), {}};
    need(s.size() < 16 * 1024 * 1024, "oversized file"); return s;
}
static void write(const fs::path& p, const std::string& s) {
    std::ofstream f(p); need(bool(f), "cannot write " + p.string()); f << s; need(bool(f), "write failure");
}
static std::string quote(const std::string& s) {
    std::string o = "\"";
    for (unsigned char c : s) {
        if (c == '\\' || c == '"') { o += '\\'; o += char(c); }
        else if (c == '\n') o += "\\n";
        else if (c == '\t') o += "\\t";
        else { need(c >= 32, "control byte"); o += char(c); }
    }
    return o + '"';
}
static std::vector<std::string> split(const std::string& s) {
    std::istringstream f(s); std::vector<std::string> out; std::string v;
    while (std::getline(f, v, '\t')) out.push_back(v);
    return out;
}
static bool decimal(const std::string& s) {
    return !s.empty() && (s.size() == 1 || s[0] != '0') && s.find_first_not_of("0123456789") == std::string::npos;
}
static bool smaller(const std::string& a, const std::string& b) {
    return a.size() < b.size() || (a.size() == b.size() && a < b);
}
static std::map<long, long> catalog(const std::string& text, bool fixture = false) {
    const std::regex pattern(R"(^\(\([0-9 ]+\) ([0-9]+) -?[0-9]+.*\)$)");
    std::map<long, long> out; std::istringstream f(text); std::string line; long rank = 0;
    while (std::getline(f, line)) {
        if (line.empty()) continue;
        std::smatch m; need(std::regex_match(line, m, pattern), "catalog grammar");
        need(out.emplace(std::stol(m[1]), ++rank).second, "duplicate catalog discriminant");
    }
    need(fixture || rank == 1684, "catalog inventory"); need(out.count(0) && out.at(0) == 1, "catalog first rank"); return out;
}
struct Level { std::string n; std::vector<long> forms; long chosen = std::numeric_limits<long>::min(); bool censored = false; };
static std::array<Count, NF> parse(const std::string& text, const std::string& root, const std::map<long, long>& ranks) {
    const std::regex nline(R"(^% N_([0-9]+)=([0-9]+)$)");
    const std::regex dline(R"(^% D\[\[([0-9]+)\]\]=(-?[0-9]+)$)");
    const std::regex next(R"(^% next D is ([0-9]+)$)");
    std::vector<Level> levels; std::istringstream f(text); std::string line;
    bool phase = false, ended = false; long factor = 0, parameters = 0, retries = 0, labels = 0;
    while (std::getline(f, line)) {
        if (line == "This number is prime") ++labels;
        need(line != "This number is composite", "composite CLI packet");
        if (line == "% Starting phase 1: building the sequence of primes") { need(!phase && !ended, "duplicate phase"); phase = true; continue; }
        if (line == "% Starting phase 2: proving") { need(phase && !ended, "unexpected phase end"); phase = false; ended = true; continue; }
        if (!phase) continue;
        std::smatch m;
        if (std::regex_match(line, m, nline)) {
            need(std::stoul(m[1]) == levels.size(), "depth gap/duplicate");
            need(decimal(m[2]), "noncanonical chain input");
            if (levels.empty()) need(m[2] == root, "wrong root");
            else { need(levels.back().chosen != std::numeric_limits<long>::min(), "missing prior route"); need(smaller(m[2], levels.back().n), "nondecreasing chain"); }
            levels.push_back({m[2], {}, std::numeric_limits<long>::min(), false});
        } else if (std::regex_match(line, m, next)) {
            need(!levels.empty() && levels.back().chosen == std::numeric_limits<long>::min(), "form outside search");
            const long D = std::stol(m[1]); need(ranks.count(D), "unknown catalog D"); levels.back().forms.push_back(D);
        } else if (std::regex_match(line, m, dline)) {
            need(!levels.empty() && std::stoul(m[1]) + 1 == levels.size(), "wrong chosen depth"); auto& level = levels.back();
            need(level.chosen == std::numeric_limits<long>::min() && !level.forms.empty(), "duplicate/empty chosen route");
            const long D = std::stol(m[2]); need(D == -1 || D == 1 || (D > 1 && ranks.count(D)), "unknown chosen route");
            need(level.forms.back() == (D <= 1 ? 0 : D), "chosen route disagrees with final form"); level.chosen = D;
        } else if (line.rfind("% Factor=", 0) == 0) { need(!levels.empty(), "factor outside level"); ++factor; }
        else if (line.rfind("% itmax=", 0) == 0) { need(!levels.empty(), "parameter outside level"); ++parameters; }
        else if (line.rfind("%!% Forced to retry", 0) == 0) { need(!levels.empty(), "retry outside level"); levels.back().censored = true; ++retries; }
        else if (line.rfind("%!% No next D", 0) == 0) { need(!levels.empty(), "exhaustion outside level"); levels.back().censored = true; }
        else need(line.rfind("% N_", 0) != 0 && line.rfind("% D[[", 0) != 0 && line.rfind("% next D", 0) != 0, "malformed count record");
    }
    need(ended && !phase && labels == 1 && !levels.empty(), "incomplete/ambiguous transcript");
    long total = 0, peak = 0, forms = 0, elliptic = 0; bool known = true;
    for (const auto& level : levels) {
        need(level.chosen != std::numeric_limits<long>::min() && !level.forms.empty(), "incomplete level");
        const long rank = ranks.at(level.forms.back()); total += rank; peak = std::max(peak, rank);
        forms += long(level.forms.size()); elliptic += level.chosen > 1; known = known && !level.censored;
    }
    return {levels.front().censored ? Count{} : Count(ranks.at(levels.front().forms.back())), known ? Count(peak) : Count{}, known ? Count(total) : Count{}, long(levels.front().forms.size()), forms, long(levels.size()), elliptic, factor, parameters, retries};
}
struct Row { std::string id, label, role, admission; int bits; std::array<Count, NF> values; };
static std::string list(const std::vector<std::string>& rows) {
    std::string out = "["; bool first = true; for (const auto& r : rows) { if (!first) out += ','; first = false; out += quote(r); } return out + ']';
}
static bool complete(const std::vector<Row>& rows, std::size_t feature) {
    return std::all_of(rows.begin(), rows.end(), [feature](const Row& r) { return bool(r.values[feature]); });
}
static std::map<int, long> bounds(const std::vector<Row>& rows, std::size_t feature, bool field, bool upper) {
    std::map<int, long> out;
    for (const auto& r : rows) if (r.role == "published") {
        const int key = field ? r.bits : 0; const long value = r.values[feature].value();
        if (!out.count(key)) out[key] = value; else out[key] = upper ? std::max(out[key], value) : std::min(out[key], value);
    }
    return out;
}
static std::string envelope(const std::vector<Row>& rows, std::size_t feature, bool field, bool upper) {
    const auto b = bounds(rows, feature, field, upper); std::vector<std::string> kept;
    std::ostringstream out; out << "{\"orientation\":" << quote(upper ? "upper" : "lower") << ",\"scope\":" << quote(field ? "per_field" : "global") << ",\"thresholds\":{";
    bool first = true; for (const auto& [k, v] : b) { if (!first) out << ','; first = false; out << quote(std::to_string(k)) << ':' << v; }
    for (const auto& r : rows) if (r.role == "skipped") { const long v = r.values[feature].value(), limit = b.at(field ? r.bits : 0); if (upper ? v <= limit : v >= limit) kept.push_back(r.label); }
    out << "},\"published_kept\":10,\"skipped_kept\":" << kept.size() << ",\"retained_skips\":" << list(kept) << ",\"separates\":" << (kept.empty() ? "true" : "false") << '}'; return out.str();
}
static void self_test() {
    const auto ranks = catalog("((0) 0 0)\n((3) 3 -1)\n((4) 4 -1)\n", true);
    const std::string good = "% Starting phase 1: building the sequence of primes\n% N_0=1009\n% next D is 0\n% next D is 4\n% itmax=0\n% D[[0]]=4\n% Factor= 2^1\n% N_1=101\n% next D is 0\n% D[[1]]=-1\n% Factor= 2^2\n% Starting phase 2: proving\nThis number is prime\n";
    const auto v = parse(good, "1009", ranks);
    need(v[0] == 3 && v[1] == 3 && v[2] == 4 && v[3] == 2 && v[4] == 3 && v[5] == 2 && v[6] == 1 && v[7] == 2 && v[8] == 1 && v[9] == 0, "synthetic message counts");
    auto altered = good; altered.insert(altered.find("% next D is 4"), "%!% Forced to retry, snif...\n"); const auto c = parse(altered, "1009", ranks);
    need(!c[0] && !c[1] && !c[2] && c[5] == 2 && c[9] == 1, "unknown catalog count retained");
    long rejected = 0;
    for (const auto& mutation : std::vector<std::pair<std::string, std::string>>{{"% N_1=101", "% N_2=101"}, {"% N_1=101", "% N_1=1010"}, {"% D[[1]]=-1", "% D[[1]]=3"}, {"% next D is 4", "% next D is 999"}, {"This number is prime", "This number is composite"}, {"% Starting phase 2: proving", "% truncated"}}) {
        altered = good; altered.replace(altered.find(mutation.first), mutation.first.size(), mutation.second);
        try { (void)parse(altered, "1009", ranks); } catch (const std::exception&) { ++rejected; }
    }
    try { (void)parse(good, "1013", ranks); } catch (const std::exception&) { ++rejected; }
    need(rejected == 7, "negative integrity controls"); std::cout << "PASS: exact counts, unknown catalog preservation, seven rejecting integrity controls\n";
}
int main(int argc, char** argv) { try {
    if (argc == 2 && std::string(argv[1]) == "--self-test") { self_test(); return 0; }
    need(argc == 4, "usage: profiler STUDY SAVED_RUN FRESH_OUTPUT");
    const fs::path study = argv[1], saved = argv[2], output = argv[3]; need(!fs::exists(output), "output exists");
    const auto ranks = catalog(read(saved / "runtime/Data/ecpp.DfD"));
    std::map<std::string, std::string> admissions; std::string line; std::istringstream a(read(study / "inputs/admission.tsv")); std::getline(a, line);
    while (std::getline(a, line)) { const auto r = split(line); need(r.size() == 7 && admissions.emplace(r[0], r[6]).second, "admission table"); }
    std::istringstream inventory(read(study / "inputs/cohort.tsv")); std::getline(inventory, line); std::vector<Row> rows; long published = 0;
    while (std::getline(inventory, line)) {
        const auto r = split(line); need(r.size() == 5 && r[0] == "curve-" + std::to_string(rows.size()), "cohort inventory");
        need(r[3] == "published" || r[3] == "skipped", "role"); published += r[3] == "published";
        const fs::path packet = saved / "cases" / r[0]; need(read(packet / "input") == r[4] + "\n", "input mismatch"); need(read(packet / "prover.status") == "0\n", "nonzero process status");
        need(admissions.count(r[1]), "missing certificate status");
        rows.push_back({r[0], r[1], r[3], admissions.at(r[1]), std::stoi(r[2]), parse(read(packet / "prover.stdout"), r[4], ranks)});
    }
    need(rows.size() == 21 && admissions.size() == 21 && published == 10, "full census"); fs::create_directories(output);
    std::ostringstream table, json, summary; table << "label\tbits\trole\tcertificate_admission"; for (const auto& name : names) table << '\t' << name; table << '\n';
    json << "{\"kind\":\"recorded-ecpp-work-profile/v1\",\"rows\":["; bool first = true;
    for (const auto& r : rows) {
        if (!first) { json << ','; }
        first = false;
        table << r.label << '\t' << r.bits << '\t' << r.role << '\t' << r.admission;
        json << "{\"label\":" << quote(r.label) << ",\"bits\":" << r.bits << ",\"role\":" << quote(r.role) << ",\"certificate_admission\":" << quote(r.admission) << ",\"features\":{";
        for (std::size_t k = 0; k < NF; ++k) { if (k) json << ','; json << quote(names[k]) << ':'; if (r.values[k]) { json << *r.values[k]; table << '\t' << *r.values[k]; } else { json << "null"; table << "\tUNKNOWN"; } }
        table << '\n'; json << "}}";
    }
    json << "],\"historical_use_established\":false,\"clock_admitted\":false}\n";
    summary << "{\"kind\":\"recorded-work-cutoff-comparison/v1\",\"rows\":21,\"features\":["; bool all = true;
    for (std::size_t k = 0; k < NF; ++k) {
        if (k) { summary << ','; }
        const bool known = complete(rows, k); all = all && known;
        summary << "{\"name\":" << quote(names[k]) << ",\"complete\":" << (known ? "true" : "false") << ",\"envelopes\":[";
        if (known) { bool efirst = true; for (bool field : {false, true}) for (bool upper : {true, false}) { if (!efirst) summary << ','; efirst = false; summary << envelope(rows, k, field, upper); } }
        summary << "]}";
    }
    summary << "],\"joint_upper\":[";
    if (all) for (bool field : {false, true}) {
        if (field) { summary << ','; }
        std::vector<std::string> kept;
        for (const auto& r : rows) if (r.role == "skipped") { bool inside = true; for (std::size_t k = 0; k < NF; ++k) inside = inside && *r.values[k] <= bounds(rows, k, field, true).at(field ? r.bits : 0); if (inside) kept.push_back(r.label); }
        summary << "{\"scope\":" << quote(field ? "per_field" : "global") << ",\"published_kept\":10,\"skipped_kept\":" << kept.size() << ",\"retained_skips\":" << list(kept) << ",\"separates\":" << (kept.empty() ? "true" : "false") << '}';
    }
    summary << "],\"joint_complete\":" << (all ? "true" : "false") << ",\"labels_visible\":true,\"historical_criterion_established\":false}\n";
    write(output / "profile.tsv", table.str()); write(output / "profile.json", json.str()); write(output / "summary.json", summary.str());
    std::cout << "Retained all21 rows; ten fixed source-count features; no clock or legacy rerun.\n"; return 0;
} catch (const std::exception& e) { std::cerr << e.what() << '\n'; return 1; } }
