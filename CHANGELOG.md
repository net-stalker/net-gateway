# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - ReleaseDate

<!-- [START AUTO UPDATE] -->
<!-- Please keep comment here to allow auto-update -->
## [-3200137] - 2024-02-24

Ns 113/add filters to endpoints (#16)

* added filters for all the chart and dashboard endpoints
<!-- [END AUTO UPDATE] -->
## [-0d9d0f6] - 2024-02-22

NS-71/dashboard-manager (#15)

* Add quinn_endpoint_manager

* Add request_former trait

* Reorganize network_bandwidth_per_endpoint

* Rename chart -> response

* Add ChartRequestManager to query requests easily

* Add ChartRequestManager for the network_bandwidth endpoint

* Add ChartRequestManager for the network_graph endpoint

* Change whole trait logic, so request methods will no longer be static and we can create trait objects from ChartRequestManager

* Change query parameters to be Arc<>

* Rewrite ChartManager and DashboardManager. Get rid of multiple json converting

* Rename requested_dashboard -> request_dashboard method

* Change net-api version

---------

Co-authored-by: net-stalker-bot <githubbot@netstalker.io>
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
