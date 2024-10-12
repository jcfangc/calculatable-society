#[macro_export]
macro_rules! enum_map {
    ($(#[$meta:meta])* $vis:vis $name:ident => $value_type:ty { $($key:ident => $value:expr),* $(,)? }) => {
        $(#[$meta])*
        #[derive(Debug, PartialEq, Eq, Hash)]
        $vis enum $name {
            $($key),*
        }

        use std::collections::HashMap;
        use once_cell::sync::Lazy;
        impl $name {
            $vis fn to_map() -> &'static HashMap<$name, $value_type> {
                static MAP: Lazy<HashMap<$name, $value_type>> = Lazy::new(
                    || {
                        let mut map = HashMap::new();
                        $(map.insert($name::$key, $value);)*
                        map
                    }
                );
                &MAP
            }
        }
    };
}
