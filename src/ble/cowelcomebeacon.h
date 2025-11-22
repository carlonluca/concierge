#ifndef COWELCOMEBEACON_H
#define COWELCOMEBEACON_H

#include <cstdint>

#include <bluefruit.h>

class COBeacon
{
public:
   COBeacon(const uint8_t uuid[16], uint8_t major, uint8_t minor, uint8_t txPower) :
      m_beacon(uuid, major, minor, -54) {
         Bluefruit.setTxPower(txPower);
         m_beacon.setManufacturer(MANUFACTURER_ID);
      }
   virtual ~COBeacon() {}

   void startAdvertising();
   void stopAdvertising();

private:
   BLEBeacon m_beacon;
   const uint16_t MANUFACTURER_ID = 0x0059;
};

class COWelcomeBeacon : public COBeacon
{
public:
   COWelcomeBeacon() : COBeacon(m_uuid, m_major, m_minor, m_txPower) {}

private:
   const uint8_t m_txPower = 8;
   const uint8_t m_major = 1;
   const uint8_t m_minor = 1;
   const uint8_t m_uuid[16] = {
      0x3A, 0x91, 0xF4, 0x27,
      0x8C, 0x56,
      0x4E, 0xA3,
      0xB2, 0x19,
      0x7D, 0xC4, 0x5A, 0x8F, 0x33, 0xE1
   };
};

#endif // COWELCOMEBEACON_H
