# easy_example

* rust base frame:log , config ,network,zeromq ,mqtt,web,rpc ,base64 
* dds:  iceoryx2

## crate

|name|replace|fun|note|
|-|-|-|-|
|tokio||async task ,tcp ,udp,channel,|async frame,tokio::spawn|
|config|yaml,toml,json,single file|read config files ,and put into struct data|simplize config function|
|tracing|log4rs,env_logger|async log with file||
|tmq||zeromq with tokio||
|once_cell|lazy_static|global reference from config file||
|rumqttc|paho-mqtt|mqtt of rust with tokio||
|tokio-serial|serial|async serial port||
|base64||base64|encode decode|
|plot|plotly|plot data all you want|plot to web,easy than plotters,(https://github.com/youngday/easy_wasm_plotly)|
|egui-plotter|plotters|native plotting in egui/eframe|vendored at vendor/egui-plotter (0.7.0), patched for egui 0.36|
|liveplot|egui-plotter|realtime plotting UI in egui/eframe|from crates.io (https://github.com/ulikoehler/liveplot-rs), multi-trace, thresholds, CSV/Parquet export|
|iceoryx2|dds|pubsub dds ipc for ros |new realtime(10us) ipc |
|zenoh|dds|pubsub dds over zenoh|real zenoh pub/sub, brokerless peer mode|
|poem_grpc| |tonic grpc   |put ./proto build.rs files same as cargo.toml path |
|axum websocket| |websocket   | axum example ,tokio-tungstenite |
|quinn quic|quiche    | webtransport | |
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
|zenoh_pub,zenoh_sub|zenoh pub sub|real zenoh publisher-subscriber|
|discovery|iceoryx2 discovery| |
|grpc-client,grpc-server,grpc-jsoncodec-server|poem grpc examples ,with json codec |⚠️ grpc branch   |
|ws_client,ws_server| | websocket   |
|wt_server,wt_client|webtransport|replace websocket with http3/quic|
|egui_3d,egui_timechart|native egui plotting|vendored egui-plotter, egui/eframe 0.36|
|egui_timechart_live|live egui XY chart|zenoh subscriber -> XyTimeData, y vs x in real time|
|liveplot_zenoh|live plot with the liveplot crate|zenoh subscriber -> liveplot PlotSink, real-time pub/sub|
## vscode build

https://code.visualstudio.com/docs/languages/rust



## dds

### target

zeromq is a good dds rpc, but rust zmq is maintained slowly.
we could have more fast dds selection,and it can bind to ros.

### iceoryx2

pub and sub  ,test ok 

## grpc 

for building proto  not work on github workflow,

⚠️  // please uncomment for grpc 

## mqtt

mqtt update to v5 protocol

```sh
cargo run --example rumqttd
```
pub and sub
```sh
cargo run --example mqtt_asyncpubsub
```

```sh
cargo run --release --example rumqttd -- -c rumqttd.toml -vvv 
```
or ./rumqtt.sh
## webtransport
please check examples/webtransport/src/README.md
and 

-   Generate a certificate: 
```sh
./cert/generate
```
-   Run the Rust server: 
```sh
cargo run --example wt_server -- --tls-cert cert/localhost.crt --tls-key cert/localhost.key
```
-   Run the Rust client: 
```sh
cargo run --example wt_client -- --tls-cert cert/localhost.crt
```
-   Run a Web client: 
```sh
cd web; npm install; npx parcel serve client.html --open
```

## examples

    base64
    discovery
    egui_3d
    egui_timechart
    egui_timechart_live
    ice_pub
    ice_sub
    liveplot_zenoh
    load_csv
    mpsc_tokio
    mqtt_asyncpubsub
    post
    rumqttd
    serial_print
    tcp_client
    tcp_server
    udp_client
    udp_echo
    ws_client
    ws_server
    wt_client
    wt_server
    zenoh_pub
    zenoh_sub
    zeromq_tmq
    zmq_pub
    zmq_sub
