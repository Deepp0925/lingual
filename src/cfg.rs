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

macro_rules! cfg_gen_blocking  {
    ($($item:item)*) => {
        $(
            #[cfg(any(target_arch = "wasm32", not(feature = "blocking")))]
            $item

            #[cfg(all(not(target_arch = "wasm32"), feature = "blocking"))]
            cfg_remove_async! {
                $item
            }
        )*
    }
}

/// removes the async keyword and the .await keyword from the function.
/// # Example
/// ```
/// cfg_remove_async! {
///     async fn hello() -> String {
///         "hello".to_string()
///     }
///
///     async fn send_request() -> String {
///         let response = reqwest::get("https://www.google.com").await.unwrap();
///         response.text().await.unwrap()
///     }
/// }
///
/// // will generate this code
///
/// fn hello() -> String {
///     "hello".to_string()
/// }
///
/// fn send_request() -> String {
///     let response = reqwest::get("https://www.google.com").unwrap();
///     response.text().unwrap()
/// }
/// ```
macro_rules! cfg_remove_async {
    ($($item:item)*) => {

            $(#[$attr:meta])*
            $vis:vis async fn $fn_name:ident $($rest:tt)* => {
                $(#[$attr])*
                $vis fn $fn_name $($rest)*
            }

    };
}

#[test]
fn test_cfg_remove_async() {
    cfg_gen_blocking! {
        async fn hello() -> String {
            "hello".to_string()
        }

        async fn send_request() -> String {
            let response = reqwest::get("https://www.google.com").await.unwrap();
            response.text().await.unwrap()
        }
    }
}
