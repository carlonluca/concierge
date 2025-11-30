/*
 * Project: Concierge
 * Date:    2025.11.22
 *
 * Copyright (C) 2025 Luca Carlon
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef COWELCOMEBEACON_H
#define COWELCOMEBEACON_H

#include <cstdint>

#include <bluefruit.h>

extern void connection_callback(uint16_t handle);
extern void disconnection_callback(uint16_t handle);

class COBeacon
{
public:
   explicit COBeacon(const uint8_t uuid[16], uint8_t major, uint8_t minor, uint8_t txPower);
   virtual ~COBeacon() {}

   void startAdvertising();
   void stopAdvertising();

protected:
   virtual void addServices() {}
   virtual void connectedCallback(uint16_t handle);
   virtual void disconnectedCallback(uint16_t handle, uint8_t reason);

protected:
   BLEAdvertising& m_adv = Bluefruit.Advertising;
   BLEPeriph& m_periph = Bluefruit.Periph;
   const uint16_t MANUFACTURER_ID = 0x0059;

private:
   friend void connection_callback(uint16_t handle);
   friend void disconnection_callback(uint16_t handle, uint8_t reason);
};

class COWelcomeBeacon : public COBeacon
{
public:
   COWelcomeBeacon();

protected:
   void addServices() override;

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

   BLEDis m_dis;
};

#endif // COWELCOMEBEACON_H
