# repository: https://pypi.org/project/fastDamerauLevenshtein/
# repository: https://github.com/ztane/python-Levenshtein/

from Levenshtein import _levenshtein as lv
from fastDamerauLevenshtein import damerauLevenshtein

s1 = "alcohol"
s2 = "acloholism"
iterations = 1_000_000


def test_benchmark_levenshtein_distance(benchmark):
    def f():
        return lv.distance(s1, s2)

    benchmark.pedantic(f, iterations=iterations)


def test_benchmark_damerau_distance(benchmark):
    def f():
        return damerauLevenshtein(s1, s2, similarity=False)

    benchmark.pedantic(f, iterations=iterations)


def test_benchmark_jarowinkler_distance(benchmark):
    def f():
        return lv.jaro_winkler(s1, s2)

    benchmark.pedantic(f, iterations=iterations)


def test_levenshtein_distance():
    d = lv.distance(s1, s2)
    assert d == 5.0


def test_damerau_distance():
    d = damerauLevenshtein(s1, s2, similarity=False)
    assert d == 4.0


def test_jarowinkler_distance():
    d = 1 - lv.jaro_winkler(s1, s2)
    assert d == 0.13285714285714278
