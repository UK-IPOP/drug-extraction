# repository: https://github.com/luozhouyang/python-string-similarity
# contains info on algorithmic complexity as well
# these are all distances which means to get similarity you need to do 1 - distance


# metrics (triangle inequality)
from strsimpy.levenshtein import Levenshtein
from strsimpy.damerau import Damerau


from strsimpy.optimal_string_alignment import OptimalStringAlignment
from strsimpy.jaro_winkler import JaroWinkler
from strsimpy.sorensen_dice import SorensenDice


s1 = "alcohol"
s2 = "acloholism"
iterations = 1_000_000


def benchmark_levenshtein_distance(benchmark):
    def f():
        return Levenshtein().distance(s1, s2)

    result = benchmark.pedantic(f, iterations=iterations)


def benchmark_damerau_distance(benchmark):
    def f():
        return Damerau().distance(s1, s2)

    result = benchmark.pedantic(f, iterations=iterations)


def benchmark_jarowinkler_distance(benchmark):
    def f():
        return JaroWinkler().distance(s1, s2)

    result = benchmark.pedantic(f, iterations=iterations)


def benchmark_sorensen_dice_distance(benchmark):
    def f():
        return SorensenDice().distance(s1, s2)

    result = benchmark.pedantic(f, iterations=iterations)


def benchmark_optimal_string_alignment_distance(benchmark):
    def f():
        return OptimalStringAlignment().distance(s1, s2)

    result = benchmark.pedantic(f, iterations=iterations)
