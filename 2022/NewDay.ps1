param(
    [Parameter(Mandatory=$true)]
    [int]$day,
    [ValidateSet("c", "odin", "zig")]
    [Parameter(Mandatory=$true)]
    [string]$language
    )

$dayId = "day{0:d2}" -f $day
mkdir $dayId

cp -rec "template\$language" "$dayId\$language"

dir -rec -File $dayId | % {
    $oldName = $_.FullName
    ((Get-Content -path $oldName -Raw) -replace 'day_template',$dayId) | Set-Content -Path $oldName -NoNewLine

    $newName = $oldName -replace "day_template",$dayId
    mv $oldName $newName
}
