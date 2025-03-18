#[allow(unused)]
pub fn space_mission_smart() {
    let apollo = Spacecraft::new("Apollo XVIII".into(), 3);
    apollo.call_on_radio("Apollo XVIII is go for launch!");
    let apollo = apollo.launch(); // <- shadowing a previous declaration is encouraged
    let apollo = apollo.jettison_booster();
    println!(
        "Bytes in orbit: {}",
        std::mem::size_of::<Spacecraft<InOrbit>>()
    );
    println!("Eureka! {}!", apollo.perform_science());

    let artemis = Spacecraft::new("Artemis I".into(), 4);
    let artemis = artemis.launch();
    let artemis = artemis.jettison_booster();

    println!("{:?}", artemis);
    println!("{:?}", apollo);

    artemis.call_on_radio("Artemis I, you are cleared to dock!");
    let mut rendezvous = artemis.dock(apollo);
    println!(
        "Bytes while docked: {}",
        std::mem::size_of::<Spacecraft<Docked>>()
    );
    rendezvous.transfer_crew(2);

    // artemis.perform_science();

    let (artemis, apollo) = rendezvous.undock();

    println!("{:?}", artemis);
    println!("{:?}", apollo);

    let newglenn = Spacecraft::new("New Glenn".into(), 2);

}

trait State {
    fn get_state_text(&self) -> String;
}

trait CanPerformScience {}

#[derive(Debug)]
struct ReadyToLaunch;
impl State for ReadyToLaunch {
    fn get_state_text(&self) -> String {
        "ready to launch".into()
    }
}

#[derive(Debug)]
struct InAtmosphere;
impl State for InAtmosphere {
    fn get_state_text(&self) -> String {
        "in atmosphere".into()
    }
}

#[derive(Debug)]
struct InOrbit;
impl State for InOrbit {
    fn get_state_text(&self) -> String {
        "in orbit".into()
    }
}
impl CanPerformScience for InOrbit {}

#[derive(Debug)]
struct Docked {
    ships: Box<(Spacecraft<InOrbit>, Spacecraft<InOrbit>)>,
}
impl State for Docked {
    fn get_state_text(&self) -> String {
        format!("docked to {}", self.ships.1.name)
    }
}
impl CanPerformScience for Docked {}

#[derive(Debug)]
struct Spacecraft<S: State> {
    name: String,
    crew: u32,
    state: S,
}

// These functions can always be called, regardless of state
// This means they can only use the general fields of the Spacecraft struct
impl<T: State> Spacecraft<T> {
    fn call_on_radio(&self, _message: &str) -> String {
        format!(
            "{} is {} and copies all!",
            self.name,
            self.state.get_state_text()
        )
    }
}

impl Spacecraft<ReadyToLaunch> {
    fn new(name: String, crew: u32) -> Self {
        Self {
            name,
            crew,
            state: ReadyToLaunch,
        }
    }

    fn launch(self) -> Spacecraft<InAtmosphere> {
        println!("3... 2... 1... Liftoff for {}!", self.name);
        Spacecraft {
            name: self.name,
            crew: self.crew,
            state: InAtmosphere,
        }

        // Optimization for types that are expensive to copy. It
        // might be better to instead introduce indirection in the data
        // and copy the pointer(s). As always, measure before you unsafe!

        // SAFETY: both state parameters are 0-sized,
        // so both types have the same bitwise representation
        // unsafe { std::mem::transmute(self) }
    }
}

impl Spacecraft<InAtmosphere> {
    fn jettison_booster(self) -> Spacecraft<InOrbit> {
        println!("Booster separation confirmed for {}", self.name);
        Spacecraft {
            name: self.name,
            crew: self.crew,
            state: InOrbit,
        }
    }
}

impl Spacecraft<InOrbit> {
    fn dock(self, other: Spacecraft<InOrbit>) -> Spacecraft<Docked> {
        println!("Docking clamps locked!");
        Spacecraft {
            name: self.name.clone(),
            crew: self.crew,
            state: Docked {
                ships: Box::new((self, other)),
            },
        }
    }
}

impl Spacecraft<Docked> {
    fn transfer_crew(&mut self, amount: u32) {
        self.state.ships.0.crew -= amount;
        self.state.ships.1.crew += amount;
    }

    fn undock(self) -> (Spacecraft<InOrbit>, Spacecraft<InOrbit>) {
        println!("Releasing docking clamps!");
        *self.state.ships
    }
}

impl<S: State + CanPerformScience> Spacecraft<S> {
    fn perform_science(&self) -> i32 {
        42
    }
}
