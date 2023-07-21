@echo off
echo Removing Uni-Sync
cargo build --release
if exist "%APPDATA%\Local\uni-sync" (
    xcopy /Y "%APPDATA%\Local\uni-sync\uni-sync.json" "uni-sync-backup.json"
    @RD /S /Q "%APPDATA%\Local\uni-sync"
)
reg delete HKCU\Software\Microsoft\Windows\CurrentVersion\Run /v UniSync /f
echo Removed Uni-Sync
@pause