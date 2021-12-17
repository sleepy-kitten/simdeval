#[macro_export]
macro_rules! impl_functions {
    ($lib: ident: $lib_namespace: ident; [$($import: ty: $import_namespace: ident),*]; [$($func_name: ident: $func: ident($arg_count: expr) $(;$is_const: expr)?),+]) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone)]
        pub enum $lib<const LANES: usize>
        where LaneCount<LANES>: SupportedLaneCount {
            $($func_name),+,
            $($import_namespace($import)),*
        }
        impl<const LANES: usize> $crate::evaluate::function::Function<$lib<LANES>, LANES> for $lib<LANES>
        where LaneCount<LANES>: SupportedLaneCount{
            const NAMESPACE: &'static str = stringify!($lib_namespace);
            const MAX_ARGS: usize = $crate::biggest!($($arg_count),+);
            fn from_string(
                namespaces: &mut std::slice::Iter<&str>,
                identifier: &str,
            ) -> Result<$lib<LANES>, $crate::error::SimdevalError> {
                if let Some(&namespace) = dbg!(namespaces.next()) {
                    Ok(match namespace {
                        $(<$import>::NAMESPACE => $lib::$import_namespace(<$import>::from_string(namespaces, identifier)?),)*
                        Self::NAMESPACE => Self::from_string(namespaces, identifier)?,
                        _ => return Err($crate::error::SimdevalError::InvalidNamespace)
                    })
                } else {
                    Ok(match dbg!(identifier) {
                        $(stringify!($func) => $lib::$func_name,)+
                        _ => return  Err($crate::error::SimdevalError::UnexpectedToken)
                    })
                }
            }
            fn call(&self, args: &[$crate::evaluate::value::Value<LANES>]) -> Result<$crate::evaluate::value::Value<LANES>, $crate::error::SimdevalError> {
                Ok(match self {
                    $($lib::$func_name => { if args.len() == $arg_count {$func(args.try_into()?)} else {return Err($crate::error::SimdevalError::InvalidArgs)}},)+
                    $($lib::$import_namespace(i) => i.call(args)?,)*
                })
            }
            fn is_const(&self) -> bool {
                match self {
                    $($lib::$import_namespace(i) => i.is_const(),)*
                    $($lib::$func_name => $crate::empty_or_input!($($is_const)?),)*
                }
            }
        }
    }
}
#[macro_export]
macro_rules! impl_functions_test {
    ($lib: ident: $lib_namespace: ident; [$($import: ty: $import_namespace: ident),*]; [$($func: ident($arg_count: expr)$body: block),*]) => {
        $(fn $func(args: &[$crate::evaluate::value::single::Value; $arg_count]) -> Value {$body})*
        #[allow(non_camel_case_types)]
        #[derive(Clone, Debug)]
        enum $lib {
            $($import_namespace($import),)+
            $($func,)*
        }
        impl $crate::evaluate::function::Function<$lib, const LANES: usize> for $lib<LANES> {
            const NAMESPACE: &'static str = stringify!($lib_namespace);
            const MAX_ARGS: usize = biggest!($($arg_count),*);
            fn from_string(
                namespaces: &mut std::slice::Iter<&str>,
                identifier: &str,
            ) -> Result<$lib<LANES>, $crate::error::SimdevalError> {
                if let Some(&namespace) = dbg!(namespaces.next()) {
                    Ok(match namespace {
                        $(<$import>::NAMESPACE => $lib::$import_namespace(<$import>::from_string(namespaces, identifier)?),)+
                        Self::NAMESPACE => Self::from_string(namespaces, identifier)?,
                        _ => return Err($crate::error::SimdevalError::InvalidNamespace)
                    })
                } else {
                    Ok(match dbg!(identifier) {
                        $(stringify!($func) => $lib::$func,)*
                        _ => return  Err($crate::error::SimdevalError::UnexpectedToken)
                    })
                }
            }
            fn call(&self, args: &[$crate::evaluate::value::single::Value]) -> Result<$crate::evaluate::value::single::Value, $crate::error::SimdevalError> {
                Ok(match self {
                    $($lib::$func => { if args.len() == $arg_count {$func(args.try_into()?)} else {return Err($crate::error::SimdevalError::InvalidArgs)}},)*
                    $($lib::$import_namespace(i) => i.call(args)?,)+
                })
            }
            fn is_const(&self) -> bool {
                true
            }
        }
    };
}
#[macro_export]
macro_rules! impl_functions_test2 {
    ($lib: ident: $lib_namespace: ident; [$($import: ty: $import_namespace: ident),*]; [$($func: ident($arg_count: expr)$body: block),*]) => {
        $(fn $func(args: &[$crate::evaluate::value::single::Value; $arg_count]) -> Value {$body})*
        #[allow(non_camel_case_types)]
        #[derive(Clone, Debug)]
        enum $lib {
            $($import_namespace($import),)+
            $($func,)*
        }
        impl $crate::evaluate::function::Function<$lib> for $lib {
            const NAMESPACE: &'static str = stringify!($lib_namespace);
            const MAX_ARGS: usize = biggest!($($arg_count),*);
            fn from_string(
                namespaces: &mut std::slice::Iter<&str>,
                identifier: &str,
            ) -> Result<$lib, $crate::error::SimdevalError> {
                if let Some(&namespace) = dbg!(namespaces.next()) {
                    Ok(match namespace {
                        $(<$import>::NAMESPACE => $lib::$import_namespace(<$import>::from_string(namespaces, identifier)?),)+
                        Self::NAMESPACE => Self::from_string(namespaces, identifier)?,
                        _ => return Err($crate::error::SimdevalError::InvalidNamespace)
                    })
                } else {
                    Ok(match dbg!(identifier) {
                        $(stringify!($func) => $lib::$func,)*
                        _ => return  Err($crate::error::SimdevalError::UnexpectedToken)
                    })
                }
            }
            fn call(&self, args: &[$crate::evaluate::value::single::Value]) -> Result<$crate::evaluate::value::single::Value, $crate::error::SimdevalError> {
                Ok(match self {
                    $($lib::$func => { if args.len() == $arg_count {$func(args.try_into()?)} else {return Err($crate::error::SimdevalError::InvalidArgs)}},)*
                    $($lib::$import_namespace(i) => i.call(args)?,)+
                })
            }
            fn is_const(&self) -> bool {
                true
            }
        }
    };
}
#[macro_export]
macro_rules! biggest {
    ($first: expr, $second: expr, $($rest: expr),*) => {
        if $first > $second {
            $crate::biggest!($first, $($rest),+)
        } else {
            $crate::biggest!($second, $($rest),+)
        }
    };
    ($first: expr, $second: expr) => {
        if $first > $second {
            $first
        } else {
            $second
        }
    };
    ($first: expr) => {
        $first
    };
    () => {
        0
    };
}
#[macro_export]
macro_rules! empty_or_input {
    () => {
        true
    };
    ($expr: expr) => {
        $expr
    };
}
