# easy_example

rust base frame ,with log , config ,network,zeromq ,mqtt,web,rpc ,base64 and other more

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

## examples

|name|fun|note|
|-|-|-|
|tcp-client|tcp client||
|tcp-server|tcp server||
|post|http client post|with dynamic json|
|zeromq-tmq|get udp,http data to zeromq|  |
|udp-client|udp client||
|udp-server|udp echo server||
|channel-mpsc|multi productor,single consummer queue|mpsc,for mpmc ,see flume,async-channel|
|mqtt-asyncpubsub|mqtt client|mqtt with tokio , run mqtt broker ,before run client |
|serial-print|async serial port||
|base64|base64|encode decode|

## vscode build

https://code.visualstudio.com/docs/languages/rust



