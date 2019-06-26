cmake_minimum_required(VERSION 3.6)
project(wrapper)

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c18")

set(SOURCE_FILES src/wrapper.c src/wrapper.h)
add_library(wrapper STATIC ${SOURCE_FILES})

INSTALL(TARGETS wrapper)