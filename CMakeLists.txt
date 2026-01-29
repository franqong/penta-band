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
    src/MainWindow.ui
)

target_link_libraries(${PROJECT_NAME} Qt6::Widgets)
