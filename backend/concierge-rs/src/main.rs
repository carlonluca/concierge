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

mod coquery;
mod coenv;

use crate::{coenv::COEnv, coquery::COQuery};

extern crate pretty_env_logger;
#[macro_use] extern crate log;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let welcome_beacon_uuid = COEnv::welcome_beacon_uuid();
    let query = COQuery { uuid: welcome_beacon_uuid };
    let _ = query.read_temp().await;
}
