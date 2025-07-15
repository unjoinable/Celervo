#include "../../include/net/TCPServer.h"
#include "../../include/net/InetSocketAddress.h"

#include <winsock2.h>
#include <ws2tcpip.h>
#include <iostream>

#pragma comment(lib, "ws2_32.lib")

TCPServer::TCPServer() {
    WSADATA wsaData;
    if (WSAStartup(MAKEWORD(2, 2), &wsaData) != 0) {
        std::cerr << "[WSA] Startup failed.\n";
    }
}

TCPServer::~TCPServer() {
    cleanup();
}

bool TCPServer::start(const std::string& address, int port) {
    const SOCKET serverSocket = socket(AF_INET, SOCK_STREAM, 0);
    if (serverSocket == INVALID_SOCKET) {
        std::cerr << "[Server] Socket creation failed.\n";
        return false;
    }

    try {
        const InetSocketAddress addr(address, port);

        if (bind(serverSocket, addr.data(), addr.size()) == SOCKET_ERROR) {
            std::cerr << "[Server] Bind failed.\n";
            closesocket(serverSocket);
            return false;
        }

        listen(serverSocket, SOMAXCONN);
        std::cout << "[Server] Listening on " << address << ":" << port << "...\n";

        const SOCKET clientSocket = accept(serverSocket, nullptr, nullptr);
        if (clientSocket == INVALID_SOCKET) {
            std::cerr << "[Server] Accept failed.\n";
            closesocket(serverSocket);
            return false;
        }

        char buffer[1024];
        int bytesReceived = recv(clientSocket, buffer, sizeof(buffer) - 1, 0);
        if (bytesReceived > 0) {
            buffer[bytesReceived] = '\0';
            std::cout << "[Server] Received: " << buffer << "\n";
        }

        closesocket(clientSocket);
        closesocket(serverSocket);
        return true;
    } catch (const std::exception& ex) {
        std::cerr << "[Server] Error: " << ex.what() << "\n";
        closesocket(serverSocket);
        return false;
    }
}

void TCPServer::cleanup() {
    WSACleanup();
}