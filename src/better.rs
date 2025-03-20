pub fn space_mission_better() {
    let apollo = Spacecraft::new("Apollo".into(), 3);
    apollo.call_on_radio("Apollo is go for launch!");
    let apollo = apollo.start_engines(); // <- shadowing a previous declaration is encouraged
    let apollo = apollo.jettison_booster();
    //apollo.call_on_radio("");
    println!("Eureka! {}!", apollo.perform_science());

    let artemis = Spacecraft::new("Artemis".into(), 4);
    let artemis = artemis.start_engines();
    let artemis = artemis.jettison_booster();

    println!("{:?}", apollo);
    println!("{:?}", artemis);

    artemis.call_on_radio("Artemis, you are cleared to dock!");
    let mut rendezvous = artemis.dock(apollo);

    println!("{:?}", rendezvous);

    rendezvous.perform_science();
    // artemis.perform_science(); // <-- This is not allowed, as the artemis binding has been moved
    rendezvous.transfer_crew(2);

    let (artemis, apollo) = rendezvous.undock();

    println!("{:?}", apollo);
    println!("{:?}", artemis);
}

#[derive(Debug)]
struct Spacecraft<S: State> {
    name: String,
    crew: u32,
    state: S,
}

trait State {
    fn get_state_text(&self) -> String;
}

trait CanPerformScience {} // <- a trait with no functions is called a "marker trait"

#[derive(Debug)]
struct OnTheGround;
impl State for OnTheGround {
    fn get_state_text(&self) -> String {
        "ready to launch".into()
    }
}

#[derive(Debug)]
struct Launching;
impl State for Launching {
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

// These functions can always be called, regardless of state
// This means they can only use the general fields of the Spacecraft struct
impl<S: State> Spacecraft<S> {
    fn call_on_radio(&self, _message: &str) -> String {
        format!(
            "{} is {} and copies all!",
            self.name,
            self.state.get_state_text()
        )
    }

    #[allow(unused)]
    fn try_teleport_home(self) -> Result<Spacecraft<OnTheGround>, Self> {
        // Experimental teleportation drive for emergencies: It may get you home, it might not!
        if rand::random_bool(0.5) {
            Ok(Spacecraft {
                name: self.name,
                crew: self.crew,
                state: OnTheGround,
            })
        } else {
            Err(self)
        }
    }
}

impl Spacecraft<OnTheGround> {
    fn new(name: String, crew: u32) -> Self {
        Self {
            name,
            crew,
            state: OnTheGround,
        }
    }

    fn start_engines(self) -> Spacecraft<Launching> {
        println!("3... 2... 1... Liftoff for {}!", self.name);
        Spacecraft {
            name: self.name,
            crew: self.crew,
            state: Launching,
        }

        // Optimization for types that are expensive to copy. It
        // might be better to instead introduce indirection in the data
        // and copy the pointer(s). As always, measure before you unsafe!

        // SAFETY: both state parameters are 0-sized,
        // so both types have the same bitwise representation
        // unsafe { std::mem::transmute(self) }
    }
}

impl Spacecraft<Launching> {
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
            name: format!("{} and {}", self.name, other.name),
            crew: self.crew + other.crew,
            state: Docked {
                ships: Box::new((self, other)),
            },
        }
    }
}

impl Spacecraft<Docked> {
    fn transfer_crew(&mut self, amount: u32) {
        let amount = amount.min(self.state.ships.0.crew);
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
