param(
    [Parameter(Mandatory=$true)]
    [int]$day
    )

$dayId = "day{0:d2}" -f $day

cp -rec template $dayId

dir -rec -File $dayId | % {
    $oldName = $_.FullName
    ((Get-Content -path $oldName -Raw) -replace 'day_template',$dayId) | Set-Content -Path $oldName -NoNewLine

    $newName = $oldName -replace "day_template",$dayId
    mv $oldName $newName
}
