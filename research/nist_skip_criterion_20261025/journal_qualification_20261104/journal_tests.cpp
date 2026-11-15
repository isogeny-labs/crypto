// Development only: use the immutable old instrument as the fixture producer.
#define main frozen_sampler_main
#include "../random_curve_validation_20261029/sampler.cpp"
#undef main
#define main journal_checker_main
#include "journal_check.cpp"
#undef main
#include "resume.hpp"
#include <functional>

namespace {
using Lines = std::vector<std::string>;
std::string read_file(const fs::path& path) {
    std::ifstream in(path, std::ios::binary);
    require(bool(in), "test input");
    return {std::istreambuf_iterator<char>(in), std::istreambuf_iterator<char>()};
}
void write_file(const fs::path& path, const std::string& value) {
    std::ofstream out(path, std::ios::binary);
    out << value;
    require(bool(out), "test fixture write");
}
Lines lines(const std::string& text) {
    Lines out;
    std::istringstream input(text);
    std::string line;
    while (std::getline(input, line)) out.push_back(line);
    return out;
}
std::string joined(const Lines& rows) {
    std::string out;
    for (const auto& row : rows) out += row + '\n';
    return out;
}
std::string row_with(const std::string& row, size_t cell, const std::string& replacement) {
    auto cells = fields(row);
    require(cell < cells.size(), "test cell");
    cells[cell] = replacement;
    std::string out;
    for (const auto& value : cells) { if (!out.empty()) out += '\t'; out += value; }
    return out;
}
struct Journals { std::string attempts, accepted; };
Journals produce_toy() {
    // Fixed development fixture, bounded to 4096 attempts in the 13-bit field.
    // This arm is never accepted by the production CLI's pinned field lookup.
    const Z p = 5003;
    constexpr int bits = 13;
    std::vector<unsigned char> nonce(32, 0);
    std::ostringstream attempts, accepted;
    attempts << audit::attempt_header << '\n';
    accepted << audit::accept_header << '\n';
    unsigned done = 0;
    for (unsigned index = 0; index < 4096 && done < 3; ++index) {
        audit::Stack frame;
        auto seed = draw(nonce, bits, index);
        Z c, b, n;
        std::string status;
        bool ok = coefficient(p, bits, seed, c, b, status);
        Point point;
        std::pair<int, int> counts;
        if (ok) {
            GEN curve = ellinit(mkvec2(stoi(-3), pari_z(b)), pari_z(p), DEFAULTPREC);
            GEN result = ellsea(curve, 1);
            // Independent exhaustive toy-field oracle, including every abort.
            unsigned exact = 1;
            for (unsigned x = 0; x < 5003; ++x) {
                Z rhs = mod(Z(x) * x * x - 3 * Z(x) + b, p);
                exact += 1 + mpz_legendre(rhs.get_mpz_t(), p.get_mpz_t());
            }
            if (signe(result) == 0) {
                require(!small_prime(static_cast<int>(exact)), "early-abort exhaustive composite oracle");
                ok = false;
                status = "composite_early_abort";
            } else {
                n = native_z(result);
                require(n == exact, "exhaustive toy order oracle");
                if (!isprime(result)) { ok = false; status = "composite_proved"; }
                else if (n == p) { ok = false; status = "anomalous"; }
                else if (mpz_sizeinbase(n.get_mpz_t(), 2) > bits) { ok = false; status = "order_width"; }
                else {
                    Z value = 1;
                    for (unsigned k = 1; k <= 1023; ++k) {
                        value = mod(value * p, n);
                        if (value == 1) { ok = false; status = "embedding_bound"; break; }
                    }
                    if (ok) {
                        status = "accepted";
                        point = witness(p, b);
                        certify_point(p, b, n, point);
                        counts = profile(p, n);
                    }
                }
            }
        }
        const auto seed_hex = hexbytes(seed.data(), seed.size());
        attempts << bits << '\t' << index << '\t' << seed_hex << '\t' << c << '\t'
                 << (b == 0 ? "NA" : b.get_str()) << '\t'
                 << (n == 0 ? "NA" : n.get_str()) << '\t' << status << '\n';
        if (ok) {
            accepted << bits << '\t' << ++done << '\t' << index << '\t' << seed_hex
                     << '\t' << c << '\t' << b << '\t' << n << '\t' << point.x << '\t'
                     << point.y << '\t' << counts.first << '\t' << counts.second << '\n';
        }
    }
    require(done == 3, "bounded toy fixture target");
    return {attempts.str(), accepted.str()};
}
} // namespace

