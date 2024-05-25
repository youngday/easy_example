# rust code slice

## path

```rs
use std::env;
use std::path::PathBuf;

use serde::Deserialize;

    let mut p = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    println!("path:{:?}",p);
    p = p.join("examples/plotly/assets").join("finance_charts_apple.csv");
    let mut rdr = csv::Reader::from_path(p).unwrap();
    let mut out = Vec::new();
    for result in rdr.deserialize() {
        let d: FinData = result.unwrap();
        out.push(d);
    }
```