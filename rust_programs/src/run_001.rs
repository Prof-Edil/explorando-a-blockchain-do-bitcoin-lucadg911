use core::str;
use std::process::Command;

pub fn run() {
    let block_hash  = Command::new("bitcoin-cli")
    .arg("-rpcconnect=84.247.182.145")
    .arg("-rpcuser=user_240")
    .arg("--rpcpassword=JQs8WLu7N4dR")
    .arg("getblockhash")
    .arg("654321")
    .output().unwrap();
    let output = str::from_utf8(&block_hash.stdout);
    let output = output.unwrap().trim();
    println!("{}", output.to_string());
}
