
pub mod declare_env_settings;
pub mod declare_settings;
pub mod pasitos;
pub mod will_did;

#[macro_export]
macro_rules! r#impl {
    (FromStr for $type:ty; strum) => {
        impl std::str::FromStr for $type {
            type Err = anyhow::Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use strum::IntoEnumIterator;
                if let Some(found) = Self::iter().find(|i| {
                    let eta = i.to_string();
                    let mut eta_iter = eta.chars();
                    let mut tst_iter = s.chars();
                    // let mut ret = true;
                    loop {
                        let eta = eta_iter.next();
                        let tst = tst_iter.next();
                        if eta.is_some() & tst.is_some() {
                            let eta = eta.unwrap();
                            let tst = tst.unwrap();
                            if !(eta == tst || match (eta, tst) { 
                                ('С', 'C') => true,
                                (_, _) => false,
                            }) { break false; }
                        } else if eta.is_none() && tst.is_none() {
                            break true;
                        } else {
                            break false;
                        }
                    }
                }) {
                    Ok(found)
                } else {
                    Err(anyhow!(
                        "failed {}::from_str({:?}): valid values: {}",
                        stringify!($type),
                        s,
                        Self::iter()
                            .map(|i| format!("{:?}", i.to_string()))
                            .collect::<Vec<String>>()
                            .join(", ")
                    ))
                }
            }
        }
    };
}

#[macro_export]
macro_rules! entry {
    ($hash_map:expr, $key:expr => 
         and_modify |$e:ident| $occupied:block 
         or_insert $vacant:expr 
    ) => {
        match $hash_map.entry($key) {
            std::collections::hash_map::Entry::Occupied(mut $e) => {
                let $e = $e.get_mut();
                $occupied;
            }
            std::collections::hash_map::Entry::Vacant($e) => {
                #[allow(unreachable_code)]
                $e.insert($vacant);
            }
        }
    };
    ($hash_map:expr, $key:expr => 
         and_modify_entry |$e:ident| $occupied:block 
         or_insert_opt $vacant:expr 
    ) => {
        match $hash_map.entry($key) {
            std::collections::hash_map::Entry::Occupied(mut $e) => {
                $occupied;
            }
            std::collections::hash_map::Entry::Vacant($e) => {
                if let Some(v) = $vacant {
                    $e.insert(v);
                }
            }
        }
    };
    ($hash_map:expr, $key:expr => 
         and_modify_entry |$e:ident| $occupied:block 
         or_insert $vacant:expr 
    ) => {
        match $hash_map.entry($key) {
            std::collections::hash_map::Entry::Occupied(mut $e) => {
                $occupied;
            }
            std::collections::hash_map::Entry::Vacant($e) => {
                #[allow(unreachable_code)]
                $e.insert($vacant);
            }
        }
    };
}

