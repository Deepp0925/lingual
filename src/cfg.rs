/// generates the sync version of the function if given the async version of the function.
/// it will tranverse the function and remove the async keyword and the .await keyword.
/// and also add the blocking feature to the function.
/// # Example
/// ```
/// cfg_gen_blocking! {
///     async fn hello() -> String {
///         "hello".to_string()
///     }
///
///    async fn send_request() -> String {
///         let response = reqwest::get("https://www.google.com").await.unwrap();
///         response.text().await.unwrap()
///     }
/// }
///
/// // will generate this code
///
///
/// #[cfg(any(target_arch = "wasm32", not(feature = "blocking")))]
/// async fn hello() -> String {
///     "hello".to_string()
/// }
///
/// #[cfg(all(not(target_arch = "wasm32"), feature = "blocking"))]
/// fn hello() -> String {
///     "hello".to_string()
/// }
///
/// #[cfg(any(target_arch = "wasm32", not(feature = "blocking")))]
/// async fn send_request() -> String {
///     let response = reqwest::get("https://www.google.com").await.unwrap();
///    response.text().await.unwrap()
/// }
/// #[cfg(all(not(target_arch = "wasm32"), feature = "blocking"))]
/// fn send_request() -> String {
///     let response = reqwest::blocking::get("https://www.google.com").unwrap();
///     response.text().unwrap()
/// }
///```
///
#[macro_export]
macro_rules! cfg_gen_blocking  {
    ($($item:item)*) => {
        $(
            #[cfg_attr(all(not(target_arch = "wasm32"), feature = "blocking"), remove_async_await::remove_async_await)]
            // adds docs to the generated function
            $item

        )*
    }
}

/// Adds the cfg "default" feature any item that is wrapped in this macro will be added to the
/// # Example
/// ```
/// cfg_non_blocking! {
///   async fn hello() -> String {
///     "hello".to_string()
///    }
/// }
///
/// // will generate this code
///
/// #[cfg(any(target_arch = "wasm32", not(feature = "blocking")))]
/// async fn hello() -> String {
///     "hello".to_string()
/// }
/// ```
#[macro_export]
macro_rules! cfg_non_blocking {
    ($($item:item)*) => {
        $(
            #[cfg(any(target_arch = "wasm32", not(feature = "blocking")))]
            $item

        )*
    }
}

/// adds the cfg "blocking" feature to any item that is wrapped in this macro.
/// # Example
/// ```
/// cfg_blocking! {
///     fn hello() -> String {
///         "hello".to_string()
///     }
///
///     static HELLO: String = "hello".to_string();
/// }
///
/// // will generate this code
///
/// #[cfg(all(not(target_arch = "wasm32"), feature = "blocking"))]
/// fn hello() -> String {
///     "hello".to_string()
/// }
///
/// #[cfg(all(not(target_arch = "wasm32"), feature = "blocking"))]
/// static HELLO: String = "hello".to_string();
///
/// ```
#[macro_export]
macro_rules! cfg_blocking {
    ($($item:item)*) => {
        $(
            #[cfg(all(not(target_arch = "wasm32"), feature = "blocking"))]
            $item

        )*
    }
}

/// To test this expand the macro using rust analyzer's macro expansion feature while having the
/// cfg_gen_blocking! macro selected.
fn _cfg_remove_async() {
    cfg_gen_blocking! {
        async fn hello(_t: String) -> String {
            "hello".to_string()
        }
    }
}
