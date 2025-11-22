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
