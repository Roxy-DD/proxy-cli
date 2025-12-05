function proxy {
    # Path to the Rust binary
    # You can move the binary to a location in your PATH or keep it here
    $binaryPath = "d:\code\proxy-cli\target\debug\proxy-cli.exe"
    
    if (-not (Test-Path $binaryPath)) {
        Write-Error "Binary not found at $binaryPath"
        return
    }

    # Run the tool and capture stdout (where the command is printed)
    # Stderr (where the TUI is printed) will pass through to the console
    $output = & $binaryPath

    # Parse the output for commands
    $output | ForEach-Object {
        if ($_ -match "^#SET_PROXY:(.+)$") {
            $proxyUrl = $matches[1]
            $env:HTTP_PROXY = $proxyUrl
            $env:HTTPS_PROXY = $proxyUrl
            Write-Host "Proxy set to $proxyUrl" -ForegroundColor Green
        }
        elseif ($_ -eq "#CLEAR_PROXY") {
            $env:HTTP_PROXY = ""
            $env:HTTPS_PROXY = ""
            Write-Host "Proxy cleared" -ForegroundColor Yellow
        }
    }
}
