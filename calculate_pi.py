import random
import rusty_pi
import time

def estimate_pi(iterations: int) -> float:
	dots_total = 0
	dots_in_circle = 0
	for _ in range(iterations):
		point1 = random.random()
		point2 = random.random()

		if point2**2 + point1**2 <= 1:
			dots_in_circle += 1

		dots_total += 1

	return 4 * dots_in_circle / dots_total


start = time.monotonic()
print(estimate_pi(10000000))
print(f"Pyhton implementation took {time.monotonic() - start} seconds to estimate Pi")

start = time.monotonic()
print(rusty_pi.estimate(10000000))
print(f"Rust implementation took {time.monotonic() - start} seconds to estimate Pi")