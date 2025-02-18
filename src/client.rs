use std::error::Error;

/// HTTP Client used to call the OANDA web services.
#[derive(Debug)]
pub struct Client {
    /// The reqwest object to use. Note that this is a synchronous client
    /// that cannot be used in async code.
    pub reqwest: reqwest::blocking::Client,
    /// OANDA host to use (do not include https://)
    pub host: String,
    /// OANDA API key
    pub authentication: String,
}

macro_rules! client_requests {
  ( $( $func:ident ( $request:ident ) -> $response:ident ),* ) => {
      $(
         use crate::$request;
         use crate::$response;
         impl Client {
           pub fn $func( &self, x: $request ) -> Result<$response, Box<dyn Error>> {
             x.remote(&self)
           }
         }
       )*
    };
}

client_requests!( candles(GetInstrumentCandlesRequest) -> GetInstrumentCandlesResponse,
                  orderbook(GetOrderBookRequest) -> GetOrderBookResponse,
                  positionbook(GetPositionBookRequest) -> GetPositionBookResponse,
                  pricing(GetPricesRequest) -> GetPricesResponse,
                  accounts(ListAccountsRequest) -> ListAccountsResponse,
                  account_summary(GetAccountSummaryRequest) -> GetAccountSummaryResponse);
