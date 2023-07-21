@echo off
echo Removing Uni-Sync
cargo build --release
if exist "%APPDATA%\Local\uni-sync" (
    xcopy /y "%APPDATA%\Local\uni-sync\uni-sync.json" "uni-sync-backup.json"
    @RD /S /Q "%APPDATA%\Local\uni-sync"
)
reg delete /f HKCU\Software\Microsoft\Windows\CurrentVersion\Run /v UniSync
echo Removed Uni-Sync
@pause