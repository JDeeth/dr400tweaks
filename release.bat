cargo build --release
@del compiled\dr400tweaks\win_x64\dr400tweaks.xpl
@copy target\release\dr400tweaks.dll compiled\dr400tweaks\win_x64\dr400tweaks.xpl
@del compiled\dr400tweaks\README.md
@copy README.md compiled\dr400tweaks\README.md
@rmdir /s /q compiled\dr400tweaks\img
@robocopy img compiled\dr400tweaks\img /E > nul
del "C:\X-Plane\12\Aircraft\JF_Robin_DR400\plugins\dr400tweaks\win_x64\dr400tweaks.xpl"
copy target\release\dr400tweaks.dll "C:\X-Plane\12\Aircraft\JF_Robin_DR400\plugins\dr400tweaks\win_x64\dr400tweaks.xpl"
