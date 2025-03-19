use std::{marker::PhantomData, ops::Deref};

pub trait State<D> {}
pub trait InitialState<D, S: State<D>> {
    fn from_data(data: D) -> StateMachine<D, S> {
        StateMachine {
            data,
            state: PhantomData
        }
    }
}

pub struct StateMachine<D, S: State<D>> {
    data: D,
    state: PhantomData<S>,
}

impl<D, S: State<D>> Deref for StateMachine<D, S> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

pub trait StateMachineOps<D, S: State<D>> {
    fn change_state<S2: State<D>>(self) -> StateMachine<D, S2>;
}

impl<D, S: State<D>> StateMachineOps<D, S> for StateMachine<D, S> {
    fn change_state<S2: State<D>>(self) -> StateMachine<D, S2> {
        StateMachine {
            data: self.data,
            state: PhantomData,
        }
    }
}

// ====================

mod use_generic {
    use super::{InitialState, State, StateMachine, StateMachineOps};

    #[derive(Debug)]
    pub struct SpacecraftData {
        pub name: &'static str,
        pub crew: u32,
    }

    pub type Spacecraft<S> = StateMachine<SpacecraftData, S>;
    impl InitialState<SpacecraftData, OnTheGround> for Spacecraft<OnTheGround> {}

    pub struct OnTheGround;
    impl State<SpacecraftData> for OnTheGround {}

    pub struct Launching;
    impl State<SpacecraftData> for Launching {}
    pub struct InOrbit;
    impl State<SpacecraftData> for InOrbit {}

    impl Spacecraft<OnTheGround> {

        pub fn new(name: &'static str, crew: u32) -> Self {
            Spacecraft::from_data(SpacecraftData { name, crew })
        }

        pub fn liftoff(self) -> Spacecraft<Launching> {
            println!("3 2 1 Liftoff!");
            self.change_state()
        }

        pub fn try_teleport_home(self) -> Result<Spacecraft<OnTheGround>, ()> {
            let rng = 0.35;
            if rng < 0.5 {
                Ok(self.change_state())
            } else {
                Err(()) // kaboom
            }
        }
    }

    fn do_work() {
        let craft = Spacecraft::new( "Apollo", 3 );
        let craft = craft.liftoff();
    }
}
