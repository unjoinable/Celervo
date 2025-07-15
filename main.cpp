#include "include/net/TCPServer.h"

int main() {
    TCPServer server;
    server.start("0.0.0.0", 25565);
    return 0;
}
