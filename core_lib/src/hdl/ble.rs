use std::time::{Duration, SystemTime};

use anyhow::anyhow;
use btleplug::api::{Central, CentralEvent, Manager as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use futures::stream::StreamExt;
use tokio::sync::broadcast::Sender;
use tokio_util::sync::CancellationToken;
use uuid::{uuid, Uuid};

const SERVICE_UUID_SHARING: Uuid = uuid!("0000fe2c-0000-1000-8000-00805f9b34fb");
const SAMSUNG_COMPANY_ID: u16 = 117;
const RESEND_THROTTLE: Duration = Duration::from_secs(5);

const INNER_NAME: &str = "BleListener";

pub struct BleListener {
    adapter: Adapter,
    sender: Sender<()>,
}

impl BleListener {
    pub async fn new(sender: Sender<()>) -> Result<Self, anyhow::Error> {
        let manager = Manager::new().await?;
        let adapters = manager.adapters().await?;
        if adapters.is_empty() {
            return Err(anyhow!("no bluetooth adapter"));
        }

        Ok(Self {
            adapter: adapters[0].clone(),
            sender,
        })
    }

    pub async fn run(self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!("{INNER_NAME}: service starting");

        let mut events = self.adapter.events().await?;
        // Samsung Quick Share on current phones can expose only manufacturer data
        // while the send sheet is searching, so scan broadly and filter events here.
        self.adapter.start_scan(ScanFilter::default()).await?;

        let mut last_alert = SystemTime::UNIX_EPOCH;

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, breaking");
                    break;
                }
                Some(e) = events.next() => {
                    match e {
                        CentralEvent::ServiceDataAdvertisement { id, service_data } => {
                            // Sanity check as per: https://github.com/Martichou/rquickshare/issues/74
                            // Seems like the filtering is not enough, so we'll add a check before
                            // proceeding with the service_data.
                            if !service_data.contains_key(&SERVICE_UUID_SHARING) {
                                continue;
                            }

                            let now = SystemTime::now();
                            if now.duration_since(last_alert)? <= RESEND_THROTTLE {
                                continue;
                            }

                            debug!("{INNER_NAME}: A device ({id}) is sharing ({service_data:?}) nearby");
                            self.sender.send(())?;
                            last_alert = now;
                        },
                        CentralEvent::ManufacturerDataAdvertisement { id, manufacturer_data } => {
                            if !is_samsung_quick_share_hint(&manufacturer_data) {
                                continue;
                            }

                            let now = SystemTime::now();
                            if now.duration_since(last_alert)? <= RESEND_THROTTLE {
                                continue;
                            }

                            debug!("{INNER_NAME}: Samsung Quick Share search hint from {id} ({manufacturer_data:?})");
                            self.sender.send(())?;
                            last_alert = now;
                        },
                        // Not interesting for us
                        _ => {
                            // trace!("{INNER_NAME}: Another CentralEvent got the same services: {:?}", e);
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

fn is_samsung_quick_share_hint(
    manufacturer_data: &std::collections::HashMap<u16, Vec<u8>>,
) -> bool {
    manufacturer_data.contains_key(&SAMSUNG_COMPANY_ID)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{is_samsung_quick_share_hint, SAMSUNG_COMPANY_ID};

    #[test]
    fn detects_samsung_manufacturer_data_as_search_hint() {
        let mut data = HashMap::new();
        data.insert(SAMSUNG_COMPANY_ID, vec![0x02, 0x18, 0x61]);

        assert!(is_samsung_quick_share_hint(&data));
    }

    #[test]
    fn ignores_other_manufacturer_data() {
        let mut data = HashMap::new();
        data.insert(0x004c, vec![0x02, 0x15]);

        assert!(!is_samsung_quick_share_hint(&data));
    }
}
