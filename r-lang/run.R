# r benchmark package: https://cran.r-project.org/web/packages/rbenchmark/rbenchmark.pdf
# r test package tutorial: https://r-pkgs.org/testing-basics.html#run-tests
# r string sim package: https://cran.r-project.org/web/packages/stringdist/stringdist.pdf



install.packages("rbenchmark", repos="http://cran.r-project.org")
install.packages("stringdist", repos="http://cran.r-project.org")
install.packages("testthat", repos="http://cran.r-project.org")

library(rbenchmark)
library(stringdist)
library(testthat)

options(scipen = 50, width = 500)

s1 <- "alcohol"
s2 <- "acloholism"
iterations <- 1000000
multiplier <- 10^9


testthat::test_file("tests/testthat/test-stringdist.R")


within(
    benchmark(
        levenshtein=stringdist(s1, s2, method="lv"),
        damerau_levenshtein=stringdist(s1, s2, method="dl"),
        optimal_string_alignment=stringdist(s1, s2, method="osa"),
        jaro_winkler=stringdist(s1, s2, method="jw"),
        replications = rep(iterations, 3),
        columns = c('test', 'elapsed', 'replications')
    ), 
    { average_ns = (elapsed/replications) * multiplier }
)
