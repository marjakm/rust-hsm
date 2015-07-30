use super::{Event, State, Action, Initializer, StateMachine, StateLookup};

macro_rules! impl_state {
    ($nam:ident, $evts:ty) => {
        impl<'a> State<'a, $evts> for $nam {
            fn handle_event(&'a mut self, evt: Event<$evts>) -> Action<'a, $evts> {
                Action::Ignore
            }
        }
    }
}

// #[test]
// fn event_enum() {
//     let events = [Event::Enter, Event::Exit, Event::User(5u8)];
//     for ev in & events[..] {
//         print!("{:?}    ",  ev);
//     }
//     println!("");
// }
//
// #[test]
// fn event_enum_complex() {
//     #[derive(Debug)]
//     enum MyEvent<T> {
//         Tere,
//         HeadAega,
//         Number(T)
//     }
//     let events = [Event::Enter, Event::Exit, Event::User(MyEvent::Tere), Event::User(MyEvent::Number(4i16))];
//     for ev in & events[..] {
//         print!("{:?}    ",  ev);
//     }
//     println!("");
// }
//
// #[test]
// fn state() {
//     struct MyState<T> {
//         _phantom: ::std::marker::PhantomData<T>
//     }
//     impl<T> Initializer for MyState<T> {
//         fn new() -> Self {
//             MyState {_phantom : ::std::marker::PhantomData}
//         }
//     }
//     impl<'a, T> State<'a, T> for MyState<T> {
//         fn handle_event(&'a mut self, evt: Event<T>) -> Action<'a, T> {
//             Action::Ignore
//         }
//     }
//     let mut st = MyState::new();
//     println!("{}", st.handle_event(Event::User(4u8)));
// }
//
// #[test]
// fn state_macro() {
//     new_state!{MyState};
//     impl<'a, T> State<'a, T> for MyState<T> {
//         fn handle_event(&'a mut self, evt: Event<T>) -> Action<'a, T> {
//             Action::Ignore
//         }
//     }
//     let mut st = MyState::new();
//     println!("{}", st.handle_event(Event::User(4u8)));
// }
// #[test]
// fn state_macro_complex() {
//     new_state!{
//         MyState {
//             tere : u8 : 5,
//             hei  : u8 : 9
//         }
//     };
//     impl<'a, T> State<'a, T> for MyState<T> {
//         fn handle_event(&'a mut self, evt: Event<T>) -> Action<'a, T> {
//             Action::Ignore
//         }
//     }
//     let mut st = MyState::new();
//     println!("{}", st.handle_event(Event::User(4u8)));
// }
#[test]
fn state_enum_and_struct_macro() {
    #[derive(Debug)]
    enum MyEvent<T> {
        Tere,
        HeadAega,
        Number(T)
    }
    new_state!{Hei, MyEvent<u8>};
    impl_state!(Hei, MyEvent<u8>);
    new_state!{Hoo, MyEvent<u8>};
    impl_state!(Hoo, MyEvent<u8>);
    create_state_enum_and_struct!{StateEnum, StateStruct, MyEvent<u8>, (Hei, Hoo)};
}
// #[test]
// fn state_machine_init() {
//     #[derive(Debug)]
//     enum MyEvent<T> {
//         Tere,
//         HeadAega,
//         Number(T)
//     }
//     new_state!{Hei};
//     impl_state!(Hei, MyEvent<u8>);
//     new_state!{Hoo};
//     impl_state!(Hoo, MyEvent<u8>);
//     create_state_enum_and_struct!{StateEnum, StateStruct, MyEvent<u8>, (Hei, Hoo)};
//     StateMachine::<MyEvent<u8>, StateEnum, StateStruct>::new(StateEnum::Hei);
// }
