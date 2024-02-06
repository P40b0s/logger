

#[macro_export(local_inner_macros)]
///ПУть к файлу/имя функции/строка с ошибкой <br>
///пример: [structure/tests/tests.rs/hash_test:97]
macro_rules! backtrace
{   
    () =>
    {{
        std::format!("🔍[{}/{}():{}]", std::file!(), function_name!(), std::line!())
    }};
}

#[macro_export(local_inner_macros)]
macro_rules! function_name 
{
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str 
        {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') 
        {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

#[macro_export(local_inner_macros)]
macro_rules! log {
    // log!(target: "my_target", Level::Info; key1 = 42, key2 = true; "a {} event", "log");
    (target: $target:expr, $lvl:expr, $($key:tt = $value:expr),+; $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= $crate::STATIC_MAX_LEVEL && lvl <= $crate::max_level() {
            $crate::__private_api_log(
                __log_format_args!($($arg)+),
                lvl,
                &($target, __log_module_path!(), __log_file!(), __log_line!()),
                $crate::__private_api::Option::Some(&[$((__log_key!($key), &$value)),+])
            );
        }
    });

    // log!(target: "my_target", Level::Info; "a {} event", "log");
    (target: $target:expr, $lvl:expr, $($arg:tt)+) => ({
        let lvl = $lvl;
        if lvl <= $crate::STATIC_MAX_LEVEL && lvl <= $crate::max_level() {
            $crate::__private_api_log(
                __log_format_args!($($arg)+),
                lvl,
                &($target, __log_module_path!(), __log_file!(), __log_line!()),
                $crate::__private_api::Option::None,
            );
        }
    });

    // log!(Level::Info, "a log event")
    ($lvl:expr, $($arg:tt)+) => (log!(target: __log_module_path!(), function_name!(), file!(), line!(), $lvl, $($arg)+));
}
