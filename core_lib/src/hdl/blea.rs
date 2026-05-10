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
    // Nearby Share fast-initiation model id fc128e + V1 notify metadata.
    0xfc, 0x12, 0x8e, 0x00, 0x42,
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
        let initial_discoverable = self.adapter.is_discoverable().await.unwrap_or(false);
        let initial_discoverable_timeout = self.adapter.discoverable_timeout().await.unwrap_or(180);
        self.set_visible_mode(true).await;
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
        self.restore_visible_mode(initial_discoverable, initial_discoverable_timeout)
            .await;

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
        let initial_discoverable = self.adapter.is_discoverable().await.unwrap_or(false);
        let initial_discoverable_timeout = self.adapter.discoverable_timeout().await.unwrap_or(180);
        let mut handle = if *visibility_receiver.borrow() == Visibility::Invisible {
            None
        } else {
            self.set_visible_mode(true).await;
            Some(self.start_advertising(service_uuid).await?)
        };

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, returning");
                    drop(handle);
                    self.restore_visible_mode(initial_discoverable, initial_discoverable_timeout).await;
                    return Ok(());
                }
                changed = visibility_receiver.changed() => {
                    changed?;
                    let visibility = *visibility_receiver.borrow_and_update();
                    debug!("{INNER_NAME}: visibility changed: {visibility:?}");

                    if visibility == Visibility::Invisible {
                        handle = None;
                        self.set_visible_mode(false).await;
                    } else if handle.is_none() {
                        self.set_visible_mode(true).await;
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
            service_uuids: std::iter::once(service_uuid).collect(),
            service_data: [(service_uuid, adv_data.into())].into(),
            discoverable: is_peripheral.then_some(true),
            discoverable_timeout: is_peripheral.then_some(Duration::from_secs(0)),
            local_name: is_peripheral.then(default_device_name),
            min_interval: Some(Duration::from_millis(100)),
            max_interval: Some(Duration::from_millis(100)),
            ..Default::default()
        }
    }

    async fn set_visible_mode(&self, discoverable: bool) {
        if discoverable {
            if let Err(err) = self.adapter.set_discoverable_timeout(0).await {
                warn!("{INNER_NAME}: could not disable adapter discoverable timeout: {err}");
            }
        }

        if let Err(err) = self.adapter.set_discoverable(discoverable).await {
            warn!("{INNER_NAME}: could not set adapter discoverable={discoverable}: {err}");
        }
    }

    async fn restore_visible_mode(&self, discoverable: bool, discoverable_timeout: u32) {
        if let Err(err) = self.adapter.set_discoverable(discoverable).await {
            warn!("{INNER_NAME}: could not restore adapter discoverable={discoverable}: {err}");
        }

        if let Err(err) = self
            .adapter
            .set_discoverable_timeout(discoverable_timeout)
            .await
        {
            warn!(
                "{INNER_NAME}: could not restore adapter discoverable timeout={discoverable_timeout}: {err}"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SERVICE_DATA;

    #[test]
    fn fast_initiation_payload_matches_nearby_share_shape() {
        assert_eq!(SERVICE_DATA.as_ref(), &[0xfc, 0x12, 0x8e, 0x00, 0x42]);
    }
}
