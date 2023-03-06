use std::collections::HashMap;
use std::fs;
use std::path::Path;

use hlfabric::{ChaincodeID, Channel, ChannelConfig, Client, ClientConfig, ClientIdentity, Config, CryptoConfig, PeerConfig, PeerConfigBuilder, TransactionResponse};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the configuration files
    let config = Config::default()?;
    let crypto = CryptoConfig::default()?;
    let client = ClientConfig::default()?;

    // Initialize the client
    let client = Client::new(client, crypto)?;

    // Load the user identity
    let identity = ClientIdentity::load_from_file(&config, "user1")?;

    // Create a channel object and load the channel configuration
    let channel_name = "mychannel";
    let mut channel = Channel::new(&client, &identity, channel_name)?;
    let channel_config_path = Path::new("channel-artifacts/mychannel.tx");
    let channel_config_bytes = fs::read(channel_config_path)?;
    let channel_config = ChannelConfig::from_bytes(&channel_config_bytes)?;

    // Initialize the channel
    channel.initialize(&channel_config)?;

    // Install and instantiate the chaincode
    let chaincode_path = "github.com/mychaincode";
    let chaincode_id = ChaincodeID::new("mychaincode", "v1.0");
    let chaincode_args = vec!["init".into(), "arg1".into(), "arg2".into()];
    let chaincode_policy = r#"{"identities":[{"role":{"name":"member","mspId":"Org1MSP"}}],"policy":{}}"#.to_string();
    channel.install_chaincode(&chaincode_id, chaincode_path)?;
    channel.instantiate_chaincode(&chaincode_id, "init", &chaincode_args, &chaincode_policy)?;

    // Invoke a transaction on the chaincode
    let args: Vec<String> = vec!["invoke".into(), "arg1".into(), "arg2".into()];
    let response: TransactionResponse = channel.invoke_chaincode(&chaincode_id, "invoke", &args)?;
    println!("Transaction ID: {}", response.txid);

    // Query the chaincode for a key
    let key = "mykey";
    let query_response = channel.query_chaincode(&chaincode_id, "query", &[key])?;
    let value = String::from_utf8(query_response.result).unwrap();
    println!("Value of key {}: {}", key, value);

    Ok(())
}
