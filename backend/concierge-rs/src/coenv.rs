/*
 * Project: Concierge
 * Date:    2025.12.14
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

use std::env;
use uuid::Uuid;

pub struct COEnv {}

impl COEnv {
   pub fn read_env_string(key: &str, def: String) -> String {
      env::var(key).unwrap_or_else(|_| def)
   }

   pub fn read_env_u64(key: &str, def: u64) -> u64 {
      let v = Self::read_env_string(key, def.to_string());
      match v.parse() {
         Ok(v) => v,
         Err(_) => def
      }
   }

   pub fn read_env_uuid(key: &str, def: Uuid, err: &str) -> Uuid {
      let uuid = COEnv::read_env_string(key, def.to_string());
      Uuid::parse_str(&uuid)
         .expect(err)
   }

   pub fn scan_interval() -> u64 {
      COEnv::read_env_u64("CONCIERGE_SCAN_INTERVAL", 4000)
   }

   pub fn welcome_beacon_uuid() -> Uuid {
      COEnv::read_env_uuid("CONCIERGE_WELCOME_BEACON_UUID",
         Uuid::parse_str(&"3a91f427-8c56-4ea3-b219-7dc45a8f33e1").unwrap(),
         "Invalid welcome beacon uuid")
   }

   pub fn temp_service_uuid() -> Uuid {
      COEnv::read_env_uuid("CONCIERGE_TEMP_SERVICE_UUID",
            Uuid::parse_str("3a91f427-8c56-4ea3-b219-7dc45a8f0000").unwrap(),
            "Invalid temp service")
   }

   pub fn temp_read_char_uuid() -> Uuid {
      COEnv::read_env_uuid("CONCIERGE_TEMP_READ_CHAR_UUID",
            Uuid::parse_str("3a91f427-8c56-4ea3-b219-7dc45a8f0001").unwrap(),
            "Invalid read temp characteristic")
   }
}
