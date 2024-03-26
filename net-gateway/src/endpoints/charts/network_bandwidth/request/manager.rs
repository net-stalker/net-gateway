use std::error::Error;
use std::sync::Arc;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::api::API;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

use net_reporter_api::api::network_bandwidth::network_bandwidth::NetworkBandwidthDTO;
use net_reporter_api::api::network_bandwidth::network_bandwidth_request::NetworkBandwidthRequestDTO;

use crate::core::service_request_management::service_request_manager::ServiceRequestManager;
use crate::core::service_request_management::service_response::ServiceResponse;
use crate::core::filter::Filters;
use crate::core::general_filters::GeneralFilters;

use crate::endpoints::charts::network_bandwidth::response::network_bandwidth::NetworkBandwidthResponse;

#[derive(Default)]
pub struct NetworkBandwidthChartManager {}

impl NetworkBandwidthChartManager {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait::async_trait]
impl ServiceRequestManager for NetworkBandwidthChartManager {
    fn get_requesting_type(&self) -> &'static str {
        NetworkBandwidthDTO::get_data_type()
    }

    fn get_request_type(&self) -> &'static str {
        NetworkBandwidthRequestDTO::get_data_type()
    }

    fn form_dto_request(
        &self,
        params: Arc<GeneralFilters>,
        filters: Option<Arc<Filters>>,
    ) -> Box<dyn API> {
        let filters = filters.as_ref().unwrap().as_ref().clone().into();
        Box::new(NetworkBandwidthRequestDTO::new(
            params.start_date,
            params.end_date,
            filters,
        ))
    }
    
    fn dispatch_received_envelope(
        &self,
        received_envelope: Envelope
    ) -> Result<Box<dyn ServiceResponse>, Box<dyn Error + Send + Sync>> {
        Ok(Box::new(NetworkBandwidthResponse::from(
            NetworkBandwidthDTO::decode(received_envelope.get_data())
        )))
    }
}