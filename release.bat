cargo build --release
del compiled\dr400tweaks\win_x64\dr400tweaks.xpl
copy target\release\dr400tweaks.dll compiled\dr400tweaks\win_x64\dr400tweaks.xpl
del "C:\X-Plane\12\Aircraft\JF_Robin_DR400\plugins\dr400tweaks\win_x64\dr400tweaks.xpl"
copy target\release\dr400tweaks.dll "C:\X-Plane\12\Aircraft\JF_Robin_DR400\plugins\dr400tweaks\win_x64\dr400tweaks.xpl"
