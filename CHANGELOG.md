# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - ReleaseDate

<!-- [START AUTO UPDATE] -->
<!-- Please keep comment here to allow auto-update -->
## [-e84b3f9] - 2024-03-06

Ns 160/endpoint for total http requests (#22)

* implemented endpoint for total http requests chart
<!-- [END AUTO UPDATE] -->
## [-738eb4c] - 2024-03-06

Ns 116/add token verification (#21)

* added fusion auth token verification and added config file for net-gatway

* updated apis, removed group id and use jwt token instead, deleted client data

* renamed client_data to jwt_token in dashboard_manager
## [-c3d21b9] - 2024-02-29

Ns 116/add token verification: added actual token verification and added config file to `net-gateway` (#20)

* added fusion auth token verification and added config file for net-gateway
## [-553ac9d] - 2024-02-28

NS-114/testing-and-fixing-errors: made some changes due to changes in frontend part (#19)

* made some changes due to changes in frontend part
## [-315d45a] - 2024-02-26

NS-82/network-bandwidth-per-protocol (#17)

* Add REST endpoint for network bandwidth per protocol

* fixed network bandwidth per protocol after adding service manager
## [-159f2aa] - 2024-02-26

Ns 95/provide endpoint for network overview filters: added and tested endpoint for network overview filters (#18)

* renamed chart manager to service request manager due to adding request for filters which is not a chart

* added overview filters manager into dashboard manager in overview endpoint

* renamed chart/overview endpoint to chart/network_overview
## [-3200137] - 2024-02-24

Ns 113/add filters to endpoints (#16)

* added filters for all the chart and dashboard endpoints
## [-0d9d0f6] - 2024-02-22

NS-71/dashboard-manager (#15)

* Implemented dashboard manager
## [-0b6a40f] - 2024-02-21

NS-100/chart-managers (#14)

* NS-100/chart-managers: Add ChartRequestManager to query requests easily

* NS-100/chart-managers: Add chart_management folder
## [-7014dda] - 2024-02-15

NS-72/endpoint-managers (#13)

* NS-71/endpoint-managers: Add QuinnClientEndpointManager

* NS-72/endpoint-managers: Rename bandwidth_per_endpoint -> network_bandwidth_per_endpoint
## [-1b23c46] - 2024-02-07

NS-99/core-module (#12)

* NS-99/core-module: Add core module. Move there all the core-related stuff
## [-70c3008] - 2024-02-07

NS-98/update-libs (#11)

* Moved from local registry to crates.io published libs
## [-857e491] - 2024-02-06

Feature/NS-18/async-gateway (#10)

* Rewrite all the charts, so they will be queried in async way

* Rewrite dashboard endpoint, so it will query all the charts in async way
## [-8815f67] - 2024-01-02

feature/CU-8693e7869: updated allowed origin in Cors configuration (#9)

* updated allowed origin in Cors configuration
## [-67ceea8] - 2024-01-02

feature/CU-8693dfdxv: added authorization module (#8)

* added authorization module and tests, modified endpoints, modified cors config
## [-f179722] - 2023-12-28

feature/CU-8693cd1hk: updating endpoints with netstalker.io/v1 api  (#5)

* added actix-cors crate and update struct field names

* provided json strucures for endpoints to make http response return json

* add Default implementation to OverviewDashboard
struct
## [-cc655d4] - 2023-12-28

Removed quinn connector and quinn core due to its move to net-transport (#7)
## [-5ac754a] - 2023-12-20

* add quic-core module and connector module

* implemented base QuicConnector, updated dependencies

* implemented tests
## [-d9f7243] - 2023-12-20

feature/CU-8693cd1hk: cleaned repo (#6)
## [-8b6f49e] - 2023-12-15

feature/CU-8693bdt94: reorginized prohject into workspace, added quinn_test crate for quinn testing (#3)
## [-0763c42] - 2023-12-08

Feature/cu 86939ue0d: initial rust project structure (#1)

* added initial rust project structure

* added README.md file and github workflows for changelog.md maintaining, building and testing the project
