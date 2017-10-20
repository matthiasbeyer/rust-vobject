use std::collections::BTreeMap;

pub type Parameters = BTreeMap<String, String>;

#[macro_export]
macro_rules! parameters(
    { $($key:expr => $value:expr),* } => {
        #[allow(unused_mut)]
        {
            let mut m : ::std::collections::BTreeMap<String, String> =
                ::std::collections::BTreeMap::new();
            $( m.insert($key.into(), $value.into()); )*
            m
        }
     };
);

#[macro_export]
macro_rules! make_getter_function_for_optional {
    ($fnname:ident, $name:expr, $mapper:ty) => {
        pub fn $fnname(&self) -> Option<$mapper> {
            self.0.get_only($name).cloned().map(From::from)
        }
    }
}

#[macro_export]
macro_rules! make_getter_function_for_values {
    ($fnname:ident, $name:expr, $mapper:ty) => {
        pub fn $fnname(&self) -> Vec<$mapper> {
            self.0
                .get_all($name)
                .iter()
                .map(Clone::clone)
                .map(From::from)
                .collect()
        }
    }
}

#[macro_export]
macro_rules! create_data_type {
    ( $name:ident ) => {
        #[derive(Eq, PartialEq)]
        pub struct $name(String, Parameters);

        impl $name {
            fn new(raw: String, params: Parameters) -> $name {
                $name(raw, params)
            }

            pub fn raw(&self) -> &String {
                &self.0
            }
        }

        impl From<Property> for $name {
            fn from(p: Property) -> $name {
                $name::new(p.raw_value, p.params)
            }
        }
    }
}

macro_rules! make_builder_fn {
    (
        fn $fnname:ident building $property_name:tt with_params,
        $mapfn:expr => $( $arg_name:ident : $arg_type:ty ),*
    ) => {
        pub fn $fnname(mut self, params: util::Parameters, $( $arg_name : $arg_type ),*) -> Self {
            let raw_value = vec![ $( $arg_name ),* ]
                .into_iter()
                .map($mapfn)
                .collect::<Vec<_>>()
                .join(";");

            let mut prop = Property::new(String::from($property_name), raw_value);
            prop.params = params;
            self.0.props.entry(String::from($property_name)).or_insert(vec![]).push(prop);
            self
        }
    };

    (
        fn $fnname:ident building $property_name:tt,
        $mapfn:expr => $( $arg_name:ident : $arg_type:ty ),*
    ) => {
        pub fn $fnname(mut self, $( $arg_name : $arg_type ),*) -> Self {
            let raw_value = vec![ $( $arg_name ),* ]
                .into_iter()
                .map($mapfn)
                .collect::<Vec<_>>()
                .join(";");

            let prop = Property::new(String::from($property_name), raw_value);
            self.0.props.entry(String::from($property_name)).or_insert(vec![]).push(prop);
            self
        }
    }
}

