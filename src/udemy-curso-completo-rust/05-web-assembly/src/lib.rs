use serde::Deserialize;
use serde_json::{Map, Value};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub async fn get_coin_price(coin: String) -> Result<String, JsValue> {
    let mut options: RequestInit = RequestInit::new();
    options.method("GET");
    options.mode(RequestMode::Cors);
    let url: String = format!("https://api.coingecko.com/api/v3/coins/{coin}?localization=false&tickers=false&market_data=true&community_data=false&developer_data=false");

    let window: web_sys::Window = web_sys::window().unwrap();
    let request: Request = Request::new_with_str_and_init(&url, &options)?;
    let response_value: JsValue = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = response_value.dyn_into().unwrap();
    let text: String = JsFuture::from(response.text()?).await?.as_string().unwrap();

    let price: String = parse_body(&text);
    Ok(price)
}

fn parse_body(body: &str) -> String {
    let coin_data: CoinData = serde_json::from_str(body).unwrap();

    let usd: String = coin_data.market_data.current_price["usd"].to_string();
    let mxn: String = coin_data.market_data.current_price["mxn"].to_string();

    log("CoinData:");
    log(format!("id: {}", coin_data.id).as_str());
    log(format!("symbol: {}", coin_data.symbol).as_str());
    log(format!("name: {}", coin_data.name).as_str());
    for image in coin_data.image {
        log(format!("image {}: {}", image.0, image.1).as_str());
    }

    format!("${usd} USD | ${mxn} MXN")
}

#[derive(Deserialize)]
struct CoinData {
    id: String,
    symbol: String,
    name: String,
    image: Map<String, Value>,
    market_data: MarketData,
}

#[derive(Deserialize)]
struct MarketData {
    current_price: Map<String, Value>,
}
