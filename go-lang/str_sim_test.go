package string_sim

// distance algorithms taken from github.com/hbollon/go-edlib

import (
	"testing"

	"github.com/hbollon/go-edlib"
)

const s1 = "alcohol"
const s2 = "acloholism"

func BenchmarkCosine(b *testing.B) {
	for i := 0; i < b.N; i++ {
		s := edlib.CosineSimilarity(s1, s2, 2)
		if s < 0.40 || s > 0.45 {
			b.Errorf("Cosine Similarity should be around 0.4 not %f", s)
		}
	}
}

func BenchmarkLevenshtein(b *testing.B) {
	for i := 0; i < b.N; i++ {
		d := edlib.LevenshteinDistance(s1, s2)
		if d != 5 {
			b.Errorf("Levenshtein Distance should be 5 not %d", d)
		}

	}
}

func BenchmarkDamerau(b *testing.B) {
	for i := 0; i < b.N; i++ {
		d := edlib.DamerauLevenshteinDistance(s1, s2)
		if d != 4 {
			b.Errorf("Damerau Levenshtein Distance should be 4 not %d", d)
		}
	}
}

func BenchmarkOptimalStringAlignment(b *testing.B) {
	for i := 0; i < b.N; i++ {
		d := edlib.OSADamerauLevenshteinDistance(s1, s2)
		if d != 4 {
			b.Errorf("OSA should be 4 not %d", d)
		}
	}
}

func BenchmarkJaccard(b *testing.B) {
	for i := 0; i < b.N; i++ {
		s := edlib.JaccardSimilarity(s1, s2, 2)
		if s != 0.25 {
			b.Errorf("Jaccard Similarity should be 0.25 not %f", s)
		}
	}
}

func BenchmarkJaroWinkler(b *testing.B) {
	for i := 0; i < b.N; i++ {
		s := edlib.JaroWinklerSimilarity(s1, s2)
		if s > 0.9 || s < 0.8 {
			b.Errorf("Expected around 0.8, got %f", s)
		}
	}
}

func BenchmarkSorensonDice(b *testing.B) {
	for i := 0; i < b.N; i++ {
		c := edlib.SorensenDiceCoefficient(s1, s2, 2)
		if c != 0.40 {
			b.Errorf("Sorensen Dice Coefficient should be 1 not %f", c)
		}
	}
}
