@echo off
echo Building Uni-Sync
cargo build --release
if exist target\release\uni-sync.exe (
    mkdir "%APPDATA%\Local\uni-sync" 2> NUL
    move "target\release\uni-sync.exe" "%APPDATA%\Local\uni-sync"
    xcopy /d "uni-sync.json" "%APPDATA%\Local\uni-sync"
    reg add HKCU\Software\Microsoft\Windows\CurrentVersion\Run /v UniSync /d "%APPDATA%\Local\uni-sync" /f
    @RD /S /Q "target"
    echo Build Complete! Added uni-sync.exe to Startup
) else (
    echo Build Failed...
)
@pause