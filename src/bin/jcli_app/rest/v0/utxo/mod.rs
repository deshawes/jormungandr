use jcli_app::utils::{DebugFlag, HostAddr, RestApiSender};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub enum Utxo {
    /// Get all UTXOs
    Get {
        #[structopt(flatten)]
        addr: HostAddr,
        #[structopt(flatten)]
        debug: DebugFlag,
    },
}

impl Utxo {
    pub fn exec(self) {
        let (addr, debug) = match self {
            Utxo::Get { addr, debug } => (addr, debug),
        };
        let url = addr.with_segments(&["v0", "utxo"]).unwrap().into_url();
        let builder = reqwest::Client::new().get(url);
        let response = RestApiSender::new(builder, &debug).send().unwrap();
        response.response().error_for_status_ref().unwrap();
        let utxos: serde_json::Value = response.body().json().unwrap();
        let utxos_yaml = serde_yaml::to_string(&utxos).unwrap();
        println!("{}", utxos_yaml);
    }
}
