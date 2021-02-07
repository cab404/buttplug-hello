use buttplug::{client::{ButtplugClient, ButtplugClientDevice, VibrateCommand}, server::ButtplugServerOptions, util::{
        async_manager
    }};
use std::{sync::Arc, time::Duration};
use futures::StreamExt;
use futures_timer::Delay;

fn main() {
  println!("start...");

  async_manager::block_on(async {
    control().await;
  });
}


async fn control () {
    println!("running control");
    
    let client = ButtplugClient::new("mowmow");
    let mut events = client.event_stream();
    println!("waiting for server");

    client.connect_in_process(&ButtplugServerOptions::default()).await.unwrap();
    
    if let Err(err) = client.start_scanning().await {
        println!("Client errored when starting scan! {}", err);
        return;
    }

    let vibr = | dev: Arc<ButtplugClientDevice> | {
        async move {
            println!("{} should start vibrating!", dev.name);
            loop {
                dev.vibrate(VibrateCommand::Speed(0.9)).await.unwrap();
                Delay::new(Duration::from_millis(500)).await;
                dev.vibrate(VibrateCommand::Speed(0.5)).await.unwrap();
                Delay::new(Duration::from_millis(500)).await;
            }
        }
    };

 
    println!("loop starting now");
    
    loop {
        match events.next().await.unwrap() {
            buttplug::client::ButtplugClientEvent::ScanningFinished => {}
            buttplug::client::ButtplugClientEvent::DeviceAdded(dev) => {
                println!("Device {devname} added!", devname = dev.name );
                vibr(dev).await;
            }
            buttplug::client::ButtplugClientEvent::DeviceRemoved(_) => {}
            buttplug::client::ButtplugClientEvent::PingTimeout => {}
            buttplug::client::ButtplugClientEvent::ServerConnect => {}
            buttplug::client::ButtplugClientEvent::ServerDisconnect => {}
            buttplug::client::ButtplugClientEvent::Error(_) => {}
        }
    }

}