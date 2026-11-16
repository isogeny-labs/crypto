#pragma once
// Shared TSV I/O only; producer and verifier implement arithmetic separately.
#include <filesystem>
#include <fstream>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>
namespace fs = std::filesystem;
inline void require(bool value, const std::string& why) {
    if (!value) throw std::runtime_error(why);
}
inline std::vector<std::string> cells(const std::string& line) {
    std::istringstream stream(line);
    std::vector<std::string> result;
    std::string value;
    while (std::getline(stream, value, '\t')) result.push_back(value);
    return result;
}
inline std::vector<std::vector<std::string>> table(const fs::path& path, size_t width) {
    std::ifstream input(path);
    require(bool(input), "input: " + path.string());
    std::vector<std::vector<std::string>> result;
    std::string line;
    while (std::getline(input, line)) {
        if (line.empty() || line.front() == '#') continue;
        auto row = cells(line);
        require(row.size() == width, "input width");
        result.push_back(row);
    }
    return result;
}
inline std::string read_file(const fs::path& path) {
    std::ifstream file(path);
    require(bool(file), "read: " + path.string());
    std::ostringstream value;
    value << file.rdbuf();
    return value.str();
}
inline void put(const fs::path& path, const std::string& value) {
    std::ofstream file(path);
    require(bool(file), "write: " + path.string());
    file << value;
    require(bool(file), "write completion");
}
inline void tsv(std::ostream& out, const std::vector<std::string>& row) {
    for (size_t i = 0; i < row.size(); ++i) out << (i ? "\t" : "") << row[i];
    out << '\n';
}
