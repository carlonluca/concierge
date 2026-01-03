/*
 * Project: Concierge
 * Date:    2025.12.10
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

use std::time::Duration;
use btleplug::{
   api::{
      Central,
      Manager as _,
      Peripheral
   },
   platform::{Adapter, Manager}
};
use tokio::time::sleep;
use uuid::Uuid;
use crate::coenv::COEnv;

pub struct COQuery {
   pub uuid: Uuid
}

pub enum ErrorType {
   BtUnavailable,
   BeaconUnavailable,
   CommunicationFailure
}

impl COQuery {
   ///
   /// Reads a sample from the WelcomeBeacon.
   ///
   pub async fn read_temp(&self) -> Result<i32, ErrorType> {
      info!("Starting to read temp info...");

      let manager = match Manager::new().await {
         Ok(m) => m,
         Err(e) => {
            warn!("Failed to create Manager: {}", e);
            return Err(ErrorType::BtUnavailable);
         }
      };

      let adapters = match manager.adapters().await {
         Ok(a) => a,
         Err(e) => {
            warn!("Failed to get adapters: {}", e);
            return Err(ErrorType::BtUnavailable);
         }
      };

      if adapters.is_empty() {
         warn!("No BLE adapters found on this system.");
         return Err(ErrorType::BtUnavailable);
      }

      for adapter in adapters {
         match self.read_temp_adapter(adapter).await {
            Ok(t) => return Ok(t),
            Err(_) => {}
         }
      }

      return Err(ErrorType::BeaconUnavailable)
   }

   ///
   /// Reads a sample from the WelcomeBeacon by searching using the
   /// specified adapter.
   /// 
   async fn read_temp_adapter(&self, adapter: Adapter) -> Result<i32, ErrorType> {
      trace!("Trying to read temp info using adapter: {:?}",
         adapter.adapter_info().await);
      
      
      let scan_int = COEnv::scan_interval();
      trace!("Starting scan and waiting {} ms:", scan_int);

      if let Err(e) = adapter.start_scan(Default::default()).await {
         warn!("Failed to scan: {:?}", e);
         return Err(ErrorType::BtUnavailable);
      }

      // Wait a little for devices to be discovered.
      sleep(Duration::from_millis(scan_int)).await;

      let peripherals = adapter.peripherals().await;
      if peripherals.is_err() {
         warn!("Cannot find peripherals");
         return Err(ErrorType::BeaconUnavailable);
      }

      if let Ok(peripherals) = peripherals {
         info!("Peripherals found: {}", peripherals.len());
         for peripheral in peripherals {
            if self.is_welcome_beacon(&peripheral).await {
               return self.read_temp_peripheral(&peripheral).await
            }
         }
      }

      Err(ErrorType::BeaconUnavailable)
   }

   ///
   /// Tests whether this is the desired welcome beacon.
   ///
   async fn is_welcome_beacon(&self, peripheral: &impl Peripheral) -> bool {
      let props = peripheral.properties().await;
      match props {
         Ok(props) => {
            if let Some(props) = props {
               if props.manufacturer_data.is_empty() {
                  trace!("Empty manufacturer data");
               }
               for (_, data) in props.manufacturer_data {
                  info!("Beacon found       : {:?}", peripheral.address());
                  info!("  Manufacturer data: {:?}", data);

                  if data.iter().count() < 17 {
                     continue
                  }

                  let uuid = Uuid::from_slice(&data[1..17]);
                  info!("  UUID             : {:?}", uuid);

                  if let Ok(uuid) = uuid {
                     return uuid == self.uuid;
                  }
               }
            }
            else {
               warn!("Props missing");
            }
         },
         Err(e) => { warn!("Props missing: {:?}", e); }
      }

      return false;
   }

   ///
   /// Reads a sample from the WelcomeBeacon by searching in the specified
   /// peripheral.
   /// 
   async fn read_temp_peripheral(&self, peripheral: &impl Peripheral) -> Result<i32, ErrorType> {
      info!("Reading temp from peripheral: {:?}", peripheral.address());

      let is_connected = peripheral.is_connected().await;
      match is_connected {
         Ok(is_connected) => {
            if !is_connected {
               let is_connected = peripheral.connect().await;
               match is_connected {
                  Ok(_) => { info!("Connected to peripheral: {:?}", peripheral.address()) }
                  Err(e) => {
                     warn!("Failed to connect to peripheral {:?}: {:?}", peripheral.address(), e);
                     return Err(ErrorType::CommunicationFailure);
                  }
               }
            }
         },
         Err(e) => {
            warn!("Failed to verify connection: {:?}", e);
            return Err(ErrorType::CommunicationFailure);
         }
      }

      if let Err(e) = peripheral.discover_services().await {
         warn!("Failed to query for services: {:?}", e);
         return Err(ErrorType::CommunicationFailure);
      }

      let services = peripheral.services();
      for service in &services {
         trace!("Service found: {:?}", service.uuid);
      }

      let temp_service_uuid = COEnv::temp_service_uuid();
      let tmp_service = services
            .iter()
            .find(|s| s.uuid == temp_service_uuid);
      if tmp_service == None {
         warn!("Could not find temp service in welcome beacon");
         return Err(ErrorType::CommunicationFailure);
      }

      let chars = &tmp_service.unwrap().characteristics;
      for char in chars {
         trace!("Characteristic found: {:?}", char.uuid);
      }

      let read_temp_char_uuid = COEnv::temp_read_char_uuid();
      let read_temp_char = chars
            .iter()
            .find(|c| c.uuid == read_temp_char_uuid);
      if read_temp_char == None {
         warn!("Could not find read temp characteristic");
         return Err(ErrorType::CommunicationFailure);
      }

      info!("Reading temp...");
      match peripheral.read(&read_temp_char.unwrap()).await {
         Ok(value) => { info!("Temp successfully read: {:?}", value); }
         Err(e) => { warn!("Failed to read temp: {:?}", e); }
      }
      
      match peripheral.disconnect().await {
         Ok(_) => { info!("Disconnected from peripheral: {:?}", peripheral.address()); }
         Err(e) => { warn!("Failed to disconnect from peripheral {:?}: {:?}", peripheral.address(), e); }
      }

      Ok(5)
   }
}
