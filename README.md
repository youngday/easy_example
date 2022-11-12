# easy-example

rust base frame ,with log , config ,network,zeromq ,mqtt,web,rpc ,and other more

## crate

|name|replace|fun|note|
|-|-|-|-|
|tokio||async task ,tcp ,udp,channel,|async frame,tokio::spawn|
|config.rs|yaml,toml,json,single file|read config files ,and put into struct data|simplize config function|
|log4rs|env_logger|log with file||
|tmq||zeromq with tokio||
|once_cell|lazy_static|global reference from config file||


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

