# Zenoh Publish-Subscribe

A real [zenoh](https://zenoh.io) publisher/subscriber example (zenoh 1.x, peer
mode — no broker required).

- **publisher** — `examples/dds_zenoh/publisher.rs`, run as `zenoh_pub`:
  publishes one `TransmissionData` sample per second on
  `demo/easy_example/transmission`, serialized as JSON.
- **subscriber** — `examples/dds_zenoh/subscriber.rs`, run as `zenoh_sub`:
  subscribes to `demo/easy_example/**` and prints every sample it receives.

`TransmissionData` is the same type used by the iceoryx2 examples
(`examples/dds_iceoryx2/`); here it is transported as JSON.

## Running The Example

Open two terminals.

### Terminal 1 — subscriber

```sh
cargo run --example zenoh_sub
```

### Terminal 2 — publisher

```sh
cargo run --example zenoh_pub
```

Both sessions use `zenoh::Config::default()`. Peers on the same network discover
each other automatically through multicast scouting, so nothing else is needed.

If multicast scouting is unavailable (containers, restricted networks), make the
sessions connect explicitly over TCP instead. For example, in both programs:

```rust
let mut config = zenoh::Config::default();
config.connect.endpoints.set(
    ["tcp/127.0.0.1:7447"]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect(),
);
let session = zenoh::open(config).await?;
```

Then use a router (`zenohd`) or point the publisher at a listening subscriber's
endpoint.
