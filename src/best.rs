#[allow(unused)]
pub fn space_mission_best() {
    println!(
        "Bytes in orbit: {}",
        std::mem::size_of::<Spacecraft<InOrbit, Undocked>>()
    );
    println!(
        "Bytes while docked: {}",
        std::mem::size_of::<Spacecraft<InOrbit, Docked<InOrbit>>>()
    );

    let apollo = Spacecraft::new("Apollo XVIII".into(), 3);
    apollo.call_on_radio("Apollo XVIII is go for launch!");
    let apollo = apollo.launch(); // <- shadowing a previous declaration is encouraged
    let apollo = apollo.jettison_booster();

    println!("Eureka! {}!", apollo.perform_science());

    let artemis = Spacecraft::new("Artemis I".into(), 4);
    let artemis = artemis.launch();
    let artemis = artemis.jettison_booster();

    println!("{:?}", artemis);
    println!("{:?}", apollo);

    artemis.call_on_radio("Artemis I, you are cleared to dock!");
    let mut rendezvous = artemis.dock(apollo);
    rendezvous.perform_science();
    println!("{:?}", rendezvous);

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

#[derive(Debug, Clone, Copy)]
struct ReadyToLaunch;
impl State for ReadyToLaunch {
    fn get_state_text(&self) -> String {
        "ready to launch".into()
    }
}

#[derive(Debug, Clone, Copy)]
struct InAtmosphere;
impl State for InAtmosphere {
    fn get_state_text(&self) -> String {
        "in atmosphere".into()
    }
}

#[derive(Debug, Clone, Copy)]
struct InOrbit;
impl State for InOrbit {
    fn get_state_text(&self) -> String {
        "in orbit".into()
    }
}
impl CanPerformScience for InOrbit {}

trait DockingState {
    fn get_docking_text(&self) -> String;
}

#[derive(Debug, Clone, Copy)]
struct Undocked;
impl DockingState for Undocked {
    fn get_docking_text(&self) -> String {
        "".into()
    }
}

#[derive(Debug, Clone)]
struct Docked<S: State> {
    ships: Box<(Spacecraft<S, Undocked>, Spacecraft<S, Undocked>)>,
}
impl<T: State> DockingState for Docked<T> {
    fn get_docking_text(&self) -> String {
        format!(", docked to {}", self.ships.1.name)
    }
}

#[derive(Debug, Clone)]
struct Spacecraft<S: State, D: DockingState> {
    name: String,
    crew: u32,
    state: S,
    docking: D
}

impl Spacecraft<ReadyToLaunch, Undocked> {
    fn new(name: String, crew: u32) -> Spacecraft<ReadyToLaunch, Undocked> {
        Spacecraft::<ReadyToLaunch, Undocked> {
            name,
            crew,
            state: ReadyToLaunch,
            docking: Undocked
        }
    }
}

// These functions can always be called, regardless of state
// This means they can only use the general fields of the Spacecraft struct
impl<S: State, D: DockingState> Spacecraft<S, D> {
    fn call_on_radio(&self, _message: &str) -> String {
        format!(
            "{} is {}{} and copies all!",
            self.name,
            self.state.get_state_text(),
            self.docking.get_docking_text()
        )
    }
}

impl <D: DockingState> Spacecraft<ReadyToLaunch, D> {
    fn launch(self) -> Spacecraft<InAtmosphere, D> {
        println!("3... 2... 1... Liftoff for {}!", self.name);
        Spacecraft::<InAtmosphere, D> {
            name: self.name,
            crew: self.crew,
            state: InAtmosphere,
            docking: self.docking
        }

        // Optimization for types that are expensive to copy. It
        // might be better to instead introduce indirection in the data
        // and copy the pointer(s). As always, measure before you unsafe!

        // SAFETY: both state parameters are 0-sized,
        // so both types have the same bitwise representation
        // unsafe { std::mem::transmute(self) }
    }
}

impl <D: DockingState> Spacecraft<InAtmosphere, D> {
    fn jettison_booster(self) -> Spacecraft<InOrbit, D> {
        println!("Booster separation confirmed for {}", self.name);
        Spacecraft::<InOrbit, D> {
            name: self.name,
            crew: self.crew,
            state: InOrbit,
            docking: self.docking
        }
    }
}

impl <S: State + Clone> Spacecraft<S, Undocked> {
    fn dock(self, other: Spacecraft<S, Undocked>) -> Spacecraft<S, Docked<S>> {
        println!("Docking clamps locked!");
        Spacecraft {
            name: format!("{} and {}", self.name, other.name),
            crew: self.crew + other.crew,
            state: self.state.clone(),
            docking: Docked {
                ships: Box::new((self, other))
            }
        }
    }
}

impl<S: State> Spacecraft<S, Docked<S>> {
    fn transfer_crew(&mut self, amount: u32) {
        self.docking.ships.0.crew -= amount;
        self.docking.ships.1.crew += amount;
    }

    fn undock(self) -> (Spacecraft<S, Undocked>, Spacecraft<S, Undocked>) {
        println!("Releasing docking clamps!");
        *self.docking.ships
    }
}

impl<S: State + CanPerformScience, D: DockingState> Spacecraft<S, D> {
    fn perform_science(&self) -> i32 {
        42
    }
}
