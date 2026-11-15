// Included after the immutable sampler and separate audit implementation.
// Each invocation writes a fresh directory; source journals are never repaired.
#include <cerrno>
#include <fcntl.h>
#include <unistd.h>

namespace recovery {
class DurableFile {
    int fd = -1;
public:
    explicit DurableFile(const fs::path& path) {
        fd = ::open(path.c_str(), O_WRONLY | O_CREAT | O_EXCL, 0600);
        require(fd >= 0, "exclusive journal creation: " + path.string());
    }
    DurableFile(const DurableFile&) = delete;
    DurableFile& operator=(const DurableFile&) = delete;
    ~DurableFile() { if (fd >= 0) ::close(fd); }
    void append(const std::string& bytes) {
        size_t offset = 0;
        while (offset < bytes.size()) {
            auto written = ::write(fd, bytes.data() + offset, bytes.size() - offset);
            if (written < 0 && errno == EINTR) continue;
            require(written > 0, "journal write failed; preserve incomplete directory");
            offset += static_cast<size_t>(written);
        }
    }
    void sync() { require(::fsync(fd) == 0, "journal fsync failed; preserve incomplete directory"); }
    void record(const std::string& bytes) { append(bytes); sync(); }
};
void sync_directory(const fs::path& path) {
    int fd = ::open(path.c_str(), O_RDONLY | O_DIRECTORY);
    require(fd >= 0, "open directory for fsync");
    int status = ::fsync(fd);
    ::close(fd);
    require(status == 0, "directory fsync failed");
}
void prefix(DurableFile& output, const fs::path& input, uint64_t count) {
    std::ifstream source(input, std::ios::binary);
    require(bool(source), "source prefix open");
    std::array<char, 32768> buffer{};
    while (count) {
        auto size = static_cast<size_t>(std::min<uint64_t>(count, buffer.size()));
        source.read(buffer.data(), size);
        require(static_cast<size_t>(source.gcount()) == size, "source prefix truncated");
        output.append(std::string(buffer.data(), size));
        count -= size;
    }
    output.sync();
}
struct Outcome { uint64_t attempts, accepted; };
Outcome run(const audit::Config& source, const fs::path& destination,
            bool resume, uint64_t cap = audit::attempt_cap) {
    require(cap <= audit::attempt_cap && cap > 0, "bounded attempt cap");
    audit::Result previous;
    if (resume) previous = audit::check(source, true);
    require(previous.next_index <= cap, "resume prefix beyond attempt cap");
    require(fs::is_directory(destination.parent_path()), "output parent must already exist");
    // Atomic exclusivity also refuses a late second writer, symlinks and reuse.
    require(fs::create_directory(destination), "fresh output directory required");
    sync_directory(destination.parent_path());
    DurableFile attempts(destination / "attempts.tsv"), accepted(destination / "accepted.tsv");
    if (resume) {
        prefix(attempts, source.attempts, previous.attempt_prefix_bytes);
        prefix(accepted, source.accepted, previous.accepted_prefix_bytes);
        require(previous.attempts_binding == audit::bind(source.attempts) &&
                previous.accepted_binding == audit::bind(source.accepted), "source changed while copying");
    } else {
        attempts.record(audit::attempt_header + '\n');
        accepted.record(audit::accept_header + '\n');
    }
    sync_directory(destination);
    uint64_t index = previous.next_index, done = previous.accepted;
    {
        DurableFile receipt(destination / "recovery.json");
        std::ostringstream text;
        text << "{\"schema\":\"nist-journal-recovery/v1\",\"resumed\":" << (resume ? "true" : "false")
             << ",\"bits\":" << source.field.bits << ",\"p\":\"" << source.field.p
             << "\",\"nonce\":\"" << audit::hex(source.nonce.data(), source.nonce.size())
             << "\",\"target\":" << source.target << ",\"first_index\":" << index
             << ",\"retained_accepted\":" << done
             << ",\"source_attempts_sha256\":\"" << previous.attempts_binding.sha256
             << "\",\"source_accepted_sha256\":\"" << previous.accepted_binding.sha256
             << "\",\"retained_attempt_bytes\":" << previous.attempt_prefix_bytes
             << ",\"retained_accepted_bytes\":" << previous.accepted_prefix_bytes
             << ",\"replay_last_accepted\":" << (previous.replay_last_accepted ? "true" : "false")
             << ",\"originals_must_be_retained\":true,\"timing_admitted\":false}\n";
        receipt.record(text.str());
    }
    sync_directory(destination);
    std::vector<unsigned char> nonce(source.nonce.begin(), source.nonce.end());
    const auto& p = source.field.p;
    const auto bits = source.field.bits;
    for (; index < cap && done < source.target; ++index) {
        audit::Stack frame;
        auto seed = draw(nonce, bits, index);
        Z c, b, n;
        std::string status;
        bool ok = coefficient(p, bits, seed, c, b, status);
        Point point;
        std::pair<int, int> counts;
        if (ok) {
            GEN curve = ellinit(mkvec2(stoi(-3), pari_z(b)), pari_z(p), DEFAULTPREC);
            GEN order = ellsea(curve, 1);
            if (signe(order) == 0) { ok = false; status = "composite_early_abort"; }
            else {
                n = native_z(order);
                if (!isprime(order)) { ok = false; status = "composite_proved"; }
                else if (n == p) { ok = false; status = "anomalous"; }
                else if (mpz_sizeinbase(n.get_mpz_t(), 2) > bits) { ok = false; status = "order_width"; }
                else {
                    Z value = 1;
                    for (unsigned k = 1; k <= 1023; ++k) {
                        value = mod(value * p, n);
                        if (value == 1) { ok = false; status = "embedding_bound"; break; }
                    }
                    if (ok) {
                        point = witness(p, b);
                        certify_point(p, b, n, point);
                        counts = profile(p, n);
                        status = "accepted";
                    }
                }
            }
        }
        auto seed_hex = hexbytes(seed.data(), seed.size());
        std::ostringstream row;
        row << bits << '\t' << index << '\t' << seed_hex << '\t' << c << '\t'
            << (b == 0 ? "NA" : b.get_str()) << '\t' << (n == 0 ? "NA" : n.get_str())
            << '\t' << status << '\n';
        // The attempt becomes durable before its accepted row can become durable.
        attempts.record(row.str());
        if (ok) {
            std::ostringstream kept;
            kept << bits << '\t' << ++done << '\t' << index << '\t' << seed_hex << '\t'
                 << c << '\t' << b << '\t' << n << '\t' << point.x << '\t' << point.y
                 << '\t' << counts.first << '\t' << counts.second << '\n';
            accepted.record(kept.str());
        }
    }
    require(done == source.target, "accepted target incomplete at attempt cap; preserve all records");
    DurableFile completion(destination / "completion.json");
    std::ostringstream text;
    std::string mode = source.target == 10 ? "--pilot" :
                       source.target == 10000 ? "--generate" : "development-fixture";
    text << "{\"kind\":\"native-random-curve-sampling/v1\",\"mode\":\"" << mode
         << "\",\"bits\":" << bits
         << ",\"accepted\":" << done << ",\"attempts\":" << index
         << ",\"nonce\":\"" << audit::hex(source.nonce.data(), source.nonce.size())
         << "\",\"resumed\":" << (resume ? "true" : "false")
         << ",\"separate_verification_pending\":true,\"timing_admitted\":false}\n";
    completion.record(text.str());
    sync_directory(destination);
    return {index, done};
}
} // namespace recovery
