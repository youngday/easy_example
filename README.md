# easy_example

* rust base frame:log , config ,network,zeromq ,mqtt,web,rpc ,base64 
* dds:  iceoryx2

## crate

|name|replace|fun|note|
|-|-|-|-|
|tokio||async task ,tcp ,udp,channel,|async frame,tokio::spawn|
|config.rs|yaml,toml,json,single file|read config files ,and put into struct data|simplize config function|
|log4rs|env_logger|log with file||
|tmq||zeromq with tokio||
|once_cell|lazy_static|global reference from config file||
|rumqttc|paho-mqtt|mqtt of rust with tokio||
|tokio-serial|serial.rs|async serial port||
|base64||base64|encode decode|
|plot|plotly|plot data all you want|plot to web,easy than plotters,(https://github.com/youngday/easy_wasm_plotly)|
|iceoryx2|dds|pubsub dds ipc for ros |new realtime(10us) ipc |
|poem_grpc| |tonic grpc   |put ./proto build.rs files same as cargo.toml path |
|axum websocket| |websocket   | axum example ,tokio-tungstenite |
|quinn quic| |quic http3   | quic http3 client server |
## examples

|name|fun|note|
|-|-|-|
|tcp-client,tcp-server|tcp client server||
|post|http client post|with dynamic json|
|zeromq-tmq|get udp,http data to zeromq|  |
|zmq_pub,zmq_sub|tmq ,zeromq lib, publish,subscriber|  |
|udp-client,udp-server|udp client,server||
|channel-mpsc|multi productor,single consummer queue|mpsc,for mpmc ,see flume,async-channel|
|mqttd|mqtt broker|mqtt with tokio , run mqtt broker ,before run client , ```cargo run --release --example rumqttd -- -c rumqttd.toml -vvv   ```|
|mqtt-asyncpubsub|mqtt client|mqtt with tokio , run mqtt broker ,before run client |
|serial-print|async serial port||
|base64|base64|encode decode|
|plot|plot data|https://github.com/youngday/easy_wasm_plotly |
|ice_pub,ice_sub|pub sub|pub sub|
|discovery|iceoryx2 discovery| |
|grpc-client,grpc-server,grpc-jsoncodec-server|poem grpc examples ,with json codec |⚠️ grpc branch   |
|ws_client,ws_server| | websocket   |
|wt-server,wt-client|webtransport|replace websocket with http3/quic|
## vscode build

https://code.visualstudio.com/docs/languages/rust



## dds

### target

zeromq is a good dds rpc, but rust zmq is maintained slowly.
we could have more fast dds selection,and it can bind to ros.

### iceoryx2

pub and sub  ,test ok 
### quiche

```sh
cargo run --example quic_server ./

cargo run --example quic_client https://localhost:4433/Cargo.toml
```

## grpc 

for building proto  not work on github workflow,

⚠️  // please uncomment for grpc 

## mqtt

mqtt update to v5 protocol

run rumqttd broker first 

```sh
cargo run --release --example rumqttd -- -c rumqttd.toml -vvv 
```
or ./rumqtt.sh
## webtransport
please check examples/webtransport/src/README.md
and 

-   Generate a certificate: `./cert/generate`
-   Run the Rust server: `cargo run --example wt-server -- --tls-cert cert/localhost.crt --tls-key cert/localhost.key`
-   Run the Rust client: `cargo run --example wt-client -- --tls-cert cert/localhost.crt`
-   Run a Web client: `cd web; npm install; npx parcel serve client.html --open`

## examples

    base64
    discovery
    ice_pub
    ice_sub
    load_csv
    mpsc_std
    mpsc_tokio
    mqtt_asyncpubsub
    post
    rumqttd
    serial-print
    tcp-client
    tcp-server
    udp-client
    udp-echo
    ws-client
    ws-server
    zeromq_tmq
    zmq_pub
    zmq_sub



