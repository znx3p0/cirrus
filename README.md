# Cirrus

This library allows for creating cloud servers through an easy-to-use api.

A simple example on how to this library, using [this vm api](https://github.com/znx3p0/vm_api)
```rust
use cirrus::{CreatorFn, ServerFn, providers::fake::Creator, providers::fake::CreatorMetadata};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let creator = Creator::new(CreatorMetadata).await;
    let server = creator.create().await?;
    server.delete().await?;

    Ok(())
}
```

This library is intended to be simple, with three fundamental structures and two traits.
These structures are:
- Server
- Creator
- CreatorMetadata

The Server structure stores information from the server, and has a `delete` method which deletes the server. Additional methods may be added to specific providers.

The Creator structure stores the necessary information to create a server from the provider's api. This may be an access token, api token, etc.
The Creator structure has a `create` method which creates a server from the provider; additional methods may be added to specific providers.

The CreatorMetadata structure is used to configure servers when creating a server.


### Contributing
To contribute, take a look at the fake and boilerplate provider.
Feel free to add new providers, and add new methods to existing providers.

If you experience any bug, create an issue at the github repo.
