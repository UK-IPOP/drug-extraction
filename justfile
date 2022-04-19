default:
    @just -l

run:
    cargo run -p extract-drugs --  cli/data/Medical_Examiner_Case_Archive.csv "cocaine|fentanyl|heroin" 0.95

