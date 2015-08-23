#[macro_use]
extern crate hsm;
#[macro_use]
extern crate log;
extern crate fern;
extern crate time;


#[derive(Debug, Clone)]
enum Events {
    Plus,
    Minus,
    Int(u32)
}

#[derive(Debug)]
enum Tokens {
    Plus,
    Minus,
    Int(i32)
}

#[derive(Debug)]
struct SharedData {
    accepted_tokens: Vec<Tokens>
}
impl SharedData {
    fn new() -> Self {
        SharedData {accepted_tokens: Vec::new()}
    }
}

hsm_define_objects!(StateStruct, States, Events, SharedData, (
    WaitMinusOrInt {counter: u8 = 0},
    WaitInt {},
    WaitOp {}
));

hsm_state_parents!(States; WaitMinusOrInt -> None, WaitInt -> None, WaitOp -> None );


impl hsm::State<Events, States, SharedData> for WaitMinusOrInt<Events, States, SharedData> {
    fn handle_event(&mut self, shr_data: &mut SharedData, evt: hsm::Event<Events>, probe: bool) -> hsm::Action<States> {
        self.counter += 1;
        info!("{} time in WaitMinusOrInt, shared: {:?}", self.counter, shr_data);
        match evt {
            hsm::Event::User(Events::Minus) => {
                info!("minus");
                hsm::Action::Transition(States::WaitInt)
            },
            hsm::Event::User(Events::Int(x)) => {
                info!("int({:?})", x);
                hsm::Action::Transition(States::WaitOp)
            },
            _ => hsm::Action::Ignore
        }
    }
}

impl hsm::State<Events, States, SharedData> for WaitInt<Events, States, SharedData> {
    fn handle_event(&mut self, shr_data: &mut SharedData, evt: hsm::Event<Events>, probe: bool) -> hsm::Action<States> {
        match evt {
            hsm::Event::User(Events::Int(x)) => hsm_delayed_transition!(probe, {
                info!("int({:?})", x);
                States::WaitOp
            }),
            _ => hsm::Action::Ignore
        }
    }
}

hsm_impl_state!(WaitOp, Events, States, SharedData,
    hsm::Event::User(Events::Minus) => {
        info!("minus");
        hsm::Action::Transition(States::WaitMinusOrInt)
    },
    hsm::Event::User(Events::Plus) => {
        info!("plus");
        hsm::Action::Transition(States::WaitMinusOrInt)
    },
    _ => hsm::Action::Ignore
);

fn main() {
    conf_logger();
    let mut sm = hsm::StateMachine::<StateStruct, States, Events, SharedData>::new(States::WaitMinusOrInt, SharedData::new());
    sm.start();
    sm.input(hsm::Event::User(Events::Int(4)));
    sm.input(hsm::Event::User(Events::Plus));
    sm.input(hsm::Event::User(Events::Minus));
    sm.input(hsm::Event::User(Events::Int(5)));
}

fn conf_logger() {
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            let t = time::now();
            let ms = t.tm_nsec/1000_000;
            format!("{}.{:3} [{}] {}", t.strftime("%Y-%m-%dT%H:%M:%S").unwrap(), ms, level, msg)
        }),
        output: vec![fern::OutputConfig::stderr()],
        level: log::LogLevelFilter::Trace,
    };

    if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Trace) {
        panic!("Failed to initialize global logger: {}", e);
    }
}
