#[macro_export]
macro_rules! hsm_define_objects {
    ($st_str:ident, $st_en:ident, $st_evt:ty, $shr_dat:ident, ( $($s:ident),* ) ) => {
        _hsm_create_states!($($s),*);
        _hsm_create_state_enum!($st_en, ($($s),*));
        _hsm_create_state_struct!($st_str, $st_en, $st_evt, $shr_dat, ($($s),*) );
    };
    ($st_str:ident, $st_en:ident, $st_evt:ty, $shr_dat:ident, ( $($s:ident $x:tt),*)) => {
        _hsm_create_states!( $($s $x),* );
        _hsm_create_state_enum!($st_en, ($($s),*));
        _hsm_create_state_struct!($st_str, $st_en, $st_evt, $shr_dat, ($($s),*) );
    }
}

#[macro_export]
macro_rules! hsm_delayed_transition {
    ($probe:ident, $x:block) => {
        match $probe {
            true  => $crate::Action::DelayedTransition,
            false => $crate::Action::Transition($x)
        }
    }
}

#[macro_export]
macro_rules! hsm_impl_state {
    ($state:ident, $events:ident, $states:ident, $shr_data:ident, $($pat:pat => $result:expr),*) => {
        impl $crate::State<$events, $states, $shr_data> for $state<$events, $states, $shr_data> {
            fn handle_event(&mut self, shr_data: &mut $shr_data, evt: $crate::Event<$events>, probe: bool) -> $crate::Action<$states> {
                match evt {
                    $( $pat => $result),*
                }
            }
        }
    }
}

#[macro_export]
macro_rules! hsm_state_parents {
    ($st_en:ident ; $($nam:ident -> $parent:ident),*) => {
        $(_hsm_impl_state_parent!($st_en ; $nam -> $parent);)*
    }
}

#[macro_export]
macro_rules! _hsm_impl_state_parent {
    ($st_en:ident ; $nam:ident -> None) => {
        impl<T, F> $crate::Parent<$st_en> for $nam<T, $st_en, F> {
            fn get_parent(&self) -> Option<$st_en> { None }
        }
    };
    ($st_en:ident ; $nam:ident -> $parent:ident) => {
        impl<T, F> $crate::Parent<$st_en> for $nam<T, $st_en, F> {
            fn get_parent(&self) -> Option<$st_en> { Some($st_en::$parent) }
        }
    }
}

#[macro_export]
macro_rules! _hsm_create_states {
    ( $($s:ident),* ) => {
        $(_hsm_create_state!($s);)*
    };
    ($($s:ident $x:tt),*) => {
        $(_hsm_create_state!($s $x);)*
    }
}

#[macro_export]
macro_rules! _hsm_create_state_common {
    ($nam:ident) => {
        impl<T, E, F> $crate::Name for $nam<T, E, F> {
            fn name(&self) -> &'static str {
                stringify!($nam)
            }
        }
    }
}

#[macro_export]
macro_rules! _hsm_create_state {
    ($nam:ident) => {
        #[derive(Debug)]
        struct $nam<T, E, F> {
            _phantom_events : ::std::marker::PhantomData<T>,
            _phantom_shr_dat: ::std::marker::PhantomData<F>,
            _phantom_state  : ::std::marker::PhantomData<E>,
        }
        impl<T, E, F> $crate::Initializer for $nam<T, E, F> {
            fn new() -> Self {
                $nam {
                    _phantom_events : ::std::marker::PhantomData,
                    _phantom_shr_dat: ::std::marker::PhantomData,
                    _phantom_state  : ::std::marker::PhantomData,
                }
            }
        }
        _hsm_create_state_common!($nam);
    };
    ($nam:ident { $($field_name:ident : $field_type:ty = $field_default:expr),* }) => {
        #[derive(Debug)]
        struct $nam<T, E, F> {
            _phantom_events : ::std::marker::PhantomData<T>,
            _phantom_shr_dat: ::std::marker::PhantomData<F>,
            _phantom_state  : ::std::marker::PhantomData<E>,
            $( $field_name  : $field_type ),*
        }
        impl<T, E, F> $crate::Initializer for $nam<T, E, F> {
            fn new() -> Self {
                $nam {
                    _phantom_events : ::std::marker::PhantomData,
                    _phantom_shr_dat: ::std::marker::PhantomData,
                    _phantom_state  : ::std::marker::PhantomData,
                    $( $field_name  : $field_default ),*
                }
            }
        }
        _hsm_create_state_common!($nam);
    }
}

#[macro_export]
macro_rules! _hsm_create_state_enum {
    ($st_en:ident, ($($s:ident),*) ) => {
        #[derive(Debug, Clone, Eq, PartialEq)]
        enum $st_en {
            $( $s ),*
        }
        impl ::std::fmt::Display for $st_en {
            fn fmt(&self, f:&mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                match *self {
                    $( $st_en::$s => try!(::std::fmt::Display::fmt(stringify!($s), f)) ),*
                };
                Ok(())
            }
        }
    }
}

#[macro_export]
macro_rules! _hsm_create_state_struct {
    ($st_str:ident, $st_en:ident, $st_evt:ty, $shr_dat:ident, ($($s:ident),*) ) => {
        #[derive(Debug)]
        #[allow(non_snake_case)]
        struct $st_str {
            $( $s : $s<$st_evt, $st_en, $shr_dat> ),*
        }
        impl $crate::Initializer for $st_str {
            fn new() -> Self {
                $st_str {
                    $( $s : $s::new() ),*
                }
            }
        }
        impl $crate::StateLookup<$st_en, $st_evt, $shr_dat> for $st_str {
            fn lookup(&mut self, typ: &$st_en) -> &mut $crate::State<$st_evt, $st_en, $shr_dat> {
                match *typ {
                    $($st_en::$s => &mut self.$s ),*
                }
            }
        }
    }
}
