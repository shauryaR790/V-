; v++ Windows installer - built in CI, signed via SignPath (see docs/SIGNING.md)
#ifndef MyAppVersion
  #define MyAppVersion "0.4.4"
#endif
#ifndef StagingDir
  #define StagingDir "..\staging"
#endif

[Setup]
AppId={{A1B2C3D4-E5F6-7890-ABCD-EF1234567890}
AppName=v++
AppVersion={#MyAppVersion}
AppPublisher=vpp-lang
AppPublisherURL=https://github.com/shauryaR790/V-
AppSupportURL=https://github.com/shauryaR790/V-/issues
AppUpdatesURL=https://github.com/shauryaR790/V-/releases
DefaultDirName={autopf}\vpp
DefaultGroupName=v++
DisableProgramGroupPage=yes
LicenseFile={#StagingDir}\LICENSE
OutputDir=..\output
OutputBaseFilename=vpp-{#MyAppVersion}-setup
UninstallDisplayIcon={app}\vpp.exe
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
PrivilegesRequired=lowest
WizardStyle=modern
Compression=lzma2
SolidCompression=yes

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "Create a desktop shortcut to run hello.vpp"; GroupDescription: "Additional icons:"; Flags: unchecked

[Files]
Source: "{#StagingDir}\*"; DestDir: "{app}"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\v++ - Run hello.vpp"; Filename: "{app}\vpp.exe"; Parameters: "run examples\hello.vpp"; WorkingDir: "{app}"
Name: "{group}\v++ - Open install folder"; Filename: "{app}"
Name: "{group}\Uninstall v++"; Filename: "{uninstallexe}"
Name: "{desktop}\v++"; Filename: "{app}\vpp.exe"; Parameters: "run examples\hello.vpp"; WorkingDir: "{app}"; Tasks: desktopicon

[Run]
Filename: "{app}\vpp.exe"; Parameters: "run examples\hello.vpp"; Description: "Run the hello.vpp example"; Flags: postinstall nowait skipifsilent

[Registry]
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; Check: NeedsAddPath(ExpandConstant('{app}'))
Root: HKCU; Subkey: "Environment"; ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}\llvm\bin"; Check: NeedsAddPath(ExpandConstant('{app}\llvm\bin'))

[Code]
function NeedsAddPath(Param: string): Boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', OrigPath) then
  begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + Param + ';', ';' + OrigPath + ';') = 0;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  Path, AppDir, LlvmDir: string;
  P: Integer;
begin
  if CurUninstallStep <> usPostUninstall then
    exit;
  if not RegQueryStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path) then
    exit;
  AppDir := ExpandConstant('{app}');
  LlvmDir := AppDir + '\llvm\bin';
  Path := ';' + Path + ';';
  StringChangeEx(Path, ';' + AppDir + ';', ';', True);
  StringChangeEx(Path, ';' + LlvmDir + ';', ';', True);
  if (Length(Path) > 0) and (Path[1] = ';') then
    Delete(Path, 1, 1);
  P := Length(Path);
  if (P > 0) and (Path[P] = ';') then
    Delete(Path, P, 1);
  RegWriteExpandStringValue(HKEY_CURRENT_USER, 'Environment', 'Path', Path);
end;
