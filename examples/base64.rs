// use log::{debug, error, info, trace, warn};
use log::info;
use log4rs;

use base64::{Engine as _, engine::general_purpose};
fn main() {
  log4rs::init_file("examples/config/log.yaml", Default::default()).unwrap();

    info!("Base64 testing.");
  //  trace!("some trace log");
 //   debug!("some debug log");
  //  info!("some information log");
 //   warn!("some warning log");
  //  error!("some error log");


    let a = b"hello world";
    let b = "aGVsbG8gd29ybGQ=";

    info!("Byte {:?}",a);
    info!("Base64 {:?}",b);

    info!("encode Base64 {:?}",general_purpose::STANDARD.encode(a));
    info!("decode Base64 {:?}",&general_purpose::STANDARD.decode(b).unwrap()[..]);

    assert_eq!(general_purpose::STANDARD.encode(a), b);
  
    assert_eq!(a, &general_purpose::STANDARD.decode(b).unwrap()[..]);

}

