import biodivine_boolean_functions as bbf
from pyeda.inter import *
from utils import run_bench, load_expressions
import hashlib
import sys

# Some expressions are too large for PyEDA to process without this.
sys.setrecursionlimit(100_000)

"""
This benchmark compares the performance of the Boolean expression
parsers in each library on a set of expressions extracted from 
real world biological models.
"""

def bench_parse_bbf(e):
	return bbf.Expression(e)

def bench_parse_pyeda(e):
	return expr(e)

header = [
	"hash + len",
	"BBF[avg]",
	"PyEDA[avg]",
	"BBF[dev]",
	"PyEDA[dev]",
	"BBF[times]",
	"PyEDA[times]"
]
print("\t".join(header))

# Expressions should be sorted by length.
for e_str in load_expressions('expressions.txt'):
	
	(bbf_avg, bbf_dev, bbf_times) = run_bench(lambda: bench_parse_bbf(e_str))

	e_str = e_str.replace("!", "~")	# PyEDA uses ~ as the negation operator.

	(pyeda_avg, pyeda_dev, pyeda_times) = run_bench(lambda: bench_parse_pyeda(e_str))

	h = hashlib.new('sha256')
	h.update(e_str.encode())
	
	row = [
		f"{h.hexdigest()}__{len(e_str)}",
		str(bbf_avg),
		str(pyeda_avg),
		str(bbf_dev),
		str(pyeda_dev),
		str(bbf_times),
		str(pyeda_times)
	]

	print("\t".join(row))
