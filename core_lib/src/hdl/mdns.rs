use std::sync::{Arc, Mutex};
use std::time::Duration;

use mdns_sd::{AddrType, ServiceDaemon, ServiceInfo};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast::Receiver;
use tokio::sync::watch;
use tokio::time::{interval_at, Instant};
use tokio_util::sync::CancellationToken;
use ts_rs::TS;

use crate::utils::{
    gen_mdns_endpoint_info, gen_mdns_name, ignored_mdns_interface_names, local_mdns_ipv4_addrs,
    mdns_host_name, DeviceType,
};

const INNER_NAME: &str = "MDnsServer";
const TICK_INTERVAL: Duration = Duration::from_secs(60);

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Visibility {
    Visible = 0,
    Invisible = 1,
    Temporarily = 2,
}

#[allow(dead_code)]
impl Visibility {
    pub fn from_raw_value(value: u64) -> Self {
        match value {
            0 => Visibility::Visible,
            1 => Visibility::Invisible,
            2 => Visibility::Temporarily,
            _ => unreachable!(),
        }
    }
}

pub struct MDnsServer {
    daemon: ServiceDaemon,
    service_info: ServiceInfo,
    endpoint_id: [u8; 4],
    service_port: u16,
    device_type: DeviceType,
    ble_receiver: Receiver<()>,
    visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
    visibility_receiver: watch::Receiver<Visibility>,
    device_name_receiver: watch::Receiver<String>,
}

impl MDnsServer {
    pub fn new(
        endpoint_id: [u8; 4],
        service_port: u16,
        ble_receiver: Receiver<()>,
        visibility_sender: Arc<Mutex<watch::Sender<Visibility>>>,
        visibility_receiver: watch::Receiver<Visibility>,
        device_name_receiver: watch::Receiver<String>,
    ) -> Result<Self, anyhow::Error> {
        let device_type = DeviceType::Laptop;
        let service_info = Self::build_service(
            endpoint_id,
            service_port,
            device_type.clone(),
            &device_name_receiver.borrow(),
        )?;

        let daemon = ServiceDaemon::new()?;
        for interface_name in ignored_mdns_interface_names() {
            debug!("{INNER_NAME}: disabling mDNS on interface {interface_name}");
            if let Err(err) = daemon.disable_interface(interface_name.as_str()) {
                warn!("{INNER_NAME}: could not disable mDNS interface {interface_name}: {err}");
            }
        }

        Ok(Self {
            daemon,
            service_info,
            endpoint_id,
            service_port,
            device_type,
            ble_receiver,
            visibility_sender,
            visibility_receiver,
            device_name_receiver,
        })
    }

