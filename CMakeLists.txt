cmake_minimum_required(VERSION 3.17)
project(monitor C)

set(CMAKE_C_STANDARD 11)

include_directories(IT8951)

add_executable(monitor
        IT8951/AsciiLib.c
        IT8951/AsciiLib.h
        IT8951/bmp.c
        IT8951/IT8951.c
        IT8951/IT8951.h
        IT8951/main.c
        IT8951/miniGUI.c
        IT8951/miniGUI.h)
