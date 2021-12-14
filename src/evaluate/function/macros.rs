#[macro_export]
macro_rules! impl_functions {
    ($lib_name: ident: $lib_namespace: ident; [$($import: ty: $import_name: ident),*]; [$($func_name: ident: $func: ident, $arg_count: expr),+]) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone)]
        enum $lib_name {
            $($func_name),+,
            $($import_name($import)),+
        }
        impl crate::evaluate::function::Function<$lib_name> for $lib_name {
            const NAMESPACE: &'static str = stringify!($lib_namespace);
            const MAX_ARGS: u8 = 4;
            fn from_string(
                namespaces: &mut std::slice::Iter<&str>,
                identifier: &str,
            ) -> Result<$lib_name, crate::error::SimdevalError> {
                if let Some(&namespace) = dbg!(namespaces.next()) {
                    // let namespace =  &namespace[0..namespace.len()-1];
                    Ok(match namespace {
                        $(<$import>::NAMESPACE => $lib_name::$import_name(<$import>::from_string(namespaces, identifier)?),)+
                        Self::NAMESPACE => Self::from_string(namespaces, identifier)?,
                        _ => return Err(crate::error::SimdevalError::InvalidNamespace)
                    })
                } else {
                    Ok(match dbg!(identifier) {
                        $(stringify!($func) => $lib_name::$func_name,)+
                        _ => return  Err(crate::error::SimdevalError::UnexpectedToken)
                    })
                }
            }
            fn call(&self, args: &[crate::evaluate::value::single::Value]) -> Result<crate::evaluate::value::single::Value, crate::error::SimdevalError> {
                Ok(match self {
                    $($lib_name::$func_name => { if args.len() == $arg_count {$func(args.try_into()?)} else {return Err(crate::error::SimdevalError::InvalidArgs)}},)+
                    $($lib_name::$import_name(i) => i.call(args)?,)+
                })
            }
            fn is_const(&self) -> bool {
                true
            }
        }
    }
}
