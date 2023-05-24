@echo off
echo Building Uni-Sync
cargo build --release
if exist target\release\uni-sync.exe (
    move "target\release\uni-sync.exe" "%USERPROFILE%\Start Menu\Programs\Startup"
    @RD /S /Q "target"
    echo Build Complete! Added uni-sync.exe to your Startup Folder
) else (
    echo Build Failed...
)
@pause