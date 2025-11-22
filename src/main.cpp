#include <Adafruit_TinyUSB.h>
#include <bluefruit.h>

#define LC_LOGGING_DISABLE_THREADING
#include "../deps/LightLogger/lc_logging.h"

#include "ble/cowelcomebeacon.h"

lightlogger::custom_log_func lightlogger::global_log_func = lightlogger::log_to_default;

COWelcomeBeacon welcomeBeacon;

void setup() {
  pinMode(LED_BUILTIN, OUTPUT);

  Bluefruit.begin();
  welcomeBeacon.startAdvertising();
}

void loop() {
}
