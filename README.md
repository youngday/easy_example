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
## examples

|name|fun|note|
|-|-|-|
|tcp-client|tcp client||
|tcp-server|tcp server||
|post|http client post|with dynamic json|
|zeromq-tmq|get udp,http data to zeromq|  |
|zmq_pub|tmq ,zeromq lib, publish|  |
|zmq_sub|tmq, zeromq lib, subscriber|  |
|udp-client|udp client||
|udp-server|udp echo server||
|channel-mpsc|multi productor,single consummer queue|mpsc,for mpmc ,see flume,async-channel|
|mqtt-asyncpubsub|mqtt client|mqtt with tokio , run mqtt broker ,before run client |
|serial-print|async serial port||
|base64|base64|encode decode|
|plot|plot data|plot to web |
|zenoh_pub|pub|pub|
|zenoh_sub|sub|sub|
|ice_pub|pub|pub|
|ice_sub|sub|sub|
|discovery|iceoryx2 discovery| |

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