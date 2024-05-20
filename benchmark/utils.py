import math
import time

def stats(bench_times):
	# Compute average and standard deviation.
	avg = sum(bench_times) / len(bench_times)
	var = sum((x - avg) * (x - avg) for x in bench_times) / len(bench_times)	
	# The numbers are in nano-seconds, so it is pretty safe to
	# round them to integers.
	return (int(avg), int(math.sqrt(var)))


WARMUP = 5
REPETITIONS = 10

def run_bench(action):
	times = []
	for _ in range(WARMUP):
		action()
	for _ in range(REPETITIONS):
		start = time.perf_counter_ns()
		action()
		times.append(time.perf_counter_ns() - start)

	(avg, dev) = stats(times)

	return (avg, dev, times)

def load_expressions(filename):
	with open(filename) as file:
		lines = [line.strip() for line in file]
		lines = sorted(lines, key = lambda x: len(x))
		return lines
		