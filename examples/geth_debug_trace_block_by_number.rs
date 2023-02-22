use ethers::prelude::*;
use eyre::Result;
use std::str::FromStr;
use ethers_core::utils::serialize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TraceFilterDto {
    pub tracer: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TraceBlockRootResultDto {
    pub result: TraceDto,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TraceDto {
    #[serde(rename = "type")]
    pub result_type: Option<String>,
    pub from: Option<Address>,
    pub to: Option<Address>,
    pub value: Option<U256>,
    pub gas: Option<U256>,
    #[serde(rename = "gasUsed")]
    pub gas_used: Option<U256>,
    pub input: Option<Bytes>,
    pub output: Option<Bytes>,
    pub calls: Option<Vec<TraceDto>>,
    pub error: Option<String>,
}


/// use `debug_traceBlockByNumber` to fetch traces
/// requires, a valid endpoint in `RPC_URL` env var that supports `debug_traceBlockByNumber`
#[tokio::main]
async fn main() -> Result<()> {
    if let Ok(url) = std::env::var("RPC_URL") {
        let client = Provider::<Http>::try_from(url)?;
        let filter = TraceFilterDto {
            tracer: "callTracer".to_string(),
        };
        let traces: Vec<TraceBlockRootResultDto> = client
            .request("debug_traceBlockByNumber", [serialize(&U64::from(16672571)), serialize(&filter)])
            .await
        .map_err(|e| {
            eprintln!("Error on chain: because of: {:?}", e);
            e
        })?;

        println!("result is: {:?}", traces);
    }

    Ok(())
}