int main(int argc, char** argv) {
    try {
        require(argc == 3, "journal_tests FROZEN_COHORT FRESH_OUTPUT_DIR");
        pari_init(256000000, 1000000);
        fs::path root = argv[2];
        require(!fs::exists(root), "fresh test output required");
        fs::create_directories(root);
        unsigned passed = 0;
        auto pass = [&](const std::string& name) { std::cout << "ok " << ++passed << " - " << name << '\n'; };
        for (const auto& model : models(argv[1])) {
            audit::Field field{static_cast<unsigned>(model.bits), model.p};
            auto mapped = audit::map_seed(field, model.seed);
            require(mapped.rejection.empty() && mod(mapped.b * mapped.b - model.b * model.b, model.p) == 0,
                    "frozen root fixture");
            if (mpz_fdiv_ui(model.p.get_mpz_t(), 4) == 3)
                require(mapped.b == model.b, "frozen exponentiation root");
            require(audit::prime(model.n), "frozen prime order");
            auto point = witness(model.p, mapped.b);
            audit::point_certificate(field, mapped.b, model.n, point.x, point.y);
            auto separate_profile = audit::splitting(field, model.n);
            auto original_profile = profile(model.p, model.n);
            require(separate_profile.first == unsigned(original_profile.first) &&
                    separate_profile.second == unsigned(original_profile.second), "frozen splitting profile");
        }
        pass("21 frozen seed/root/primality and independent multiplier fixtures");
        for (unsigned bits : {13, 160, 161, 192, 224, 239, 256, 384, 521}) {
            for (unsigned value : {0, 255}) {
                Seed seed{};
                seed.fill(value);
                require(audit::expanded(seed, bits) == expand(seed, bits), "seed expansion wrap");
            }
            audit::Nonce nonce{};
            for (uint64_t index : {uint64_t(0), uint64_t(1), uint64_t(256), uint64_t(1) << 40})
                require(audit::seed_at(nonce, bits, index) == draw(std::vector<unsigned char>(32), bits, index),
                        "counter encoding");
        }
        pass("counter encoding, leading-zero seeds, 160-bit wrap and field separation");
        require(audit::fixed_field(argv[1], 521).bits == 521, "production field pin");
        pass("frozen cohort SHA-256 and production field lookup");
        auto fixture = produce_toy(), again = produce_toy();
        require(fixture.attempts == again.attempts && fixture.accepted == again.accepted,
                "complete toy A/A bytes");
        pass("bounded exhaustive toy oracle and complete producer A/A bytes");
        auto attempt_rows = lines(fixture.attempts), accepted_rows = lines(fixture.accepted);
        auto last_index = audit::counter(fields(accepted_rows.back())[2]);
        auto first_index = audit::counter(fields(accepted_rows[1])[2]);
        auto run = [&](const std::string& name, const Journals& data, bool inspect,
                       const std::function<void(const audit::Result&)>& assertion) {
            fs::path dir = root / name;
            fs::create_directory(dir);
            write_file(dir / "attempts.tsv", data.attempts);
            write_file(dir / "accepted.tsv", data.accepted);
            audit::Config config{{13, 5003}, {}, dir / "attempts.tsv", dir / "accepted.tsv", 3};
            auto result = audit::check(config, inspect);
            assertion(result);
            require(read_file(config.attempts) == data.attempts && read_file(config.accepted) == data.accepted,
                    "read-only check");
            pass(name);
        };
        auto reject = [&](const std::string& name, const Journals& data, bool inspect = false) {
            fs::path dir = root / name;
            fs::create_directory(dir);
            write_file(dir / "attempts.tsv", data.attempts);
            write_file(dir / "accepted.tsv", data.accepted);
            audit::Config config{{13, 5003}, {}, dir / "attempts.tsv", dir / "accepted.tsv", 3};
            bool refused = false;
            try { audit::check(config, inspect); }
            catch (const std::runtime_error& error) {
                refused = true;
                std::cout << "# refusal " << name << ": " << error.what() << '\n';
            }
            require(refused, "corruption accepted: " + name);
            require(read_file(config.attempts) == data.attempts && read_file(config.accepted) == data.accepted,
                    "refusal changed inputs");
            pass(name);
        };
        run("positive", fixture, false, [&](const audit::Result& result) {
            require(result.complete && result.accepted == 3 && result.next_index == last_index + 1,
                    "complete prefix accounting");
        });
        for (const auto& mutation : std::vector<std::pair<std::string, size_t>>{
                {"changed-seed", 2}, {"changed-c", 3}, {"changed-b", 4}, {"changed-n", 5},
                {"changed-status", 6}, {"index-gap", 1}, {"wrong-field", 0}}) {
            auto changed = attempt_rows;
            auto cells = fields(changed[first_index + 1]);
            std::string value;
            if (mutation.second == 2) {
                value = cells[2];
                value[0] = value[0] == 'f' ? '0' : 'f';
            } else if (mutation.second != 6) {
                Z number(cells[mutation.second], 10);
                value = Z(number + 1).get_str();
            }
            if (mutation.second == 6) value = "timeout";
            changed[first_index + 1] = row_with(changed[first_index + 1], mutation.second, value);
            reject(mutation.first, {joined(changed), fixture.accepted});
        }
        for (const auto& mutation : std::vector<std::pair<std::string, size_t>>{
                {"changed-point", 8}, {"changed-split", 9}, {"changed-ramified", 10},
                {"wrong-ordinal", 1}, {"accepted-binding", 4}}) {
            auto changed = accepted_rows;
            auto cells = fields(changed[1]);
            Z value(cells[mutation.second], 10);
            value += 1;
            if (mutation.second == 8) {
                Z x(cells[7], 10), b(cells[5], 10);
                while (mod(value * value - x * x * x + 3 * x - b, Z(5003)) == 0) value += 1;
            }
            changed[1] = row_with(changed[1], mutation.second, value.get_str());
            reject(mutation.first, {fixture.attempts, joined(changed)});
        }
        auto reordered = attempt_rows;
        std::swap(reordered[1], reordered[2]);
        reject("reordered-attempts", {joined(reordered), fixture.accepted}, true);
        auto duplicate = attempt_rows;
        duplicate.insert(duplicate.begin() + 2, duplicate[1]);
        reject("duplicate-attempt", {joined(duplicate), fixture.accepted}, true);
        auto missing = accepted_rows;
        missing.erase(missing.begin() + 1);
        reject("missing-middle-acceptance", {fixture.attempts, joined(missing)}, true);
        auto extra = accepted_rows;
        extra.push_back(extra.back());
        reject("extra-acceptance", {fixture.attempts, joined(extra)}, true);
        auto after_target = attempt_rows;
        after_target.push_back(after_target.back());
        reject("attempt-after-target", {joined(after_target), fixture.accepted}, true);
        auto extra_cell = attempt_rows;
        extra_cell[1] += '\t';
        reject("trailing-empty-cell", {joined(extra_cell), fixture.accepted});
        auto leading_zero = attempt_rows;
        leading_zero[1] = row_with(leading_zero[1], 1, "00");
        reject("noncanonical-counter", {joined(leading_zero), fixture.accepted});
        auto false_reject = attempt_rows;
        false_reject[first_index + 1] = row_with(false_reject[first_index + 1], 5, "NA");
        false_reject[first_index + 1] = row_with(false_reject[first_index + 1], 6, "composite_early_abort");
        reject("prime-labelled-early-abort", {joined(false_reject), fixture.accepted});
        // Both composite representations must replay their actual full order.
        auto early_abort = attempt_rows;
        bool found_composite = false;
        for (size_t i = 1; i < early_abort.size(); ++i) {
            auto cells = fields(early_abort[i]);
            if (cells[6] == "composite_early_abort") {
                Z b(cells[4], 10);
                unsigned exact = 1;
                for (unsigned x = 0; x < 5003; ++x) {
                    Z rhs = mod(Z(x) * x * x - 3 * Z(x) + b, Z(5003));
                    exact += 1 + mpz_legendre(rhs.get_mpz_t(), Z(5003).get_mpz_t());
                }
                early_abort[i] = row_with(early_abort[i], 5, std::to_string(exact));
                early_abort[i] = row_with(early_abort[i], 6, "composite_proved");
                found_composite = true;
                break;
            } else if (cells[6] == "composite_proved") {
                early_abort[i] = row_with(early_abort[i], 5, "NA");
                early_abort[i] = row_with(early_abort[i], 6, "composite_early_abort");
                found_composite = true;
                break;
            }
        }
        require(found_composite, "composite control exists");
        run("composite-full-order-replay", {joined(early_abort), fixture.accepted}, false,
            [](const audit::Result& result) { require(result.complete, "composite replay complete"); });
        auto before_last = accepted_rows;
        before_last.pop_back();
        Journals orphan{fixture.attempts, joined(before_last)};
        reject("strict-orphan-refusal", orphan);
        run("inspect-orphan", orphan, true, [&](const audit::Result& result) {
            require(!result.complete && result.replay_last_accepted && result.next_index == last_index &&
                    result.accepted == 2, "orphan replay boundary");
        });
        Journals torn_accept = orphan;
        torn_accept.accepted += accepted_rows.back().substr(0, accepted_rows.back().size() / 2);
        run("inspect-torn-acceptance", torn_accept, true, [&](const audit::Result& result) {
            require(result.replay_last_accepted && result.next_index == last_index &&
                    result.accepted_tail_bytes > 0, "torn acceptance boundary");
        });
        reject("strict-torn-acceptance-refusal", torn_accept);
        auto prefix = attempt_rows;
        prefix.pop_back();
        Journals stopped{joined(prefix), joined(before_last)};
        run("inspect-prefix", stopped, true, [&](const audit::Result& result) {
            require(!result.complete && !result.replay_last_accepted && result.next_index == last_index,
                    "incomplete prefix boundary");
        });
        stopped.attempts += attempt_rows.back().substr(0, attempt_rows.back().size() / 2);
        run("inspect-torn-attempt", stopped, true, [&](const audit::Result& result) {
            require(result.next_index == last_index && result.attempt_tail_bytes > 0, "torn attempt boundary");
        });
        reject("strict-torn-attempt-refusal", stopped);
        orphan.attempts += "13\t";
        reject("orphan-followed-by-attempt", orphan, true);
        run("inspect-empty-prefix", {audit::attempt_header + '\n', audit::accept_header + '\n'}, true,
            [](const audit::Result& result) { require(result.next_index == 0 && !result.complete, "empty prefix"); });
        auto oversized = fixture;
        oversized.attempts += std::string(4097, '0');
        reject("oversized-row", oversized, true);
        auto bad_pin = root / "changed-cohort.tsv";
        write_file(bad_pin, read_file(argv[1]) + "\n");
        bool pin_refused = false;
        try { audit::fixed_field(bad_pin, 192); } catch (const std::exception&) { pin_refused = true; }
        require(pin_refused, "cohort pin refusal");
        pass("changed-cohort-pin");
        for (const std::string name : {"positive", "inspect-orphan", "inspect-torn-acceptance",
                                       "inspect-prefix", "inspect-torn-attempt", "inspect-empty-prefix"}) {
            audit::Config source{{13, 5003}, {}, root / name / "attempts.tsv", root / name / "accepted.tsv", 3};
            auto original_attempts = read_file(source.attempts), original_accepted = read_file(source.accepted);
            fs::path output = root / ("resumed-" + name);
            auto result = recovery::run(source, output, true, 4096);
            require(result.accepted == 3 && result.attempts == last_index + 1 &&
                    read_file(output / "attempts.tsv") == fixture.attempts &&
                    read_file(output / "accepted.tsv") == fixture.accepted,
                    "recovered journals differ from uninterrupted producer");
            require(read_file(source.attempts) == original_attempts && read_file(source.accepted) == original_accepted,
                    "resume changed source");
            audit::Config recovered = source;
            recovered.attempts = output / "attempts.tsv";
            recovered.accepted = output / "accepted.tsv";
            require(audit::check(recovered, false).complete, "recovered journal replay");
            pass("recovery-" + name);
        }
        audit::Config fresh{{13, 5003}, {}, {}, {}, 3};
        auto fresh_output = root / "fresh-worker";
        recovery::run(fresh, fresh_output, false, 4096);
        require(read_file(fresh_output / "attempts.tsv") == fixture.attempts &&
                read_file(fresh_output / "accepted.tsv") == fixture.accepted, "fresh wrapper bytes");
        pass("fresh-wrapper-matches-frozen-producer");
        bool reused_refused = false;
        try { recovery::run(fresh, fresh_output, false, 4096); }
        catch (const std::exception&) { reused_refused = true; }
        require(reused_refused, "existing output directory accepted");
        pass("worker-refuses-existing-output");
        auto corrupted_dir = root / "missing-middle-acceptance";
        audit::Config corrupt{{13, 5003}, {}, corrupted_dir / "attempts.tsv", corrupted_dir / "accepted.tsv", 3};
        bool corrupt_refused = false;
        try { recovery::run(corrupt, root / "refused-recovery", true, 4096); }
        catch (const std::exception&) { corrupt_refused = true; }
        require(corrupt_refused && !fs::exists(root / "refused-recovery"), "corrupt source copied/continued");
        pass("worker-refuses-corrupt-source-before-output");
        std::cout << "# controls=" << passed << " toy_attempts=" << last_index + 1
                  << " toy_accepted=3 baseline_accepted=0 signed_verification=false\n";
        pari_close();
        return 0;
    } catch (const std::exception& error) {
        std::cerr << "TEST FAILURE: " << error.what() << '\n';
        return 1;
    }
}
