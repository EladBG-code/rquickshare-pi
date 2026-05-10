use std::sync::Arc;

use bluer::adv::{Advertisement, AdvertisementHandle, Type};
use bluer::UuidExt;
use bytes::Bytes;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::utils::default_device_name;
use crate::Visibility;

const SERVICE_DATA: Bytes = Bytes::from_static(&[
    252, 18, 142, 1, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 191, 45, 91, 160, 225, 216, 117, 36, 202, 0,
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
        self.set_discoverable(true).await;
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
        self.set_discoverable(false).await;

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
        let mut handle = if *visibility_receiver.borrow() == Visibility::Invisible {
            None
        } else {
            self.set_discoverable(true).await;
            Some(self.start_advertising(service_uuid).await?)
        };

        loop {
            tokio::select! {
                _ = ctk.cancelled() => {
                    info!("{INNER_NAME}: tracker cancelled, returning");
                    drop(handle);
                    self.set_discoverable(initial_discoverable).await;
                    return Ok(());
                }
                changed = visibility_receiver.changed() => {
                    changed?;
                    let visibility = *visibility_receiver.borrow_and_update();
                    debug!("{INNER_NAME}: visibility changed: {visibility:?}");

                    if visibility == Visibility::Invisible {
                        handle = None;
                        self.set_discoverable(false).await;
                    } else if handle.is_none() {
                        self.set_discoverable(true).await;
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
            local_name: is_peripheral.then(default_device_name),
            ..Default::default()
        }
    }

    async fn set_discoverable(&self, discoverable: bool) {
        if let Err(err) = self.adapter.set_discoverable(discoverable).await {
            warn!("{INNER_NAME}: could not set adapter discoverable={discoverable}: {err}");
        }
    }
}
