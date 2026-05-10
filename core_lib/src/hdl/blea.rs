use std::sync::Arc;

use bluer::adv::{Advertisement, AdvertisementHandle, Type};
use bluer::UuidExt;
use bytes::Bytes;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

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
        Advertisement {
            advertisement_type,
            service_data: [(service_uuid, adv_data.into())].into(),
            ..Default::default()
        }
    }
}
