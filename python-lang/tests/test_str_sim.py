# repository: https://github.com/luozhouyang/python-string-similarity
# contains info on algorithmic complexity as well
# these are all distances which means to get similarity you need to do 1 - distance


# metrics (triangle inequality)
from strsimpy.levenshtein import Levenshtein
from strsimpy.damerau import Damerau
from strsimpy.jaccard import Jaccard


from strsimpy.optimal_string_alignment import OptimalStringAlignment
from strsimpy.jaro_winkler import JaroWinkler
from strsimpy.cosine import Cosine
from strsimpy.sorensen_dice import SorensenDice


s1 = "alcohol"
s2 = "acloholism"
iterations = 1_000_000


def test_levenshtein_distance(benchmark):
    def f():
        return Levenshtein().distance(s1, s2)

    result = benchmark.pedantic(f, iterations=iterations)
    assert result == 5


def test_damerau_distance(benchmark):
    def f():
        return Damerau().distance(s1, s2)

    result = benchmark.pedantic(f, iterations=iterations)
    assert result == 4


def test_jaccard_distance(benchmark):
    def f():
        return Jaccard(k=2).distance(s1, s2)

    result = benchmark.pedantic(f, iterations=iterations)
    assert result == 0.75


def test_jarowinkler_distance(benchmark):
    def f():
        return JaroWinkler().distance(s1, s2)

    result = benchmark.pedantic(f, iterations=iterations)
    assert result == 0.1328571428571429


def test_cosine_distance(benchmark):
    def f():
        return Cosine(k=2).distance(s1, s2)

    result = benchmark.pedantic(f, iterations=iterations)
    assert result == 0.5917517095361369


def test_sorensen_dice_distance(benchmark):
    def f():
        return SorensenDice().distance(s1, s2)

    result = benchmark.pedantic(f, iterations=iterations)
    assert result == 0.6923076923076923


def test_optimal_string_alignment_distance(benchmark):
    def f():
        return OptimalStringAlignment().distance(s1, s2)

    result = benchmark.pedantic(f, iterations=iterations)
    d = OptimalStringAlignment().distance(s1, s2)
    assert result == 4
