cmake_policy(SET CMP0037 OLD)
if(COMMAND cmake_policy)
      cmake_policy(SET CMP0003 NEW)
endif(COMMAND cmake_policy)

#set(OPENSSL_ROOT_DIR "C:/dev/vcpkg/installed/x86-windows-static")
set(OPENSSL_ROOT_DIR "C:/dev/vcpkg/installed/x64-windows-static")

add_compile_options(
  /std:c++17
  /MP
  /wd4018 # signed/unsigned msismatch
  /DVERSION="${VERSION}"
  /DUNICODE # config.pri
  /D_UNICODE
)

add_library(TextractorWebhook SHARED extension.cpp extensionimpl.cpp network.cpp)

target_link_libraries(TextractorWebhook wsock32 ws2_32)
target_link_libraries(TextractorWebhook winhttp)

find_package(OpenSSL REQUIRED)
target_link_libraries(TextractorWebhook OpenSSL::SSL OpenSSL::Crypto)
