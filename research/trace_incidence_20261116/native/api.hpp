#pragma once
// Shared native interface. Polynomials use canonical constant-first coefficients.
// These are native development implementations, not admitted timing evidence.
#include <cstdint>
#include <map>
#include <string>
#include <tuple>
#include <vector>

namespace paper_ec {
using Word = std::uint64_t;
using Coefficients = std::vector<Word>;
struct Point {
    Word x = 0, y = 0;
    bool infinity = true;
    friend bool operator==(const Point& a, const Point& b) {
        return (a.infinity && b.infinity) || (!a.infinity && !b.infinity && a.x == b.x && a.y == b.y);
    }
    friend bool operator!=(const Point& a, const Point& b) { return !(a == b); }
    friend bool operator<(const Point& a, const Point& b) {
        return std::tie(a.infinity, a.x, a.y) < std::tie(b.infinity, b.x, b.y);
    }
};
struct Curve { Word p = 0, a = 0, b = 0; };
struct Component { Coefficients u, v; };
struct Divisor {
    std::vector<Component> components;
    bool infinity = false;
};
struct Stats {
    std::map<std::string, Word> counts;
    void add(const std::string& key, Word n = 1) { counts[key] += n; }
    void merge(const Stats& other) { for (const auto& e : other.counts) counts[e.first] += e.second; }
};
enum class TranslationMethod { Trace, Dense };

Point point_neg(const Curve&, Point);
Point point_add(const Curve&, Point, Point, Stats* = nullptr);
Point point_mul(const Curve&, Word scalar, Point, Stats* = nullptr);
bool on_curve(const Curve&, Point);
Divisor translate(const Curve&, const Divisor&, Point,
                  TranslationMethod, Word nonsquare, Stats* = nullptr);
} // namespace paper_ec
