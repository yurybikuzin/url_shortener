#[macro_export]
macro_rules! declare_env_settings {
    (
        $( $config_fields:ident : $config_types:ty, )*
    ) => {
        common_macros::declare_env_settings!(@parse {}, {}, {};
            $( $config_fields : $config_types, )*
        );
    };
    (@parse
        { $($config_body:tt)* },
        { $($init_body:tt)* },
        { $($field_list_body:tt)* };

        $config_field:ident : $config_type:ty,
        $( $config_fields:ident : $config_types:ty, )*
    ) => {
        paste::paste! {
            common_macros::declare_env_settings!(@parse
                { $($config_body)*
                    pub $config_field: $config_type,
                },
                { $($init_body)*
                    common_macros::declare_env_settings!(@let_from_env $config_field, [< $config_field:upper >], $config_type);
                },
                { $($field_list_body)*
                    $config_field,
                };
                $( $config_fields : $config_types, )*
            );
        }
    };
    (@parse
        { $($config_body:tt)* },
        { $($init_body:tt)* },
        { $($field_list_body:tt)* };
    ) => {
        #[derive(Clone, Debug, Default)]
        pub struct EnvSettings {
            $($config_body)*
        }
        impl EnvSettings {
            pub fn init() -> Result<Self> {
                $($init_body)*
                Ok(Self {
                    $($field_list_body)*
                })
            }
        }
        lazy_static::lazy_static! {
            pub static ref ENV_SETTINGS: std::sync::RwLock<EnvSettings> = std::sync::RwLock::new({
                EnvSettings::init().unwrap()
            });
            // pub static ref OP_MODE: RwLock<OpMode> = RwLock::new(OpMode::default());
        }
    };
    (@let_from_env $var: ident, $env: ident) => {
        let $var =
            std::env::var(stringify!($env)).context(format!("{} required", stringify!($env)))?;
    };
    (@let_from_env $var: ident, $env: ident, String) => {
        common_macros::declare_env_settings!(@let_from_env $var, $env);
    };
    (@let_from_env $var: ident, $env: ident, $type: ty) => {
        common_macros::declare_env_settings!(@let_from_env $var, $env);
        let $var = $var.parse::<$type>().context(format!(
            "{}: failed to {:?}.parse::<{}>",
            stringify!($env),
            $var,
            stringify!($type),
        ))?;
    };
    (@let_from_env expect => $var: ident, $env: ident) => {
        let $var = std::env::var(stringify!($env))
            .expect(&format!("{} required", stringify!($env)));
    };
    (@let_from_env expect => $var: ident, $env: ident, $type: ty) => {
        common_macros::declare_env_settings!(@let_from_env expect => $var, $env);
        let $var = $var.parse::<$type>().expect(&format!(
            "{}: failed to {:?}.parse::<{}>",
            stringify!($env),
            $var,
            stringify!($type),
        ));
    };
}

#[macro_export]
macro_rules! declare_env_settings_for_server {
    (
        $( $config_fields:ident : $config_types:ty, )*
    ) => {
        common_macros::declare_env_settings!(@parse {}, {}, {};
            $( $config_fields : $config_types, )*
            port: u16,
            port_dev: u16,
            port_demo: u16,
            port_rc: u16,
            port_local: u16,
        );
        lazy_static::lazy_static! {
            pub static ref OP_MODE: std::sync::RwLock<op_mode::OpMode> = std::sync::RwLock::new(op_mode::OpMode::default());
        }
        impl EnvSettings {
            pub fn set_port(port: &Option<u16>) {
                let op_mode = *OP_MODE.read().unwrap();
                match op_mode {
                    op_mode::OpMode::Prod => {
                        common_macros::env_settings!(port = *port);
                    }
                    op_mode::OpMode::Dev => {
                        common_macros::env_settings!(port_dev = *port);
                    }
                    op_mode::OpMode::Demo => {
                        common_macros::env_settings!(port_demo = *port);
                    }
                    op_mode::OpMode::Rc => {
                        common_macros::env_settings!(port_rc = *port);
                    }
                    op_mode::OpMode::Local => {
                        common_macros::env_settings!(port_local = *port);
                    }
                }
            }
            pub fn port() -> u16 {
                let op_mode = *OP_MODE.read().unwrap();
                match op_mode {
                    op_mode::OpMode::Prod => {
                        common_macros::env_settings!(port)
                    }
                    op_mode::OpMode::Dev => {
                        common_macros::env_settings!(port_dev)
                    }
                    op_mode::OpMode::Demo => {
                        common_macros::env_settings!(port_demo)
                    }
                    op_mode::OpMode::Rc => {
                        common_macros::env_settings!(port_rc)
                    }
                    op_mode::OpMode::Local => {
                        common_macros::env_settings!(port_local)
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! env_settings {
    ($field:ident) => {
        (*ENV_SETTINGS.read().unwrap()).$field
    };
    ($name:ident = $from:expr) => {
        paste::paste!{
            if let Some($name) = $from {
                if common_macros::env_settings!($name) != $name {
                    warn!(
                        concat!("will use ", stringify!($name), " {:?} by opt instead of ", stringify!([< $name:upper >])," = {:?} in .env"),
                        $name,
                        common_macros::env_settings!($name)
                    );
                    (*ENV_SETTINGS.write().unwrap()).$name = $name;
                }
            };
        }
    }
}
