#[macro_export]
macro_rules! functions {
    ($lib_name: ident; $($fn_name: ident($($args: ty),*) -> $fn_body: expr $(,$is_const: expr)?);+) => {
            use crate::error::SimdevalError;
            use crate::evaluate::enums::Value;
            use crate::evaluate::function::Function;
            use crate::evaluate::node::Node;
            use std::slice::Iter;
            #[allow(non_camel_case_types)]
            enum $lib_name {
                $($fn_name),+
            }
            impl Function<$lib_name> for $lib_name {
                const NAMESPACE: &'static str = stringify!($ty);
                const MAX_ARGS: u8 = 4;
                fn from_string(
                    namespaces: &mut Iter<&str>,
                    identifier: &str,
                ) -> Result<$lib_name, crate::error::SimdevalError> {
                    Err(crate::error::SimdevalError::UnexpectedToken)
                }
                fn call(&self, node: &[Value]) -> Value {
                    match self {
                        $($lib_name::$fn_name => {let node = node; $fn_body},)+
                    }
                }
                fn is_const(&self) -> bool {
                    true
                }
            }
    };
}
