cmake_minimum_required(VERSION 3.29)
project(Celervo)

set(CMAKE_CXX_STANDARD 20)

# 🔧 Export compile_commands.json (for clangd, Fleet, VSCode, etc.)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_executable(Celervo
    main.cpp
    src/net/TCPServer.cpp
    include/net/TCPServer.h
        src/net/InetSocketAddress.cpp
        include/net/InetSocketAddress.h
        src/net/Buffer.cpp
        include/net/Buffer.h
        src/util/UUID.cpp
        include/util/UUID.h
)

# ✅ Link against Winsock for networking on Windows
target_link_libraries(Celervo PRIVATE ws2_32)