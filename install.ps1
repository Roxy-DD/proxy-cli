$ErrorActionPreference = "Stop"

# Define the path to the binary
# We assume this script is run from the project root or we can find the binary relative to the script
$scriptPath = $PSScriptRoot
# Check for binary in the same directory (Release package structure)
$binaryPathRelease = Join-Path $scriptPath "proxy.exe"
# Check for binary in target/debug (Dev structure)
$binaryPathDev = Join-Path $scriptPath "target\debug\proxy.exe"

if (Test-Path $binaryPathRelease) {
    $binaryPath = $binaryPathRelease
} elseif (Test-Path $binaryPathDev) {
    $binaryPath = $binaryPathDev
} else {
    Write-Error "Binary not found. Please ensure proxy.exe is in the same directory or run 'cargo build'."
    exit 1
}

# Get the absolute path of the binary
$binaryAbsolutePath = (Resolve-Path $binaryPath).Path

# Define the function code to add
$functionCode = @"

# Proxy CLI Wrapper
function proxy {
    `$binaryPath = "$binaryAbsolutePath"
    
    if (-not (Test-Path `$binaryPath)) {
        Write-Error "Binary not found at `$binaryPath"
        return
    }

    `$output = & `$binaryPath

    `$output | ForEach-Object {
        if (`$_ -match "^#SET_PROXY:(.+)$") {
            `$proxyUrl = `$matches[1]
            `$env:HTTP_PROXY = `$proxyUrl
            `$env:HTTPS_PROXY = `$proxyUrl
            Write-Host "Proxy set to `$proxyUrl" -ForegroundColor Green
        }
        elseif (`$_ -eq "#CLEAR_PROXY") {
            `$env:HTTP_PROXY = ""
            `$env:HTTPS_PROXY = ""
            Write-Host "Proxy cleared" -ForegroundColor Yellow
        }
    }
}
"@

# Check if Profile exists
if (-not (Test-Path $PROFILE)) {
    Write-Host "Creating PowerShell profile at $PROFILE..."
    New-Item -Path $PROFILE -ItemType File -Force | Out-Null
}

# Check if function already exists in profile
$profileContent = Get-Content $PROFILE -Raw
if ($profileContent -match "function proxy") {
    Write-Warning "The 'proxy' function seems to already exist in your profile ($PROFILE)."
    Write-Host "Please check your profile manually to avoid duplicates."
} else {
    Write-Host "Adding 'proxy' function to $PROFILE..."
    Add-Content -Path $PROFILE -Value "`n$functionCode`n"
    Write-Host "Installation complete!" -ForegroundColor Green
    Write-Host "Please restart your PowerShell or run '. `$PROFILE' to load the new command." -ForegroundColor Cyan
}
