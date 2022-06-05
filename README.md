# budgie

[![pipeline](https://github.com/ectobit/budgie/actions/workflows/pipeline.yml/badge.svg)](https://github.com/ectobit/budgie/actions/workflows/pipeline.yml)
[![License](https://img.shields.io/badge/license-BSD--2--Clause--Patent-orange.svg)](https://github.com/ectobit/budgie/blob/main/LICENSE)

Micro service in Rust to send e-mails using SMTP relay.

## Test service using curl

```
curl -X POST localhost:3000/send -H 'Content-Type: application/json' -H 'Accept: application/json' -d \
    '{ "from": "john.doe@sixpack.com", "to": "john.doe@sixpack.com", "subject": "Hello", "plain": "Some message" }'
```

## Custom message-id

```rust
use gethostname::gethostname;
use rand::Rng;
use std::process;
use std::time::SystemTime;

fn message_id(&self) -> anyhow::Result<String> {
    let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;

    let mut rng = rand::thread_rng();
    let r: u32 = rng.gen();

    Ok(format!(
        "<{}.{}.{}@{}>",
        t.as_nanos(),
        process::id(),
        r,
        gethostname().into_string().unwrap()
    ))
}
```
