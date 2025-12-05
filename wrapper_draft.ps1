function proxy {
    # Adjust this path to point to your compiled binary
    $binaryPath = "$PSScriptRoot\target\debug\proxy-cli.exe"
    
    if (-not (Test-Path $binaryPath)) {
        Write-Error "Binary not found at $binaryPath. Please run 'cargo build' first."
        return
    }

    # Run the tool and capture output
    # We use Invoke-Expression to run it and capture stdout, while letting stderr pass through if needed
    $output = & $binaryPath

    # Process the output
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
        else {
            # Print other lines (TUI output usually goes to stderr or direct to tty, 
            # but if we captured stdout, we might catch some. 
            # However, dialoguer usually writes to stderr/tty for interactive parts.
            # We need to make sure the TUI shows up.)
            # Wait, capturing output might hide the TUI if not handled correctly.
            # dialoguer uses Term::stderr() by default? No, Term::stdout().
            # If we capture stdout, the user won't see the menu.
            
            # Correction: We need the TUI to be visible.
            # We can't easily capture stdout AND show it in real-time for TUI if we are piping.
            # BUT, the tool prints the special command at the END.
            # So we can just let the tool run, but we need to capture the LAST line?
            # Or we can have the tool write the command to a temporary file?
            # Or we can have the tool write the command to stdout, but the TUI to stderr?
        }
    }
}
