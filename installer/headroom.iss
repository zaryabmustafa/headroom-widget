; Headroom installer (Inno Setup 6). Per-user, no admin.
; Build: ISCC /DAppVersion=0.1.4 installer\headroom.iss  ->  target\installer\headroom-setup.exe
; Needs target\release\headroom.exe built first.

#ifndef AppVersion
  #define AppVersion "0.0.0"
#endif

[Setup]
AppId={{6B0E4C1A-6F0B-4E8E-9C7E-2A51D7F3B9A4}
AppName=Headroom
AppVersion={#AppVersion}
AppVerName=Headroom {#AppVersion}
AppPublisher=Zaryab Mustafa
AppCopyright=Copyright (c) 2026 Zaryab Mustafa
VersionInfoCompany=Zaryab Mustafa
VersionInfoCopyright=Copyright (c) 2026 Zaryab Mustafa
VersionInfoDescription=Headroom setup
VersionInfoProductName=Headroom
VersionInfoVersion={#AppVersion}
AppPublisherURL=https://github.com/zaryabmustafa/headroom-widget
AppSupportURL=https://github.com/zaryabmustafa/headroom-widget/issues
AppUpdatesURL=https://github.com/zaryabmustafa/headroom-widget/releases/latest
PrivilegesRequired=lowest
DefaultDirName={autopf}\Headroom
DisableProgramGroupPage=yes
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
MinVersion=10.0
WizardStyle=modern
SetupIconFile=..\assets\icon.ico
UninstallDisplayIcon={app}\headroom.exe
UninstallDisplayName=Headroom
LicenseFile=..\LICENSE
OutputDir=..\target\installer
OutputBaseFilename=headroom-setup
Compression=lzma2
SolidCompression=yes
; Headroom is closed in PrepareToInstall instead.
CloseApplications=no

[Tasks]
Name: startup; Description: "Start Headroom when I sign in"

[Files]
Source: "..\target\release\headroom.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "..\LICENSE"; DestDir: "{app}"; DestName: "LICENSE.txt"

[Icons]
Name: "{autoprograms}\Headroom"; Filename: "{app}\headroom.exe"

[Registry]
; Same value the widget's own "Start with Windows" menu item writes.
Root: HKCU; Subkey: "Software\Microsoft\Windows\CurrentVersion\Run"; ValueType: string; ValueName: "Headroom"; ValueData: """{app}\headroom.exe"""; Tasks: startup
; Settings: removed on uninstall.
Root: HKCU; Subkey: "Software\Headroom"; Flags: uninsdeletekey dontcreatekey

[Run]
Filename: "{app}\headroom.exe"; Description: "Start Headroom now"; Flags: postinstall nowait skipifsilent

[Code]
const
  RunKey = 'Software\Microsoft\Windows\CurrentVersion\Run';

// Closes only the copy in the install folder; a portable copy elsewhere keeps running.
procedure CloseInstalledCopy();
var
  Exe, Cmd: String;
  Code: Integer;
begin
  Exe := ExpandConstant('{app}\headroom.exe');
  StringChangeEx(Exe, '''', '''''', True);
  Cmd := '-NoProfile -NonInteractive -Command "Get-Process headroom -ErrorAction SilentlyContinue | ' +
    'Where-Object { $_.Path -eq ''' + Exe + ''' } | Stop-Process -Force; Start-Sleep -Milliseconds 500"';
  Exec('powershell.exe', Cmd, '', SW_HIDE, ewWaitUntilTerminated, Code);
end;

function PrepareToInstall(var NeedsRestart: Boolean): String;
begin
  CloseInstalledCopy();
  Result := '';
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  Value: String;
begin
  if CurUninstallStep = usUninstall then
    CloseInstalledCopy();
  // Remove the sign-in entry only if it starts this copy (the widget's menu writes
  // the same value); a portable copy's entry is left alone.
  if (CurUninstallStep = usPostUninstall) and RegQueryStringValue(HKCU, RunKey, 'Headroom', Value) and
     (CompareText(Value, '"' + ExpandConstant('{app}\headroom.exe') + '"') = 0) then
    RegDeleteValue(HKCU, RunKey, 'Headroom');
end;
