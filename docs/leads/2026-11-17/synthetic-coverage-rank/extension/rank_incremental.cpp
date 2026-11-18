#include <algorithm>
#include <array>
#include <cstdint>
#include <iostream>
#include <set>
#include <stdexcept>
#include <string>
#include <vector>

using Row = std::vector<int>;
using Matrix = std::vector<Row>;

struct Shape {
    int prime;
    int primary;
    int auxiliary;

    int columns() const { return auxiliary + primary; }
};

enum class Event { reject, row, repeat, shifted_repeat };

struct Attempt {
    int work;
    Event event;
    Row row;
};

int residue(int value, int prime) {
    const int remainder = value % prime;
    return remainder < 0 ? remainder + prime : remainder;
}

int inverse(int value, int prime) {
    int power = prime - 2;
    int result = 1;
    while (power > 0) {
        if (power % 2 == 1) {
            result = residue(result * value, prime);
        }
        value = residue(value * value, prime);
        power /= 2;
    }
    return result;
}

// Auxiliary columns precede primary columns in all full rows.
class PivotBasis {
public:
    PivotBasis(int pivot_columns, int prime) : prime_(prime), pivots_(pivot_columns) {}

    bool insert(Row& row) {
        for (std::size_t column = 0; column < pivots_.size(); ++column) {
            const int coefficient = row[column];
            if (coefficient == 0) {
                continue;
            }
            Row& pivot = pivots_[column];
            if (pivot.empty()) {
                const int scale = inverse(coefficient, prime_);
                for (std::size_t j = column; j < row.size(); ++j) {
                    row[j] = residue(scale * row[j], prime_);
                }
                pivot = row;
                ++rank_;
                return true;
            }
            for (std::size_t j = column; j < row.size(); ++j) {
                row[j] = residue(row[j] - coefficient * pivot[j], prime_);
            }
        }
        return false;
    }

    int rank() const { return rank_; }

    std::size_t coefficients() const {
        std::size_t total = 0;
        for (const Row& pivot : pivots_) {
            total += pivot.size();
        }
        return total;
    }

private:
    int prime_;
    Matrix pivots_;
    int rank_ = 0;
};

// Batch pivoting uses cross multiplication over the field, without inverses.
std::size_t eliminate(Matrix& rows, int pivot_columns, int prime) {
    std::size_t pivot_row = 0;
    for (int column = 0; column < pivot_columns && pivot_row < rows.size(); ++column) {
        auto first = rows.begin() + pivot_row;
        auto pivot = std::find_if(first, rows.end(), [column](const Row& row) {
            return row[column] != 0;
        });
        if (pivot == rows.end()) {
            continue;
        }
        std::iter_swap(first, pivot);
        for (auto row = first + 1; row != rows.end(); ++row) {
            const int coefficient = (*row)[column];
            if (coefficient == 0) {
                continue;
            }
            for (std::size_t j = column; j < row->size(); ++j) {
                (*row)[j] = residue((*first)[column] * (*row)[j] - coefficient * (*first)[j], prime);
            }
        }
        ++pivot_row;
    }
    return pivot_row;
}

struct BatchRanks {
    int total;
    int auxiliary;
    int induced;
};

BatchRanks batch_ranks(const Matrix& prefix, const Shape& shape) {
    Matrix projected = prefix;
    const auto auxiliary = eliminate(projected, shape.auxiliary, shape.prime);
    Matrix induced_rows;
    for (auto row = projected.begin() + auxiliary; row != projected.end(); ++row) {
        const auto primary_begin = row->begin() + shape.auxiliary;
        if (std::any_of(row->begin(), primary_begin, [](int value) { return value != 0; })) {
            throw std::runtime_error("batch auxiliary elimination left a nonzero auxiliary entry");
        }
        induced_rows.emplace_back(primary_begin, row->end());
    }

    Matrix full = prefix;
    for (Row& row : full) {
        std::rotate(row.begin(), row.begin() + shape.auxiliary, row.end());
    }
    return {static_cast<int>(eliminate(full, shape.columns(), shape.prime)),
            static_cast<int>(auxiliary),
            static_cast<int>(eliminate(induced_rows, shape.primary, shape.prime))};
}

