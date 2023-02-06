import sys
sys.set_int_max_str_digits(2147483647)



def fib(n):
    if n <= 2:
        return 1
    # if n is even
    if n % 2 == 0:
        m = n >> 1
        fib_m = fib(m)
        return fib_m*(fib_m+2*fib(m-1))
    m = (n+1) >> 1
    fib_m = fib(m)
    fib_m_1 = fib(m-1)
    return fib_m*fib_m + fib_m_1*fib_m_1

def fib2(n, memo={}):
    if n in memo:
        return memo[n]
    if n <= 2:
        return 1
    if n % 2 == 0:
        m = n // 2
        fib_m = fib2(m, memo)
        result = fib_m * (fib_m + 2 * fib2(m-1, memo))
        memo[n] = result
        return result
    else:
        m = (n + 1) // 2
        fib_m = fib2(m, memo)
        fib_m_1 = fib2(m-1, memo)
        result = fib_m * fib_m + fib_m_1 * fib_m_1
        memo[n] = result
        return result

def main():
    # get command line arguments
    args = sys.argv[1:]
    if args[0] == '1':
        fib(int(args[1]))
    elif args[0] == '2':
        fib2(int(args[1]))
    print("done")

if __name__ == '__main__':
    main()