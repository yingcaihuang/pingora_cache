# Install script for directory: /Users/betty/.cargo/registry/src/index.crates.io-6f17d22bba15001f/libz-ng-sys-1.1.23/src/zlib-ng

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/Users/betty/Downloads/cdn-cache-ai/pingora_cache/target/debug/build/libz-ng-sys-87b6767cc1955227/out")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Debug")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE STATIC_LIBRARY FILES "/Users/betty/Downloads/cdn-cache-ai/pingora_cache/target/debug/build/libz-ng-sys-87b6767cc1955227/out/build/libz-ng.a")
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libz-ng.a" AND
     NOT IS_SYMLINK "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libz-ng.a")
    execute_process(COMMAND "/usr/bin/ranlib" "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libz-ng.a")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE FILE RENAME "zlib-ng.h" FILES "/Users/betty/Downloads/cdn-cache-ai/pingora_cache/target/debug/build/libz-ng-sys-87b6767cc1955227/out/build/zlib-ng.h")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE FILE RENAME "zlib_name_mangling-ng.h" FILES "/Users/betty/Downloads/cdn-cache-ai/pingora_cache/target/debug/build/libz-ng-sys-87b6767cc1955227/out/build/zlib_name_mangling-ng.h")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE FILE RENAME "zconf-ng.h" FILES "/Users/betty/Downloads/cdn-cache-ai/pingora_cache/target/debug/build/libz-ng-sys-87b6767cc1955227/out/build/zconf-ng.h")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/pkgconfig" TYPE FILE FILES "/Users/betty/Downloads/cdn-cache-ai/pingora_cache/target/debug/build/libz-ng-sys-87b6767cc1955227/out/build/zlib-ng.pc")
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/zlib-ng/zlib-ng.cmake")
    file(DIFFERENT EXPORT_FILE_CHANGED FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/zlib-ng/zlib-ng.cmake"
         "/Users/betty/Downloads/cdn-cache-ai/pingora_cache/target/debug/build/libz-ng-sys-87b6767cc1955227/out/build/CMakeFiles/Export/lib/cmake/zlib-ng/zlib-ng.cmake")
    if(EXPORT_FILE_CHANGED)
      file(GLOB OLD_CONFIG_FILES "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/zlib-ng/zlib-ng-*.cmake")
      if(OLD_CONFIG_FILES)
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/zlib-ng/zlib-ng.cmake\" will be replaced.  Removing files [${OLD_CONFIG_FILES}].")
        file(REMOVE ${OLD_CONFIG_FILES})
      endif()
    endif()
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/zlib-ng" TYPE FILE FILES "/Users/betty/Downloads/cdn-cache-ai/pingora_cache/target/debug/build/libz-ng-sys-87b6767cc1955227/out/build/CMakeFiles/Export/lib/cmake/zlib-ng/zlib-ng.cmake")
  if("${CMAKE_INSTALL_CONFIG_NAME}" MATCHES "^([Dd][Ee][Bb][Uu][Gg])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/zlib-ng" TYPE FILE FILES "/Users/betty/Downloads/cdn-cache-ai/pingora_cache/target/debug/build/libz-ng-sys-87b6767cc1955227/out/build/CMakeFiles/Export/lib/cmake/zlib-ng/zlib-ng-debug.cmake")
  endif()
endif()

if("x${CMAKE_INSTALL_COMPONENT}x" STREQUAL "xUnspecifiedx" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/zlib-ng" TYPE FILE FILES
    "/Users/betty/Downloads/cdn-cache-ai/pingora_cache/target/debug/build/libz-ng-sys-87b6767cc1955227/out/build/zlib-ng-config.cmake"
    "/Users/betty/Downloads/cdn-cache-ai/pingora_cache/target/debug/build/libz-ng-sys-87b6767cc1955227/out/build/zlib-ng-config-version.cmake"
    )
endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/Users/betty/Downloads/cdn-cache-ai/pingora_cache/target/debug/build/libz-ng-sys-87b6767cc1955227/out/build/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
