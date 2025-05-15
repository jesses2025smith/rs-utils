/// python like `with` macro for synchronize code
///
/// # Example
///
/// ```rust
///
/// use rsutil::with;
///
/// struct A {}
///
/// impl A {
///     pub fn __enter__(&mut self) -> Result<(), String> {
///         println!("__enter__");
///         Ok(())
///     }
///
///     pub fn __exit__(&mut self) {
///         println!("__exit__");
///     }
/// }
///
/// with!(A {} => |a| {
///     println!("body");
///
///     Ok::<_, String>(())
/// }).unwrap();
#[macro_export]
macro_rules! with {
    ($init:expr => |$name:ident| $body:block) => {{
        let mut $name = $init;
        match $name.__enter__() {
            Ok(()) => {
                let result = $body;
                $name.__exit__();
                result
            }
            Err(e) => Err(e),
        }
    }};
}

/// python like `async_with` macro for asynchronize function
///
/// # Example
///
/// ```rust
///
/// use rsutil::async_with;
///
/// struct A {}
///
/// impl A {
///     pub async fn __aenter__(&mut self) -> Result<(), String> {
///         println!("__enter__");
///         Ok(())
///     }
///
///     pub async fn test(&self) -> Result<(), String> {
///         println!("test....");
///         Ok(())
///     }
///
///     pub async fn __aexit__(&mut self) {
///         println!("__exit__");
///     }
/// }
///
/// #[tokio::test]
/// async fn example() {
///     async_with!(A {} => |a| {
///         println!("body");
///         a.test().await?;
///         Ok::<_, String>(())
///     })
///     .await
///     .unwrap();
/// }
#[macro_export]
macro_rules! async_with {
    ($init:expr => |$name:ident| $body:block) => {{
        let mut $name = $init;
        async move {
            match $name.__aenter__().await {
                Ok(()) => {
                    let result = (async { $body }).await;
                    $name.__aexit__().await;

                    result
                }
                Err(e) => Err(e),
            }
        }
    }};
}
