use std::collections::HashMap;
use std::marker::PhantomData;
use crate::comm::*;
use crate::env::*;
use crate::protocol::*;
use crate::SMap;

pub struct EnvGen<
    'a,
    DP: DomainParameters,
    S: EnvironmentState<DP>,
    C: EnvCommEndpoint<DP> + 'a,
    MC: SMap<'a, DP::AgentId, C>,
    MR: SMap<'a, DP::AgentId, DP::UniversalReward> + Default
> {

    comm_endpoints: MC,
    penalties: MR,
    game_state: S,
    _phantom: &'a PhantomData<(DP, C)>
}

impl<
    'a,
    DP: DomainParameters,
    S: EnvironmentState<DP>,
    C: EnvCommEndpoint<DP> + 'a,
    MC: SMap<'a, DP::AgentId, C>,
    MR: SMap<'a, DP::AgentId, DP::UniversalReward> + Default>
EnvGen<'a, DP, S, C, MC, MR>{


    pub fn new(
        game_state: S,
        comm_endpoints: HashMap<DP::AgentId, C>
    ) -> Self{

        let
    }
}
