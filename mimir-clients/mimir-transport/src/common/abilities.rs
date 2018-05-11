use common::{MSG,CMD,Role};


/// specification of an entity's capabilities
///
#[derive(Default,Debug,Copy,Clone,PartialEq,Eq,Serialize,Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Abilities {
    produce_query: bool,
    produce_notarize: bool,
    produce_yield: bool,
    produce_route: bool,
    produce_verify: bool,
    execute_identify: bool,
    execute_debug: bool,
    execute_kick: bool,
}


impl Abilities {

    /// full capabilities
    pub fn full() -> Self {
        Self {
            produce_query: true,
            produce_notarize: true,
            produce_yield: true,
            produce_route: true,
            produce_verify: true,
            execute_identify: true,
            execute_debug: true,
            execute_kick: true,
        }
    }

    /// no capabilities
    pub fn none() -> Self { Default::default() }


    /// get default capabilities of specified role
    pub fn new(role: Role) -> Self {
        match role {
            Role::Oracle => Self { produce_notarize: true, ..Self::none() },
            Role::Notary => Self { produce_yield: true, ..Self::none() },
            Role::Requester => Self {
                produce_query: true,
                produce_route: true,
                produce_verify: true,
                ..Self::none()
            },
            Role::Router => Self { produce_notarize: true, ..Self::none() },
            Role::Verifier => Self { produce_notarize: true, ..Self::none() },
            Role::Admin => Self::full(),
        }
    }

    /// check `produce` capability for `msg`.
    pub fn can_produce(&self, msg: MSG) -> bool {
        match msg {
            MSG::QUERY    => self.produce_query,
            MSG::NOTARIZE => self.produce_notarize,
            MSG::YIELD    => self.produce_yield,
            MSG::ROUTE    => self.produce_route,
            MSG::VERIFY   => self.produce_verify,
        }
    }

    /// check `execute` capability for `cmd`
    pub fn can_execute(&self, cmd: CMD) -> bool {
        match cmd {
            CMD::IDENTIFY => self.execute_identify,
            CMD::DEBUG     => self.execute_debug,
            CMD::KICK     => self.execute_kick,
        }
    }
}

