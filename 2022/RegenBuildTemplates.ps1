param(
    [Parameter(Mandatory=$true)]
    [int]$day
    )

$dayId = "day{0:d2}" -f $day

$oldName = "template/c/build.sh"
$newName = "$dayId/c/build.sh"
((Get-Content -path $oldName -Raw) -replace 'day_template',$dayId) | Set-Content -Path $newName -NoNewLine
