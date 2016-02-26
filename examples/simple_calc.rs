/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2015 Mattis Marjak (mattis.marjak@gmail.com)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#[macro_use]
extern crate hsm;
#[macro_use]
extern crate log;
extern crate fern;
extern crate time;


#[derive(Debug)]
pub enum Events {
    Plus,
    Minus,
    Int(u32)
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Tokens {
    Plus,
    Minus,
    Int(i32)
}

#[derive(Debug)]
pub struct SharedData {
    accepted_tokens: Vec<Tokens>
}
impl SharedData {
    fn new() -> Self {
        SharedData {accepted_tokens: Vec::new()}
    }
}

hsm_define_objects_noparents!(StateStruct, States, Events, SharedData, (
    WaitMinusOrInt {counter: u8 = 0},
    WaitInt {},
    WaitOp {}
));

impl hsm::State<Events, States, SharedData> for WaitMinusOrInt {
    #[allow(unused_variables)]
    fn handle_event(&mut self, shr_data: &mut SharedData, evt: &mut hsm::Event<Events>, probe: bool) -> hsm::Action<States> {
        self.counter += 1;
        info!("{} time in WaitMinusOrInt, shared: {:?}", self.counter, shr_data);
        match *evt {
            hsm::Event::User(&mut Events::Minus) => {
                info!("minus");
                hsm::Action::Transition(States::WaitInt)
            },
            hsm::Event::User(&mut Events::Int(x)) => {
                info!("int({:?})", x);
                hsm::Action::Transition(States::WaitOp)
            },
            _ => hsm::Action::Ignore
        }
    }
}

// Can't use self
hsm_impl_state!(WaitInt, Events, States, SharedData, shr, evt, probe,
    hsm::Event::User(&mut Events::Int(x)) => hsm_delayed_transition!(probe, {
        println!("{:?} {:?} {:?}", shr, evt, probe);
        info!("int({:?})", x);
        States::WaitOp
    }),
    _ => hsm::Action::Ignore
);

// Can't use self, shrared_data, event or probe
hsm_impl_state!(WaitOp, Events, States, SharedData,
    hsm::Event::User(&mut Events::Minus) => {
        info!("minus");
        hsm::Action::Transition(States::WaitMinusOrInt)
    },
    hsm::Event::User(&mut Events::Plus) => {
        info!("plus");
        hsm::Action::Transition(States::WaitMinusOrInt)
    },
    _ => hsm::Action::Ignore
);

fn main() {
    conf_logger();
    let mut sm = hsm::StateMachine::<StateStruct, States, SharedData>::new(States::WaitMinusOrInt, SharedData::new());
    sm.start();
    sm.input(&mut Events::Int(4));
    sm.input(&mut Events::Plus);
    sm.input(&mut Events::Minus);
    sm.input(&mut Events::Int(5));
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
