use std::sync::Arc;
use std::time::Duration;

use bluer::adv::{Advertisement, AdvertisementHandle, Type};
use bluer::UuidExt;
use bytes::Bytes;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::utils::default_device_name;
use crate::Visibility;

const SERVICE_DATA: Bytes = Bytes::from_static(&[
    // Nearby Share fast-initiation model id fc128e + Android receiver metadata.
    0xfc, 0x12, 0x8e, 0x01, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xbf, 0x2d,
    0x5b, 0xa0, 0xe1, 0xd8, 0x75, 0x24, 0xca, 0x00,
]);

const INNER_NAME: &str = "BleAdvertiser";

#[derive(Debug, Clone)]
pub struct BleAdvertiser {
    adapter: Arc<bluer::Adapter>,
}

impl BleAdvertiser {
    pub async fn new() -> Result<Self, anyhow::Error> {
        let session = bluer::Session::new().await?;
        let adapter = session.default_adapter().await?;
        adapter.set_powered(true).await?;

        Ok(Self {
            adapter: Arc::new(adapter),
        })
    }

    pub async fn run(&self, ctk: CancellationToken) -> Result<(), anyhow::Error> {
        info!(
            "{INNER_NAME}: advertising on Bluetooth adapter {} with address {}",
            self.adapter.name(),
            self.adapter.address().await?
        );

        let service_uuid = Uuid::from_u16(0xFE2C);
        let handle = match self
            .advertise(service_uuid, SERVICE_DATA, Type::Broadcast)
            .await
        {
            Ok(handle) => handle,
            Err(err) => {
                warn!(
                    "{INNER_NAME}: broadcast advertisement failed ({err}); retrying as peripheral"
                );
                self.advertise(service_uuid, SERVICE_DATA, Type::Peripheral)
                    .await?
            }
        };

        ctk.cancelled().await;
        info!("{INNER_NAME}: tracker cancelled, returning");
        drop(handle);

        Ok(())
    }

    pub async fn run_for_visibility(
        &self,
        ctk: CancellationToken,
        mut visibility_receiver: watch::Receiver<Visibility>,
    ) -> Result<(), anyhow::Error> {
        info!(
            "{INNER_NAME}: visibility-controlled advertising on Bluetooth adapter {} with address {}",
            self.adapter.name(),
            self.adapter.address().await?
        );

        let service_uuid = Uuid::from_u16(0xFE2C);
        let mut handle = if *visibility_receiver.borrow() == Visibility::Invisible {
            None
        } else {
            Some(self.start_advertising(service_uuid).await?)
        };

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, returning");
                    drop(handle);
                    return Ok(());
                }
                changed = visibility_receiver.changed() => {
                    changed?;
                    let visibility = *visibility_receiver.borrow_and_update();
                    debug!("{INNER_NAME}: visibility changed: {visibility:?}");

                    if visibility == Visibility::Invisible {
                        handle = None;
                    } else if handle.is_none() {
                        handle = Some(self.start_advertising(service_uuid).await?);
                    }
                }
            }
        }
    }

    async fn start_advertising(
        &self,
        service_uuid: Uuid,
    ) -> Result<AdvertisementHandle, anyhow::Error> {
        match self
            .advertise(service_uuid, SERVICE_DATA, Type::Broadcast)
            .await
        {
            Ok(handle) => Ok(handle),
            Err(err) => {
                warn!(
                    "{INNER_NAME}: broadcast advertisement failed ({err}); retrying as peripheral"
                );
                self.advertise(service_uuid, SERVICE_DATA, Type::Peripheral)
                    .await
            }
        }
    }

    async fn advertise(
        &self,
        service_uuid: Uuid,
        adv_data: Bytes,
        advertisement_type: Type,
    ) -> Result<AdvertisementHandle, anyhow::Error> {
        self.adapter
            .advertise(self.get_advertisement(service_uuid, adv_data, advertisement_type))
            .await
            .map_err(Into::into)
    }

    fn get_advertisement(
        &self,
        service_uuid: Uuid,
        adv_data: Bytes,
        advertisement_type: Type,
    ) -> Advertisement {
        let is_peripheral = advertisement_type == Type::Peripheral;
        Advertisement {
            advertisement_type,
            service_data: [(service_uuid, adv_data.into())].into(),
            discoverable: is_peripheral.then_some(true),
            discoverable_timeout: None,
            local_name: is_peripheral.then(default_device_name),
            min_interval: Some(Duration::from_millis(100)),
            max_interval: Some(Duration::from_millis(100)),
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SERVICE_DATA;

    #[test]
    fn fast_initiation_payload_matches_nearby_share_shape() {
        assert_eq!(SERVICE_DATA.len(), 24);
        assert_eq!(
            &SERVICE_DATA[..14],
            &[0xfc, 0x12, 0x8e, 0x01, 0x42, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
