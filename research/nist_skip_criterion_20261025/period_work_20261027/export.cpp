// Native serialization of already checked observations and literal proof data.
#include <algorithm>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <map>
#include <openssl/evp.h>
#include <regex>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
namespace fs = std::filesystem;
static void require(bool b, const std::string& s) { if (!b) throw std::runtime_error(s); }
static std::string read(const fs::path& p) { std::ifstream f(p, std::ios::binary); require(bool(f), "missing " + p.string()); return {std::istreambuf_iterator<char>(f), {}}; }
static std::string quote(const std::string& s) { std::string out = "\""; for (unsigned char c : s) { if (c == '\\' || c == '"') { out += '\\'; out += char(c); } else if (c == '\n') out += "\\n"; else if (c == '\t') out += "\\t"; else { require(c >= 32, "control byte"); out += char(c); } } return out + '"'; }
static std::string hash(const std::string& s) { unsigned char h[32]; unsigned int n; require(EVP_Digest(s.data(), s.size(), h, &n, EVP_sha256(), nullptr) == 1 && n == 32, "sha256"); std::ostringstream out; for (auto b : h) out << std::hex << std::setw(2) << std::setfill('0') << unsigned(b); return out.str(); }
static std::string file(const fs::path& base, const std::string& name, const std::string& kind) { const auto b = read(base / name); return "{\"path\":" + quote(name) + ",\"sha256\":" + quote(hash(b)) + ",\"bytes\":" + std::to_string(b.size()) + ",\"kind\":" + quote(kind) + '}'; }
static std::vector<std::string> fields(const std::string& s) { std::vector<std::string> out; std::istringstream f(s); std::string p; while (std::getline(f, p, '\t')) out.push_back(p); return out; }
static std::string capture(const std::string& text, const std::string& pattern) { std::smatch m; require(std::regex_search(text, m, std::regex(pattern)), "metadata field absent"); return m[1]; }
int main(int argc, char** argv) { try {
    require(argc == 4, "usage: export STUDY CREATED OUTPUT"); const fs::path base = argv[1];
    require(read(base / "runs/analysis001/status") == "0\n" && read(base / "runs/validation002/validation.status") == "0\n", "incomplete execution");
    for (const std::string c : {"wrong-count", "missing-row", "wrong-status"}) require(read(base / "runs/validation002/controls" / c / "validation.status") == "1\n", "refusal control failed");
    const std::string verified = read(base / "results/verification.json");
    require(capture(verified, "\"cells_checked\"\\s*:\\s*([0-9]+)") == "210", "cell inventory");
    std::string commit = read(base / "runs/analysis001/source-commit.txt"); while (!commit.empty() && commit.back() == '\n') commit.pop_back(); require(std::regex_match(commit, std::regex("[0-9a-f]{40}")), "source commit");
    const std::string protocol_hash = capture(read(base / "plan.json"), "\"sha256_excluding_freeze\"\\s*:\\s*\"([0-9a-f]{64})\"");
    std::ofstream out(argv[3]); require(bool(out), "export output");
    out << "{\"schema\":\"experiment.export/v1\",\"export_id\":\"nist-ecpp-recorded-work-20261027-001\",\"created\":" << quote(argv[2])
        << ",\"protocol\":{\"path\":\"plan.json\",\"sha256\":" << quote(protocol_hash) << ",\"protocol_id\":\"nist-ecpp-recorded-work-20261027\",\"version\":1},\"producer\":{\"runtime\":\"codex\",\"model\":null,\"session\":\"nist-skip-criterion\",\"role\":\"author\"},\"source\":{\"repo\":\"crypto\",\"commit\":" << quote(commit)
        << ",\"dirty\":null},\"backend\":\"local\",\"host\":{\"path\":null,\"sha256\":null,\"class_id\":null,\"summary\":{\"cpu_model\":null,\"logical_cpus\":null,\"memory_bytes\":null,\"kernel\":null,\"hypervisor\":null,\"isolation_tier_max\":null}},\"arms\":[{\"id\":\"recorded-work\",\"role\":\"baseline\",\"binary_sha256\":" << quote(hash(read(base / "runs/analysis001/bin/profiler")))
        << ",\"commit\":\"83bbc8d85ab1454e0ff299b88fb83fe746891d30\"}],\"runs\":[{\"run_id\":\"analysis001\",\"arm\":\"recorded-work\",\"instance_id\":\"all21-recorded-orders\",\"seed\":null,\"replicate\":0,\"warmup\":false,\"status\":\"succeeded\",\"outcome_class\":\"completed\",\"attempts\":1,\"resumed\":0,\"checkpoint\":null,\"measurements\":{\"counted\":{\"curve_rows\":21,\"feature_cells\":210},\"wall_seconds\":null,\"wall_valid\":false,\"cpu_user_seconds\":null,\"cpu_sys_seconds\":null,\"max_rss_bytes\":null,\"counters\":{}},\"isolation\":{\"tier\":null,\"contended\":null,\"blockers\":[\"Shared-host analysis of existing transcripts; no clock admitted\"]},\"verification\":{\"status\":\"verified\",\"verifier\":\"Independent native token scanner checks210 cells and40 envelopes; no new primality certificate\"},\"exclusion\":{\"excluded\":false,\"rule\":null},\"artifacts\":[";
    bool first = true;
    for (const std::string f : {"runs/analysis001/stdout", "runs/analysis001/stderr", "runs/analysis001/status", "results/profile.tsv", "results/verification.json", "results/metadata-comparison.json"}) { if (!first) out << ','; first = false; out << file(base, f, "raw"); }
    out << "],\"started_at\":null,\"finished_at\":null,\"worker\":\"native-local-saved-result-analysis\",\"backend_ref\":\"runs/analysis001\"}],\"flow\":{\"planned\":1,\"attempted\":1,\"completed\":1,\"not_completed\":0,\"interrupted\":0,\"resumed\":0,\"excluded\":[]},\"files\":[";
    std::vector<std::string> paths;
    for (const auto& entry : fs::recursive_directory_iterator(base / "runs")) if (entry.is_regular_file()) paths.push_back(fs::relative(entry.path(), base).generic_string());
    for (const std::string f : {"plan.json", "PROTOCOL.md", "VALIDATION_PROTOCOL.md", "profiler.cpp", "verify.cpp", "results/profile.tsv", "results/profile.json", "results/summary.json", "results/verification.json", "results/witnesses.tsv", "results/metadata-comparison.json", "evidence/input-copy-map.json", "lean/WorkLimits.lean", "lean/lean-toolchain", "lean/count-data.json", "proof/PROOF_MAP.json", "proof/axiom-audit.txt", "proof/toolchain-release.json", "proof/attempts.json"}) paths.push_back(f);
    std::sort(paths.begin(), paths.end()); paths.erase(std::unique(paths.begin(), paths.end()), paths.end()); first = true;
    for (const auto& f : paths) { if (!first) out << ','; first = false; out << file(base, f, "raw"); }
    out << "],\"analysis\":{\"script\":" << file(base, "profiler.cpp", "analysis") << ",\"blinded\":false,\"blinding_key_sha256\":null,\"outputs\":[" << file(base, "results/profile.tsv", "summary") << ',' << file(base, "results/summary.json", "summary") << ',' << file(base, "results/verification.json", "summary")
        << "],\"unmasked_at\":null},\"notes\":[{\"kind\":\"observation\",\"text\":\"One native analysis invocation reuses21 original ECPP packets. Saved verification and three refusal controls are validation phases, not new historical samples. All12 prior certificate unknowns remain text labels. The plan is a custom saved-result plan; no canonical experiment.protocol checker ran. Labels and one example transcript were visible. The additional dominance diagnostic and verifier were declared after primary outputs. No legacy executable or clock measurement ran.\",\"by\":\"native-source study\"}],\"reviews_dir\":\"reviews\"}\n";
    require(bool(out), "export write");
    std::istringstream table(read(base / "results/profile.tsv")); std::string line; std::getline(table, line); const auto header = fields(line); require(header.size() == 14, "profile schema");
    const std::map<std::string, std::string> definitions = {{"p239-1562", "skipped1562"}, {"p239-2682", "skipped2682"}, {"p239-2185", "published2185"}, {"p239-3048", "published3048"}};
    std::ofstream bridge(base / "lean/count-data.json"); require(bool(bridge), "proof-data output"); bridge << "{\"kind\":\"literal-lean-data-binding/v1\",\"source\":" << file(base, "results/profile.tsv", "summary") << ",\"records\":["; first = true; long count = 0;
    while (std::getline(table, line)) { const auto row = fields(line); require(row.size() == 14, "profile row"); if (!definitions.count(row[0])) continue;
        if (!first) { bridge << ','; } first = false; ++count;
        bridge << "{\"label\":" << quote(row[0]) << ",\"definition\":" << quote(definitions.at(row[0])) << ",\"values\":[";
        for (std::size_t k = 4; k < 14; ++k) { require(!row[k].empty() && row[k].find_first_not_of("0123456789") == std::string::npos, "noninteger proof literal"); if (k > 4) bridge << ','; bridge << row[k]; } bridge << "]}";
    }
    require(count == 4, "proof literal inventory"); bridge << "],\"arithmetic_recomputed_here\":false}\n"; require(bool(bridge), "proof data write");
    std::cout << "Native unified export and four literal proof bindings written; no new work-count arithmetic.\n"; return 0;
} catch (const std::exception& e) { std::cerr << e.what() << '\n'; return 1; } }
