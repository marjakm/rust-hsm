macro_rules! impl_display {
    (enum $nam:ident { $( $s:ident ),* }) => {
        #[derive(Debug)]
        enum $nam { $( $s ),* }
        impl ::std::fmt::Display for $nam {
            fn fmt(&self, f:&mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                match *self {
                    $( $nam::$s => ::std::fmt::Display::fmt(stringify!($s), f) ),*
                };
                Ok(())
            }
        }
    }
}

macro_rules! new_state {
    ($nam:ident, $typ:ty) => {
        struct $nam {
            _phantom: ::std::marker::PhantomData<$typ>
        }
        impl Initializer for $nam {
            fn new() -> Self {
                $nam {_phantom : ::std::marker::PhantomData}
            }
        }
    };
    ($nam:ident { $($field_name:ident: $field_type:ty: $field_default:expr),* }) => {
        struct $nam<T> {
            _phantom       : ::std::marker::PhantomData<T>,
            $( $field_name : $field_type ),*
        }
        impl<T> Initializer for $nam<T> {
            fn new() -> Self {
                $nam {
                    _phantom       : ::std::marker::PhantomData,
                    $( $field_name : $field_default ),*
                }
            }
        }
    }
}

macro_rules! create_state_enum_and_struct {
    ($st_en:ident, $st_str:ident, $st_evt:ty, ( $($s:ident),* ) ) => {
        impl_display!(
            enum $st_en {
                $( $s ),*
            }
        )
        struct $st_str {
            $( $s : $s ),*
        }
        impl Initializer for $st_str {
            fn new() -> Self {
                $st_str {
                    $( $s : $s::new() ),*
                }
            }
        }
        impl StateLookup<$st_en, $st_evt> for $st_str {
            fn lookup(&mut self, typ: &$st_en) -> &mut State<$st_evt> {
                match *typ {
                    $($st_en::$s => &mut self.$s ),*
                }
            }
        }
    }
}