struct Prefix {
    int work = 0;
    int accepted = 0;
    int rejected = 0;
    int duplicates = 0;
    int covered = 0;
    int total = 0;
    int auxiliary = 0;
    int useful = 0;
};

struct Mistakes {
    int false_ready = 0;
    int false_wait = 0;

    void record(bool predicted, bool ready) {
        false_ready += predicted && !ready;
        false_wait += !predicted && ready;
    }
};

struct Replay {
    std::vector<Prefix> prefixes;
    std::vector<int> thresholds;
    std::array<Mistakes, 4> mistakes;
    std::size_t dual_coefficients = 0;
    std::size_t auxiliary_first_coefficients = 0;
};

Replay calibrate(const Shape& shape, const std::vector<Attempt>& attempts, const std::string& name) {
    PivotBasis dual_total(shape.columns(), shape.prime);
    PivotBasis dual_auxiliary(shape.auxiliary, shape.prime);
    PivotBasis auxiliary_first(shape.auxiliary, shape.prime);
    PivotBasis primary_only(shape.primary, shape.prime);
    Matrix retained;
    std::set<Row> seen;
    std::vector<bool> covered(shape.columns());
    Replay replay;
    replay.thresholds.resize(shape.primary);
    Prefix previous;

    for (std::size_t i = 0; i <= attempts.size(); ++i) {
        Prefix current = previous;
        const auto check = [&](bool condition, const std::string& message) {
            if (!condition) {
                throw std::runtime_error(name + " F" + std::to_string(shape.prime)
                                         + " prefix " + std::to_string(i) + ": " + message);
            }
        };
        if (i > 0) {
            const Attempt& attempt = attempts[i - 1];
            check(attempt.work > 0, "nonpositive charge");
            current.work += attempt.work;
            if (attempt.event == Event::reject) {
                check(attempt.row.empty(), "rejection supplied a row");
                ++current.rejected;
            } else {
                check(static_cast<int>(attempt.row.size()) == shape.columns(), "wrong row width");
                Row normalized = attempt.row;
                for (int column = 0; column < shape.columns(); ++column) {
                    normalized[column] = residue(normalized[column], shape.prime);
                    covered[column] = covered[column] || normalized[column] != 0;
                }
                ++current.accepted;
                const bool duplicate = !seen.insert(normalized).second;
                current.duplicates += duplicate;
                const bool declared_repeat = attempt.event == Event::repeat || attempt.event == Event::shifted_repeat;
                check(!declared_repeat || duplicate, "declared repeat is new over the field");
                retained.push_back(normalized);

                Row full = normalized;
                Row auxiliary(normalized.begin(), normalized.begin() + shape.auxiliary);
                dual_total.insert(full);
                dual_auxiliary.insert(auxiliary);
                if (!auxiliary_first.insert(normalized)) {
                    Row primary(normalized.begin() + shape.auxiliary, normalized.end());
                    primary_only.insert(primary);
                }
            }
        }

        const BatchRanks batch = batch_ranks(retained, shape);
        current.covered = static_cast<int>(std::count(covered.begin(), covered.end(), true));
        current.total = dual_total.rank();
        current.auxiliary = dual_auxiliary.rank();
        current.useful = current.total - current.auxiliary;
        check(current.total == batch.total && current.auxiliary == batch.auxiliary
              && current.useful == batch.induced, "dual and batch bases disagree");
        check(current.auxiliary == auxiliary_first.rank() && current.useful == primary_only.rank(),
              "dual and auxiliary-first incremental bases disagree");
        check(current.useful >= previous.useful && current.useful <= previous.useful + 1,
              "invalid useful rank increment");
        if (current.useful > previous.useful) {
            check(i > 0 && current.accepted > previous.accepted && current.duplicates == previous.duplicates,
                  "rank increased on a rejection or repeat");
            replay.thresholds.at(current.useful - 1) = current.work;
        }

        const bool ready = current.useful == shape.primary;
        replay.mistakes[0].record(current.covered == shape.columns(), ready);
        replay.mistakes[1].record(current.accepted >= shape.primary, ready);
        replay.mistakes[2].record(current.total >= shape.primary, ready);
        replay.mistakes[3].record(current.total == shape.columns(), ready);
        replay.prefixes.push_back(current);
        previous = current;
    }

    replay.dual_coefficients = dual_total.coefficients() + dual_auxiliary.coefficients();
    replay.auxiliary_first_coefficients = auxiliary_first.coefficients() + primary_only.coefficients();
    return replay;
}

