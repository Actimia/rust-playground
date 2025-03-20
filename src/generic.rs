use std::{marker::PhantomData, ops::Deref};

use use_generic::Spacecraft;

#[allow(unused)]
pub fn space_mission_generic() {
    let apollo = Spacecraft::new("Apollo", 3);
    let apollo = apollo.liftoff();
    let apollo = apollo.jettison_booster();

    println!("{}: {} crew", apollo.name, apollo.crew);

    if let Ok(apollo) = apollo.try_teleport_home() {
        println!("We made it!")
    } else {
        println!("BOOM")
    }
}

pub trait State<D> {}
pub trait InitialState<D, S: State<D>> {
    fn from_data(data: D) -> StateMachine<D, S> {
        StateMachine {
            data,
            state: PhantomData,
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

    impl<S: State<SpacecraftData>> Spacecraft<S> {
        pub fn try_teleport_home(self) -> Result<Spacecraft<OnTheGround>, ()> {
            // VERY experimental drive.
            if rand::random_bool(0.5) {
                Ok(self.change_state())
            } else {
                Err(()) // kaboom
            }
        }
    }

    impl Spacecraft<OnTheGround> {
        pub fn new(name: &'static str, crew: u32) -> Self {
            Spacecraft::from_data(SpacecraftData { name, crew })
        }

        pub fn liftoff(self) -> Spacecraft<Launching> {
            println!("3 2 1 Liftoff!");
            self.change_state()
        }
    }

    impl Spacecraft<Launching> {
        pub fn jettison_booster(self) -> Spacecraft<InOrbit> {
            self.change_state()
        }
    }
}
