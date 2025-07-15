#include "../../include/net/InetSocketAddress.h"
#include <stdexcept>
#include <ws2tcpip.h>

InetSocketAddress::InetSocketAddress(const std::string& ip, const int port) : addr_{} {
    addr_.sin_family = AF_INET;
    addr_.sin_port = htons(port);

    if (inet_pton(AF_INET, ip.c_str(), &addr_.sin_addr) != 1) {
        throw std::runtime_error("Invalid IP address: " + ip);
    }
}

const sockaddr* InetSocketAddress::data() const {
    return reinterpret_cast<const sockaddr*>(&addr_);
}

int InetSocketAddress::size() const {
    return sizeof(addr_);
}