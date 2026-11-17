// Read-only replay of the frozen v1 sampler's journals. No sampler/resume runner.
#include <pari/pari.h>
#include <gmpxx.h>
#include <openssl/evp.h>
#include <algorithm>
#include <array>
#include <cstdint>
#include <filesystem>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <limits>
#include <map>
#include <memory>
#include <optional>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

namespace audit {
namespace fs = std::filesystem;
using Int = mpz_class;
using SeedBytes = std::array<unsigned char, 20>;
using Nonce = std::array<unsigned char, 32>;
constexpr uint64_t attempt_cap = 50000000;
const std::string attempt_header = "bits\tindex\tseed\tc\tb\tn\tstatus";
const std::string accept_header =
    "bits\tordinal\tindex\tseed\tc\tb\tn\tGx\tGy\tstrict691\tramified691";
const std::string cohort_hash =
    "f425dc7c323bdbe6660fc599d1c625e1851a0e30d2a83d38a1c62d35e002f8fe";

void need(bool ok, const std::string& message) {
    if (!ok) throw std::runtime_error(message);
}
Int decimal(const std::string& text) {
    need(!text.empty() && text.find_first_not_of("0123456789") == std::string::npos &&
         (text.size() == 1 || text[0] != '0'), "noncanonical decimal");
    Int value;
    need(value.set_str(text, 10) == 0, "invalid decimal");
    return value;
}
uint64_t counter(const std::string& text) {
    Int value = decimal(text);
    need(value <= Int(std::to_string(std::numeric_limits<uint64_t>::max())),
         "counter overflow");
    return std::stoull(text);
}
std::vector<std::string> columns(const std::string& line, size_t count) {
    std::vector<std::string> out;
    size_t start = 0;
    for (;;) {
        size_t end = line.find('\t', start);
        out.push_back(line.substr(start, end == std::string::npos ? end : end - start));
        need(!out.back().empty(), "empty cell");
        if (end == std::string::npos) break;
        start = end + 1;
    }
    need(out.size() == count, "column count");
    return out;
}
std::string hex(const unsigned char* data, size_t size) {
    std::ostringstream out;
    out << std::hex << std::setfill('0');
    for (size_t i = 0; i < size; ++i) out << std::setw(2) << unsigned(data[i]);
    return out.str();
}
template<size_t N> std::array<unsigned char, N> bytes(const std::string& text) {
    need(text.size() == 2 * N &&
         text.find_first_not_of("0123456789abcdef") == std::string::npos,
         "noncanonical hex");
    std::array<unsigned char, N> out{};
    for (size_t i = 0; i < N; ++i)
        out[i] = static_cast<unsigned char>(std::stoul(text.substr(2 * i, 2), nullptr, 16));
    return out;
}
using Hash = std::unique_ptr<EVP_MD_CTX, decltype(&EVP_MD_CTX_free)>;
Hash hasher(const EVP_MD* algorithm) {
    Hash context(EVP_MD_CTX_new(), EVP_MD_CTX_free);
    need(bool(context) && EVP_DigestInit_ex(context.get(), algorithm, nullptr) == 1,
         "digest init");
    return context;
}
std::vector<unsigned char> digest(const EVP_MD* algorithm,
                                const std::vector<unsigned char>& message) {
    auto context = hasher(algorithm);
    need(EVP_DigestUpdate(context.get(), message.data(), message.size()) == 1, "digest update");
    std::vector<unsigned char> result(EVP_MAX_MD_SIZE);
    unsigned int size = 0;
    need(EVP_DigestFinal_ex(context.get(), result.data(), &size) == 1, "digest final");
    result.resize(size);
    return result;
}
struct Binding {
    uint64_t size = 0;
    std::string sha256;
    bool operator==(const Binding& b) const { return size == b.size && sha256 == b.sha256; }
};
Binding bind(const fs::path& path) {
    need(fs::is_regular_file(path), "not a regular input: " + path.string());
    std::ifstream in(path, std::ios::binary);
    need(bool(in), "cannot open input");
    auto context = hasher(EVP_sha256());
    std::array<char, 32768> buffer{};
    Binding result;
    while (in.read(buffer.data(), buffer.size()) || in.gcount()) {
        auto size = static_cast<size_t>(in.gcount());
        result.size += size;
        need(EVP_DigestUpdate(context.get(), buffer.data(), size) == 1, "file digest update");
    }
    need(in.eof() && !in.bad(), "input read failure");
    unsigned char output[EVP_MAX_MD_SIZE];
    unsigned int size = 0;
    need(EVP_DigestFinal_ex(context.get(), output, &size) == 1, "file digest final");
    result.sha256 = hex(output, size);
    return result;
}
Int from_bytes(const unsigned char* data, size_t size) {
    Int out;
    mpz_import(out.get_mpz_t(), size, 1, 1, 1, 0, data);
    return out;
}
Int residue(const Int& value, const Int& p) {
    Int out;
    mpz_mod(out.get_mpz_t(), value.get_mpz_t(), p.get_mpz_t());
    return out;
}
Int pow_mod(const Int& value, const Int& exponent, const Int& p) {
    Int out;
    mpz_powm(out.get_mpz_t(), value.get_mpz_t(), exponent.get_mpz_t(), p.get_mpz_t());
    return out;
}
GEN pari_int(const Int& value) { return gp_read_str(value.get_str().c_str()); }
Int from_pari(GEN value) {
    char* raw = GENtostr(value);
    Int result(raw, 10);
    pari_free(raw);
    return result;
}
struct Stack {
    pari_sp saved = avma;
    ~Stack() { avma = saved; }
};
SeedBytes seed_at(const Nonce& nonce, unsigned bits, uint64_t index) {
    const std::string domain = "ICV1-random-curve-calibration/v1";
    std::vector<unsigned char> message(domain.begin(), domain.end());
    message.push_back(0);
    message.insert(message.end(), nonce.begin(), nonce.end());
    for (int shift : {8, 0}) message.push_back((bits >> shift) & 255);
    for (int shift = 56; shift >= 0; shift -= 8) message.push_back((index >> shift) & 255);
    auto hash = digest(EVP_sha256(), message);
    SeedBytes out{};
    std::copy_n(hash.begin(), out.size(), out.begin());
    return out;
}
Int expanded(const SeedBytes& seed, unsigned bits) {
    need(bits >= 3 && bits <= 65535, "field bit width");
    unsigned blocks = (bits - 1) / 160, first_bits = bits - 160 * blocks - 1;
    auto hash = digest(EVP_sha1(), {seed.begin(), seed.end()});
    Int result = from_bytes(hash.data(), hash.size());
    mpz_fdiv_r_2exp(result.get_mpz_t(), result.get_mpz_t(), first_bits);
    Int current = from_bytes(seed.data(), seed.size());
    for (unsigned i = 0; i < blocks; ++i) {
        current += 1;
        mpz_fdiv_r_2exp(current.get_mpz_t(), current.get_mpz_t(), 160);
        std::vector<unsigned char> encoded(20, 0);
        // Fixed-width encoding preserves leading zero bytes after seed wrap.
        for (int j = 19; j >= 0; --j) {
            Int shifted = current >> (8 * (19 - j));
            encoded[j] = mpz_get_ui(shifted.get_mpz_t()) & 255;
        }
        hash = digest(EVP_sha1(), encoded);
        result <<= 160;
        result += from_bytes(hash.data(), hash.size());
    }
    return result;
}
struct Field { unsigned bits; Int p; };
struct Mapping { Int c, b = 0; std::string rejection; };
Mapping map_seed(const Field& field, const SeedBytes& seed) {
    Mapping result{expanded(seed, field.bits), 0, ""};
    Int reduced = residue(result.c, field.p);
    if (reduced == 0) { result.rejection = "zero_c"; return result; }
    if (residue(4 * reduced + 27, field.p) == 0) {
        result.rejection = "singular";
        return result;
    }
    Int inverse;
    need(mpz_invert(inverse.get_mpz_t(), reduced.get_mpz_t(), field.p.get_mpz_t()) != 0,
         "coefficient inverse");
    Int rhs = residue(-27 * inverse, field.p);
    if (mpz_legendre(rhs.get_mpz_t(), field.p.get_mpz_t()) == -1) {
        result.rejection = "nonsquare_b";
        return result;
    }
    if (mpz_fdiv_ui(field.p.get_mpz_t(), 4) == 3) {
        result.b = pow_mod(rhs, (field.p + 1) / 4, field.p);
    } else {
        Stack frame;
        GEN root = Fp_sqrt(pari_int(rhs), pari_int(field.p));
        need(root != nullptr, "square root");
        result.b = from_pari(root);
        Int other = field.p - result.b;
        if (other < result.b) result.b = other;
    }
    need(residue(reduced * result.b * result.b + 27, field.p) == 0, "seed equation");
    return result;
}
bool prime(const Int& n) { Stack frame; return isprime(pari_int(n)) != 0; }
std::string prime_disposition(const Field& field, const Int& n) {
    if (n == field.p) return "anomalous";
    if (mpz_sizeinbase(n.get_mpz_t(), 2) > field.bits) return "order_width";
    Int value = 1;
    for (unsigned k = 1; k <= 1023; ++k) {
        value = residue(value * field.p, n);
        if (value == 1) return "embedding_bound";
    }
    return "accepted";
}
std::pair<unsigned, unsigned> splitting(const Field& field, const Int& n) {
    Int trace = field.p + 1 - n, delta = trace * trace - 4 * field.p;
    unsigned split = 0, ramified = 0;
    for (unsigned ell = 3; ell <= 691; ell += 2) {
        bool is_prime = true;
        for (unsigned d = 2; d <= ell / d; ++d) if (ell % d == 0) { is_prime = false; break; }
        if (!is_prime) continue;
        Int ch = pow_mod(residue(delta, Int(ell)), Int((ell - 1) / 2), Int(ell));
        split += ch == 1;
        ramified += ch == 0;
    }
    return {split, ramified};
}
void point_certificate(const Field& field, const Int& b, const Int& n,
                       const Int& x, const Int& y) {
    Int trace = field.p + 1 - n;
    need(n > 1 && trace * trace <= 4 * field.p && n * n > 16 * field.p,
         "order Hasse uniqueness");
    need(x >= 0 && x < field.p && y >= 0 && y < field.p, "canonical point");
    need(residue(y * y - x * x * x + 3 * x - b, field.p) == 0, "point equation");
    Stack frame;
    GEN curve = ellinit(mkvec2(stoi(-3), pari_int(b)), pari_int(field.p), DEFAULTPREC);
    // Sampler uses its own GMP affine multiplier; replay uses PARI's multiplier.
    GEN multiple = ellmul(curve, mkvec2(pari_int(x), pari_int(y)), pari_int(n));
    need(ell_is_inf(multiple), "point order multiplication");
}

// Constant-memory parsing; an unterminated final row is never promoted to data.
class Rows {
    std::ifstream input;
    bool allow_tail;
public:
    uint64_t complete_bytes = 0, tail_bytes = 0;
    Rows(const fs::path& path, const std::string& header, bool partial)
        : input(path, std::ios::binary), allow_tail(partial) {
        need(bool(input), "journal open");
        auto first = next();
        need(first && *first == header, "journal header");
    }
    std::optional<std::string> next() {
        std::string line;
        char c;
        while (input.get(c)) {
            if (c == '\n') { complete_bytes += line.size() + 1; return line; }
            need(line.size() < 4096, "oversized journal row");
            need(c != '\r' && c != '\0', "noncanonical journal character");
            line.push_back(c);
        }
        need(input.eof() && !input.bad(), "journal read failure");
        if (!line.empty()) {
            need(allow_tail, "unterminated journal row");
            tail_bytes = line.size();
        }
        return std::nullopt;
    }
};
struct Config {
    Field field;
    Nonce nonce;
    fs::path attempts, accepted;
    uint64_t target;
};
struct Result {
    uint64_t attempts = 0, accepted = 0, next_index = 0;
    uint64_t attempt_prefix_bytes = 0, accepted_prefix_bytes = 0;
    uint64_t attempt_tail_bytes = 0, accepted_tail_bytes = 0;
    bool replay_last_accepted = false, complete = false;
    Binding attempts_binding, accepted_binding;
};
Result check(const Config& config, bool inspect) {
    need(config.target > 0 && config.target <= 10000, "accepted target");
    Result result;
    result.attempts_binding = bind(config.attempts);
    result.accepted_binding = bind(config.accepted);
    Rows attempts(config.attempts, attempt_header, inspect);
    Rows accepted(config.accepted, accept_header, inspect);
    auto accepted_line = accepted.next();
    uint64_t orphan_start = 0;
    while (auto line = attempts.next()) {
        need(!result.replay_last_accepted, "missing accepted row before later attempt");
        need(result.accepted < config.target, "records beyond target-th accepted curve");
        need(result.attempts < attempt_cap, "attempt cap exceeded");
        auto row = columns(*line, 7);
        need(counter(row[0]) == config.field.bits && counter(row[1]) == result.attempts,
             "field/index gap or reorder");
        auto seed = seed_at(config.nonce, config.field.bits, result.attempts);
        need(row[2] == hex(seed.data(), seed.size()), "seed derivation");
        auto mapped = map_seed(config.field, seed);
        need(decimal(row[3]) == mapped.c, "expanded coefficient");
        need(row[4] == (mapped.b == 0 ? "NA" : mapped.b.get_str()), "coefficient root");
        if (!mapped.rejection.empty()) {
            need(row[5] == "NA" && row[6] == mapped.rejection, "map rejection disposition");
        } else {
            Stack frame;
            GEN curve = ellinit(mkvec2(stoi(-3), pari_int(mapped.b)),
                                pari_int(config.field.p), DEFAULTPREC);
            Int n = from_pari(ellcard(curve, nullptr));
            bool n_prime = prime(n);
            if (row[6] == "composite_early_abort") {
                need(row[5] == "NA" && !n_prime, "early-abort composite order");
            } else {
                need(decimal(row[5]) == n, "full point count");
                std::string disposition = n_prime ? prime_disposition(config.field, n)
                                                  : "composite_proved";
                need(row[6] == disposition, "order rejection/acceptance disposition");
                if (disposition == "accepted") {
                    if (!accepted_line) {
                        need(inspect, "missing accepted row");
                        result.replay_last_accepted = true;
                        orphan_start = attempts.complete_bytes - line->size() - 1;
                    } else {
                        auto kept = columns(*accepted_line, 11);
                        need(counter(kept[0]) == config.field.bits &&
                             counter(kept[1]) == result.accepted + 1 &&
                             counter(kept[2]) == result.attempts, "accepted field/ordinal/index");
                        need(kept[3] == row[2] && kept[4] == row[3] &&
                             kept[5] == row[4] && kept[6] == row[5], "accepted/attempt binding");
                        point_certificate(config.field, mapped.b, n, decimal(kept[7]), decimal(kept[8]));
                        auto profile = splitting(config.field, n);
                        need(counter(kept[9]) == profile.first && counter(kept[10]) == profile.second,
                             "splitting profile");
                        ++result.accepted;
                        accepted_line = accepted.next();
                    }
                }
            }
        }
        ++result.attempts;
    }
    need(!accepted_line, "extra/unmatched accepted row");
    need(!result.replay_last_accepted || attempts.tail_bytes == 0,
         "orphan followed by partial attempt");
    need(accepted.tail_bytes == 0 || result.replay_last_accepted,
         "partial accepted row without final accepted attempt");
    result.next_index = result.attempts - (result.replay_last_accepted ? 1 : 0);
    result.attempt_prefix_bytes = result.replay_last_accepted ? orphan_start : attempts.complete_bytes;
    result.accepted_prefix_bytes = accepted.complete_bytes;
    result.attempt_tail_bytes = attempts.tail_bytes;
    result.accepted_tail_bytes = accepted.tail_bytes;
    result.complete = result.accepted == config.target && !result.replay_last_accepted &&
                      attempts.tail_bytes == 0 && accepted.tail_bytes == 0;
    need(inspect || result.complete, "accepted target incomplete");
    need(result.accepted != config.target || result.complete, "data after completed target");
    need(result.attempts_binding == bind(config.attempts) &&
         result.accepted_binding == bind(config.accepted), "input changed during replay");
    return result;
}
Field fixed_field(const fs::path& path, unsigned bits) {
    need(bind(path).sha256 == cohort_hash, "frozen cohort SHA-256");
    Rows rows(path, "# label\tbits\trole\trun\tindex\tp\tb\tn\tseed", false);
    std::map<unsigned, Int> fields;
    unsigned count = 0;
    while (auto line = rows.next()) {
        auto row = columns(*line, 9);
        unsigned b = static_cast<unsigned>(counter(row[1]));
        Int p;
        need(p.set_str(row[5], 0) == 0, "field modulus");
        need(!fields.count(b) || fields.at(b) == p, "inconsistent field modulus");
        fields[b] = p;
        ++count;
    }
    need(count == 21 && fields.size() == 6 && fields.count(bits), "six frozen fields");
    need(mpz_sizeinbase(fields.at(bits).get_mpz_t(), 2) == bits && prime(fields.at(bits)),
         "prime field bit width");
    return {bits, fields.at(bits)};
}
void print(const Result& result, const Config& config, bool inspect) {
    std::cout << "{\"schema\":\"nist-journal-check/v1\",\"mode\":\""
        << (inspect ? "inspect" : "verify") << "\",\"status\":\""
        << (result.complete ? "complete" : "incomplete")
        << "\",\"order_replay\":\"ellcard-all-mapped\",\"bits\":" << config.field.bits
        << ",\"p\":\"" << config.field.p << "\",\"nonce\":\""
        << hex(config.nonce.data(), config.nonce.size()) << "\",\"target\":" << config.target
        << ",\"cohort_sha256\":\"" << cohort_hash << "\",\"attempts\":" << result.attempts
        << ",\"accepted\":" << result.accepted << ",\"next_index\":" << result.next_index
        << ",\"replay_last_accepted\":" << (result.replay_last_accepted ? "true" : "false")
        << ",\"attempt_prefix_bytes\":" << result.attempt_prefix_bytes
        << ",\"accepted_prefix_bytes\":" << result.accepted_prefix_bytes
        << ",\"attempt_tail_bytes\":" << result.attempt_tail_bytes
        << ",\"accepted_tail_bytes\":" << result.accepted_tail_bytes
        << ",\"attempts_bytes\":" << result.attempts_binding.size
        << ",\"attempts_sha256\":\"" << result.attempts_binding.sha256
        << "\",\"accepted_bytes\":" << result.accepted_binding.size
        << ",\"accepted_sha256\":\"" << result.accepted_binding.sha256
        << "\",\"mutated_inputs\":false,\"production_qualified\":false,"
           "\"signed_verification\":false}\n";
}
} // namespace audit

int main(int argc, char** argv) {
    try {
        audit::need(argc == 9, "journal_check verify|inspect COHORT BITS NONCE ATTEMPTS ACCEPTED sampler-v1 TARGET");
        std::string mode = argv[1];
        audit::need(mode == "verify" || mode == "inspect", "unknown mode");
        pari_init(256000000, 1000000);
        auto bits = audit::counter(argv[3]);
        audit::need(bits <= 65535, "field bit width");
        auto target = audit::counter(argv[8]);
        audit::need(target == 10 || target == 10000, "protocol pilot or primary target");
        auto field = audit::fixed_field(argv[2], static_cast<unsigned>(bits));
        audit::Config config{field, audit::bytes<32>(argv[4]), argv[5], argv[6], target};
        // argv[7] is reserved for an explicit v1 format tag, to prevent accidental
        // interpretation of a different journal format under the same columns.
        audit::need(std::string(argv[7]) == "sampler-v1", "format must be sampler-v1");
        auto result = audit::check(config, mode == "inspect");
        audit::print(result, config, mode == "inspect");
        pari_close();
        return 0;
    } catch (const std::exception& error) {
        std::cerr << "REFUSED: " << error.what() << '\n';
        return 1;
    }
}
