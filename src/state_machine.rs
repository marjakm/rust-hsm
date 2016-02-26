use std::fmt;
use ::traits::*;
use ::{Task, Action, Event, EnterOrExit};


#[derive(Debug)]
pub struct StateMachine<UsrStStr, UsrStEnum, UsrShrData>
    where UsrStStr:   fmt::Debug,
          UsrStEnum:  fmt::Debug,
          UsrShrData: fmt::Debug,
{
    current     : UsrStEnum,
    started     : bool,
    states      : UsrStStr,
    shr_data    : UsrShrData,
    exit_tasks  : Vec<Task<UsrStEnum>>,
    enter_tasks : Vec<Task<UsrStEnum>>,
}

impl<UsrStStr, UsrStEnum, UsrShrData> StateMachine<UsrStStr, UsrStEnum, UsrShrData>
    where UsrStStr   : fmt::Debug + Initializer,
          UsrStEnum  : fmt::Debug + Eq + Clone + InstanceParent<UsrStEnum>,
          UsrShrData : fmt::Debug,
{
    pub fn new(initial: UsrStEnum, shared_data: UsrShrData) -> Self {
        StateMachine {
            current     : initial,
            started     : false,
            states      : UsrStStr::new(),
            shr_data    : shared_data,
            exit_tasks  : Vec::new(),
            enter_tasks : Vec::new(),
        }
    }

    pub fn data(&self) -> &UsrShrData {
        &self.shr_data
    }

    pub fn data_mut(&mut self) -> &mut UsrShrData {
        &mut self.shr_data
    }

    pub fn start<UsrEvtEnum>(&mut self) where
        UsrEvtEnum: fmt::Debug,
        UsrStStr:   StateLookup<UsrStEnum, UsrEvtEnum, UsrShrData>
    {
        let mut parent = Some(self.current.clone());
        while let Some(state) = parent {
            parent = state.get_parent();
            self.enter_tasks.push(Task::new(state, EnterOrExit::Enter));
        }
        self.process_enter_tasks();
        self.started = true;
    }

    fn process_exit_tasks<UsrEvtEnum>(&mut self) where
        UsrEvtEnum: fmt::Debug,
        UsrStStr:   StateLookup<UsrStEnum, UsrEvtEnum, UsrShrData>
    {
        for task in self.exit_tasks.iter_mut() {
            debug!("send {:?} to {:?}", task.enter_or_exit, task.state);
            match self.states.lookup(&task.state).handle_event(
                  &mut self.shr_data, &mut task.event(), false){
                Action::Ignore | Action::Parent => {},
                _ => panic!("Transitions from exit events are not allowed, \
                            ignoring transition from state {:?} on event {:?}",
                            task.state, task.enter_or_exit)
            };
        }
        self.exit_tasks.clear();
    }

    fn process_enter_tasks<UsrEvtEnum>(&mut self) where
        UsrEvtEnum: fmt::Debug,
        UsrStStr:   StateLookup<UsrStEnum, UsrEvtEnum, UsrShrData>
    {
        self.enter_tasks.reverse();
        for task in self.enter_tasks.iter_mut() {
            debug!("send {:?} to {:?}", task.enter_or_exit, task.state);
            match self.states.lookup(&task.state).handle_event(
                  &mut self.shr_data, &mut task.event(), false){
                Action::Ignore | Action::Parent => {},
                _ => panic!("Transitions from enter events are not allowed, \
                            ignoring transition from state {:?} on event {:?}",
                            task.state, task.enter_or_exit)
            }
        }
        self.enter_tasks.clear();
    }

    fn transition<UsrEvtEnum>(&mut self, from_state: UsrStEnum, to_state: UsrStEnum) where
        UsrEvtEnum: fmt::Debug,
        UsrStStr:   StateLookup<UsrStEnum, UsrEvtEnum, UsrShrData>
    {
        let mut parent = Some(from_state);
        while let Some(state) = parent {
            parent = state.get_parent();
            self.exit_tasks.push(Task::new(state, EnterOrExit::Exit));
        }
        let mut same_idx: Option<usize> = None;
        parent = Some(to_state);
        'outer: while let Some(state) = parent {
            for (i, task) in self.exit_tasks.iter().enumerate() {
                if task.state == state {
                    same_idx = Some(i);
                    break 'outer;
                }
            }
            parent = state.get_parent();
            self.enter_tasks.push(Task::new(state, EnterOrExit::Enter));
        }
        if let Some(i) = same_idx {
            let drop_num = self.exit_tasks.len() - i;
            for _ in 0..drop_num {
                self.exit_tasks.pop();
            }
        }
        self.process_exit_tasks();
        self.process_enter_tasks();
    }

    pub fn input<UsrEvtEnum>(&mut self, evt: &mut UsrEvtEnum) where
        UsrEvtEnum: fmt::Debug,
        UsrStStr:   StateLookup<UsrStEnum, UsrEvtEnum, UsrShrData>
    {
        assert!(self.started, "Can't call input before starting the state machine with start()");
        let mut evt = Event::User(evt);
        debug!("state:  {:?}", self.current);
        debug!("input:  {:?}", evt);
        let mut action;
        let mut state = self.current.clone();
        loop {
            action = self.states.lookup(&state).handle_event(&mut self.shr_data, &mut evt, true);
            match action {
                Action::Ignore               => {
                    self.exit_tasks.clear();
                    break;
                },
                Action::Parent               => {
                    if let Some(parent) = state.get_parent() {
                        self.exit_tasks.push(Task::new(state.clone(), EnterOrExit::Exit));
                        state = parent;
                    } else {
                        panic!("State {:?} responded with Action::Parent to event {:?}, but the state has no parent", state, evt);
                        // break;
                    }
                },
                Action::Transition(x)        => {
                    debug!("send {:?} to {:?}", evt, state);
                    self.process_exit_tasks();  // exit until in the parent that handles the signal
                    self.current = x.clone(); // signal allready handled
                    self.transition(state.clone(), x);
                    break;
                },
                Action::DelayedTransition => {
                    self.process_exit_tasks(); // exit until in the parent that handles the signal
                    debug!("send {:?} to {:?}", evt, state);
                    if let Action::Transition(x) = self.states.lookup(&state).handle_event(&mut self.shr_data, &mut evt, false) { // handle the signal
                        self.current = x.clone();
                        self.transition(state.clone(), x);
                    } else {
                        panic!("State {:?} probed Action::DelayedTransition to event {:?}, but doesn't return Action::Transition", state, evt);
                        // self.current = state;
                    }
                    break;
                },
            }
        }
    }
}
