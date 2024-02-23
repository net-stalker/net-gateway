// here we need to impl MapFilterToDTO for our DTO type which we still need to update
use net_reporter_api::api::network_overview_dashboard_filters::network_overview_dashbord_filters::NetworkOverviewDashboardFiltersDTO;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::filters::{Filters, MapFiltersToDTO};

use super::filter_entry::NetworkOverviewFilter;

impl MapFiltersToDTO for NetworkOverviewDashboardFiltersDTO {
    fn map_filters(filters: Filters) -> Self {
        todo!("implement map_filters for NetworkOverviewDashboardFiltersDTO");
    }
}

#[derive(Serialize, Deserialize, Validate, Default, Debug, Clone)]
pub struct NetworkOverviewFilters {
    pub entries: Vec<NetworkOverviewFilter>,
}

impl From<NetworkOverviewDashboardFiltersDTO> for NetworkOverviewFilters {
    fn from(dto: NetworkOverviewDashboardFiltersDTO) -> Self {
        NetworkOverviewFilters {
            entries: dto
                .get_entries()
                .iter()
                .map(|entry| NetworkOverviewFilter::from(entry.clone()))
                .collect(),
        }
    }
}


#[cfg(test)]
mod tests {
    // use super::*;
    
    #[test]
    fn test_map_filters() {

        // let filters = Filters::new("test".to_string());
        
        // let result = filters.map_filters();
        
        // Add assertions to verify the correctness of the mapping
        
        // Example assertion:
        // assert_eq!(result.some_field, expected_value);
        assert!(false, "Test not implemented");
    }
}
