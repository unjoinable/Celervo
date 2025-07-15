#pragma once

#include <vector>
#include <string>
#include <cstdint>
#include <cstddef>

class Buffer {
public:
    Buffer();
    explicit Buffer(std::vector<std::byte> data);

    // Positioning
    [[nodiscard]] size_t position() const;
    void setPosition(size_t pos);
    void reset();

    // Access
    [[nodiscard]] const std::vector<std::byte>& data() const;
    [[nodiscard]] size_t length() const;
    [[nodiscard]] size_t remaining() const;

    // Primitives
    void writeByte(std::byte);
    std::byte readByte();

    void writeBool(bool);
    bool readBool();

    void writeInt32(int32_t);
    int32_t readInt32();

    void writeString(const std::string&);
    std::string readString();

private:
    void ensureWrite(size_t bytes);
    void ensureRead(size_t bytes) const;

    std::vector<std::byte> buffer_;
    size_t pos_ = 0;
};
