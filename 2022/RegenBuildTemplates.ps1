param(
    [Parameter(Mandatory=$true)]
    [int]$day
    )

$dayId = "day{0:d2}" -f $day

$oldName = "template/c/day_template_build.sh"
$newName = "$dayId/c/$($dayId)_build.sh"
((Get-Content -path $oldName -Raw) -replace 'day_template',$dayId) | Set-Content -Path $newName -NoNewLine
