extern crate rusted;
use rusted::config::{Config};

#[test]
pub fn test_config() {
    let local_config = Config::new("config.toml").local.unwrap();
    assert!(local_config.contains_key("public_ssh_key"), "public_ssh_key is missing from config.toml");
}
