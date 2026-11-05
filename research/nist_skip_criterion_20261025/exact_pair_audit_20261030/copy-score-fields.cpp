// Active native replacement for frozen historical flatten.py.
// Copies existing integer fields; computes no trait or statistical result.
#include "io.hpp"
#include <json-c/json.h>
#include <fcntl.h>
#include <unistd.h>
#include <cerrno>
#include <cstdint>
#include <cstring>
#include <iostream>
#include <limits>
#include <memory>
#include <set>

static json_object* field(json_object* object, const char* name, json_type type) {
    json_object* value = nullptr;
    require(json_object_is_type(object, json_type_object) &&
            json_object_object_get_ex(object, name, &value) &&
            json_object_is_type(value, type), std::string("missing/wrong field: ") + name);
    return value;
}
static std::string text_field(json_object* object, const char* name) {
    auto* value = field(object, name, json_type_string);
    std::string result(json_object_get_string(value), json_object_get_string_len(value));
    require(!result.empty() && result.find_first_not_of(
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_:-") == std::string::npos,
        std::string("unsafe token: ") + name);
    return result;
}
static int integer(json_object* object, const char* name, int maximum) {
    const int64_t value = json_object_get_int64(field(object, name, json_type_int));
    require(value >= 0 && value <= maximum, std::string("integer range: ") + name);
    return static_cast<int>(value);
}
int main(int argc, char** argv) {
    try {
        require(argc == 3, "usage: copy-score-fields SOURCE_JSON FRESH_TSV");
        require(!fs::exists(argv[2]), "output already exists");
        const auto input = read_file(argv[1]);
        require(input.size() < 4 * 1024 * 1024, "source JSON too large");
        std::unique_ptr<json_tokener, decltype(&json_tokener_free)>
            parser(json_tokener_new(), json_tokener_free);
        require(bool(parser), "JSON parser allocation");
        json_tokener_set_flags(parser.get(), JSON_TOKENER_STRICT | JSON_TOKENER_VALIDATE_UTF8);
        std::unique_ptr<json_object, decltype(&json_object_put)> source(
            json_tokener_parse_ex(parser.get(), input.c_str(), static_cast<int>(input.size())),
            json_object_put);
        require(json_tokener_get_error(parser.get()) == json_tokener_success && bool(source),
                "invalid JSON");
        require(input.find_first_not_of(" \n\r\t", json_tokener_get_parse_end(parser.get())) ==
                std::string::npos, "trailing JSON content");
        require(integer(source.get(), "cutoff", 5000) == 691, "unexpected cutoff");
        require(integer(source.get(), "n_odd_primes", 5000) == 124, "unexpected prime count");
        auto* heldout = field(source.get(), "heldout", json_type_array);
        auto* named = field(source.get(), "named19", json_type_array);
        require(json_object_array_length(heldout) == 49 && json_object_array_length(named) == 19,
                "unexpected source cohort size");
        std::set<std::string> identities, seeds;
        std::ostringstream output;
        output << "# source_id\tseed_id\tbits\trole\tcategory\tstrict691\n";
        const auto append = [&](json_object* row, bool is_named) {
            const int bits = integer(row, "bits", 521);
            require(bits == 192 || bits == 224 || bits == 239 || bits == 256 || bits == 384 || bits == 521,
                    "unknown field size");
            const auto width = std::to_string(bits);
            const auto label = text_field(row, "label");
            const int score = integer(row, "split", 124);
            std::string id, seed, role, category;
            if (!is_named) {
                const auto stream = text_field(row, "stream");
                const auto index = std::to_string(integer(row, "index", std::numeric_limits<int>::max()));
                require(label == "heldout" || label == "published", "invalid heldout label");
                category = text_field(row, "kind");
                require(category == "published" || category == "opposite_root_only" ||
                        category == "order_too_wide_only", "invalid heldout category");
                require((label == "published") == (category == "published"), "label/category mismatch");
                id = "heldout:" + stream + ":" + index;
                seed = "p" + width + "-" + index;
                role = label == "published" ? "published" : "unpublished";
            } else {
                const auto name = text_field(row, "name");
                id = "named:" + name;
                require(label == "omitted" || label == "published", "invalid named label");
                if (label == "omitted") {
                    const auto prefix = "omitted_" + width + "_";
                    require(name.compare(0, prefix.size(), prefix) == 0, "omission name/field mismatch");
                    const auto index = name.substr(prefix.size());
                    require(!index.empty() && index.find_first_not_of("0123456789") == std::string::npos &&
                            (index.size() == 1 || index.front() != '0'), "invalid omission index");
                    seed = "p" + width + "-" + index;
                    role = "unpublished";
                    category = "retrospective_omission";
                } else {
                    seed = "NA_no_seed_id_in_named_row";
                    role = category = "published";
                }
            }
            require(identities.insert(id).second, "duplicate source identity");
            require(seed == "NA_no_seed_id_in_named_row" || seeds.insert(seed).second,
                    "duplicate seed identity");
            tsv(output, {id, seed, width, role, category, std::to_string(score)});
        };
        for (size_t i = 0; i < json_object_array_length(heldout); ++i)
            append(json_object_array_get_idx(heldout, i), false);
        for (size_t i = 0; i < json_object_array_length(named); ++i)
            append(json_object_array_get_idx(named, i), true);
        // Validate everything before creating an output; never overwrite evidence.
        const std::string bytes = output.str();
        const int fd = open(argv[2], O_WRONLY | O_CREAT | O_EXCL, 0600);
        require(fd >= 0, std::string("create output: ") + std::strerror(errno));
        size_t offset = 0;
        while (offset < bytes.size()) {
            const auto written = write(fd, bytes.data() + offset, bytes.size() - offset);
            if (written < 0 && errno == EINTR) continue;
            if (written <= 0) { close(fd); throw std::runtime_error("output write failure"); }
            offset += static_cast<size_t>(written);
        }
        require(close(fd) == 0, "output close failure");
        std::cout << "Copied 68 existing score/label rows with native code.\n";
    } catch (const std::exception& error) {
        std::cerr << error.what() << '\n';
        return 1;
    }
}
