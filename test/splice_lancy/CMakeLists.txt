if(CMAKE_CROSSCOMPILING
   OR NOT CMAKE_SYSTEM_NAME STREQUAL "Linux"
   OR NOT CMAKE_SYSTEM_PROCESSOR MATCHES "^(x86_64|amd64|AMD64)$"
   OR NOT CMAKE_SIZEOF_VOID_P EQUAL 8)
  message(
    FATAL_ERROR "The Splice and Lancy smoke test requires native Linux x86-64.")
endif()

if(NOT EXISTS "${PROJECT_SOURCE_DIR}/third_party/lancy3/splice-lancy/Cargo.toml")
  message(
    FATAL_ERROR
      "Initialize Lancy3 with git submodule update --init third_party/lancy3.")
endif()

find_program(SPLICE_LANCY_CARGO NAMES cargo)
if(NOT SPLICE_LANCY_CARGO)
  message(
    FATAL_ERROR
      "The Splice and Lancy smoke test requires Cargo and Rust 1.94 or newer.")
endif()

set(SPLICE_LANCY_TARGET_DIR "${CMAKE_CURRENT_BINARY_DIR}/cargo")
set(SPLICE_LANCY_ARCHIVE
    "${SPLICE_LANCY_TARGET_DIR}/release/libduckdb_splice_lancy_smoke.a")

# Cargo tracks changes in the local Lancy3 dependencies.
add_custom_target(
  splice_lancy_smoke_rust
  COMMAND "${SPLICE_LANCY_CARGO}" build --release --locked --target-dir
          "${SPLICE_LANCY_TARGET_DIR}"
  BYPRODUCTS "${SPLICE_LANCY_ARCHIVE}"
  WORKING_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}"
  COMMENT "Building the Splice and Lancy smoke test with Cargo"
  VERBATIM)

add_executable(splice_lancy_smoke splice_lancy_smoke.cpp)
add_dependencies(splice_lancy_smoke splice_lancy_smoke_rust)
target_link_libraries(
  splice_lancy_smoke PRIVATE "${SPLICE_LANCY_ARCHIVE}" Threads::Threads
                            ${CMAKE_DL_LIBS} m rt util)
add_test(NAME splice_lancy_smoke COMMAND splice_lancy_smoke)
