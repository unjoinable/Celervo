#pragma once
#include <string>

class TCPServer {
public:
    TCPServer();
    ~TCPServer();

    bool start(const std::string& address, int port);

private:
    void cleanup();
};