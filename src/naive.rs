
#[allow(unused)]
pub fn space_mission_naive() {
    let mut apollo = Spacecraft::new("Apollo XX".into(), 3);
    apollo.launch();
    apollo.jettison_booster();
    println!(
        "Size in orbit: {}",
        std::mem::size_of::<Spacecraft>()
    );
    let science = apollo.perform_science();
    println!("Eureka! {science}!");

    let mut artemis = Spacecraft::new("Artemis I".into(), 4);
    artemis.launch();
    artemis.call_on_radio("MECO confirmed!");
    artemis.jettison_booster();

    println!("{}: {} crew", artemis.name, artemis.crew);
    println!("{}: {} crew", apollo.name, apollo.crew);

    artemis.dock(apollo);
    println!("Size docked: {}", std::mem::size_of::<Spacecraft>());
    artemis.transfer_crew(2);

   //artemis.perform_science();

    let apollo = artemis.undock().unwrap();

    println!("{}: {} crew", artemis.name, artemis.crew);
    println!("{}: {} crew", apollo.name, apollo.crew);
}

#[derive(Debug, PartialEq)]
enum State {
    ReadyToLaunch,
    InAtmosphere,
    InOrbit,
    Docked {
        docked_to: Box<Spacecraft>
    }
}

#[derive(Debug, PartialEq)]
struct Spacecraft {
    name: String,
    crew: u32,
    state: State,
}

impl Spacecraft {
    fn new(name: String, crew: u32) -> Self {
        Spacecraft { name, crew, state: State::ReadyToLaunch }
    }

    fn call_on_radio(&self, _message: &str) -> String {
        let state_text = match &self.state {
            State::ReadyToLaunch => "ready to launch".to_owned(),
            State::InAtmosphere => "in atmosphere".to_owned(),
            State::InOrbit => "in orbit".to_owned(),
            State::Docked { docked_to } => format!("docked to {}", docked_to.name),
        };
        format!("{} is {} and copies all!", self.name, state_text)
    }

    fn launch(&mut self) {
        if self.state != State::ReadyToLaunch { panic!("Can only launch if ready to launch!") }

        println!("3... 2... 1... Liftoff for {}!", self.name);
        self.state = State::InAtmosphere;
    }

    fn jettison_booster(&mut self) {
        if self.state != State::InAtmosphere { panic!("Can only jettison booster in atmosphere!") }

        println!("Booster separation confirmed for {}", self.name);
        self.state = State::InOrbit;
    }

    fn dock(&mut self, other: Spacecraft) {
        if self.state != State::InOrbit { panic!("Can only dock in orbit!") }

        println!("Docking clamps locked!");
        self.state = State::Docked { docked_to: Box::new(other) }
        // We do not need to update `other`, as we have ownership, and will not access it again
    }

    fn undock(&mut self) -> Option<Spacecraft> {
        let prev_state = std::mem::replace(&mut self.state, State::InOrbit);
        if let State::Docked { docked_to } = prev_state { // <- `if let` statements allows for concise matching 
            Some(*docked_to)
        } else {
            None
        }
    }

    fn transfer_crew(&mut self, amount: u32) {
        match self.state {
            State::Docked { ref mut docked_to } => {
                self.crew -= amount;
                docked_to.crew += amount;
            }
            _ => panic!("Can only transfer crew while docked!")
        }
    }
    
    fn perform_science(&self) -> i32 {
        if self.state != State::InOrbit { panic!("Can only perform science in orbit!") }
        42 // <- the last statement of a block is the return value (if it does not have a `;`)
    }
}