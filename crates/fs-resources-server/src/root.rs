//! The root capability `Resources`: what this server offers today.
//!
//! Pattern: Adapter: the generated `resources::Server` trait in, a value (`Root`) that only reports.

use capnp::capability::Rc;
use fs_interfaces::resources_capnp::resources;

/// The program name that `Control.version` and the support report carry; clients expect exactly this.
pub const PROGRAM: &str = "fs-resources";

/// The contract this server speaks: `resources.capnp`, draft 0.1 (`fs-interfaces` v0.0.27). The number before the dot is
/// the main level, the one after it the minor level: a client accepts a server of the same main level whose minor level
/// is not lower (`fs-resources-client`). The server's one source; the client names the same number and a test compares them.
pub const CONTRACT_VERSION: &str = "0.1";

type Done = Result<(), capnp::Error>;

/// The root capability. It holds the contract version it reports (tests give another one).
pub struct Root {
    contract: String,
}

impl Root {
    /// A root that reports `contract` as its contract version.
    #[must_use]
    pub fn new(contract: impl Into<String>) -> Self {
        Self {
            contract: contract.into(),
        }
    }

    /// The capability to hand to the transport.
    #[must_use]
    pub fn into_client(self) -> capnp::capability::Client {
        capnp_rpc::new_client::<resources::Client, _>(self).client
    }
}

impl resources::Server for Root {
    /// What this program offers: no capability ids yet (a frontend locks everything), and the contract version.
    async fn capabilities(
        self: Rc<Self>,
        _: resources::CapabilitiesParams,
        mut results: resources::CapabilitiesResults,
    ) -> Done {
        let mut list = results.get().init_capabilities();
        list.set_contract_version(self.contract.as_str());
        list.init_ids(0);
        Ok(())
    }

    /// The provider rule (ADR 0045): the contract version and the provider; no entries, because nothing is offered yet.
    async fn support_report(
        self: Rc<Self>,
        _: resources::SupportReportParams,
        mut results: resources::SupportReportResults,
    ) -> Done {
        let mut report = results.get().init_report();
        report.set_contract_version(self.contract.as_str());
        report.set_provider(PROGRAM);
        report.init_entries(0);
        Ok(())
    }
}
