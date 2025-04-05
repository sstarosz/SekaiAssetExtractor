
function Make-SymbolicLink {
    param (
        [string]$sourcePath,
        [string]$targetPath
    )

    # Check if the source path exists
    if (-Not (Test-Path -Path $sourcePath)) {
        Write-Host "Source path does not exist: $sourcePath" -ForegroundColor Red
        return $false
    }

    # Check if the symlink already exists
    if (Test-Path -Path $targetPath) {
        Write-Host "Symlink already exists at: $targetPath" -ForegroundColor Yellow
        return $true
    }

    # Create the symbolic link
    New-Item -ItemType SymbolicLink -Path $targetPath -Target $sourcePath

    # Check if the symlink was created successfully
    if (-Not (Test-Path -Path $targetPath)) {
        Write-Host "Failed to create symbolic link." -ForegroundColor Red
        return $false
    }

    # Confirm success
    Write-Host "Symbolic link created successfully:" -ForegroundColor Green
    Write-Host "Source: $sourcePath" -ForegroundColor Green
    Write-Host "Target: $targetPath" -ForegroundColor Green

    return $true
}



#Define the source and target paths
$sourcePath = (Resolve-Path "..\master-db").Path
$targetPath = Join-Path $(Get-Location).Path "master-db"
Make-SymbolicLink -sourcePath $sourcePath -targetPath $targetPath

# Define the source and target paths for the second symlink
$sourcePath2 = (Resolve-Path "..\bundles\").Path
$targetPath2 = Join-Path $(Get-Location).Path "bundles"
Make-SymbolicLink -sourcePath $sourcePath2 -targetPath $targetPath2