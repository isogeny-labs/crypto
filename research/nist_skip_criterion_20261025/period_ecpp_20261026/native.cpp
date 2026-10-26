#include "common.hpp"
#include <array>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <map>
#include <regex>
#include <set>

namespace fs = std::filesystem;
static std::string read(const fs::path& path) {
    std::ifstream in(path); require(bool(in), "missing " + path.string());
    std::string value((std::istreambuf_iterator<char>(in)), {});
    require(value.size() < 16 * 1024 * 1024, "input exceeds bound"); return value;
}
static void write(const fs::path& path, const std::string& text) {
    std::ofstream out(path); require(bool(out), "cannot write " + path.string()); out << text;
}
static GEN signed_integer(const std::string& text) {
    require(std::regex_match(text, std::regex("-?[0-9]+")), "invalid decimal integer");
    return gp_read_str(text.c_str());
}
static std::vector<std::string> fields(const std::string& line, char delimiter) {
    std::vector<std::string> out; std::istringstream in(line); std::string part;
    while (std::getline(in, part, delimiter)) out.push_back(part);
    return out;
}
static int process_status(const fs::path& path) {
    if (!fs::exists(path)) return -1;
    std::istringstream in(read(path)); int result; std::string extra;
    require(bool(in >> result) && !(in >> extra), "bad process status"); return result;
}
static GEN input_root(const fs::path& file) {
    std::istringstream in(read(file)); std::string value, extra;
    require(bool(in >> value) && !(in >> extra), "input must be one integer");
    GEN n = signed_integer(value); require(cmpis(n, 2) >= 0, "invalid input root"); return n;
}
static std::vector<std::string> labels(const std::string& text) {
    std::vector<std::string> result; std::istringstream in(text); std::string line;
    const std::regex pattern(R"rx(^(?:% )?This number is (prime|composite)\b.*$)rx");
    while (std::getline(in, line)) { std::smatch match;
        if (std::regex_match(line, match, pattern)) result.push_back(match[1]); }
    return result;
}
struct Step { GEN n; long D; };
static std::vector<Step> parse_certificate(std::string bytes, GEN root) {
    for (size_t pos; (pos = bytes.find("\\\n")) != std::string::npos;) bytes.erase(pos, 2);
    std::istringstream input(bytes); std::vector<GEN> tokens; std::string word;
    while (input >> word) tokens.push_back(signed_integer(word));
    size_t cursor = 0;
    auto take = [&]() { require(cursor < tokens.size(), "truncated certificate"); return tokens[cursor++]; };
    auto list = [&]() { std::vector<GEN> values; for (;;) { GEN n = take();
        if (!signe(n)) return values;
        require(signe(n) > 0, "negative list element"); values.push_back(n); } };
    std::vector<Step> steps; GEN old_order = nullptr;
    for (;;) {
        GEN n = take();
        if (!signe(n)) { require(!steps.empty() && cursor == tokens.size(), "empty or trailing certificate"); break; }
        require(cmpis(n, 2) >= 0 && (steps.empty() || cmpii(n, steps.back().n) < 0), "bad recursive order");
        if (old_order) require(!signe(modii(old_order, n)), "broken chain divisor");
        GEN raw_D = take(), raw_h = take();
        require(bit_length(raw_D) < 31 && bit_length(raw_h) < 31, "oversized certificate route");
        const long D = itos(raw_D), h = itos(raw_h);
        if (D == -1) {
            if (!h) { const auto pratt = list(); require(!pratt.empty() && equalii(pratt[0], n), "wrong Pratt root");
                require(cursor < tokens.size() && !signe(tokens[cursor]), "Pratt chain must terminate"); }
            else { require(h == 1 || h == 3, "unknown N-1 route"); list(); list(); old_order = subis(n, 1); }
        } else if (D == 1) { require(h == 1, "unknown N+1 route"); list(); take(); list(); old_order = addis(n, 1); }
        else { require(D > 1, "unknown elliptic route"); old_order = take(); require(signe(old_order) > 0, "invalid auxiliary order");
            list(); take(); take(); take(); take(); list(); }
        steps.push_back({n, D});
    }
    require(equalii(steps[0].n, root), "wrong requested certificate root"); return steps;
}
static void admit(const fs::path& directory) {
    GEN n = input_root(directory / "input"); require(isprime(n), "requested root is not independently proved prime");
    require(process_status(directory / "prover.status") == 0 && process_status(directory / "checker.status") == 0, "process failed");
    const auto observed = labels(read(directory / "prover.stdout"));
    require(observed.size() == 1 && observed[0] == "prime", "no unique prime label");
    const auto steps = parse_certificate(read(directory / "certificate.cformat"), n);
    const std::string transcript = read(directory / "checker.stdout");
    require(transcript.find("%!%") == std::string::npos, "legacy checker error marker");
    const std::regex row(R"rx(^\s*([0-9]+):\s*(-?[0-9]+)\s+([01])\s+.*$)rx");
    std::istringstream in(transcript); std::string line; size_t count = 0;
    while (std::getline(in, line)) { std::smatch match;
        if (!std::regex_match(line, match, row)) continue;
        require(count < steps.size() && std::stoul(match[1]) == count && std::stol(match[2]) == steps[count].D && match[3] == "1", "checker step disagreement"); ++count;
    }
    require(count == steps.size(), "checker step inventory incomplete");
    std::cout << "Input-bound complete certificate admitted; " << count << " successful legacy theorem rows.\n";
}
static void prepare(const fs::path& historical, const fs::path& output) {
    std::ifstream in(historical); require(bool(in), "missing historical input");
    std::ofstream cohort(output / "cohort.tsv"); require(bool(cohort), "cannot write cohort");
    cohort << "id\tlabel\tbits\trole\torder_decimal\n";
    std::string line; size_t count = 0; long pub = 0, skip = 0; std::set<std::string> identities;
    while (std::getline(in, line)) {
        if (line.empty() || line[0] == '#') continue;
        const auto row = fields(line, '\t'); require(row.size() == 9, "historical schema mismatch");
        require(identities.insert(row[0]).second, "duplicate cohort identity");
        require(row[2] == "published" || row[2] == "skipped", "unknown role");
        GEN n = integer(row[7]); require(isprime(n), "original order not proved prime");
        const std::string id = "curve-" + std::to_string(count++); fs::create_directory(output / "cases" / id);
        write(output / "cases" / id / "input", decimal(n) + "\n");
        cohort << id << '\t' << row[0] << '\t' << row[1] << '\t' << row[2] << '\t' << decimal(n) << '\n';
        pub += row[2] == "published"; skip += row[2] == "skipped";
    }
    require(count == 21 && pub == 10 && skip == 11, "incomplete cohort");
    for (const auto& [name, n] : std::vector<std::pair<std::string, long>>{{"prime101", 101}, {"prime65537", 65537}, {"composite15", 15}, {"composite341", 341}, {"cm101", 101}, {"empty101", 101}, {"missing101", 101}}) {
        fs::create_directory(output / "controls" / name); write(output / "controls" / name / "input", std::to_string(n) + "\n");
    }
    std::ofstream vectors(output / "arithmetic.input"), expected(output / "arithmetic.expected");
    require(bool(vectors) && bool(expected), "cannot write arithmetic fixtures"); long number = 0;
    auto pair = [&](GEN a, GEN b) {
        vectors << decimal(a) << ' ' << decimal(b) << '\n';
        expected << "ROW " << number++ << ' ' << decimal(addii(a,b)) << ' ' << decimal(subii(a,b)) << ' '
                 << decimal(mulii(a,b)) << ' ' << decimal(divii(a,b)) << ' ' << decimal(modii(a,b)) << '\n';
    };
    pair(gen_1, gen_1); pair(gen_1, stoi(3));
    for (long bits : {31,32,33,63,64,65,127,128,129,191,192,193,223,224,225,255,256,257,383,384,385,519,520,521}) {
        pair(subis(int2n(bits),1), addis(int2n((bits+1)/2),1));
        pair(addis(int2n(bits-1),3), subis(int2n((bits+1)/2),1));
    }
    require(number == 50, "arithmetic control count mismatch");
    std::cout << "All21 exact proved-prime inputs and50 native arithmetic fixtures prepared.\n";
}
static void check_arithmetic(const fs::path& actual, const fs::path& expected, const fs::path& exit_file) {
    require(process_status(exit_file) == 0, "arithmetic executable failed");
    std::istringstream in(read(actual)); std::string line;
    require(bool(std::getline(in, line)) && line == "ABI 4 4 4 4", "incorrect or unsupported32 ABI");
    std::istringstream reference(read(expected)); long count = 0;
    while (std::getline(reference, line)) {
        std::string observed; require(bool(std::getline(in, observed)), "truncated arithmetic output");
        std::istringstream a(line), b(observed); std::string prefix_a,prefix_b; long i,j;
        require(bool(a >> prefix_a >> i) && bool(b >> prefix_b >> j) && prefix_a == "ROW" && prefix_b == "ROW" && i == count && j == count, "wrong arithmetic row identity");
        for (long k = 0; k < 5; ++k) { std::string x,y; require(bool(a >> x) && bool(b >> y) && equalii(signed_integer(x), signed_integer(y)), "legacy arithmetic disagreement"); }
        std::string extra; require(!(a >> extra) && !(b >> extra), "extra arithmetic fields"); ++count;
    }
    require(count == 50 && !std::getline(in, line), "arithmetic inventory mismatch");
    std::cout << "ABI4/4/4/4 and250 exact native arithmetic comparisons passed.\n";
}
static void check_label(const fs::path& directory, const std::string& expected) {
    const int code = process_status(directory / "prover.status"); const auto observed = labels(read(directory / "prover.stdout"));
    if (expected == "indeterminate") require(code > 0, "missing-catalog process did not fail as expected");
    else require(code == 0 && observed.size() == 1 && observed[0] == expected, "unexpected control classification");
    if (expected == "composite") require(!fs::exists(directory / "certificate.cformat") || !fs::file_size(directory / "certificate.cformat"), "rejected control has unexpected certificate");
    std::cout << "Expected control label/process behavior observed.\n";
}
static void invalid_fixtures(const fs::path& positive, const fs::path& root) {
    const std::string original = read(positive / "certificate.cformat"); parse_certificate(original, stoi(101));
    for (const std::string name : {"wrong-root", "truncated", "appended"}) {
        const fs::path dir = root / name; fs::create_directory(dir);
        for (const std::string file : {"input", "prover.status", "prover.stdout", "checker.status", "checker.stdout"}) fs::copy_file(positive / file, dir / file);
        std::string altered = original;
        if (name == "wrong-root") { size_t start = altered.find_first_not_of(" \t\r\n"); require(altered.compare(start,3,"101") == 0, "unexpected fixture root encoding"); altered.replace(start,3,"105"); }
        else if (name == "truncated") { size_t end = altered.find_last_not_of(" \t\r\n"); require(end != std::string::npos && altered[end] == '0', "missing fixture sentinel"); altered.erase(end); }
        else altered += original;
        write(dir / "certificate.cformat", altered);
        bool rejected = false; try { admit(dir); } catch (const std::exception&) { rejected = true; }
        require(rejected, "tampered binding accepted");
    }
    std::cout << "Wrong-root, truncated and appended certificate bindings rejected.\n";
}
static void summarize(const fs::path& run, const fs::path& output) {
    std::istringstream in(read(run / "cohort.tsv")); std::string line; std::getline(in,line);
    std::ofstream table(output / "cohort.tsv"); require(bool(table), "cannot write cohort result");
    table << "label\tbits\trole\tprover_exit\tchecker_exit\tcli_label\tclassification\n";
    std::map<std::string, long> accepted,rejected,unknown,cli_prime,cli_composite; long total = 0;
    while (std::getline(in,line)) {
        const auto row = fields(line,'\t'); require(row.size() == 5, "cohort inventory corrupt");
        const fs::path dir = run / "cases" / row[0]; require(equalii(input_root(dir / "input"), signed_integer(row[4])), "cohort input changed");
        const int prover = process_status(dir / "prover.status"), checker = process_status(dir / "checker.status");
        std::string result = "INDETERMINATE", cli = "UNAVAILABLE";
        if (prover == 0 && fs::exists(dir / "prover.stdout")) {
            const auto observed = labels(read(dir / "prover.stdout"));
            if (observed.size() == 1) {
                cli = observed[0];
                if (cli == "prime") ++cli_prime[row[3]];
                else if (cli == "composite") ++cli_composite[row[3]];
            }
            if (observed.size() == 1 && observed[0] == "prime" && process_status(dir / "admission.status") == 0) result = "ACCEPTED";
            else if (observed.size() == 1 && observed[0] == "composite" && (!fs::exists(dir / "certificate.cformat") || !fs::file_size(dir / "certificate.cformat"))) result = "REJECTED_CLI";
        }
        if (result == "ACCEPTED") ++accepted[row[3]];
        else if (result == "REJECTED_CLI") ++rejected[row[3]];
        else ++unknown[row[3]];
        table << row[1] << '\t' << row[2] << '\t' << row[3] << '\t' << prover << '\t' << checker << '\t' << cli << '\t' << result << '\n'; ++total;
    }
    require(total == 21, "missing cohort outcomes");
    std::ofstream summary(output / "summary.json"); require(bool(summary), "cannot write summary");
    summary << "{\"kind\":\"native-period-ecpp-cohort/v2\",\"rows\":21,\"roles\":{";
    bool first = true; for (const std::string role : {"published","skipped"}) {
        if (!first) summary << ',';
        first = false;
        summary << '"' << role << "\":{\"accepted\":" << accepted[role] << ",\"rejected_cli\":" << rejected[role] << ",\"indeterminate\":" << unknown[role]
                << ",\"raw_cli_prime_labels\":" << cli_prime[role] << ",\"raw_cli_composite_labels\":" << cli_composite[role] << '}';
    }
    summary << "},\"historical_use_established\":false,\"measured_dlp\":false}\n";
    std::cout << "All21 software outcomes retained, with indeterminate separate.\n";
}
int main(int argc, char** argv) {
  try {
    pari_init(256000000, 2000000);
    setdefault("nbthreads", "1", d_SILENT);
    require(mt_nbthreads() == 1, "native PARI thread limit not applied");
    std::cout << "Native PARI worker limit: 1\n";
    require(argc >= 2, "missing mode"); const std::string mode = argv[1];
    if (mode == "prepare") { require(argc == 4, "prepare arguments"); prepare(argv[2],argv[3]); }
    else if (mode == "arithmetic") { require(argc == 5, "arithmetic arguments"); check_arithmetic(argv[2],argv[3],argv[4]); }
    else if (mode == "label") { require(argc == 4, "label arguments"); check_label(argv[2],argv[3]); }
    else if (mode == "admit") { require(argc == 3, "admit arguments"); admit(argv[2]); }
    else if (mode == "parse-only") { require(argc == 3, "parse arguments"); const fs::path directory = argv[2];
        const auto steps = parse_certificate(read(directory / "certificate.cformat"), input_root(directory / "input"));
        std::cout << "Complete root/chain grammar parsed: " << steps.size() << " steps; no mathematical admission inferred.\n"; }
    else if (mode == "fixtures") { require(argc == 4, "fixture arguments"); invalid_fixtures(argv[2],argv[3]); }
    else if (mode == "summarize") { require(argc == 4, "summary arguments"); summarize(argv[2],argv[3]); }
    else throw std::runtime_error("unknown mode");
    pari_close(); return 0;
  } catch (const std::exception& error) { std::cerr << "ERROR: " << error.what() << '\n'; return 1; }
}
