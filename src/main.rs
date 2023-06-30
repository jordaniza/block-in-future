use clap::Parser;
use chrono::prelude::*;
use ethers::prelude::*;
use std::convert::TryFrom;

/// A simple script that fetches a block number at a given point in time
/// Give the script a unix timestamp and it will return the block number
/// Alternatively, pass --mins --hours --days --weeks and an integer to get
/// the block X minutes/hours/days/weeks in the future.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Timestamp to fetch the date for
    #[arg(short, long)]
    timestamp: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let block_number = get_current_block_number().await.unwrap();
    let seconds_in_future = seconds_from_now(args.timestamp);
    let blocks_in_future = seconds_to_block(seconds_in_future);
    let future_block = block_number + blocks_in_future;

    println!("{}", future_block.to_string());
}

fn seconds_from_now(timestamp: usize) -> usize {
    let ts = Utc::now();
    let now: usize = ts.timestamp().try_into().unwrap();
    return timestamp - now;
}

fn seconds_to_block(seconds: usize) -> usize {
    return seconds/12;
}

async fn get_current_block_number() -> Result<usize, Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from("https://rpc.ankr.com/eth")?;
    let block_number = provider.get_block_number().await?;
    Ok(block_number.as_u64() as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        let args = Args::parse_from(&["test", "--timestamp", "1234567890"]);
        assert_eq!(args.timestamp, 1234567890);
        let args = Args::parse_from(&["test", "-t", "1234567890"]);
        assert_eq!(args.timestamp, 1234567890);
    }

    #[test]
    fn test_seconds_from_now() {
        let now: usize = Utc::now().timestamp().try_into().unwrap();
        assert_eq!(seconds_from_now(now), 0);
        assert_eq!(seconds_from_now(now + 10_000), 10_000);
        assert_eq!(seconds_from_now(now + 1), 1);
        assert_eq!(seconds_from_now(now + 123456789876543212), 123456789876543212);
    }

    #[tokio::test]
    async fn test_get_block() {
        let block = get_current_block_number().await.unwrap();
        assert_eq!(block>0, true);

    }
}
