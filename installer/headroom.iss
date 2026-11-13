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

[UninstallRun]
Filename: "{sys}\taskkill.exe"; Parameters: "/F /IM headroom.exe"; Flags: runhidden; RunOnceId: "CloseHeadroom"

[Code]
function PrepareToInstall(var NeedsRestart: Boolean): String;
var
  Code: Integer;
begin
  // Close a running copy so its exe can be replaced.
  Exec(ExpandConstant('{sys}\taskkill.exe'), '/F /IM headroom.exe', '', SW_HIDE, ewWaitUntilTerminated, Code);
  Sleep(500);
  Result := '';
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
  // Unticked "start at sign-in" on an upgrade: drop an older Run entry.
  if (CurStep = ssPostInstall) and not WizardIsTaskSelected('startup') then
    RegDeleteValue(HKCU, 'Software\Microsoft\Windows\CurrentVersion\Run', 'Headroom');
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
  // Also covers the value the widget's own menu wrote.
  if CurUninstallStep = usPostUninstall then
    RegDeleteValue(HKCU, 'Software\Microsoft\Windows\CurrentVersion\Run', 'Headroom');
end;