Row shift_by_modulus(Row row, int prime) {
    row.front() += prime;
    row.back() -= prime;
    return row;
}

std::vector<Attempt> ordered_stream(const Shape& shape, bool correlated_first) {
    Row anchor(shape.columns());
    for (int column = 0; column < shape.columns(); ++column) {
        anchor[column] = column % 2 == 0 ? 1 : -1;
    }
    Matrix rows{anchor};
    const auto add_direction = [&](int column) {
        Row row = anchor;
        row[column] += column % 2 == 0 ? -1 : 1;
        rows.push_back(row);
    };
    const auto add_primary = [&] {
        for (int column = shape.auxiliary; column < shape.columns(); ++column) {
            add_direction(column);
        }
    };
    const auto add_auxiliary = [&] {
        for (int column = 0; column < shape.auxiliary - 1; ++column) {
            add_direction(column);
        }
    };
    if (correlated_first) {
        add_primary();
        add_auxiliary();
    } else {
        add_auxiliary();
        add_primary();
    }

    std::vector<Attempt> attempts;
    for (int i = 0; i < shape.columns(); ++i) {
        if (i % 3 == 0) {
            attempts.push_back({2 + i % 2, Event::reject, {}});
        }
        attempts.push_back({3 + 2 * i % 5, Event::row, rows[i]});
        if (i % 2 == 0) {
            attempts.push_back({1 + i % 3, Event::repeat, rows[i]});
        }
        if (i % 4 == 3) {
            attempts.push_back({2, Event::shifted_repeat, shift_by_modulus(rows[i], shape.prime)});
        }
    }
    return attempts;
}

std::uint32_t stress_seed(const Shape& shape) {
    return 0x9e3779b9u ^ (static_cast<std::uint32_t>(shape.prime) << 16)
           ^ (static_cast<std::uint32_t>(shape.primary) << 8) ^ static_cast<std::uint32_t>(shape.auxiliary);
}

std::vector<Attempt> stress_stream(const Shape& shape) {
    std::uint32_t state = stress_seed(shape);
    const auto signed_value = [&] {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        return static_cast<int>(state % (4 * shape.prime + 1)) - 2 * shape.prime;
    };

    Row repeated;
    std::vector<Attempt> attempts;
    for (int i = 0; i < 48; ++i) {
        const int work = 1 + (7 * i + i / 5) % 6;
        const int kind = i % 8;
        if (kind == 0) {
            attempts.push_back({work, Event::reject, {}});
        } else if (kind == 2) {
            attempts.push_back({work, Event::repeat, repeated});
        } else if (kind == 3) {
            attempts.push_back({work, Event::shifted_repeat, shift_by_modulus(repeated, shape.prime)});
        } else {
            Row row(shape.columns());
            const int start = kind == 5 ? shape.auxiliary : 0;
            const int end = kind == 4 ? shape.auxiliary : shape.columns();
            if (kind != 6) {
                std::generate(row.begin() + start, row.begin() + end, signed_value);
            }
            if (kind == 1) {
                repeated = row;
            }
            attempts.push_back({work, Event::row, row});
        }
    }
    return attempts;
}

void check_ordered_pair(const Shape& shape, const Replay& correlated, const Replay& mixed) {
    if (correlated.prefixes.size() != mixed.prefixes.size()) {
        throw std::runtime_error("ordered streams have different lengths");
    }
    for (std::size_t i = 0; i < correlated.prefixes.size(); ++i) {
        const Prefix& a = correlated.prefixes[i];
        const Prefix& b = mixed.prefixes[i];
        const int independent = a.accepted - a.duplicates;
        const int expected_a = std::clamp(independent - 1, 0, shape.primary);
        const int expected_b = std::max(independent - shape.auxiliary, 0);
        if (a.work != b.work || a.accepted != b.accepted || a.rejected != b.rejected
            || a.duplicates != b.duplicates || a.covered != b.covered
            || a.total != independent || b.total != independent
            || a.useful != expected_a || b.useful != expected_b) {
            throw std::runtime_error("ordered control disagrees at prefix " + std::to_string(i));
        }
    }
}

