#[macro_export]
macro_rules! hsm_define_objects {
    ($st_str:ident, $st_en:ident, $st_evt:ty, ( $($s:ident),* ) ) => {
        _hsm_create_states!($($s),*);
        _hsm_create_state_enum!($st_en, ($($s),*));
        _hsm_create_state_struct!($st_str, $st_en, $st_evt, ($($s),*) );
    }
}

#[macro_export]
macro_rules! _hsm_create_states {
    ( $($s:ident),* ) => {
        $(_hsm_create_state!($s);)*
    }
}

#[macro_export]
macro_rules! _hsm_create_state {
    ($nam:ident) => {
        #[derive(Debug)]
        struct $nam<T> {
            _phantom: ::std::marker::PhantomData<T>
        }
        impl<T> $crate::Initializer for $nam<T> {
            fn new() -> Self {
                $nam {_phantom : ::std::marker::PhantomData}
            }
        }
        impl<T> $crate::Name for $nam<T> {
            fn name(&self) -> &'static str {
                stringify!($nam)
            }
        }
    };
    ($nam:ident, { $($field_name:ident: $field_type:ty: $field_default:expr),* }) => {
        #[derive(Debug)]
        struct $nam<T> {
            _phantom       : ::std::marker::PhantomData<T>,
            $( $field_name : $field_type ),*
        }
        impl<T> $crate::Initializer for $nam<T> {
            fn new() -> Self {
                $nam {
                    _phantom       : ::std::marker::PhantomData,
                    $( $field_name : $field_default ),*
                }
            }
        }
        impl<T> $crate::Name for $nam<T> {
            fn name(&self) -> &'static str {
                stringify!($nam)
            }
        }
    }
}

#[macro_export]
macro_rules! _hsm_create_state_enum {
    ($st_en:ident, ($($s:ident),*) ) => {
        #[derive(Debug)]
        enum $st_en {
            $( $s ),*
        }
        impl ::std::fmt::Display for $st_en {
            fn fmt(&self, f:&mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                match *self {
                    $( $st_en::$s => ::std::fmt::Display::fmt(stringify!($s), f) ),*
                };
                Ok(())
            }
        }
    }
}

#[macro_export]
macro_rules! _hsm_create_state_struct {
    ($st_str:ident, $st_en:ident, $st_evt:ty, ($($s:ident),*) ) => {
        #[derive(Debug)]
        #[allow(non_snake_case)]
        struct $st_str {
            $( $s : $s<$st_evt> ),*
        }
        impl $crate::Initializer for $st_str {
            fn new() -> Self {
                $st_str {
                    $( $s : $s::new() ),*
                }
            }
        }
        impl $crate::StateLookup<$st_en, $st_evt> for $st_str {
            fn lookup(&mut self, typ: &$st_en) -> &mut $crate::State<$st_evt> {
                match *typ {
                    $($st_en::$s => &mut self.$s ),*
                }
            }
        }
    }
}
