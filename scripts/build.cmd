@pushd "%~dp0.." && setlocal

:: all features
cargo test --all-features
@if ERRORLEVEL 1 goto :die

:die
@popd && endlocal && exit /b %ERRORLEVEL%
