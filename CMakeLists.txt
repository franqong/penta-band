cmake_minimum_required(VERSION 3.16)

project(PentaBand LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

find_package(Qt6 REQUIRED COMPONENTS Widgets)

add_executable(${PROJECT_NAME}
    src/main.cpp
    src/MainWindow.cpp
    src/MainWindow.h
    src/Header.cpp
    src/Header.h
    resources.qrc
)

target_link_libraries(${PROJECT_NAME} Qt6::Widgets)

if (WIN32)
    add_custom_command(TARGET PentaBand POST_BUILD
        COMMAND "C:/Qt/6.10.1/mingw_64/bin/windeployqt.exe"
                "$<TARGET_FILE:PentaBand>"
        COMMENT "Deploying Qt runtime with windeployqt"
    )
endif()
