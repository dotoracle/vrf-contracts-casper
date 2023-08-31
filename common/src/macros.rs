#[macro_export]
macro_rules! get_set {
    ($name: ident, $arg_name: expr, $t: ty, $default: expr, $save: ident, $read: ident, $get: ident, $set: ident) => {
        pub fn $save($name: $t) {
            helpers::set_key($arg_name, $name);
        }

        pub fn $read() -> $t {
            helpers::get_key("payment_token").unwrap_or($default)
        }

        #[no_mangle]
        pub extern "C" fn $get() {
            runtime::ret(CLValue::from_t($read()).unwrap_or_revert())
        }

        #[no_mangle]
        pub extern "C" fn $set() {
            common::owner::only_owner();
            let $name: Key = runtime::get_named_arg($arg_name);
            $save($name);
        }
    };
}

#[macro_export]
macro_rules! get_set_no_set {
    ($name: ident, $arg_name: expr, $t: ty, $default: expr, $save: ident, $read: ident, $get: ident) => {
        pub fn $save($name: $t) {
            helpers::set_key($arg_name, $name);
        }

        pub fn $read() -> $t {
            helpers::get_key("payment_token").unwrap_or($default)
        }

        #[no_mangle]
        pub extern "C" fn $get() {
            runtime::ret(CLValue::from_t($read()).unwrap_or_revert())
        }
    };
}

#[macro_export]
macro_rules! get_set_dict {
    ($dict_arg_name: expr, $key_name: expr, $t: ty, $v: ty, $default: expr, $save: ident, $read: ident, $get: ident, $set: ident) => {
        pub fn $save(k: &$t, v: &$v) {
            helpers::write_dictionary_value_from_key(
                $dict_arg_name,
                &helpers::encode_key(&helpers::encode_1(k)),
                v.clone(),
            );
        }

        pub fn $read(k: &$t) -> $v {
            helpers::get_dictionary_value_from_key(
                $dict_arg_name,
                &helpers::encode_key(&helpers::encode_1(k)),
            )
            .unwrap_or($default)
        }

        #[no_mangle]
        pub extern "C" fn $get() {
            let k: $t = runtime::get_named_arg($key_name);
            runtime::ret(CLValue::from_t($read(&k)).unwrap_or_revert())
        }
    };
}

#[macro_export]
macro_rules! get_set_nested_dict {
    ($dict_arg_name: expr, $key1_name: expr, $key2_name: expr, $t1: ty, $t2: ty, $v: ty, $default: expr, $save: ident, $read: ident, $get: ident, $set: ident) => {
        pub fn $save(k1: &$t1, k2: &$t2, v: &$v) {
            helpers::write_dictionary_value_from_key(
                $dict_arg_name,
                &helpers::encode_key(&helpers::encode_2(k1, k2)),
                v.clone(),
            );
        }

        pub fn $read(k1: &$t1, k2: &$t2) -> $v {
            helpers::get_dictionary_value_from_key(
                $dict_arg_name,
                &helpers::encode_key(&helpers::encode_2(k1, k2)),
            )
            .unwrap_or($default)
        }

        #[no_mangle]
        pub extern "C" fn $get() {
            let k1: $t1 = runtime::get_named_arg($key1_name);
            let k2: $t2 = runtime::get_named_arg($key2_name);
            runtime::ret(CLValue::from_t($read(&k1, &k2)).unwrap_or_revert())
        }
    };
}
