#include "../../include/util/UUID.h"
#include <cctype>

UUID::UUID(const std::string& str) {
    if (!isValidUUIDString(str)) {
        throw std::invalid_argument("Invalid UUID string: " + str);
    }

    size_t byteIndex = 0;
    for (size_t i = 0; i < str.length(); ) {
        if (str[i] == '-') {
            ++i;
            continue;
        }

        const char high = str[i++];
        const char low = str[i++];
        bytes_[byteIndex++] = hexToByte(high, low);
    }
}

std::string UUID::toString() const {
    std::ostringstream oss;
    for (size_t i = 0; i < Size; ++i) {
        if (i == 4 || i == 6 || i == 8 || i == 10)
            oss << '-';
        oss << std::hex << std::setw(2) << std::setfill('0') << static_cast<int>(bytes_[i]);
    }
    return oss.str();
}

uint8_t UUID::hexToByte(const char high, const char low) {
    auto hexCharToInt = [](char c) -> uint8_t {
        if ('0' <= c && c <= '9') return c - '0';
        if ('a' <= c && c <= 'f') return 10 + (c - 'a');
        if ('A' <= c && c <= 'F') return 10 + (c - 'A');
        throw std::invalid_argument("Invalid hex digit");
    };
    return (hexCharToInt(high) << 4) | hexCharToInt(low);
}

bool UUID::isValidUUIDString(const std::string& str) {
    if (str.length() != 36) return false;
    for (size_t i = 0; i < str.length(); ++i) {
        if (i == 8 || i == 13 || i == 18 || i == 23) {
            if (str[i] != '-') return false;
        } else {
            if (!std::isxdigit(str[i])) return false;
        }
    }
    return true;
}