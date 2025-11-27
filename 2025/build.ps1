param(
    [int]$Day
    )

$DayArg = if (!$Day) { "all" } else { "$Day" }
function diag { Write-Host -ForegroundColor DarkGray $args }

diag "making output dir"
$outputDir = "$PSScriptRoot/output"

diag "configuring (day=$DayArg)"
cmake -DBUILD_DAY="$DayArg" -G Ninja -S "$PSScriptRoot" -B $outputDir

diag "building (day=$DayArg)"
cmake --build $outputDir