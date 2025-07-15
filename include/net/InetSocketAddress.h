#pragma once

#include <string>
#include <winsock2.h>

class InetSocketAddress {
public:
    InetSocketAddress(const std::string& ip, int port);

    [[nodiscard]] const sockaddr* data() const;
    [[nodiscard]] int size() const;

private:
    sockaddr_in addr_;
};