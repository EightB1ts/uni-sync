@echo off
echo Building Uni-Sync
cargo build --release
if exist target\release\uni-sync.exe (
    mkdir "%APPDATA%\Local\uni-sync" 2> NUL
    move "target\release\uni-sync.exe" "%APPDATA%\Local\uni-sync"
    xcopy /d "uni-sync.json" "%APPDATA%\Local\uni-sync"
    reg add /f HKCU\Software\Microsoft\Windows\CurrentVersion\Run /v UniSync /d "%APPDATA%\Local\uni-sync"
    @RD /S /Q "target"
    echo Build Complete! Added uni-sync.exe to your Startup Folder
) else (
    echo Build Failed...
)
@pause