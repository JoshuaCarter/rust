use anyhow::Result;
use infra::model::venue::Venue;
use infra::model::common::Exchange;
use super::Binance;

// Q: Will we need to persist and share venue instances between calls?

// static BINANCE: Mutex<Binance> = Mutex::new(Binance::new());

// pub async fn make_venue(exchange: Exchange) -> Option<&'static Mutex<Binance>> {
//     match exchange {
//         Exchange::Binance => {
//             return Some(&BINANCE);
//         },
//         _ => { return None; }
//     }
// }

// type VenueContainer = Mutex<Box<dyn Venue>>;

// static mut VENUES: HashMap<Exchange, VenueContainer> = HashMap::new();

// pub async fn make_venue(exchange: Exchange) -> Option<&'static VenueContainer> {
//     if !VENUES.contains_key(&exchange) {
//         match exchange {
//             Exchange::Binance => {
//                 VENUES.insert(exchange, Mutex::new(Box::new(Binance::new())));
//             },
//             _ => { return None; }
//         }
//     }

//     return VENUES.get(&exchange);
// }

pub fn create_venue(exchange: Exchange) -> Result<impl Venue> {
    match exchange {
        Exchange::Binance => { return Ok(Binance::new()); },
        _ => { anyhow::bail!("Unsupported exchange") }
    }
}
