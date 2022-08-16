#define CPPHTTPLIB_OPENSSL_SUPPORT
#include "extension.h"
#include "httplib.h"
#include "common.h"
#include "network.h"
#include <fstream>
#include <sstream>
//#include <chrono>

std::string webhook_base;
std::string webhook_endpoint;
//std::wstring accumulator;
//std::chrono::steady_clock::time_point rate_limiter;

BOOL WINAPI DllMain(HMODULE hModule, DWORD ul_reason_for_call,
                    LPVOID lpReserved) {

  switch (ul_reason_for_call) {
  case DLL_PROCESS_ATTACH: {
    std::ifstream file("webhook_url.txt", std::ios::in);
    if (!file.is_open()) {
      MessageBoxW(NULL, L"Error loading file", L"Textractor Webhook", MB_OK);
      break;
      return FALSE;
    }

    //TODO better error handling here
    std::getline(file, webhook_base);
    std::getline(file, webhook_endpoint);
    //rate_limiter = std::chrono::steady_clock::now();
    file.close();
  } break;
  case DLL_PROCESS_DETACH:
    break;
  }
  return TRUE;
}

bool ProcessSentence(std::wstring &sentence, SentenceInfo sentenceInfo) {
  if (sentenceInfo["text number"] == 0 || sentenceInfo["text number"] == 1)
    return false;

  // Me just struggling with function pointers don't mind me
  // https://github.com/Artikash/Textractor/blob/15db478e62eb955d2299484dbf2c9c7e707bb4cb/GUI/mainwindow.cpp
  // https://www.cprogramming.com/tutorial/function-pointers.html
  // int64_t text_thread = sentenceInfo["text_number"];
  // int64_t (*AddText)(int64_t number, const wchar_t *text);
  // AddText = sentenceInfo["add_text"];
  // AddText(text_thread, "baba");

  if (sentenceInfo["current select"] && sentenceInfo["process id"] != 0) {
    // Textractor actually plays nice with the rate limiter, it seems?
    /*
    std::chrono::steady_clock::time_point elapsed = std::chrono::steady_clock::now();
    auto delta = std::chrono::duration_cast<std::chrono::milliseconds>(elapsed - rate_limiter).count();

    
    if (delta < 2000) {
        sentence += L"TOO FAST";
        rate_limiter = std::chrono::steady_clock::now();
    }
    */
    std::string str = FormatString(R"({"content":"%s"})", JSON::Escape(WideStringToString(sentence)));
    httplib::Client cli(webhook_base);
    cli.Post(webhook_endpoint,
             str, "application/json");
  }
  return true;
}
