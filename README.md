# easy-example

rust base frame ,with log , config ,network,zeromq ,mqtt,web,rpc ,and other more

## crate

|name|replace|fun|note|
|-|-|-|-|
|tokio||async task ,tcp ,udp|async frame|
|config.rs|yaml,toml,json,single file|read config files ,and put into struct data|simplize config function|
|log4rs||env_logger||
|tmq||zeromq with tokio||
|once_cell|lazy_static|global reference from config file||


## examples

|name|fun|note|
|-|-|-|
|tcp-client|tcp client||
|tcp-server|tcp server||
|post|http client post|with dynamic json|
|zeromq-tmq|get udp,http data to zeromq|  |

