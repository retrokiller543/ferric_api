#![feature(fn_traits)]
#![feature(associated_type_defaults)]
#![allow(async_fn_in_trait)]

use ferric_api::setup::setup;
use ferric_api::statics::PORT;
use ferric_api::{ServerResult, server};

#[actix::main]
async fn main() -> ServerResult<()> {
    let _guard = setup().await?;

    server!()
        .bind(format!("0.0.0.0:{}", *PORT))?
        .bind(format!("[::1]:{}", *PORT))?
        .run()
        .await?;

    Ok(())
}
