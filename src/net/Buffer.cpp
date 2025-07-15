#include "../../include/net/Buffer.h"

#include <cstring>
#include <stdexcept>
#include <limits>

namespace {
    constexpr size_t defaultCapacity = 64;

    // Utility to convert integer to big-endian bytes
    void writeBigEndian(uint8_t* dest, const uint32_t value) {
        dest[0] = static_cast<uint8_t>((value >> 24) & 0xFF);
        dest[1] = static_cast<uint8_t>((value >> 16) & 0xFF);
        dest[2] = static_cast<uint8_t>((value >> 8) & 0xFF);
        dest[3] = static_cast<uint8_t>(value & 0xFF);
    }

    uint32_t readBigEndian(const uint8_t* src) {
        return (static_cast<uint32_t>(src[0]) << 24) |
               (static_cast<uint32_t>(src[1]) << 16) |
               (static_cast<uint32_t>(src[2]) << 8)  |
               static_cast<uint32_t>(src[3]);
    }
}

Buffer::Buffer() {
    buffer_.reserve(defaultCapacity);
}

Buffer::Buffer(std::vector<std::byte> data) : buffer_{std::move(data)} {}

size_t Buffer::position() const {
    return pos_;
}

void Buffer::setPosition(size_t pos) {
    if (pos > buffer_.size())
        throw std::out_of_range("Position out of bounds");
    pos_ = pos;
}

void Buffer::reset() {
    pos_ = 0;
}

const std::vector<std::byte>& Buffer::data() const {
    return buffer_;
}

size_t Buffer::length() const {
    return buffer_.size();
}

size_t Buffer::remaining() const {
    return buffer_.size() - pos_;
}

void Buffer::ensureWrite(const size_t bytes) {
    if (pos_ + bytes > buffer_.size()) {
        buffer_.resize(pos_ + bytes);
    }
}

void Buffer::ensureRead(const size_t bytes) const {
    if (pos_ + bytes > buffer_.size())
        throw std::runtime_error("Buffer underflow: not enough data to read");
}

void Buffer::writeByte(const std::byte b) {
    ensureWrite(1);
    buffer_[pos_++] = b;
}

std::byte Buffer::readByte() {
    ensureRead(1);
    return buffer_[pos_++];
}

void Buffer::writeBool(const bool value) {
    writeByte(value ? std::byte{1} : std::byte{0});
}

bool Buffer::readBool() {
    return readByte() != std::byte{0};
}

void Buffer::writeInt32(const int32_t value) {
    ensureWrite(4);
    writeBigEndian(reinterpret_cast<uint8_t*>(&buffer_[pos_]), static_cast<uint32_t>(value));
    pos_ += 4;
}

int32_t Buffer::readInt32() {
    ensureRead(4);
    const uint32_t raw = readBigEndian(reinterpret_cast<const uint8_t*>(&buffer_[pos_]));
    pos_ += 4;
    return static_cast<int32_t>(raw);
}

void Buffer::writeString(const std::string& str) {
    // Write length as 4-byte int
    if (str.size() > std::numeric_limits<int32_t>::max()) {
        throw std::runtime_error("String too long to write");
    }

    writeInt32(static_cast<int32_t>(str.size()));
    ensureWrite(str.size());

    std::memcpy(&buffer_[pos_], str.data(), str.size());
    pos_ += str.size();
}

std::string Buffer::readString() {
    const int32_t len = readInt32();
    if (len < 0)
        throw std::runtime_error("Negative string length");

    ensureRead(static_cast<size_t>(len));

    std::string result(reinterpret_cast<const char*>(&buffer_[pos_]), len);
    pos_ += len;
    return result;
}
