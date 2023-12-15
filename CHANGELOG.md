# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - ReleaseDate

<!-- [START AUTO UPDATE] -->
<!-- Please keep comment here to allow auto-update -->
## [-5fbb8df] - 2023-12-15

Feature/cu 86939ue2u: added actix_web and implemented initial endpoints for charts and overview dashboard (#2)

* added actix web, added initial endpoints. Need to think more about communication between services

* added  tokio_tungstenite for communication between net-hub and net-gateway

* managed to send a request to net-hub

* added endpoints for charts and overview dashboard

* need to update code to use macros for boilerplate
<!-- [END AUTO UPDATE] -->
## [-0763c42] - 2023-12-08

Feature/cu 86939ue0d: initial rust project structure (#1)

* added initial rust project structure

* added README.md file and github workflows for changelog.md maintaining, building and testing the project
