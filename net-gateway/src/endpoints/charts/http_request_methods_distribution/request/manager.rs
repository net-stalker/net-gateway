use std::sync::Arc;

use net_core_api::api::API;
use net_core_api::decoder_api::Decoder;
use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;

use net_reporter_api::api::http_request_methods_distribution::http_request_methods_distribution::HttpRequestMethodsDistributionDTO;
use net_reporter_api::api::http_request_methods_distribution::http_request_methods_distribution_request::HttpRequestMethodsDistributionRequestDTO;

use crate::core::service_request_management::service_request_manager::ServiceRequestManager;
use crate::core::service_request_management::service_response::ServiceResponse;
use crate::core::filter::Filters;
use crate::core::general_filters::GeneralFilters;

use crate::endpoints::charts::http_request_methods_distribution::response::http_request_methods_distribution::HttpRequestMethodsDistributionResponse;

#[derive(Default)]
pub struct HttpRequestMethodsDistChartManager {}

impl HttpRequestMethodsDistChartManager {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait::async_trait]
impl ServiceRequestManager for HttpRequestMethodsDistChartManager {
    fn get_requesting_type(&self) -> &'static str {
        HttpRequestMethodsDistributionDTO::get_data_type()
    }

    fn get_request_type(&self) -> &'static str {
        HttpRequestMethodsDistributionRequestDTO::get_data_type()
    }

    fn form_dto_request(
        &self,
        params: Arc<GeneralFilters>,
        filters: Option<Arc<Filters>>,
    ) -> Box<dyn API> {
        let filters = filters.as_ref().unwrap().as_ref().clone().into();
        Box::new(HttpRequestMethodsDistributionRequestDTO::new(
            params.start_date,
            params.end_date,
            filters,
        ))
    }
    
    fn decode_received_envelope(
        &self,
        received_envelope: Envelope
    ) -> Result<Box<dyn ServiceResponse>, std::string::String> {
        Ok(Box::new(HttpRequestMethodsDistributionResponse::from(
            HttpRequestMethodsDistributionDTO::decode(received_envelope.get_data())
        )))
    }
}