void print_replay(const Shape& shape, const std::string& name, const Replay& replay) {
    const Prefix& end = replay.prefixes.back();
    std::cout << "field=" << shape.prime << " primary=" << shape.primary << " auxiliary=" << shape.auxiliary
              << " stream=" << name << " prefixes=" << replay.prefixes.size() << " work=" << end.work
              << " accepted=" << end.accepted << " rejected=" << end.rejected << " duplicates=" << end.duplicates
              << " ranks=" << end.total << ',' << end.auxiliary << ',' << end.useful << " threshold_work=";
    for (std::size_t i = 0; i < replay.thresholds.size(); ++i) {
        if (i > 0) {
            std::cout << ',';
        }
        const int threshold = replay.thresholds[i];
        std::cout << (threshold == 0 ? "unreached" : std::to_string(threshold));
    }
    static const std::array<const char*, 4> labels = {
        "covered_all", "accepted_ge_primary", "total_ge_primary", "total_full"
    };
    for (std::size_t i = 0; i < labels.size(); ++i) {
        std::cout << ' ' << labels[i] << '=' << replay.mistakes[i].false_ready << ',' << replay.mistakes[i].false_wait;
    }
    std::cout << " pivot_coefficients=" << replay.dual_coefficients << ',' << replay.auxiliary_first_coefficients << '\n';
}

Replay characteristic_control(int prime) {
    const std::vector<Attempt> attempts{
        {2, Event::reject, {}},
        {3, Event::row, {1, 0}},
        {4, Event::row, {1, 2}},
        {1, Event::shifted_repeat, shift_by_modulus({1, 2}, prime)},
        {5, Event::row, {-1, -1}},
    };
    const Replay replay = calibrate({prime, 1, 1}, attempts, "characteristic");
    const std::array<int, 6> expected = {0, 0, 0, prime == 2 ? 0 : 1, prime == 2 ? 0 : 1, 1};
    for (std::size_t i = 0; i < expected.size(); ++i) {
        if (replay.prefixes[i].useful != expected[i]) {
            throw std::runtime_error("characteristic control disagrees at prefix " + std::to_string(i));
        }
    }
    return replay;
}

int main() {
    try {
        std::cout << "prefixes include empty; ranks are total,auxiliary,useful; shortcut counts are false_ready,false_wait\n"
                  << "pivot_coefficients counts stored row entries at stream end as dual,auxiliary_first\n";
        std::size_t prefixes = 0;
        for (const Shape shape : {Shape{2, 3, 2}, Shape{3, 5, 4}, Shape{5, 7, 5}, Shape{101, 9, 6}}) {
            const Replay correlated = calibrate(shape, ordered_stream(shape, true), "correlated");
            const Replay mixed = calibrate(shape, ordered_stream(shape, false), "mixed");
            const Replay stress = calibrate(shape, stress_stream(shape), "stress");
            check_ordered_pair(shape, correlated, mixed);
            std::cout << "field=" << shape.prime << " stress_seed=" << stress_seed(shape) << '\n';
            print_replay(shape, "correlated", correlated);
            print_replay(shape, "mixed", mixed);
            print_replay(shape, "stress", stress);
            prefixes += correlated.prefixes.size() + mixed.prefixes.size() + stress.prefixes.size();
        }
        for (const int prime : {2, 3}) {
            const Replay control = characteristic_control(prime);
            print_replay({prime, 1, 1}, "characteristic", control);
            prefixes += control.prefixes.size();
        }
        std::cout << "matching_prefixes=" << prefixes << '\n';
    } catch (const std::exception& error) {
        std::cerr << error.what() << '\n';
        return 1;
    }
}
