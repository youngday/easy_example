# easy_example

* rust base frame:log , config ,network,zeromq ,mqtt,web,rpc ,base64 
* dds: zenoh, iceoryx2

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
|plot|plotly|plot data all you want|plot to web,easy than plotters|
|zenoh|dds|pubsub dds ipc for ros |new realtime(10us) ipc |
|iceoryx2|dds|pubsub dds ipc for ros |new realtime(10us) ipc |
|tonic| |grpc   |⚠️ proto genarate .rs files   |
|axum websocket| |websocket   | axum example ,tokio-tungstenite |
## examples

|name|fun|note|
|-|-|-|
|tcp-client,tcp-server|tcp client server||
|post|http client post|with dynamic json|
|zeromq-tmq|get udp,http data to zeromq|  |
|zmq_pub,zmq_sub|tmq ,zeromq lib, publish,subscriber|  |
|udp-client,udp-server|udp client,server||
|channel-mpsc|multi productor,single consummer queue|mpsc,for mpmc ,see flume,async-channel|
|mqtt-asyncpubsub|mqtt client|mqtt with tokio , run mqtt broker ,before run client |
|serial-print|async serial port||
|base64|base64|encode decode|
|plot|plot data|plot to web |
|zenoh_pub,zenoh_sub|pub sub|pub sub|
|ice_pub,ice_sub|pub sub|pub sub|
|discovery|iceoryx2 discovery| |
|grpc_client,grpc_server| |⚠️ proto genarate .rs files   |
|ws_client,ws_server| | websocket   |
## vscode build

https://code.visualstudio.com/docs/languages/rust



## dds

### target

zeromq is a good dds rpc, but rust zmq is maintained slowly.
we could have more fast dds selection,and it can bind to ros.

### easy-zenoh

pub and sub ,test ok

#### zenoh-cpp 

c++ app,test ok ,can link with rust zenoh,check:


### iceoryx2

pub and sub  ,test ok ,

but not c++ and python app,now are planning,waiting for  release

⚠️