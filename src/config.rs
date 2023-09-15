use bitcoin::network::constants::Network;
use chrono::Utc;
use lightning::ln::msgs::NetAddress;
use std::{env, fs};
use tempfile::{Builder, TempDir};

pub struct LdkUserInfo {
	pub bitcoind_rpc_username: String,
	pub bitcoind_rpc_password: String,
	pub bitcoind_rpc_port: u16,
	pub bitcoind_rpc_host: String,
	pub ldk_peer_listening_port: u16,
	pub ldk_announced_listen_addr: Vec<NetAddress>,
	pub ldk_announced_node_name: [u8; 32],
	pub network: Network,
}


// Here we initialize three layers of directories needed for our tests. We won't persist ldk data, but we'll persist
// the logs to help with debugging.
pub(crate) fn setup_data_and_log_dirs(test_name: &str) -> (String, TempDir, String) {
    // If necessary, we initialize the lndk-tests dir, which holds all of the lndk related test data.
    let lndk_tests_folder = "lndk-tests";
    let lndk_test_dir = env::temp_dir().join(lndk_tests_folder);
    fs::create_dir_all(lndk_test_dir.clone()).unwrap();

    let ldk_dir_binding = Builder::new()
        .prefix("ldk-data-dir")
        .tempdir_in(lndk_test_dir.clone())
        .unwrap();
    let ldk_data_dir = String::from(ldk_dir_binding.path().to_str().unwrap());

    // Create the ldk-logs dir, which we'll persist after the tests are over for debugging.
    let now_timestamp = Utc::now();
    let timestamp = now_timestamp.format("%d-%m-%Y-%H%M");
    let ldk_log_dir_binding = lndk_test_dir.join(format!("ldk-logs-{test_name}-{timestamp}"));
    let ldk_log_dir = String::from(ldk_log_dir_binding.as_path().to_str().unwrap());

    // Initialize the LDK data directory if necessary.
    fs::create_dir_all(ldk_data_dir.clone()).unwrap();
    fs::create_dir_all(ldk_log_dir.clone()).unwrap();

    (ldk_data_dir, ldk_dir_binding, ldk_log_dir)
}
