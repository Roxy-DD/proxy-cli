# Mock the binary output
function Mock-ProxyTool {
    Write-Output "#SET_PROXY:http://127.0.0.1:9999"
}

# Define the wrapper function using the mock
function proxy-test {
    $output = Mock-ProxyTool
    $output | ForEach-Object {
        if ($_ -match "^#SET_PROXY:(.+)$") {
            $proxyUrl = $matches[1]
            $env:HTTP_PROXY = $proxyUrl
            $env:HTTPS_PROXY = $proxyUrl
        }
    }
}

# Run test
$env:HTTP_PROXY = ""
proxy-test

if ($env:HTTP_PROXY -eq "http://127.0.0.1:9999") {
    Write-Host "Test Passed: Proxy set correctly" -ForegroundColor Green
} else {
    Write-Host "Test Failed: Proxy not set. Got '$env:HTTP_PROXY'" -ForegroundColor Red
}

# Test Clear
function Mock-ProxyTool-Clear {
    Write-Output "#CLEAR_PROXY"
}

function proxy-test-clear {
    $output = Mock-ProxyTool-Clear
    $output | ForEach-Object {
        if ($_ -eq "#CLEAR_PROXY") {
            $env:HTTP_PROXY = ""
            $env:HTTPS_PROXY = ""
        }
    }
}

$env:HTTP_PROXY = "something"
proxy-test-clear

if ($env:HTTP_PROXY -eq "") {
    Write-Host "Test Passed: Proxy cleared correctly" -ForegroundColor Green
} else {
    Write-Host "Test Failed: Proxy not cleared. Got '$env:HTTP_PROXY'" -ForegroundColor Red
}
