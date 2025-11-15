#include <Adafruit_TinyUSB.h>

#define LC_LOGGING_DISABLE_THREADING
#include "../deps/LightLogger/lc_logging.h"

lightlogger::custom_log_func lightlogger::global_log_func = lightlogger::log_to_default;

void setup() {
  pinMode(LED_BUILTIN, OUTPUT);
}

void loop() {
  digitalWrite(LED_RED, HIGH);
  digitalWrite(LED_BLUE, HIGH);
  delay(1000);
  digitalWrite(LED_RED, LOW);
  digitalWrite(LED_BLUE, LOW);
  delay(1000);
  lightlogger::log_debug("HELLO!");
}
