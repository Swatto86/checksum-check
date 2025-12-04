@echo off
setlocal enabledelayedexpansion

echo ========================================
echo   Checksum Check - Test Suite Runner
echo ========================================
echo.

set TOTAL_PASSED=0
set TOTAL_FAILED=0

REM Install dependencies if needed
if not exist "node_modules\" (
    echo [INFO] Installing npm dependencies...
    call npm install
    if errorlevel 1 (
        echo [ERROR] Failed to install npm dependencies
        exit /b 1
    )
    echo.
)

REM Frontend Tests
echo ========================================
echo   Running Frontend Tests
echo ========================================
echo.

call npm test -- --run --silent
if errorlevel 1 (
    echo.
    echo [FAIL] Frontend tests failed
    set /a TOTAL_FAILED+=1
) else (
    echo.
    echo [PASS] Frontend tests passed
    set /a TOTAL_PASSED+=1
)

REM Backend Tests
echo.
echo ========================================
echo   Running Backend Tests
echo ========================================
echo.

cd src-tauri
call cargo test --quiet
if errorlevel 1 (
    echo.
    echo [FAIL] Backend tests failed
    set /a TOTAL_FAILED+=1
) else (
    echo.
    echo [PASS] Backend tests passed
    set /a TOTAL_PASSED+=1
)
cd ..

REM Summary
echo.
echo ========================================
echo   Test Summary
echo ========================================
echo.
echo Tests Passed: !TOTAL_PASSED!
echo Tests Failed: !TOTAL_FAILED!
echo.

if !TOTAL_FAILED! gtr 0 (
    echo [RESULT] Some tests failed
    exit /b 1
) else (
    echo [RESULT] All tests passed!
    exit /b 0
)
