# Info

This package implements string searching algorithms taken from the
[strsimpy package](https://github.com/luozhouyang/python-string-similarity). It specifically uses
the Levenshtein distance and the JaroWinkler Similarity. It does calculations for one algorithm at a time
depending on user input. Previous iterations streamed results of each comparison to file; however, performance
and file size limitations resulted in a switch to manual analysis and simple logging of results.

Comparisons are made to _each_ word in _each_ record and the total time is recorded for each metric and an average is then computed.
