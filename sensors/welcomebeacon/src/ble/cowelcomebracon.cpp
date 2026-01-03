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

#include <core/cologging.h>

#include "cowelcomebeacon.h"

static COBeacon* g_beacon = nullptr;

void connection_callback(uint16_t handle)
{
   if (!g_beacon) {
      log_critical("Connection callback received without global instance");
      return;
   }

   g_beacon->connectedCallback(handle);
}

void disconnection_callback(uint16_t handle, uint8_t reason)
{
   if (!g_beacon) {
      log_critical("Disconnection callback received without global instance");
      return;
   }

   g_beacon->disconnectedCallback(handle, reason);
}

COBeacon::COBeacon(const UuidData uuid) : m_uuid(uuid)
{
   m_periph.setConnectCallback(connection_callback);
   m_periph.setDisconnectCallback(disconnection_callback);
   if (g_beacon) {
      log_critical("Only one beacon instance is allowed!");
      return;
   }

   g_beacon = this;

   Bluefruit.setTxPower(8);
}

void COBeacon::startAdvertising()
{
   stopAdvertising();
   m_adv.clearData();
   m_adv.addFlags(BLE_GAP_ADV_FLAGS_LE_ONLY_GENERAL_DISC_MODE);
   m_adv.addTxPower();
   setupBeaconAdvData();
   addServices();
   m_adv.restartOnDisconnect(true);
   m_adv.setInterval(160, 160);
   m_adv.setFastTimeout(30);
   m_adv.start(0);
}

void COBeacon::stopAdvertising()
{
   m_adv.stop();
}

void COBeacon::connectedCallback(uint16_t handle)
{
   auto* connection = Bluefruit.Connection(handle);
   assert(connection);
   if (!connection) {
      log_warn("Could get handle to BLE connection");
      return;
   }

   std::string centralName;
   centralName.resize(32);
   connection->getPeerName(&centralName[0], centralName.size());

   log_info("BLE connection established to %s", centralName.c_str());
}

void COBeacon::disconnectedCallback(uint16_t handle, uint8_t reason)
{
   log_info("BLE disconnection, reason: %u", reason);
}

void COBeacon::setupBeaconAdvData()
{
   uint8_t mfg_data[2 + 1 + 16];

   mfg_data[0] = 0xAA;   // Company ID LSB
   mfg_data[1] = 0xAA;   // Company ID MSB
   mfg_data[2] = 0x01;   // Custom type/version

   memcpy(&mfg_data[3], m_uuid.data(), 16);

   m_adv.addManufacturerData(mfg_data, sizeof(mfg_data));
}

template <typename V>
V reversed(const V& in)
{
    V out = in;
    std::reverse(out.begin(), out.end());
    return out;
}

COWelcomeBeacon::COWelcomeBeacon() :
   COBeacon(m_uuid)
{
   m_dis.setManufacturer("Luke");
   m_dis.setModel("Welcome beacon");
   m_dis.begin();

   m_tempService.setUuid(reversed(m_uuidTempService).data());
   m_tempService.begin();
   m_measChar.setUuid(reversed(m_uuidTempMeas).data());
   m_measChar.setProperties(CHR_PROPS_READ);
   m_measChar.setPermission(SECMODE_OPEN, SECMODE_NO_ACCESS);
   m_measChar.setFixedLen(4);
   m_measChar.begin();
   m_measChar.write32(0);
}

void COWelcomeBeacon::addServices()
{
   m_adv.addService(m_dis);
   m_adv.addService(m_tempService);
}

void COWelcomeBeacon::setCurrentMeas(uint32_t m)
{
   m_measurement = m;
   m_measChar.write32(m);
}
