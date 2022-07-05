s1 <- "alcohol"
s2 <- "acloholism"


test_that("leven",
  {
    expect_equal(stringdist(s1, s2, method="lv"), 5.0)
  }
)

test_that("damerau",
  {
    expect_equal(stringdist(s1, s2, method="dl"), 4.0)
  }
)

test_that("osa",
  {
    expect_equal(stringdist(s1, s2, method="osa"), 4.0)
  }
)

test_that("jw",
  {
    expect_lt(stringdist(s1, s2, method="jw"), 0.15)
    expect_gt(stringdist(s1, s2, method="jw"), 0.145)
  }
)