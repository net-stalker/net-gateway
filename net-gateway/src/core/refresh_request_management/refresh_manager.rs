use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::api::primitives::integer::Integer;
use net_core_api::api::result::result::ResultDTO;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::encoder_api::Encoder;
use net_core_api::core::typed_api::Typed;
use net_updater_api::api::refreshers::refresh_pcap_parsed_data::refresh_pcap_parsed_data_request::RefreshPcapParsedDataRequestDTO;
use net_updater_api::api::refreshers::refresh_views::refresh_views_request::RefreshViewsRequestDTO;

use crate::config::Config;
use crate::core::quinn_client_endpoint_manager::QuinnClientEndpointManager;

pub struct RefreshManager<'a> {
    config: &'a Config,
    tenant_id: &'a str,
}

impl<'a> RefreshManager<'a> {
    pub fn new(
        config: &'a Config,
        tenant_id: &'a str,
    ) -> Self {
        Self { 
            config,
            tenant_id,
        }
    }

    pub async fn refresh(self, updated_rows_count: &Integer) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if updated_rows_count.get_value() == 0 {
            log::debug!("No need to refresh");
            return Ok(());
        }
        log::debug!("refreshing");
        match tokio::join!(
            self.refresh_materialized_views(),
            self.refresh_network_packets(),
        ) {
            (Ok(_), Ok(_)) => {
                log::debug!("refreshed both");
                Ok(())
            },
            (Ok(_), Err(err)) => {
                log::debug!("coudn't refresh network packets: {}", err);
                Err(err)
            },
            (Err(err), Ok(_)) => {
                log::debug!("coudn't refresh materialized views: {}", err);
                Err(err)
            },
            (Err(err1), Err(err2)) => {
                let error_message = format!("cound't refresh both: {err1}. {err2}");
                log::debug!("{error_message}");
                Err(error_message.into())
            },
        }
    }

    async fn refresh_materialized_views(
        &self
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let refresh_materialzied_views_request = RefreshViewsRequestDTO::default();
        let request = Envelope::new(
            self.tenant_id,
            refresh_materialzied_views_request.get_type(),
            &refresh_materialzied_views_request.encode()
        ).encode();

        self.send_request(&request).await
    }

    async fn refresh_network_packets(
        &self
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let refresh_materialzied_views_request = RefreshPcapParsedDataRequestDTO::default();
        let request = Envelope::new(
            self.tenant_id,
            refresh_materialzied_views_request.get_type(),
            &refresh_materialzied_views_request.encode(),
        ).encode();

        self.send_request(&request).await
    }

    async fn send_request(&self, request: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut server_connection = QuinnClientEndpointManager::start_server_connection(
            &self.config.quin_client_address.addr,
            &self.config.quin_inserter.addr,
            &self.config.quin_server_application.app,
        ).await?;
    
        server_connection.send_all_reliable(request).await?;
    
        let response = server_connection.receive_reliable().await?;
        let result = ResultDTO::decode(Envelope::decode(&response).get_data());
        if result.is_ok() {
            return Ok(());
        } 
        Err(result.get_description().unwrap_or("no description").to_string().into())
    }
}
