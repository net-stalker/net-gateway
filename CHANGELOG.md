# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - ReleaseDate

<!-- [START AUTO UPDATE] -->
<!-- Please keep comment here to allow auto-update -->
## [-8815f67] - 2024-01-02

feature/CU-8693e7869: updated allowed origin in Cors configuration (#9)

* updated allowed origin in Cors configuration
<!-- [END AUTO UPDATE] -->
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
