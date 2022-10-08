@pushd "%~dp0.." && setlocal

@cd "%~dp0../crates/thindx-xaudio2-sys"
cargo publish
@if ERRORLEVEL 1 goto :die

@echo Waiting 15 seconds to give crates.io a chance to index thindx-xaudio2-sys...
@ping -n 16 localhost >NUL 2>NUL

@cd "%~dp0../crates/thindx-xaudio2"
cargo publish
@if ERRORLEVEL 1 goto :die

:die
@popd && endlocal && exit /b %ERRORLEVEL%

