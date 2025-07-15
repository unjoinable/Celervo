#pragma once

#include <array>
#include <string>
#include <cstdint>
#include <iomanip>

class UUID {
public:
    static constexpr size_t Size = 16;

    UUID() = default;

    explicit UUID(const std::string& str);         // Parse from string
    [[nodiscard]] std::string toString() const;    // Convert to string

    bool operator==(const UUID& other) const = default;
    bool operator!=(const UUID& other) const = default;

    [[nodiscard]] const std::array<uint8_t, Size>& data() const {
        return bytes_;
    }

private:
    std::array<uint8_t, Size> bytes_{};

    static uint8_t hexToByte(char high, char low);
    static bool isValidUUIDString(const std::string& str);
};