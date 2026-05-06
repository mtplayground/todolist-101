#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {}

#[cfg(not(feature = "ssr"))]
fn main() {}