    pub async fn run(&mut self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: service starting");
        let monitor = self.daemon.monitor()?;
        let mut ble_receiver = self.ble_receiver.resubscribe();
        let mut visibility = *self.visibility_receiver.borrow();
        let mut interval = interval_at(Instant::now() + TICK_INTERVAL, TICK_INTERVAL);
        let mut registered = false;

        if visibility != Visibility::Invisible {
            self.register_current_service("startup")?;
            registered = true;
        }

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, breaking");
                    break;
                }
                r = monitor.recv_async() => {
                    match r {
                        Ok(_) => continue,
                        Err(err) => return Err(err.into()),
                    }
                },
                _ = self.visibility_receiver.changed() => {
                    visibility = *self.visibility_receiver.borrow_and_update();

                    debug!("{INNER_NAME}: visibility changed: {visibility:?}");
                    if visibility == Visibility::Visible {
                        if registered {
                            self.daemon.register_resend(self.service_info.get_fullname())?;
                        } else {
                            self.register_current_service("visibility")?;
                            registered = true;
                        }
                    } else if visibility == Visibility::Invisible {
                        if registered {
                            let receiver = self.daemon.unregister(self.service_info.get_fullname())?;
                            let _ = receiver.recv();
                            registered = false;
                        }
                    } else if visibility == Visibility::Temporarily {
                        if registered {
                            self.daemon.register_resend(self.service_info.get_fullname())?;
                        } else {
                            self.register_current_service("temporary visibility")?;
                            registered = true;
                        }
                        interval.reset();
                    }
                }
                _ = ble_receiver.recv() => {
                    if visibility == Visibility::Invisible {
                        continue;
                    }

                    debug!("{INNER_NAME}: ble_receiver: got event");
                    if visibility == Visibility::Visible || visibility == Visibility::Temporarily {
                        // Android can sometime not see the mDNS service if the service
                        // was running BEFORE Android started the Discovery phase for QuickShare.
                        // So resend a broadcast if there's a android device sending.
                        if registered {
                            self.daemon.register_resend(self.service_info.get_fullname())?;
                        } else {
                            self.register_current_service("nearby BLE event")?;
                            registered = true;
                        }
                    } else {
                        self.register_current_service("nearby BLE event")?;
                        registered = true;
                    }
                },
                _ = interval.tick() => {
                    if visibility != Visibility::Temporarily {
                        continue;
                    }

                    if registered {
                        let receiver = self.daemon.unregister(self.service_info.get_fullname())?;
                        let _ = receiver.recv();
                        registered = false;
                    }
                    let _ = self.visibility_sender.lock().unwrap().send(Visibility::Invisible);
                }
                _ = self.device_name_receiver.changed() => {
                    let device_name = self.device_name_receiver.borrow_and_update().clone();
                    debug!("{INNER_NAME}: device name changed: {device_name}");

                    if registered {
                        if let Ok(receiver) = self.daemon.unregister(self.service_info.get_fullname()) {
                            let _ = receiver.recv();
                        }
                        registered = false;
                    }

                    self.service_info = Self::build_service(
                        self.endpoint_id,
                        self.service_port,
                        self.device_type.clone(),
                        &device_name,
                    )?;

                    if visibility != Visibility::Invisible {
                        self.register_current_service("device name change")?;
                        registered = true;
                    }
                }
            }
        }

        // Unregister the mDNS service - we're shutting down
        if registered {
            let receiver = self.daemon.unregister(self.service_info.get_fullname())?;
            if let Ok(event) = receiver.recv() {
                info!("MDnsServer: service unregistered: {:?}", &event);
            }
        }

        Ok(())
    }

    fn register_current_service(&self, reason: &str) -> Result<(), anyhow::Error> {
        info!(
            "{INNER_NAME}: registering {} on {} because {reason}",
            self.service_info.get_fullname(),
            self.service_info.get_hostname()
        );
        self.daemon.register(self.service_info.clone())?;
        Ok(())
    }

    fn build_service(
        endpoint_id: [u8; 4],
        service_port: u16,
        device_type: DeviceType,
        device_name: &str,
    ) -> Result<ServiceInfo, anyhow::Error> {
        let name = gen_mdns_name(endpoint_id);
        info!("Broadcasting with: {device_name}");
        let endpoint_info = gen_mdns_endpoint_info(device_type as u8, device_name);
        let hostname = mdns_host_name();
        let addresses = local_mdns_ipv4_addrs();
        let ip_csv = addresses
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");

        if ip_csv.is_empty() {
            warn!("{INNER_NAME}: no physical LAN IPv4 address found; falling back to automatic mDNS address selection");
        } else {
            info!("{INNER_NAME}: advertising IPv4 address(es): {ip_csv}");
        }

        let properties = [("n", endpoint_info)];
        let mut si = ServiceInfo::new(
            "_FC9F5ED42C8A._tcp.local.",
            &name,
            &hostname,
            ip_csv.as_str(),
            service_port,
            &properties[..],
        )?;

        if ip_csv.is_empty() {
            si = si.enable_addr_auto(AddrType::V4);
        }

        Ok(si)
    }
}
