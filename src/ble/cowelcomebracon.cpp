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

#include "cowelcomebeacon.h"

void COBeacon::startAdvertising()
{
   BLEAdvertising& adv = Bluefruit.Advertising;

   adv.setBeacon(m_beacon);
   adv.setType(BLE_GAP_ADV_TYPE_NONCONNECTABLE_SCANNABLE_UNDIRECTED);
   adv.restartOnDisconnect(true);
   adv.setInterval(160, 160);
   adv.setFastTimeout(30);
   adv.start();
}

void COBeacon::stopAdvertising()
{
   BLEAdvertising& adv = Bluefruit.Advertising;

   adv.stop();
}
