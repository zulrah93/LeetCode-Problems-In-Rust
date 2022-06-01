Get-ChildItem -Filter *.rs | ForEach-Object { # For each rust source file we are going to call rustfmt tool -- see bash script version if you like to clone on a Unix-like machine
    $file_name = $_.FullName;
    Write-Host "Formating With Rust Tool [rustfmt]: " $file_name; 
    rustfmt $file_name
}