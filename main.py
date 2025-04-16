import time

def faster_fib_explprod_4k2only(n):
    '''
    Only works correctly on numbers that are of the form 4k+2
    :param n: integer
    :return: Fibo(n) if n = 4k+2, with k positive integer
    '''
    p = (n-1)*(n-2) / 60
    pbis = 1
    a = n - 2
    b = 2

    for _ in range(0, n >> 2):
        k = b / a * (b + 1) / (a + 1)
        p = p * 5 * k + 1
        pbis = pbis * 5 / k + 1
        a -= 2
        b += 2
    r = p * pbis

    return r

start_time = time.time_ns()
###############################
n = 20_000_002
faster_fib_explprod_4k2only(n)
###############################
t = time.time_ns() - start_time
print(f"--- {t} nanoseconds --- ~{round(t/10000000)/100} seconds ---")
