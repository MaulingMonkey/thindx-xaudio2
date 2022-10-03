@pushd "%~dp0.." && setlocal

:: all features
:: https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#doctest-in-workspace : fixes paths for problems tab
cargo +nightly test --all-features -Z doctest-in-workspace
@if ERRORLEVEL 1 goto :die

:: no features, no doctest-in-workspace
cargo test
@if ERRORLEVEL 1 goto :die

:die
@popd && endlocal && exit /b %ERRORLEVEL%
