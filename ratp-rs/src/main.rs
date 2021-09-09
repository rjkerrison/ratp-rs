#[macro_use]
extern crate log;

mod soap {
    include!(concat!(env!("OUT_DIR"), "/example.rs"));
}

#[tokio::main]
async fn main() -> Result<(), savon::Error> {
    pretty_env_logger::init();

    let base_url = "http://ratp.wsiv";
    info!("Hello, world!");

    let client = soap::Wsiv::new(base_url.to_string());

    let res = client
        .get_stations(soap::messages::GetStationsRequest(
            soap::types::getStations {},
        ))
        .await?;

    info!("res: {:?}", res);

    Ok(())
}
