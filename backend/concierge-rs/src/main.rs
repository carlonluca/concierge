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

use btleplug::api::{
    Central,
    Manager as _,
    Peripheral,
    CharPropFlags
};
use btleplug::platform::Manager;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    println!("Scanning for BLE adapters (using btleplug)...");

    let manager = match Manager::new().await {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to create Manager: {}", e);
            return;
        }
    };

    let adapters = match manager.adapters().await {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Failed to get adapters: {}", e);
            return;
        }
    };

    if adapters.is_empty() {
        println!("No BLE adapters found on this system.");
        return;
    }

    for adapter in adapters {
        println!("Found adapter: {:?}", adapter.adapter_info().await);

        println!("Starting scan on adapter...");
        if let Err(e) = adapter.start_scan(Default::default()).await {
            eprintln!("Failed to start scan: {}", e);
            continue;
        }

        // Wait a little for devices to be discovered
        sleep(Duration::from_secs(4)).await;

        let peripherals = adapter.peripherals().await;
        let peripherals = peripherals.unwrap();
        println!("Peripherals found: {}", peripherals.len());

        for peripheral in peripherals {
            println!("Peripheral: {}", peripheral.id());
            let props = peripheral.properties().await;
            let services = props.unwrap().unwrap().services;
            for service in services {
                println!("Props: {}", service);
            }

            // Connect if not already connected
            if peripheral.is_connected().await.is_ok() {
                println!("Connecting to peripheral...");
                peripheral.connect().await;
                // Optional: discover characteristics/services after connect
                peripheral.discover_services().await;
            }

            // Discover services & characteristics (populates characteristic list)
            peripheral.discover_services().await;

            // List all services and characteristics
            println!("Discovered characteristics:");
            for c in peripheral.characteristics() {
                println!("- UUID: {}, props: {:?}", c.uuid, c.properties);
            }

            // Example 1: Read all readable characteristics
            println!("\nReading all readable characteristics:");
            for c in peripheral.characteristics() {
                if c.properties.contains(CharPropFlags::READ) {
                    match peripheral.read(&c).await {
                        Ok(value) => {
                            println!("Read {} bytes from {}: {:02x?}", value.len(), c.uuid, value);
                            // If you expect UTF-8 text:
                            if let Ok(text) = std::str::from_utf8(&value) {
                                println!("  as UTF-8: {:?}", text);
                            }
                        }
                        Err(e) => println!("Failed to read {}: {}", c.uuid, e),
                    }
                } else {
                    println!("Skipping {} (not readable)", c.uuid);
                }
            }
        }

        if let Err(e) = adapter.stop_scan().await {
            eprintln!("Failed to stop scan: {}", e);
        }
    }
}
