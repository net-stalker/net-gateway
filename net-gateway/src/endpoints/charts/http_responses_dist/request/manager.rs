use std::sync::Arc;

use net_core_api::api::API;
use net_core_api::decoder_api::Decoder;
use net_core_api::envelope::envelope::Envelope;
use net_core_api::typed_api::Typed;
use net_reporter_api::api::http_responses_dist::http_responses_dist::HttpResponsesDistDTO;
use net_reporter_api::api::http_responses_dist::http_responses_dist_request::HttpResponsesDistRequestDTO;

use crate::core::service_request_management::service_request_manager::ServiceRequestManager;
use crate::core::service_request_management::service_response::ServiceResponse;
use crate::core::filter::Filters;
use crate::core::general_filters::GeneralFilters;
use crate::endpoints::charts::http_responses_dist::response::http_responses_dist::HttpResponsesDistResponse;

#[derive(Default)]
pub struct HttpResponsesDistChartManager {}

impl HttpResponsesDistChartManager {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[async_trait::async_trait]
impl ServiceRequestManager for HttpResponsesDistChartManager {
    fn get_requesting_type(&self) -> &'static str {
        HttpResponsesDistDTO::get_data_type()
    }

    fn get_request_type(&self) -> &'static str {
        HttpResponsesDistRequestDTO::get_data_type()
    }

    fn form_dto_request(
        &self,
        params: Arc<GeneralFilters>,
        filters: Option<Arc<Filters>>,
    ) -> Box<dyn API> {
        let filters = filters.as_ref().unwrap().as_ref().clone().into();
        Box::new(HttpResponsesDistRequestDTO::new(
            params.start_date,
            params.end_date,
            filters,
        ))
    }
    
    fn decode_received_envelope(
        &self,
        received_envelope: Envelope
    ) -> Result<Box<dyn ServiceResponse>, std::string::String> {
        Ok(Box::new(HttpResponsesDistResponse::from(
            HttpResponsesDistDTO::decode(received_envelope.get_data())
        )))
    }
}