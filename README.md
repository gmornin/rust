# GM Rust bindings

This crate provides Rust binding structs for the GM Services API.

## Overview

The binding structs is the standardised *API glue* which holds the project together, it allows Rust programs to easily communicate with the server.

Although [API docs](https://siriusmart.github.io/gm-services) is available, specifying the fields and datatypes of valid JSON request and responses, it may be outdated or incorrect. This repository provides an always up-to-date, living docs for the GM API, and can be useful as a reference for using the API without bindings libraries, or creating your own binding libraries.

### Structure

```
├── README.md (you are here)
└── src
    ├── lib.rs
    ├── services
    │   ├── mod.rs
    │   └── v1
    │       ├── error.rs (enum of all possible error responses)
    │       ├── requests.rs (one struct for each type of request)
    │       └── response.rs (enum of all possible success responses)
    ├── structs
    │   └── profile.rs (how user profiles are actually stored on disk)
    └── traits (various traits)
```

### Importance

A quick introduction to how this crate is used in this project.

- GM services provides account management, and API endpoints for specialised servers based on it.
- GMTex is based on GM services, and is a website frontend for using GM services as a LaTex publishing site.
- GM CLI allows the user to interact with GM services and provide various tasks.

### Development

- **DO NOT** rename or remove fields for no apparent reason
- **Avoid** making changes (such as adding fields) to an already stable version (v1/v2...)

Changes are usually made this crate because GM services required so, mainly for adding features and API endpoints - which requires new binding structs.
