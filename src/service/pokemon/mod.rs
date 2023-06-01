

pub async fn get_pokemon_count() -> u32 {
        let client = get_client();

        match rustemon::pokemon::pokemon::get_all_entries(&client).await {
                Ok(values) => values.len() as u32,
                Err(_) => 0,
        }
}


fn get_client() -> rustemon::client::RustemonClient {
        rustemon::client::RustemonClient::default()
}