use std::time::Duration;
use btleplug::api::{Central, Manager as _, Peripheral};
use btleplug::platform::Manager;
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
        }

        if let Err(e) = adapter.stop_scan().await {
            eprintln!("Failed to stop scan: {}", e);
        }
    }
}
