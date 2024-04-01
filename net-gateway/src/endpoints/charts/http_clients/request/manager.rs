use std::error::Error;
use std::sync::Arc;

use net_core_api::api::envelope::envelope::Envelope;
use net_core_api::core::api::API;
use net_core_api::core::decoder_api::Decoder;
use net_core_api::core::typed_api::Typed;

use net_reporter_api::api::http_clients::http_clients::HttpClientsDTO;
use net_reporter_api::api::http_clients::http_clients_request::HttpClientsRequestDTO;

use crate::core::service_request_management::service_request_manager::ServiceRequestManager;
use crate::core::service_request_management::service_response::ServiceResponse;
use crate::core::filter::Filters;
use crate::core::general_filters::GeneralFilters;
use crate::endpoints::charts::http_clients::response::http_clients::HttpClientsResponse;

#[derive(Default)]
pub struct HttpClientsChartManager {}

impl HttpClientsChartManager {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait::async_trait]
impl ServiceRequestManager for HttpClientsChartManager {
    fn get_requesting_type(&self) -> &'static str {
        HttpClientsDTO::get_data_type()
    }

    fn get_request_type(&self) -> &'static str {
        HttpClientsRequestDTO::get_data_type()
    }

    fn form_dto_request(
        &self,
        params: Arc<GeneralFilters>,
        filters: Option<Arc<Filters>>,
    ) -> Box<dyn API> {
        let filters = filters.as_ref().unwrap().as_ref().clone().into();
        Box::new(HttpClientsRequestDTO::new(
            params.start_date,
            params.end_date,
            filters,
        ))
    }
    
    fn dispatch_received_envelope(
        &self,
        received_envelope: Envelope
    ) -> Result<Box<dyn ServiceResponse>, Box<dyn Error + Send + Sync>> {
        Ok(Box::new(HttpClientsResponse::from(
            HttpClientsDTO::decode(received_envelope.get_data())
        )))
    }
}