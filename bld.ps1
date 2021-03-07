
# ----------------------------------------------------------------------------
# For sanity.
# ----------------------------------------------------------------------------
Set-StrictMode -Version Latest

# Default to croak on any error
$ErrorActionPreference = "Stop"

# Release build
& cargo build --examples
if($LastExitCode -ne 0) {
  Exit
}


# Microsoft Jump-Through-Hoops(tm)
$progdir = "${env:ProgramFiles(x86)}"
$vsinstdir = "${progdir}\Microsoft Visual Studio\Installer"
$vswhere = "$vsinstdir\vswhere.exe"

$cmdargs = '-latest', '-find', 'VC\Tools\**\HostX64\x64\dumpbin.exe'
$DumpBin = & $vswhere $cmdargs
$DumpBin = $DumpBin[0]


& $DumpBin /nologo /dependents target\debug\examples\panic_dbg_output.exe
& $DumpBin /nologo /dependents target\debug\examples\panic_dbg_output_break.exe

$Exists = Test-Path -Path $HOME\vboxshares\win10 -PathType Container
if($Exists) {
  # PDB's are required for dumping backtraces on the debugee
  Copy-Item target\debug\examples\panic_dbg_output.exe -Destination $HOME\vboxshares\win10\
  Copy-Item target\debug\examples\panic_dbg_output.pdb -Destination $HOME\vboxshares\win10\
  Copy-Item target\debug\examples\panic_dbg_output_break.exe -Destination $HOME\vboxshares\win10\
  Copy-Item target\debug\examples\panic_dbg_output_break.pdb -Destination $HOME\vboxshares\win10\
}

# vim: set ft=ps1 et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
